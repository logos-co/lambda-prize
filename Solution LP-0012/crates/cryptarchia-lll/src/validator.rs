use serde::{Deserialize, Serialize};

use crate::{stake::StakeTable, LllError, LllResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorHealth {
    pub node_commitment: [u8; 32],
    pub online: bool,
    pub participating: bool,
    pub stake: u128,
}

pub fn validate_validator_table(table: &StakeTable) -> LllResult<()> {
    if table.validators.is_empty() {
        return Err(LllError::NoValidators);
    }
    if table.total_stake() == 0 {
        return Err(LllError::ZeroTotalStake);
    }
    Ok(())
}
