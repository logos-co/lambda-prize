use crate::pol::commitment::hash_bytes32;
use crate::pol::types::{
    LeadershipProofV2, LeadershipPublicInputsV2, LeadershipWitnessV2,
};

pub fn proof_witness_commitment(witness: &LeadershipWitnessV2) -> [u8; 32] {
    hash_bytes32(
        "cryptarchia/pol/v2/witness",
        &[
            &witness.leader_secret,
            &witness.epoch_vrf_secret,
            &witness.blinded_slot_nonce,
            &witness.stake.to_le_bytes(),
            &witness.total_stake.to_le_bytes(),
            &witness.threshold.to_le_bytes(),
            &witness.ticket,
            &witness.vrf_output,
            &witness.vrf_proof,
            &witness.leader_identity_commitment,
            &witness.stake_commitment,
            &witness.total_stake_commitment,
            &witness.threshold_commitment,
            &witness.ticket_commitment,
        ],
    )
}

pub fn proof_challenge(
    public_inputs: &LeadershipPublicInputsV2,
    witness_commitment: &[u8; 32],
) -> [u8; 32] {
    hash_bytes32(
        "cryptarchia/pol/v2/challenge",
        &[
            &public_inputs.version.to_le_bytes(),
            &public_inputs.chain_id.to_le_bytes(),
            &public_inputs.epoch_id.to_le_bytes(),
            &public_inputs.slot.to_le_bytes(),
            &public_inputs.epoch_seed,
            &public_inputs.previous_epoch_output_commitment,
            &public_inputs.beacon_seed,
            &public_inputs.validator_root,
            &public_inputs.stake_root,
            &public_inputs.leader_identity_commitment,
            &public_inputs.slot_nonce_commitment,
            &public_inputs.threshold_commitment,
            &public_inputs.ticket_commitment,
            &public_inputs.proposal_commitment,
            witness_commitment,
        ],
    )
}

pub fn proof_commitment(
    public_inputs: &LeadershipPublicInputsV2,
    witness_commitment: &[u8; 32],
    challenge_digest: &[u8; 32],
) -> [u8; 32] {
    hash_bytes32(
        "cryptarchia/pol/v2/proof-commitment",
        &[
            &public_inputs.version.to_le_bytes(),
            &public_inputs.chain_id.to_le_bytes(),
            &public_inputs.epoch_id.to_le_bytes(),
            &public_inputs.slot.to_le_bytes(),
            &public_inputs.epoch_seed,
            &public_inputs.previous_epoch_output_commitment,
            &public_inputs.beacon_seed,
            &public_inputs.validator_root,
            &public_inputs.stake_root,
            &public_inputs.leader_identity_commitment,
            &public_inputs.slot_nonce_commitment,
            &public_inputs.threshold_commitment,
            &public_inputs.ticket_commitment,
            &public_inputs.proposal_commitment,
            witness_commitment,
            challenge_digest,
        ],
    )
}

pub trait LeadershipBackend {
    fn prove(
        &self,
        public_inputs: &LeadershipPublicInputsV2,
        witness: &LeadershipWitnessV2,
    ) -> Result<LeadershipProofV2, LeadershipBackendError>;

    fn verify(
        &self,
        public_inputs: &LeadershipPublicInputsV2,
        proof: &LeadershipProofV2,
    ) -> Result<bool, LeadershipBackendError>;
}

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum LeadershipBackendError {
    #[error("invalid proof version")]
    InvalidVersion,
    #[error("signature verification failed")]
    SignatureFailed,
    #[error("challenge mismatch")]
    ChallengeMismatch,
    #[error("proof commitment mismatch")]
    CommitmentMismatch,
}

pub struct SignatureTranscriptBackend;

impl LeadershipBackend for SignatureTranscriptBackend {
    fn prove(
        &self,
        public_inputs: &LeadershipPublicInputsV2,
        witness: &LeadershipWitnessV2,
    ) -> Result<LeadershipProofV2, LeadershipBackendError> {
        let witness_commitment = proof_witness_commitment(witness);
        let challenge_digest = proof_challenge(public_inputs, &witness_commitment);

        Ok(LeadershipProofV2 {
            version: public_inputs.version,
            public_key: witness.epoch_vrf_secret,
            witness_commitment,
            challenge_digest,
            signature: witness.vrf_proof,
        })
    }

    fn verify(
        &self,
        public_inputs: &LeadershipPublicInputsV2,
        proof: &LeadershipProofV2,
    ) -> Result<bool, LeadershipBackendError> {
        if proof.version != public_inputs.version {
            return Err(LeadershipBackendError::InvalidVersion);
        }

        let expected_challenge = proof_challenge(public_inputs, &proof.witness_commitment);
        if expected_challenge != proof.challenge_digest {
            return Err(LeadershipBackendError::ChallengeMismatch);
        }

        Ok(true)
    }
}
