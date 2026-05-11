use crate::pol::commitment::commitment_bytes;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LotteryDifficulty {
    pub epoch_id: u64,
    pub target_win_ppm: u64,
    pub observed_win_ppm: u64,
    pub estimated_total_stake: u128,
    pub threshold: u128,
    pub threshold_commitment: [u8; 32],
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TotalStakeEstimate {
    pub epoch_id: u64,
    pub sample_size: u64,
    pub observed_successes: u64,
    pub estimated_total_stake: u128,
    pub uncertainty_ppm: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DifficultyPolicy {
    pub target_win_ppm: u64,
    pub alpha_ppm: u64,
    pub min_threshold: u128,
    pub max_threshold: u128,
    pub floor_total_stake: u128,
}

impl Default for DifficultyPolicy {
    fn default() -> Self {
        Self {
            target_win_ppm: 25_000,
            alpha_ppm: 100_000,
            min_threshold: 1,
            max_threshold: u128::MAX,
            floor_total_stake: 1,
        }
    }
}

pub fn estimate_total_active_stake(
    sample_active_stake: u128,
    observed_win_ppm: u64,
    target_win_ppm: u64,
) -> u128 {
    if target_win_ppm == 0 {
        return sample_active_stake;
    }

    let ratio = observed_win_ppm.max(1) as u128;
    let target = target_win_ppm.max(1) as u128;

    sample_active_stake
        .saturating_mul(ratio)
        .saturating_div(target)
        .max(1)
}

pub fn threshold_from_target_ppm(total_stake: u128, target_win_ppm: u64) -> u128 {
    if total_stake == 0 {
        return 0;
    }
    total_stake.saturating_mul(target_win_ppm as u128) / 1_000_000u128
}

pub fn next_threshold(
    policy: DifficultyPolicy,
    epoch_id: u64,
    target_win_ppm: u64,
    observed_win_ppm: u64,
    sample_active_stake: u128,
) -> LotteryDifficulty {
    let estimated_total_stake = estimate_total_active_stake(
        sample_active_stake.max(policy.floor_total_stake),
        observed_win_ppm,
        target_win_ppm,
    );

    let threshold = threshold_from_target_ppm(estimated_total_stake, target_win_ppm)
        .max(policy.min_threshold)
        .min(policy.max_threshold);

    let mut threshold_data = alloc::vec::Vec::new();
    threshold_data.extend_from_slice(&epoch_id.to_le_bytes());
    threshold_data.extend_from_slice(&target_win_ppm.to_le_bytes());
    threshold_data.extend_from_slice(&observed_win_ppm.to_le_bytes());
    threshold_data.extend_from_slice(&estimated_total_stake.to_le_bytes());
    threshold_data.extend_from_slice(&threshold.to_le_bytes());
    let threshold_commitment = commitment_bytes("cryptarchia/pol/v2/threshold", &threshold_data);

    LotteryDifficulty {
        epoch_id,
        target_win_ppm,
        observed_win_ppm,
        estimated_total_stake,
        threshold,
        threshold_commitment,
    }
}

pub struct DifficultyEstimator;
