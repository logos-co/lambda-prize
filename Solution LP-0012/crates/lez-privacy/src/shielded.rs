use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{
    commitment::{commitment_bytes, CommitmentDomain},
    nullifier::{generate_nullifier, NullifierDomain},
    policy::{AccessDecision, AccessPolicy, PolicyAction},
    PrivacyError, PrivacyResult,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShieldedBalance {
    pub owner_commitment:    String,
    pub asset_id:            String,
    pub balance_commitment:  String,
    pub spent_nullifiers:    Vec<String>,
    pub available:           u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShieldedAccount {
    pub owner_commitment:   String,
    pub asset_id:           String,
    pub balance_commitment: String,
    pub salt:               String,
    pub available:          u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShieldedTransfer {
    pub from_commitment:   String,
    pub to_commitment:     String,
    pub asset_id:          String,
    pub amount:            u128,
    pub note_commitment:   String,
    pub nullifier:         String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShieldedTransferReceipt {
    pub status:              String,
    pub nullifier:           String,
    pub balance_before:      u128,
    pub balance_after:       u128,
    pub transfer_commitment: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShieldedLedger {
    pub accounts:         Vec<ShieldedAccount>,
    pub spent_nullifiers: Vec<String>,
}

impl ShieldedAccount {
    pub fn new(
        owner_seed: &[u8],
        asset_id: impl Into<String>,
        available: u128,
        salt: impl Into<String>,
    ) -> Self {
        let asset_id = asset_id.into();
        let salt = salt.into();
        let owner_commitment = hex::encode(commitment_bytes(
            CommitmentDomain::NOTE,
            &[owner_seed, asset_id.as_bytes(), salt.as_bytes()].concat(),
        ));
        let balance_commitment =
            hex::encode(commitment_bytes(CommitmentDomain::BALANCE, &available.to_le_bytes()));

        Self { owner_commitment, asset_id, balance_commitment, salt, available }
    }

    pub fn reveal_balance_to<P: AccessPolicy>(
        &self,
        subject: &str,
        policy: &P,
    ) -> PrivacyResult<u128> {
        match policy.decide(subject, PolicyAction::Read, Some(&self.asset_id)) {
            AccessDecision::Allowed => Ok(self.available),
            AccessDecision::Denied { .. } => Err(PrivacyError::AccessDenied),
        }
    }
}

impl ShieldedLedger {
    pub fn add_account(&mut self, account: ShieldedAccount) {
        self.accounts.push(account);
    }

    pub fn find_account_by_owner(
        &self,
        owner_commitment: &str,
        asset_id: &str,
    ) -> Option<&ShieldedAccount> {
        self.accounts
            .iter()
            .find(|a| a.owner_commitment == owner_commitment && a.asset_id == asset_id)
    }

    pub fn mark_nullifier_spent(&mut self, nullifier: &str) -> PrivacyResult<()> {
        if self.spent_nullifiers.iter().any(|n| n == nullifier) {
            return Err(PrivacyError::NullifierAlreadySpent);
        }
        self.spent_nullifiers.push(nullifier.to_string());
        Ok(())
    }
}

pub fn apply_private_transfer(
    ledger: &mut ShieldedLedger,
    from_owner_secret: &[u8],
    from_account: &mut ShieldedAccount,
    to_owner_commitment: &str,
    amount: u128,
    note_seed: &[u8],
) -> PrivacyResult<ShieldedTransferReceipt> {
    if amount == 0 {
        return Err(PrivacyError::InvalidAmount);
    }
    if from_account.available < amount {
        return Err(PrivacyError::InsufficientBalance);
    }

    let before = from_account.available;
    let after  = before - amount;

    let transfer_note = commitment_bytes(
        CommitmentDomain::TRANSFER,
        &[&amount.to_le_bytes()[..], note_seed].concat(),
    );
    let nullifier = generate_nullifier(
        from_owner_secret,
        NullifierDomain::SPEND,
        from_account.balance_commitment.as_bytes(),
    );

    let transfer = ShieldedTransfer {
        from_commitment:  from_account.owner_commitment.clone(),
        to_commitment:    to_owner_commitment.to_string(),
        asset_id:         from_account.asset_id.clone(),
        amount,
        note_commitment:  hex::encode(transfer_note),
        nullifier:        hex::encode(nullifier.value),
    };

    ledger.mark_nullifier_spent(&transfer.nullifier)?;
    from_account.available = after;
    from_account.balance_commitment =
        hex::encode(commitment_bytes(CommitmentDomain::BALANCE, &after.to_le_bytes()));

    Ok(ShieldedTransferReceipt {
        status:              "ok".into(),
        nullifier:           transfer.nullifier,
        balance_before:      before,
        balance_after:       after,
        transfer_commitment: transfer.note_commitment,
    })
}
