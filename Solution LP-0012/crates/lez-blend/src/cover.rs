/// Improvement C — Poisson-rate cover traffic generator.
///
/// Cover (dummy) traffic prevents a passive observer from using silence
/// (no packets) to infer that a node is *not* proposing.  This module
/// provides a `CoverEngine` that generates indistinguishable dummy
/// `SphinxPacket`s at a configurable Poisson rate.
///
/// # Design
///
/// - Packet size is identical to real packets (`SPHINX_PACKET_SIZE` bytes).
/// - Dummy payloads are random bytes encrypted with ephemeral keys so they
///   look identical to real ciphertexts on the wire.
/// - Interval between dummy emissions follows Exp(rate_hz) — the
///   memoryless distribution that is hardest to fingerprint.
/// - A `should_emit()` helper takes elapsed milliseconds since the last
///   emission and returns `true` when the expected emission interval has passed,
///   making it trivial to integrate into an async event loop.
///
/// # Integration
///
/// ```rust,ignore
/// let mut engine = CoverEngine::new(CoverConfig::default());
/// loop {
///     let elapsed_ms = timer.elapsed_ms();
///     if engine.should_emit(elapsed_ms) {
///         let pkt = engine.next_packet(&available_nodes)?;
///         send_to_random_mix(pkt).await?;
///         engine.reset_timer();
///     }
///     tokio::time::sleep(Duration::from_millis(10)).await;
/// }
/// ```
use alloc::vec::Vec;
extern crate alloc;

use rand::{rngs::SmallRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::{
    delay::{ExponentialDelay, DelayStrategy},
    error::{BlendError, BlendResult},
    sphinx::{sphinx_wrap, SphinxHop, SphinxPacket, SPHINX_PAYLOAD_SIZE},
};

/// Configuration for the cover traffic engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverConfig {
    /// Target emission rate in packets per second (λ for the Poisson process).
    pub rate_hz: f64,
    /// Minimum hop count for dummy packets (≥ 1).
    pub min_hops: usize,
    /// Maximum hop count for dummy packets (≤ SPHINX_MAX_HOPS).
    pub max_hops: usize,
    /// Whether cover traffic is globally enabled.
    pub enabled: bool,
}

impl Default for CoverConfig {
    fn default() -> Self {
        Self {
            rate_hz: 1.0,
            min_hops: 2,
            max_hops: 3,
            enabled: true,
        }
    }
}

/// A live cover-traffic engine.
pub struct CoverEngine {
    pub config: CoverConfig,
    delay: ExponentialDelay,
    /// Milliseconds since the last emission (monotonically set by the caller).
    elapsed_since_last_ms: u64,
    /// The next scheduled emission delay drawn from the distribution.
    next_emission_ms: u64,
    rng: SmallRng,
}

impl CoverEngine {
    pub fn new(config: CoverConfig) -> Self {
        let mean_ms = if config.rate_hz > 0.0 {
            1000.0 / config.rate_hz
        } else {
            1000.0
        };
        let mut delay = ExponentialDelay::new(mean_ms, 1, 60_000);
        let next_emission_ms = delay.next_delay_ms();
        Self {
            config,
            delay,
            elapsed_since_last_ms: 0,
            next_emission_ms,
            rng: SmallRng::from_entropy(),
        }
    }

    /// Inform the engine that `delta_ms` milliseconds have elapsed.
    /// Returns `true` if a cover packet should be emitted *now*.
    pub fn tick(&mut self, delta_ms: u64) -> bool {
        if !self.config.enabled {
            return false;
        }
        self.elapsed_since_last_ms = self.elapsed_since_last_ms.saturating_add(delta_ms);
        self.elapsed_since_last_ms >= self.next_emission_ms
    }

    /// Reset the emission timer after a packet (real or cover) has been sent.
    pub fn reset_timer(&mut self) {
        self.elapsed_since_last_ms = 0;
        self.next_emission_ms = self.delay.next_delay_ms();
    }

    /// Generate a single dummy `SphinxPacket` routed through randomly-chosen
    /// nodes from `available_nodes`.
    ///
    /// The packet has a random plaintext payload of exactly
    /// `SPHINX_PAYLOAD_SIZE - 16 * hop_count` bytes so it is indistinguishable
    /// from a real packet at the network layer.
    pub fn next_packet(&mut self, available_nodes: &[SphinxHop]) -> BlendResult<SphinxPacket> {
        if available_nodes.is_empty() {
            return Err(BlendError::EmptyHops);
        }

        let max_hops = self.config.max_hops.min(available_nodes.len());
        let min_hops = self.config.min_hops.min(max_hops).max(1);
        let hop_count = if min_hops == max_hops {
            min_hops
        } else {
            min_hops + (self.rng.next_u64() as usize % (max_hops - min_hops + 1))
        };

        // Pick hop_count distinct nodes uniformly at random.
        let hops = sample_hops(available_nodes, hop_count, &mut self.rng);

        // Fill payload with random bytes (indistinguishable from real ciphertext).
        let max_pt = SPHINX_PAYLOAD_SIZE.saturating_sub(16 * hop_count);
        let mut payload = alloc::vec![0u8; max_pt];
        self.rng.fill_bytes(&mut payload);

        sphinx_wrap(&hops, &payload, &mut self.rng)
    }

    /// Generate `n` cover packets in one call.
    pub fn generate_batch(
        &mut self,
        available_nodes: &[SphinxHop],
        n: usize,
    ) -> BlendResult<Vec<SphinxPacket>> {
        (0..n).map(|_| self.next_packet(available_nodes)).collect()
    }
}

/// Sample `k` distinct hops from `nodes` using Fisher-Yates partial shuffle.
fn sample_hops(nodes: &[SphinxHop], k: usize, rng: &mut SmallRng) -> Vec<SphinxHop> {
    let mut indices: Vec<usize> = (0..nodes.len()).collect();
    let k = k.min(nodes.len());
    for i in 0..k {
        let j = i + (rng.next_u64() as usize % (nodes.len() - i));
        indices.swap(i, j);
    }
    indices[..k].iter().map(|&i| nodes[i].clone()).collect()
}
