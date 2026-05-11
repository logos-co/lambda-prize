use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{types::StakeWeight, LllError, LllResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StakeAccount {
    pub owner_commitment: [u8; 32],
    pub stake: u128,
    pub online: bool,
    pub jailed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatorRecord {
    pub node_commitment: [u8; 32],
    pub stake: u128,
    pub online: bool,
    pub participating: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidatorRoot {
    pub root: [u8; 32],
    pub total_stake: u128,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StakeTable {
    pub validators: Vec<ValidatorRecord>,
}

impl StakeTable {
    pub fn total_stake(&self) -> u128 {
        self.validators
            .iter()
            .filter(|v| v.online && v.participating && v.stake > 0)
            .map(|v| v.stake)
            .fold(0u128, |acc, x| acc.saturating_add(x))
    }

    pub fn active_count(&self) -> usize {
        self.validators
            .iter()
            .filter(|v| v.online && v.participating && v.stake > 0)
            .count()
    }

    pub fn validator_root(&self) -> ValidatorRoot {
        let mut bytes = Vec::new();
        for v in &self.validators {
            bytes.extend_from_slice(&v.node_commitment);
            bytes.extend_from_slice(&v.stake.to_le_bytes());
            bytes.push(v.online as u8);
            bytes.push(v.participating as u8);
        }
        let root = crate::crypto::hash32(&bytes);
        ValidatorRoot {
            root,
            total_stake: self.total_stake(),
            count: self.active_count(),
        }
    }

    pub fn sort_by_stake_desc(&mut self) {
        self.validators.sort_by(|a, b| b.stake.cmp(&a.stake));
    }

    pub fn validate(&self) -> LllResult<()> {
        if self.validators.is_empty() {
            return Err(LllError::NoValidators);
        }
        if self.total_stake() == 0 {
            return Err(LllError::ZeroTotalStake);
        }
        Ok(())
    }

    pub fn weight_of(&self, node_commitment: &[u8; 32]) -> Option<StakeWeight> {
        self.validators
            .iter()
            .find(|v| &v.node_commitment == node_commitment)
            .map(|v| StakeWeight {
                raw: v.stake,
                effective: v.stake,
            })
    }
}

pub fn stake_probability_ppm(stake: u128, total_stake: u128) -> u64 {
    if stake == 0 || total_stake == 0 {
        return 0;
    }
    let ppm = stake.saturating_mul(1_000_000u128) / total_stake;
    ppm.min(u64::MAX as u128) as u64
}

/// Compute the per-node lottery threshold as `(stake / total_stake) * base_threshold`.
///
/// Done without overflow by applying the Euclidean decomposition:
///   base_threshold = q * total_stake + r
///   result = q * stake + (r * stake) / total_stake
pub fn effective_leader_threshold(
    stake: u128,
    total_stake: u128,
    base_threshold: u128,
) -> u128 {
    if total_stake == 0 || stake == 0 {
        return 0;
    }
    // Euclidean split avoids overflow in the common case where
    // base_threshold and stake are both < u64::MAX.
    let q = base_threshold / total_stake;
    let r = base_threshold % total_stake;
    let main = q.saturating_mul(stake);
    // Correction term: r * stake / total_stake, where r < total_stake so no
    // overflow as long as stake <= u128::MAX / r, which holds when
    // r <= u64::MAX (the typical case).  For edge cases saturating_mul is safe.
    let correction = r.saturating_mul(stake) / total_stake;
    main.saturating_add(correction).min(base_threshold)
}
