/// Epoch VRF chaining and blinded slot nonce derivation.
///
/// ## Problem
/// If a node uses the raw slot number as VRF input, an adversary could
/// ask the node to "prove" leadership for any future slot, potentially
/// recovering the secret key through repeated queries.
///
/// ## Solution (per the March 2025 PoL v2 design)
/// Each epoch, the VRF input is derived from:
///   blinded_nonce = H(prev_epoch_vrf_out || local_secret || slot)
///
/// The `prev_epoch_vrf_out` is committed on-chain, binding the node to
/// its output.  The `local_secret` is never revealed, so an adversary
/// cannot ask the node to evaluate the VRF at an arbitrary slot without
/// first learning the local secret.
use serde::{Deserialize, Serialize};

use crate::{
    crypto::hash32,
    types::{ChainId, EpochId, Slot},
    vrf::{vrf_prove, VrfOutput, VrfProof},
    LllResult,
};

use crate::crypto::NodeSecret;

const DOMAIN_BLINDING: &[u8] = b"cryptarchia/blinded-nonce-v1";
const DOMAIN_EPOCH_ADVANCE: &[u8] = b"cryptarchia/epoch-vrf-advance-v1";

/// A node's per-epoch state for blinded leadership evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochVrfChain {
    pub chain_id: ChainId,
    /// The current epoch whose VRF output is stored.
    pub epoch_id: EpochId,
    /// VRF output for `epoch_id`.  Used as randomness seed for the next epoch.
    pub epoch_vrf_out: [u8; 32],
    /// Local per-node secret blinding factor.
    /// Never transmitted; only the hash commitment to it is shared.
    pub local_blinding_secret: [u8; 32],
}

impl EpochVrfChain {
    /// Initialise from a genesis beacon seed, unique per node (from NodeSecret).
    pub fn genesis(
        chain_id: ChainId,
        genesis_seed: &[u8; 32],
        node_secret: &NodeSecret,
    ) -> Self {
        let mut data = Vec::new();
        data.extend_from_slice(DOMAIN_EPOCH_ADVANCE);
        data.extend_from_slice(genesis_seed);
        data.extend_from_slice(&node_secret.secret);
        data.extend_from_slice(&0u64.to_le_bytes()); // epoch 0
        let epoch_vrf_out = hash32(&data);

        let mut lb = Vec::new();
        lb.extend_from_slice(b"cryptarchia/local-blinding");
        lb.extend_from_slice(&node_secret.secret);
        lb.extend_from_slice(genesis_seed);
        let local_blinding_secret: [u8; 32] = hash32(&lb);

        Self {
            chain_id,
            epoch_id: 0,
            epoch_vrf_out,
            local_blinding_secret,
        }
    }

    /// Advance to `new_epoch_id`.
    ///
    /// The new epoch VRF output is derived from the previous one via VRF
    /// evaluation, so every epoch's randomness is publicly verifiable while
    /// remaining unpredictable before the epoch boundary.
    pub fn advance(
        &mut self,
        new_epoch_id: EpochId,
        node_secret: &NodeSecret,
    ) -> LllResult<EpochAdvanceProof> {
        let alpha = epoch_advance_alpha(self.chain_id, self.epoch_id, &self.epoch_vrf_out);
        let (vrf_out, vrf_proof) = vrf_prove(node_secret, &alpha);

        let proof = EpochAdvanceProof {
            chain_id: self.chain_id,
            prev_epoch_id: self.epoch_id,
            new_epoch_id,
            prev_vrf_out: self.epoch_vrf_out,
            new_vrf_out: vrf_out,
            vrf_proof,
        };

        self.epoch_id = new_epoch_id;
        self.epoch_vrf_out = vrf_out.0;
        Ok(proof)
    }

    /// Derive the blinded nonce for a given slot.
    ///
    /// The nonce is a deterministic function of:
    ///   - the current epoch's committed VRF output (public, verifiable)
    ///   - the node's local blinding secret   (private, never transmitted)
    ///   - the slot number
    ///
    /// This means the leader cannot be forced to evaluate the VRF at a future
    /// slot without also revealing its `local_blinding_secret`.
    pub fn blinded_nonce(&self, slot: Slot) -> [u8; 32] {
        derive_blinded_nonce(&self.epoch_vrf_out, &self.local_blinding_secret, slot)
    }
}

/// Derive a blinded slot nonce.
pub fn derive_blinded_nonce(
    epoch_vrf_out: &[u8; 32],
    local_secret: &[u8; 32],
    slot: Slot,
) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(DOMAIN_BLINDING);
    data.extend_from_slice(epoch_vrf_out);
    data.extend_from_slice(local_secret);
    data.extend_from_slice(&slot.to_le_bytes());
    hash32(&data)
}

/// Construct the alpha string fed into the VRF when advancing epochs.
pub fn epoch_advance_alpha(
    chain_id: ChainId,
    current_epoch: EpochId,
    current_vrf_out: &[u8; 32],
) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(DOMAIN_EPOCH_ADVANCE);
    v.extend_from_slice(&chain_id.to_le_bytes());
    v.extend_from_slice(&current_epoch.to_le_bytes());
    v.extend_from_slice(current_vrf_out);
    v
}

/// Proof that an epoch VRF advance was computed correctly.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpochAdvanceProof {
    pub chain_id: ChainId,
    pub prev_epoch_id: EpochId,
    pub new_epoch_id: EpochId,
    pub prev_vrf_out: [u8; 32],
    pub new_vrf_out: VrfOutput,
    pub vrf_proof: VrfProof,
}

use alloc::vec::Vec;
