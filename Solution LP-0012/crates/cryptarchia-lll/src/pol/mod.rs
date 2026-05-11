pub mod backend;
pub mod blind;
pub mod commitment;
pub mod difficulty;
pub mod engine;
pub mod proof;
pub mod types;
pub mod verifier;
pub mod vrf;

pub use backend::{LeadershipBackend, LeadershipBackendError, SignatureTranscriptBackend};
pub use blind::{blind_slot_nonce, blind_ticket_nonce, blind_transcript_key};
pub use commitment::{
    commitment_bytes, commitment_u128, commitment_u64, domain_hash32, hash_bytes32, hash_concat32,
};
pub use difficulty::{
    estimate_total_active_stake, next_threshold, threshold_from_target_ppm, DifficultyEstimator,
    DifficultyPolicy, LotteryDifficulty, TotalStakeEstimate,
};
pub use engine::ProofOfLeadershipEngine;
pub use proof::{proof_challenge, proof_commitment, proof_witness_commitment};
pub use types::{
    EpochLeadershipState, LeaderIdentityCommitment, LeaderSlotCommitment, LeaderSlotContext,
    LeadershipClaimStatus, LeadershipClaimV2, LeadershipProofV2, LeadershipPublicInputsV2,
    LeadershipWitnessV2, ProofOfLeadershipConfig, ProofOfLeadershipWitnessDigest,
    ProposalCommitment,
};
pub use verifier::{verify_leadership_claim, verify_public_inputs_only};
pub use vrf::{derive_epoch_vrf_keypair, verify_vrf, LeadershipVrfKeypair, LeadershipVrfProof};

#[cfg(feature = "std")]
pub use commitment::commitment_hex;
