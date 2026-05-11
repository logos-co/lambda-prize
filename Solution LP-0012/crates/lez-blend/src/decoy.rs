/// Improvement E — Decoy broadcast module.
///
/// When a node receives (or sends) a real proposal, it immediately generates
/// and broadcasts a configurable number of indistinguishable **decoy packets**
/// to randomly-chosen mix nodes.  This has two effects:
///
/// 1. **Sender anonymity** — even if an adversary watches all outbound traffic
///    from the proposer's IP, it sees `fan_out + 1` simultaneous identical-size
///    packets with no way to identify which (if any) carries the real proposal.
///
/// 2. **Recipient anonymity** — observers cannot use receipt-timing to
///    distinguish the path of the real packet from the decoys.
///
/// # Configuration
///
/// `DecoyConfig::enabled` is a run-time toggle so operators can disable decoys
/// during testing without recompiling.
///
/// `fan_out` controls the number of decoy packets per real packet.  A value of
/// 5 means the real packet is 1-in-6 — sufficient for basic sender anonymity.
///
/// # Security note
///
/// Decoy packets *must* use freshly-sampled Sphinx ephemeral keys (ensured by
/// `sphinx_wrap`).  Re-using the same ephemeral key across real and decoy
/// packets would allow correlation attacks.
use alloc::vec::Vec;
extern crate alloc;

use rand::{rngs::SmallRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::{
    error::{BlendError, BlendResult},
    mix_select::{MixNode, VrfMixSelector},
    sphinx::{sphinx_wrap, SphinxHop, SphinxPacket, SPHINX_PAYLOAD_SIZE},
};
use cryptarchia_lll::NodeSecret;

/// Configuration for the decoy broadcast module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoyConfig {
    /// Toggle decoy generation at runtime.
    pub enabled: bool,
    /// Number of decoy packets generated per real packet (0 = disabled).
    pub fan_out: usize,
    /// Hop count for each decoy packet (2–5 recommended).
    pub hop_count: usize,
}

impl Default for DecoyConfig {
    fn default() -> Self {
        Self { enabled: true, fan_out: 5, hop_count: 3 }
    }
}

/// A batch of decoy packets produced for a single real packet emission.
#[derive(Debug, Clone)]
pub struct DecoyBatch {
    /// The decoy packets (len == config.fan_out when successful).
    pub packets: Vec<SphinxPacket>,
    /// The real packet (for convenience — callers bundle it with decoys).
    pub real_packet: SphinxPacket,
}

impl DecoyBatch {
    /// Combine `real_packet` and all `decoys` into a single Vec in random order.
    ///
    /// Randomising the order prevents timing-based identification of the real
    /// packet based on its position in the batch.
    pub fn shuffle_all(&self, rng: &mut SmallRng) -> Vec<SphinxPacket> {
        let mut all = self.packets.clone();
        all.push(self.real_packet.clone());
        // Fisher-Yates shuffle
        let n = all.len();
        for i in (1..n).rev() {
            let j = rng.next_u64() as usize % (i + 1);
            all.swap(i, j);
        }
        all
    }
}

/// The decoy broadcast module.
pub struct DecoyBroadcast {
    pub config: DecoyConfig,
    rng: SmallRng,
}

impl DecoyBroadcast {
    pub fn new(config: DecoyConfig) -> Self {
        Self { config, rng: SmallRng::from_entropy() }
    }

    /// Generate decoys for a real packet, using VRF-selected mix paths.
    ///
    /// Each decoy uses an independently-drawn random path (via VRF + unique nonce)
    /// and a freshly-sampled ephemeral Sphinx key so all packets are
    /// cryptographically unlinkable to each other and to the real packet.
    ///
    /// Returns `BlendResult<DecoyBatch>`.
    pub fn wrap_with_decoys(
        &mut self,
        real_packet: SphinxPacket,
        candidates: &[MixNode],
        node_secret: &NodeSecret,
        base_nonce: &[u8],
    ) -> BlendResult<DecoyBatch> {
        if !self.config.enabled || self.config.fan_out == 0 {
            return Ok(DecoyBatch { packets: Vec::new(), real_packet });
        }
        if self.config.fan_out == 0 {
            return Err(BlendError::ZeroFanOut);
        }

        let mut decoys = Vec::with_capacity(self.config.fan_out);

        for i in 0..self.config.fan_out {
            // Each decoy uses a different nonce so VRF paths diverge.
            let mut nonce = alloc::vec![0u8; base_nonce.len() + 8];
            nonce[..base_nonce.len()].copy_from_slice(base_nonce);
            nonce[base_nonce.len()..].copy_from_slice(&(i as u64).to_le_bytes());

            let path = VrfMixSelector::select_path(
                candidates,
                node_secret,
                &nonce,
                self.config.hop_count.min(candidates.len()).max(1),
            )?;
            let hops = VrfMixSelector::to_sphinx_hops(&path);

            // Decoy payload = random bytes of the same size as a real proposal.
            let max_pt = SPHINX_PAYLOAD_SIZE
                .saturating_sub(16 * hops.len())
                .max(1);
            let mut dummy_payload = alloc::vec![0u8; max_pt];
            self.rng.fill_bytes(&mut dummy_payload);

            let pkt = sphinx_wrap(&hops, &dummy_payload, &mut self.rng)?;
            decoys.push(pkt);
        }

        Ok(DecoyBatch { packets: decoys, real_packet })
    }

    /// Simpler variant: generate `n` decoys routed to arbitrary hops from a
    /// pre-built `SphinxHop` list (for callers that do their own path selection).
    pub fn generate_raw_decoys(
        &mut self,
        available_hops: &[SphinxHop],
        n: usize,
    ) -> BlendResult<Vec<SphinxPacket>> {
        if available_hops.is_empty() {
            return Err(BlendError::EmptyHops);
        }
        if n == 0 {
            return Err(BlendError::ZeroFanOut);
        }

        let hop_count = self.config.hop_count.min(available_hops.len()).max(1);
        (0..n)
            .map(|_| {
                let hops = sample_k(available_hops, hop_count, &mut self.rng);
                let max_pt = SPHINX_PAYLOAD_SIZE.saturating_sub(16 * hops.len()).max(1);
                let mut payload = alloc::vec![0u8; max_pt];
                self.rng.fill_bytes(&mut payload);
                sphinx_wrap(&hops, &payload, &mut self.rng)
            })
            .collect()
    }
}

fn sample_k(hops: &[SphinxHop], k: usize, rng: &mut SmallRng) -> Vec<SphinxHop> {
    let mut idx: Vec<usize> = (0..hops.len()).collect();
    let k = k.min(hops.len());
    for i in 0..k {
        let j = i + rng.next_u64() as usize % (hops.len() - i);
        idx.swap(i, j);
    }
    idx[..k].iter().map(|&i| hops[i].clone()).collect()
}
