/// Improvement D — VRF-based Sybil-resistant mix node selection.
///
/// In a naive mixnet, a proposer picks relay nodes randomly.  An adversary
/// who controls many peers can bias the selection (Sybil attack) and gain
/// a view of the full path.  This module replaces random selection with a
/// **Verifiable Random Function** (VRF) keyed to the proposer's private key
/// and the current slot nonce, so:
///
/// 1. **Determinism** — given a nonce, the same node always picks the same path.
/// 2. **Unpredictability** — without the proposer's VRF secret, the path is
///    pseudorandom and cannot be predicted before the VRF proof is revealed.
/// 3. **Verifiability** — the path choice is publicly verifiable after the fact
///    via the VRF proof, enabling slashing / audit.
/// 4. **Stake-weighting** — nodes are weighted by their declared stake so
///    well-capitalised honest nodes appear more often than cheap Sybil nodes.
///
/// # Integration with `cryptarchia-lll`
///
/// This module reuses `cryptarchia_lll::vrf_prove` (Ristretto255-SHA512) so
/// both the leadership lottery and the mixnet path use the same cryptographic
/// primitive.
use alloc::vec::Vec;
extern crate alloc;

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use cryptarchia_lll::{vrf_prove, NodeSecret};

use crate::error::{BlendError, BlendResult};
use crate::sphinx::SphinxHop;

/// Metadata for a candidate mix node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MixNode {
    /// Unique 32-byte node identifier (e.g., truncated public key hash).
    pub id: [u8; 32],
    /// X25519 public key used for Sphinx packet encryption.
    pub x25519_public_key: [u8; 32],
    /// Stake weight (higher = selected more often in stake-weighted draw).
    pub stake_weight: u64,
    /// Optional human-readable label for telemetry.
    pub label: alloc::string::String,
}

impl MixNode {
    pub fn new(id: [u8; 32], x25519_public_key: [u8; 32], stake_weight: u64) -> Self {
        Self {
            id,
            x25519_public_key,
            stake_weight,
            label: alloc::string::String::new(),
        }
    }

    /// Convert to a `SphinxHop` for packet construction.
    pub fn to_sphinx_hop(&self) -> SphinxHop {
        let mut routing_id = [0u8; 16];
        routing_id.copy_from_slice(&self.id[..16]);
        SphinxHop {
            public_key: self.x25519_public_key,
            routing_id,
        }
    }
}

/// Result of a VRF path selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedPath {
    /// The ordered list of chosen mix nodes.
    pub hops: Vec<MixNode>,
    /// The VRF output (beta) used as entropy for this selection.
    pub vrf_output: [u8; 32],
    /// The VRF proof; peers can verify the path choice was honestly derived.
    pub vrf_proof_bytes: Vec<u8>,
    /// The nonce that was signed (slot-specific or proposal-specific).
    pub nonce: Vec<u8>,
}

/// VRF-based mix node path selector.
pub struct VrfMixSelector;

impl VrfMixSelector {
    /// Select `path_length` mix nodes from `candidates` using the proposer's
    /// VRF key and a slot/proposal-specific `nonce`.
    ///
    /// # Nonce construction
    ///
    /// Callers should pass `nonce = H(chain_id || epoch_id || slot || proposal_id)`
    /// so the path is bound to a specific proposal and cannot be precomputed.
    ///
    /// # Stake-weighted sampling
    ///
    /// Nodes are selected without replacement using the **A-Res** (reservoir
    /// sampling with weights) algorithm.  Each node's selection key is
    /// `VRF_output_i = BLAKE3(vrf_beta || node.id || i)` and its effective
    /// score is `log(key) / stake_weight` — nodes with higher stake appear
    /// more often in expectation.
    pub fn select_path(
        candidates: &[MixNode],
        node_secret: &NodeSecret,
        nonce: &[u8],
        path_length: usize,
    ) -> BlendResult<SelectedPath> {
        if candidates.is_empty() || path_length == 0 {
            return Err(BlendError::InsufficientMixNodes {
                needed: path_length,
                have: candidates.len(),
            });
        }
        if candidates.len() < path_length {
            return Err(BlendError::InsufficientMixNodes {
                needed: path_length,
                have: candidates.len(),
            });
        }

        // 1. Run VRF over the nonce to get the selection seed.
        let (vrf_out, vrf_proof) = vrf_prove(node_secret, nonce);

        // 2. Derive a per-node score using BLAKE3(vrf_beta || node_id).
        //    Score = uniform u64 / stake_weight → lower score = higher priority.
        let mut scored: Vec<(u64, usize)> = candidates
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let score = derive_node_score(&vrf_out.0, &node.id, i as u64, node.stake_weight);
                (score, i)
            })
            .collect();

        // 3. Sort ascending by score → first `path_length` entries win.
        scored.sort_unstable_by_key(|&(score, _)| score);

        let hops: Vec<MixNode> = scored[..path_length]
            .iter()
            .map(|(_, i)| candidates[*i].clone())
            .collect();

        // Serialise the proof for the `SelectedPath` struct.
        let mut proof_bytes = Vec::new();
        proof_bytes.extend_from_slice(&vrf_proof.c);
        proof_bytes.extend_from_slice(&vrf_proof.s);
        proof_bytes.extend_from_slice(&vrf_proof.gamma);

        Ok(SelectedPath {
            hops,
            vrf_output: vrf_out.0,
            vrf_proof_bytes: proof_bytes,
            nonce: nonce.to_vec(),
        })
    }

    /// Convert a `SelectedPath` to the `SphinxHop` list needed by `sphinx_wrap`.
    pub fn to_sphinx_hops(path: &SelectedPath) -> Vec<SphinxHop> {
        path.hops.iter().map(|n| n.to_sphinx_hop()).collect()
    }
}

/// Compute a per-node selection score.
///
/// Returns a u64 in `[0, u64::MAX / stake_weight]`.  Lower scores are
/// selected first; stake weighting means heavy nodes get lower expected scores.
fn derive_node_score(vrf_beta: &[u8; 32], node_id: &[u8; 32], index: u64, stake_weight: u64) -> u64 {
    let mut h = Hasher::new_keyed(vrf_beta);
    h.update(node_id);
    h.update(&index.to_le_bytes());
    h.update(b"blend/mix-select-v1");
    let digest = h.finalize();
    let raw = u64::from_le_bytes(digest.as_bytes()[..8].try_into().unwrap());
    // Divide by stake_weight so heavier nodes get smaller scores on average.
    raw / stake_weight.max(1)
}
