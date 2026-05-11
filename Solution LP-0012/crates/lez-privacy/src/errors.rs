use alloc::string::String;

pub type PrivacyResult<T> = core::result::Result<T, PrivacyError>;

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum PrivacyError {
    #[error("serialization failed")]
    SerializationFailed,

    #[error("payload too large: {size} > {limit}")]
    PayloadTooLarge { size: usize, limit: usize },

    #[error("invalid private envelope: {0}")]
    InvalidEnvelope(String),

    #[error("unsupported private envelope version: {0}")]
    UnsupportedVersion(u8),

    #[error("commitment mismatch")]
    CommitmentMismatch,

    #[error("nullifier already spent")]
    NullifierAlreadySpent,

    #[error("invalid policy")]
    InvalidPolicy,

    #[error("access denied")]
    AccessDenied,

    #[error("encryption failed")]
    EncryptionFailed,

    #[error("decryption failed")]
    DecryptionFailed,

    #[error("invalid key length: expected {expected}, found {found}")]
    InvalidKeyLength { expected: usize, found: usize },

    #[error("invalid nonce length: expected {expected}, found {found}")]
    InvalidNonceLength { expected: usize, found: usize },

    #[error("invalid hex: {0}")]
    InvalidHex(String),

    #[error("invalid amount")]
    InvalidAmount,

    #[error("insufficient shielded balance")]
    InsufficientBalance,

    #[error("io error: {0}")]
    Io(String),
}
