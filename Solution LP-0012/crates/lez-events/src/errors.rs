use alloc::string::String;

/// Convenience alias: `Result<T, EventError>`.
pub type EventResult<T> = core::result::Result<T, EventError>;

/// Structured error type for the `lez-events` SDK.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventError {
    // ── Serialization ───────────────────────────────────────────────────────
    SerializationFailed,
    EventTooLarge { size: usize, limit: usize },

    // ── Transaction budget ───────────────────────────────────────────────────
    TxBudgetExceeded { used: usize, added: usize, limit: usize },
    TxCountExceeded  { used: usize, limit: usize },

    // ── Envelope / decoding ──────────────────────────────────────────────────
    /// Raw event bytes are structurally invalid.  The inner `String` describes
    /// the problem (e.g. "too short: need at least 5 bytes").
    InvalidEnvelope(String),
    /// Version byte is not `EVENT_VERSION`.  Decoders must fail closed.
    InvalidVersion(u8),

    // ── Encryption ───────────────────────────────────────────────────────────
    /// Wrong key length.  `expected` = 32, `found` = actual length.
    InvalidEncryptionKey   { expected: usize, found: usize },
    /// Wrong nonce length.  `expected` = 12, `found` = actual length.
    InvalidEncryptionNonce { expected: usize, found: usize },
    EncryptionFailed,

    // ── Input validation ─────────────────────────────────────────────────────
    InvalidHex(String),
    InvalidProgramId(String),
    InvalidTxHash(String),
    MissingField(&'static str),
    UnsupportedFormat(String),

    // ── I/O and networking ───────────────────────────────────────────────────
    Io(String),
    Rpc(String),

    // ── Retry ────────────────────────────────────────────────────────────────
    RetryExhausted { attempts: usize, last_error: alloc::string::String },

    // ── Syscall ──────────────────────────────────────────────────────────────
    SyscallError(i32),
}

impl core::fmt::Display for EventError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SerializationFailed =>
                write!(f, "event serialisation failed"),
            Self::EventTooLarge { size, limit } =>
                write!(f, "event too large: {size} bytes exceeds {limit}-byte limit"),
            Self::TxBudgetExceeded { used, added, limit } =>
                write!(f, "tx event byte budget exceeded: {used} + {added} > {limit}"),
            Self::TxCountExceeded { used, limit } =>
                write!(f, "tx event count exceeded: {used} >= {limit}"),
            Self::InvalidEnvelope(msg) =>
                write!(f, "invalid event envelope: {msg}"),
            Self::InvalidVersion(v) =>
                write!(f, "unsupported event version: {v}"),
            Self::InvalidEncryptionKey { expected, found } =>
                write!(f, "invalid encryption key length: expected {expected}, found {found}"),
            Self::InvalidEncryptionNonce { expected, found } =>
                write!(f, "invalid encryption nonce length: expected {expected}, found {found}"),
            Self::EncryptionFailed =>
                write!(f, "event encryption failed"),
            Self::InvalidHex(s) =>
                write!(f, "invalid hex string: {s}"),
            Self::InvalidProgramId(s) =>
                write!(f, "invalid program id: {s}"),
            Self::InvalidTxHash(s) =>
                write!(f, "invalid transaction hash: {s}"),
            Self::MissingField(name) =>
                write!(f, "missing required field: {name}"),
            Self::UnsupportedFormat(fmt) =>
                write!(f, "unsupported format: {fmt}"),
            Self::Io(s) =>
                write!(f, "i/o error: {s}"),
            Self::Rpc(s) =>
                write!(f, "rpc error: {s}"),
            Self::RetryExhausted { attempts, last_error } =>
                write!(f, "retry exhausted after {attempts} attempts: {last_error}"),
            Self::SyscallError(code) =>
                write!(f, "sys_emit_event returned error code {code}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EventError {}

/// Type alias kept for compatibility: `CliError = EventError`.
pub type CliError = EventError;
