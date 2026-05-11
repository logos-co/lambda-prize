use crate::pol::proof::{proof_commitment, LeadershipBackend, LeadershipBackendError};
use crate::pol::types::{LeadershipClaimV2, LeadershipPublicInputsV2};

pub fn verify_public_inputs_only(public_inputs: &LeadershipPublicInputsV2) -> bool {
    public_inputs.version > 0
        && public_inputs.chain_id != 0
        && public_inputs.epoch_seed != [0u8; 32]
        && public_inputs.validator_root != [0u8; 32]
        && public_inputs.stake_root != [0u8; 32]
}

pub fn verify_leadership_claim<B: LeadershipBackend>(
    backend: &B,
    claim: &LeadershipClaimV2,
) -> Result<(), LeadershipBackendError> {
    if !verify_public_inputs_only(&claim.public_inputs) {
        return Err(LeadershipBackendError::ChallengeMismatch);
    }

    let proof_ok = backend.verify(&claim.public_inputs, &claim.proof)?;
    if !proof_ok {
        return Err(LeadershipBackendError::SignatureFailed);
    }

    let expected = proof_commitment(
        &claim.public_inputs,
        &claim.proof.witness_commitment,
        &claim.proof.challenge_digest,
    );

    if expected != claim.claim_commitment {
        return Err(LeadershipBackendError::CommitmentMismatch);
    }

    Ok(())
}
