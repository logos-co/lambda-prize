#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod beacon;
pub mod blinding;
pub mod codec;
pub mod crypto;
pub mod enhanced_lottery;
pub mod error;
pub mod estimator;
pub mod lottery;
pub mod nullifier;
pub mod pol;
pub mod policy;
pub mod proof;
pub mod schedule;
#[cfg(feature = "std")]
pub mod simulator;
pub mod stake;
pub mod state;
pub mod telemetry;
pub mod types;
pub mod utils;
pub mod validator;
pub mod vrf;

pub use beacon::{BeaconMix, EpochBeacon, SlotSeed};
pub use codec::{decode_compact, encode_compact, CompactEnvelope};
pub use crypto::{
    commitment_hex, derive_alias, derive_epoch_keypair, derive_ticket, hash32, hash64,
    random_node_secret, ticket_below_threshold, NodeSecret,
};
pub use error::{LllError, LllResult};
pub use lottery::{
    LeadershipOutcome, LocalLeadershipLottery, LotteryConfig, LotteryTicket, ProposalAnnounce,
    ProposalEnvelope,
};
pub use policy::{
    CommitteePolicy, ProposalPolicy, ProposalPolicyDecision, ProposalVisibilityPolicy,
};
pub use proof::{
    build_leadership_proof, verify_leadership_proof, LeadershipProof, LeadershipProofHeader,
    LeadershipPublicInputs,
};
pub use schedule::{EpochSchedule, SlotIndex};
pub use stake::{
    effective_leader_threshold, stake_probability_ppm, StakeAccount, StakeTable, ValidatorRecord,
    ValidatorRoot,
};
pub use state::{LeadershipState, ProposalHistory, ProposalState};
pub use telemetry::{AuditEvent, AuditLevel, LotteryMetrics, LotteryTrace};
pub use types::{
    ChainId, EpochId, LeaderAlias, LeaderCommitment, LeaderIdentityHint, NodePublicKey,
    ProposalId, Slot, StakeWeight, WinnerStatus,
};
pub use utils::{
    bounded_u64, format_ppm, redact_hex, rolling_mix, stable_ratio_u128, support_preview,
};
pub use validator::{validate_validator_table, ValidatorHealth};

pub use vrf::{
    vrf_prove, vrf_public_key, vrf_verify, vrf_wins, vrf_output_to_u128,
    VrfInput, VrfOutput, VrfProof,
};
pub use blinding::{
    derive_blinded_nonce, epoch_advance_alpha, EpochAdvanceProof, EpochVrfChain,
};
pub use nullifier::{
    derive_nullifier, nullifier_commitment, NullifierSet,
};
pub use estimator::{
    EpochStakeEstimator, EstimatorConfig, EstimatorSummary,
};
pub use enhanced_lottery::{
    build_vrf_alpha, EnhancedLeadershipOutcome, EnhancedLeadershipProof, EnhancedLottery,
};
pub use pol::{
    LeadershipBackend, LeadershipBackendError, LeadershipClaimV2, LeadershipProofV2,
    LeadershipPublicInputsV2, LeadershipWitnessV2, ProofOfLeadershipConfig,
    ProofOfLeadershipEngine, SignatureTranscriptBackend,
};

#[cfg(feature = "std")]
pub use simulator::{build_random_validator_table, run_simulation, SimulationConfig, SimulationStats};

pub const LLL_VERSION: u8 = 1;
pub const MAX_PROPOSAL_BYTES: usize = 1_048_576;
pub const MAX_PROPOSAL_HISTORY: usize = 10_000;
pub const MAX_AUDIT_EVENTS: usize = 50_000;

#[cfg(feature = "std")]
pub fn to_json_string<T: serde::Serialize>(value: &T) -> Result<alloc::string::String, LllError> {
    serde_json::to_string_pretty(value)
        .map_err(|e| LllError::Serialization(e.to_string()))
}

#[cfg(feature = "std")]
pub fn from_json_str<T: for<'de> serde::Deserialize<'de>>(
    raw: &str,
) -> Result<T, LllError> {
    serde_json::from_str(raw).map_err(|e| LllError::Serialization(e.to_string()))
}
