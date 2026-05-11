use thiserror::Error;

#[cfg(feature = "std")]
use std::string::String;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LllError {
    #[error("no validators in stake table")]
    NoValidators,

    #[error("total stake is zero")]
    ZeroTotalStake,

    #[error("insufficient stake to participate")]
    InsufficientStake,

    #[error("commitment mismatch")]
    CommitmentMismatch,

    #[error("proof verification failed: {0}")]
    ProofVerificationFailed(String),

    #[error("proposal denied: {0}")]
    ProposalDenied(String),

    #[error("unknown proposer")]
    UnknownProposer,

    #[error("unsupported version")]
    UnsupportedVersion,

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("invalid key material")]
    InvalidKeyMaterial,

    #[error("slot out of range")]
    SlotOutOfRange,

    #[error("epoch mismatch")]
    EpochMismatch,

    #[error("alias collision")]
    AliasCollision,

    #[error("signature error: {0}")]
    SignatureError(String),

    #[error("encoding error: {0}")]
    EncodingError(String),

    #[error("nullifier already spent — double-leadership attempt")]
    NullifierCollision,

    #[error("VRF error: {0}")]
    VrfError(String),
}

pub type LllResult<T> = Result<T, LllError>;
