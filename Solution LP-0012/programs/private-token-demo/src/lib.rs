use borsh::{BorshDeserialize, BorshSerialize};
use lez_privacy::{
    apply_private_transfer, commitment_bytes, generate_nullifier_hex,
    AccessDecision, AccessPolicy, AccessPolicySet, CommitmentDomain, NullifierDomain,
    PrivacyError, PrivacyReceipt, ShieldedAccount, ShieldedLedger,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Instruction {
    Mint {
        owner_seed: Vec<u8>,
        asset_id:   String,
        amount:     u128,
        salt:       String,
    },
    Transfer {
        from_owner_secret:    Vec<u8>,
        from_owner_seed:      Vec<u8>,
        asset_id:             String,
        to_owner_commitment:  String,
        amount:               u128,
        salt:                 String,
    },
    RevealBalance {
        subject:          String,
        owner_commitment: String,
        asset_id:         String,
    },
    SpendNullifier {
        secret:     Vec<u8>,
        commitment: Vec<u8>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DemoState {
    pub ledger:   ShieldedLedger,
    pub receipts: Vec<PrivacyReceipt>,
}

impl DemoState {
    pub fn new() -> Self {
        Self { ledger: ShieldedLedger::default(), receipts: Vec::new() }
    }
}

impl Default for DemoState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn process_instruction(
    state: &mut DemoState,
    instruction: Instruction,
) -> Result<PrivacyReceipt, PrivacyError> {
    match instruction {
        Instruction::Mint { owner_seed, asset_id, amount, salt } => {
            let account = ShieldedAccount::new(&owner_seed, asset_id.clone(), amount, salt);
            state.ledger.add_account(account.clone());

            let receipt = PrivacyReceipt::success(format!("mint-{}", account.owner_commitment))
                .add_commitment(account.owner_commitment.clone())
                .add_commitment(account.balance_commitment.clone())
                .finalize();

            state.receipts.push(receipt.clone());
            Ok(receipt)
        }

        Instruction::Transfer {
            from_owner_secret,
            from_owner_seed,
            asset_id,
            to_owner_commitment,
            amount,
            salt,
        } => {
            let from_owner_commitment = hex::encode(commitment_bytes(
                CommitmentDomain::NOTE,
                &[from_owner_seed.as_slice(), asset_id.as_bytes(), salt.as_bytes()].concat(),
            ));

            let pos = state
                .ledger
                .accounts
                .iter()
                .position(|a| {
                    a.owner_commitment == from_owner_commitment && a.asset_id == asset_id
                })
                .ok_or_else(|| PrivacyError::InvalidEnvelope("source account not found".into()))?;

            let mut from_account = state.ledger.accounts[pos].clone();
            let transfer_receipt = apply_private_transfer(
                &mut state.ledger,
                &from_owner_secret,
                &mut from_account,
                &to_owner_commitment,
                amount,
                b"transfer-note",
            )?;
            state.ledger.accounts[pos] = from_account;

            let receipt = PrivacyReceipt::success("transfer")
                .add_commitment(transfer_receipt.transfer_commitment.clone())
                .add_nullifier(transfer_receipt.nullifier.clone())
                .finalize();

            state.receipts.push(receipt.clone());
            Ok(receipt)
        }

        Instruction::RevealBalance { subject, owner_commitment, asset_id } => {
            let account = state
                .ledger
                .find_account_by_owner(&owner_commitment, &asset_id)
                .ok_or_else(|| PrivacyError::InvalidEnvelope("account not found".into()))?;

            let policy = AccessPolicySet::allow_all_read();
            match policy.decide(
                &subject,
                lez_privacy::policy::PolicyAction::Read,
                Some(&asset_id),
            ) {
                AccessDecision::Allowed => {
                    let receipt = PrivacyReceipt::success("reveal")
                        .add_commitment(account.balance_commitment.clone())
                        .finalize();
                    state.receipts.push(receipt.clone());
                    Ok(receipt)
                }
                AccessDecision::Denied { reason } => {
                    let receipt = PrivacyReceipt::failed("reveal", reason).finalize();
                    state.receipts.push(receipt.clone());
                    Ok(receipt)
                }
            }
        }

        Instruction::SpendNullifier { secret, commitment } => {
            let nullifier =
                generate_nullifier_hex(&secret, NullifierDomain::SPEND, &commitment);
            if state.ledger.spent_nullifiers.iter().any(|n| n == &nullifier) {
                return Err(PrivacyError::NullifierAlreadySpent);
            }
            state.ledger.spent_nullifiers.push(nullifier.clone());

            let receipt = PrivacyReceipt::success("spend-nullifier")
                .add_nullifier(nullifier)
                .finalize();
            state.receipts.push(receipt.clone());
            Ok(receipt)
        }
    }
}
