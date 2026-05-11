/// Epoch-adaptive stake estimator for lottery difficulty relativization.
///
/// ## Problem
/// A node only knows its own stake and the total_stake field in the
/// stake table — but the table may be stale or adversarially manipulated.
/// If an attacker can artificially inflate `total_stake`, they reduce
/// everyone else's win probability.
///
/// ## Solution (from "Lottery Difficulty in Private PoS: The Case of Cryptarchia")
/// Each node tracks its observed win rate over a sliding window.
/// If actual wins deviate from the expected rate (stake/total_stake),
/// the node adjusts the effective threshold it uses internally.
/// This relativizes difficulty without requiring other nodes to reveal stakes.
use serde::{Deserialize, Serialize};

use crate::stake::stake_probability_ppm;

/// Configuration for the stake estimator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatorConfig {
    /// Exponential moving average decay: 0 < α ≤ 1 (fixed-point: scale by 1_000_000).
    /// Higher values react faster to changes; lower values smooth more.
    /// Recommended: 100_000 (= 0.1 per epoch).
    pub ema_alpha_ppm: u64,

    /// Minimum number of slots observed before the estimator starts adjusting.
    pub warmup_slots: u64,

    /// Maximum ratio by which the estimated threshold can be scaled up or down
    /// relative to the stake-table threshold.  Fixed-point, scale 1_000_000.
    /// E.g., 500_000 means the threshold can deviate ±50%.
    pub max_adjustment_ppm: u64,
}

impl Default for EstimatorConfig {
    fn default() -> Self {
        Self {
            ema_alpha_ppm: 100_000,
            warmup_slots: 128,
            max_adjustment_ppm: 300_000,
        }
    }
}

/// Running statistics for one epoch.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct EpochStats {
    evaluated_slots: u64,
    wins: u64,
}

impl EpochStats {
    fn win_rate_ppm(&self) -> u64 {
        if self.evaluated_slots == 0 {
            return 0;
        }
        self.wins
            .saturating_mul(1_000_000)
            .saturating_div(self.evaluated_slots)
    }
}

/// Epoch-adaptive stake estimator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochStakeEstimator {
    pub config: EstimatorConfig,
    /// EMA of the observed win-rate (parts-per-million).
    pub ema_win_rate_ppm: u64,
    /// The stake claimed by this node (from the stake table).
    pub reported_stake: u128,
    /// The total stake claimed by the stake table.
    pub reported_total_stake: u128,
    /// Running stats for the current epoch.
    current: EpochStats,
    /// Total slots evaluated since genesis.
    pub total_evaluated: u64,
}

impl EpochStakeEstimator {
    pub fn new(
        config: EstimatorConfig,
        reported_stake: u128,
        reported_total_stake: u128,
    ) -> Self {
        let expected_ppm = stake_probability_ppm(reported_stake, reported_total_stake);
        Self {
            config,
            ema_win_rate_ppm: expected_ppm,
            reported_stake,
            reported_total_stake,
            current: EpochStats::default(),
            total_evaluated: 0,
        }
    }

    /// Record the outcome of evaluating a single slot.
    pub fn observe_slot(&mut self, won: bool) {
        self.current.evaluated_slots += 1;
        self.total_evaluated += 1;
        if won {
            self.current.wins += 1;
        }
    }

    /// Advance to the next epoch.
    ///
    /// Folds the current epoch's win-rate into the EMA and resets running stats.
    pub fn advance_epoch(&mut self) {
        let observed_ppm = self.current.win_rate_ppm();
        let alpha = self.config.ema_alpha_ppm;

        // EMA update: ema = alpha * observed + (1 - alpha) * ema
        self.ema_win_rate_ppm = alpha
            .saturating_mul(observed_ppm)
            .saturating_div(1_000_000)
            .saturating_add(
                (1_000_000 - alpha.min(1_000_000))
                    .saturating_mul(self.ema_win_rate_ppm)
                    .saturating_div(1_000_000),
            );

        self.current = EpochStats::default();
    }

    /// Update the reported stake values (e.g., after a stake table refresh).
    pub fn update_stake(&mut self, reported_stake: u128, reported_total_stake: u128) {
        self.reported_stake = reported_stake;
        self.reported_total_stake = reported_total_stake;
    }

    /// Expected win-rate in ppm based on the reported stake table.
    pub fn expected_win_rate_ppm(&self) -> u64 {
        stake_probability_ppm(self.reported_stake, self.reported_total_stake)
    }

    /// Adjustment ratio in ppm: `ema_win_rate / expected_win_rate`.
    ///
    /// > 1_000_000: winning more than expected → raise threshold (harder to keep winning unfairly).
    /// < 1_000_000: winning less than expected → could lower threshold, but we stay conservative.
    pub fn adjustment_ratio_ppm(&self) -> u64 {
        let expected = self.expected_win_rate_ppm();
        if expected == 0 {
            return 1_000_000;
        }
        self.ema_win_rate_ppm
            .saturating_mul(1_000_000)
            .saturating_div(expected)
    }

    /// Compute an adjusted threshold for the next slot evaluation.
    ///
    /// The threshold is scaled by the inverse of the adjustment ratio:
    /// if this node is winning too often, the threshold is reduced, making
    /// future wins harder.
    pub fn adjusted_threshold(&self, base_threshold: u128) -> u128 {
        if self.total_evaluated < self.config.warmup_slots {
            return base_threshold;
        }

        let ratio = self.adjustment_ratio_ppm();
        let max_adj = self.config.max_adjustment_ppm;

        // Clamp ratio to [1_000_000 - max_adj, 1_000_000 + max_adj]
        let clamped_ratio = ratio.clamp(
            1_000_000u64.saturating_sub(max_adj),
            1_000_000u64.saturating_add(max_adj),
        );

        // Adjust threshold inversely: higher ratio → lower threshold
        let inverse_ratio = 2_000_000u64.saturating_sub(clamped_ratio); // mirror around 1.0
        base_threshold
            .saturating_mul(inverse_ratio as u128)
            .saturating_div(1_000_000)
    }

    /// Return a summary for logging / telemetry.
    pub fn summary(&self) -> EstimatorSummary {
        EstimatorSummary {
            total_evaluated: self.total_evaluated,
            ema_win_rate_ppm: self.ema_win_rate_ppm,
            expected_win_rate_ppm: self.expected_win_rate_ppm(),
            adjustment_ratio_ppm: self.adjustment_ratio_ppm(),
        }
    }
}

/// Lightweight summary for telemetry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatorSummary {
    pub total_evaluated: u64,
    pub ema_win_rate_ppm: u64,
    pub expected_win_rate_ppm: u64,
    pub adjustment_ratio_ppm: u64,
}
