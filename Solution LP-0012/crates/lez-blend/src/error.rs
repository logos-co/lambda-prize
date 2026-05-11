use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BlendError {
    #[error("packet too large: max payload is {max} bytes, got {got}")]
    PayloadTooLarge { max: usize, got: usize },

    #[error("too many hops: max is {max}, requested {requested}")]
    TooManyHops { max: usize, requested: usize },

    #[error("empty hop list — at least one hop is required")]
    EmptyHops,

    #[error("AEAD decryption failed (bad key, corrupted ciphertext, or wrong hop order)")]
    AeadDecryptFailed,

    #[error("MAC verification failed on hop header")]
    MacMismatch,

    #[error("mix node set is too small: need {needed}, have {have}")]
    InsufficientMixNodes { needed: usize, have: usize },

    #[error("decoy fan-out is zero — nothing to generate")]
    ZeroFanOut,

    #[error("VRF error: {0}")]
    Vrf(String),

    #[error("serialization error: {0}")]
    Serialization(String),
}

pub type BlendResult<T> = Result<T, BlendError>;
