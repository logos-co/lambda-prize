use alloc::vec::Vec;
use ed25519_dalek::Signer;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::{derive_epoch_keypair, hash32, verify_signature, NodeSecret},
    types::{ChainId, EpochId, LeaderAlias, NodePublicKey, Slot},
    LllError, LllResult,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeadershipPublicInputs {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub slot: Slot,
    pub beacon_seed: [u8; 32],
    pub validator_root: [u8; 32],
    pub stake: u128,
    pub total_stake: u128,
    pub threshold: u128,
    pub ticket: [u8; 32],
    pub alias: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeadershipProofHeader {
    pub version: u8,
    pub public_key: NodePublicKey,
    pub alias: LeaderAlias,
    pub witness_digest: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeadershipProof {
    pub header: LeadershipProofHeader,
    pub public_inputs: LeadershipPublicInputs,
    pub signature: Vec<u8>,
    pub challenge_digest: [u8; 32],
}

pub fn build_leadership_proof(
    node_secret: &NodeSecret,
    epoch_seed: &[u8; 32],
    public_inputs: LeadershipPublicInputs,
) -> LllResult<LeadershipProof> {
    let signing_key = derive_epoch_keypair(node_secret, epoch_seed);
    let public_key = NodePublicKey(signing_key.verifying_key().to_bytes());
    let challenge = proof_challenge(&public_inputs);
    let signature = signing_key.sign(&challenge);
    let challenge_digest = hash32(&challenge);

    Ok(LeadershipProof {
        header: LeadershipProofHeader {
            version: 1,
            public_key,
            alias: LeaderAlias(public_inputs.alias),
            witness_digest: witness_digest(node_secret, &public_inputs),
        },
        public_inputs,
        signature: signature.to_bytes().to_vec(),
        challenge_digest,
    })
}

pub fn verify_leadership_proof(proof: &LeadershipProof) -> LllResult<()> {
    if proof.header.version != 1 {
        return Err(LllError::UnsupportedVersion);
    }

    if proof.public_inputs.alias != proof.header.alias.0 {
        return Err(LllError::CommitmentMismatch);
    }

    let challenge = proof_challenge(&proof.public_inputs);
    if hash32(&challenge) != proof.challenge_digest {
        return Err(LllError::ProofVerificationFailed(
            "challenge digest mismatch".into(),
        ));
    }

    if !verify_signature(&proof.header.public_key, &challenge, &proof.signature) {
        return Err(LllError::ProofVerificationFailed(
            "signature invalid".into(),
        ));
    }

    Ok(())
}

pub fn proof_challenge(inputs: &LeadershipPublicInputs) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"cryptarchia/proof");
    out.extend_from_slice(&inputs.chain_id.to_le_bytes());
    out.extend_from_slice(&inputs.epoch_id.to_le_bytes());
    out.extend_from_slice(&inputs.slot.to_le_bytes());
    out.extend_from_slice(&inputs.beacon_seed);
    out.extend_from_slice(&inputs.validator_root);
    out.extend_from_slice(&inputs.stake.to_le_bytes());
    out.extend_from_slice(&inputs.total_stake.to_le_bytes());
    out.extend_from_slice(&inputs.threshold.to_le_bytes());
    out.extend_from_slice(&inputs.ticket);
    out.extend_from_slice(&inputs.alias);
    out
}

pub fn witness_digest(node_secret: &NodeSecret, inputs: &LeadershipPublicInputs) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&node_secret.secret);
    data.extend_from_slice(&proof_challenge(inputs));
    hash32(&data)
}
