//! # lez-events  
//! Guest-side SDK and host-side helpers for structured, deterministic LEZ events.
//! Events survive transaction failure via the host's write-ahead journal.
//!
//! ## Quick Start
//! ```rust,ignore
//! use lez_events::{emit_event, EventSchema};
//! use borsh::{BorshSerialize, BorshDeserialize};
//!
//! #[derive(BorshSerialize, BorshDeserialize)]
//! pub struct Transfer { pub amount: u64 }
//!
//! impl EventSchema for Transfer {
//!     const NAME: &'static str = "my_program::Transfer";
//! }
//!
//! fn process() -> Result<(), EventError> {
//!     emit_event!(Transfer { amount: 100 })
//! }
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use alloc::vec::Vec;
use borsh::BorshSerialize;

// ── Sub-modules ───────────────────────────────────────────────────────────────
// Always-available (no_std compatible)
pub mod backoff;
pub mod batch;
pub mod codec;
pub mod errors;
pub mod human;
pub mod builder;
pub mod merkle;
pub mod retry;

// std-only modules
#[cfg(feature = "std")] pub mod atomic;
#[cfg(feature = "std")] pub mod bundle;
#[cfg(feature = "std")] pub mod diagnostics;
#[cfg(feature = "std")] pub mod health;
#[cfg(feature = "std")] pub mod index;
#[cfg(feature = "std")] pub mod integrity;
#[cfg(feature = "std")] pub mod perf;
#[cfg(feature = "std")] pub mod support;
#[cfg(feature = "std")] pub mod runtime;
#[cfg(feature = "std")] pub mod rpc;
#[cfg(feature = "std")] pub mod filter;
#[cfg(feature = "std")] pub mod event_log;
#[cfg(feature = "std")] pub mod bus;
#[cfg(feature = "std")] pub mod decoder;
#[cfg(feature = "std")] pub mod validation;
#[cfg(feature = "std")] pub mod receipt;
#[cfg(feature = "std")] pub mod config;

// ── Re-exports ────────────────────────────────────────────────────────────────
pub use errors::{CliError, EventError, EventResult};
pub use human::{human_bytes, human_error_chain, human_hex_preview};
pub use builder::EventBuilder;
pub use backoff::{Backoff, BackoffConfig};
pub use batch::{BatchEncoder, BatchEnvelope};
pub use codec::{
    decode_envelope_ref, encode_event, encode_event_bytes, encode_event_bytes_into,
    encode_event_into, DecodedEnvelopeRef, EnvelopeEncoding,
};
pub use retry::{retry, RetryConfig, RetryError};

#[cfg(feature = "std")] pub use atomic::{atomic_write_bytes, atomic_write_json, read_json_file};
#[cfg(feature = "std")] pub use bundle::{Bundle, BundleEntry, BundleKind};
#[cfg(feature = "std")] pub use config::{AppConfig, CliConfig, OutputFormat};
#[cfg(feature = "std")] pub use diagnostics::{DiagnosticLevel, DiagnosticRecord, DiagnosticReport, SupportContext};
#[cfg(feature = "std")] pub use health::{HealthCheck, HealthStatus};
#[cfg(feature = "std")] pub use support::{
    build_support_bundle, capture_support_context, render_support_report,
    SupportBundleConfig, SupportBundleWriter, SupportCommand,
};
#[cfg(feature = "std")] pub use decoder::{
    build_idl, decode_hex_envelope, decode_hex_envelopes, decode_raw, register_schema,
    register_type, DecodedEvent,
};
#[cfg(feature = "std")] pub use index::{EventIndex, EventIndexEntry, EventIndexQuery};
#[cfg(feature = "std")] pub use integrity::{checksum_hex, verify_checksum_hex};
#[cfg(feature = "std")] pub use receipt::DecodedEnvelope;
#[cfg(feature = "std")] pub use receipt::{DecodedReceipt, ReceiptEnvelope, ReceiptStatus};
#[cfg(feature = "std")] pub use filter::EventFilter;
#[cfg(feature = "std")] pub use bus::EventBus;
#[cfg(feature = "std")] pub use validation::{
    validate_event_bytes, validate_hex_string, validate_program_id, validate_tx_hash,
};

// ── Size limits ───────────────────────────────────────────────────────────────
/// Maximum serialised payload of a single event (64 KiB).
pub const MAX_EVENT_SIZE: usize    = 64 * 1024;
/// Maximum total event bytes per transaction (1 MiB).
pub const MAX_TX_EVENT_BYTES: usize = 1024 * 1024;
/// Maximum number of events per transaction.
pub const MAX_EVENTS_PER_TX: usize  = 256;
/// Wire-format version byte.
pub const EVENT_VERSION: u8         = 0x00;

// ── EventSchema trait ─────────────────────────────────────────────────────────
/// Implement this trait on every event struct.  The `NAME` constant is used to
/// derive the FNV-1a discriminant and populate the IDL map.
pub trait EventSchema: BorshSerialize {
    /// Fully-qualified, stable type name.  Must never change after deployment.
    const NAME: &'static str;

    /// 4-byte FNV-1a discriminant derived from `NAME` — compile-time constant.
    const DISCRIMINANT: [u8; 4] = fnv1a_discriminant(Self::NAME);

    /// 4-byte FNV-1a discriminant (method form, identical value to `DISCRIMINANT`).
    fn discriminant() -> [u8; 4] { Self::DISCRIMINANT }

    /// Human-readable type name (alias for `NAME`).
    fn event_name() -> &'static str { Self::NAME }

    /// Encode this event to wire format via the zero-overhead single-arg path.
    fn encode(&self) -> Result<Vec<u8>, EventError> where Self: Sized {
        crate::codec::encode_event(self)
    }
}

// ── Syscall binding ───────────────────────────────────────────────────────────
#[cfg(not(any(test, feature = "test-stub")))]
extern "C" {
    fn sys_emit_event(ptr: *const u8, len: usize) -> i32;
}

#[cfg(feature = "test-stub")]
#[no_mangle]
pub extern "C" fn sys_emit_event(_ptr: *const u8, _len: usize) -> i32 { 0 }

// ── FNV-1a discriminant ───────────────────────────────────────────────────────
/// Derive a 4-byte little-endian discriminant from a type name.  `const fn`.
pub const fn fnv1a_discriminant(name: &str) -> [u8; 4] {
    let bytes = name.as_bytes();
    let mut h: u32 = 0x811c_9dc5;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u32;
        h = h.wrapping_mul(0x0100_0193);
        i += 1;
    }
    h.to_le_bytes()
}

/// Alias for [`fnv1a_discriminant`] — same algorithm, same result.  `const fn`.
pub const fn fnv1a_32(name: &str) -> [u8; 4] { fnv1a_discriminant(name) }

/// Derive a discriminant from a name string (non-const, for use in generic contexts).
pub fn event_discriminant(name: &str) -> [u8; 4] { fnv1a_discriminant(name) }

// ── Wire encoding ─────────────────────────────────────────────────────────────
/// Encode any `BorshSerialize` event to wire format with an **explicit** type name.
///
/// Wire layout: `[version(1)][discriminant(4)][borsh_payload]`
///
/// This two-argument form accepts any `BorshSerialize` value — useful when the
/// type cannot implement [`EventSchema`] or when the name is dynamic.
/// For types implementing [`EventSchema`] prefer the single-arg [`encode_event`]
/// (re-exported from `codec`) or [`emit`].
pub fn encode_event_named<E: BorshSerialize>(event: &E, type_name: &str) -> Result<Vec<u8>, EventError> {
    let payload = borsh::to_vec(event).map_err(|_| EventError::SerializationFailed)?;
    if payload.len() > MAX_EVENT_SIZE {
        return Err(EventError::EventTooLarge { size: payload.len(), limit: MAX_EVENT_SIZE });
    }
    let disc = fnv1a_discriminant(type_name);
    let mut buf = Vec::with_capacity(1 + 4 + payload.len());
    buf.push(EVENT_VERSION);
    buf.extend_from_slice(&disc);
    buf.extend_from_slice(&payload);
    Ok(buf)
}

// ── EventEnvelope ─────────────────────────────────────────────────────────────
/// Decoded header of an event wire-format slice (owned form).
///
/// For a zero-copy borrowed form, use [`decode_envelope_ref`] which returns
/// [`DecodedEnvelopeRef`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventEnvelope {
    pub version:      u8,
    pub discriminant: [u8; 4],
    pub payload:      Vec<u8>,
}

/// Decode the wire-format header from a raw byte slice, returning an owned
/// [`EventEnvelope`] (allocates a copy of the payload).
///
/// Fails closed: unknown version bytes return `Err(InvalidVersion(v))`.
/// Returns `Err(InvalidEnvelope(...))` if the slice is too short.
///
/// For a zero-allocation variant, use [`decode_envelope_ref`].
pub fn decode_envelope(raw: &[u8]) -> Result<EventEnvelope, EventError> {
    if raw.len() < 5 {
        return Err(EventError::InvalidEnvelope(
            alloc::format!("too short: need ≥ 5 bytes, got {}", raw.len())
        ));
    }
    let version = raw[0];
    if version != EVENT_VERSION {
        return Err(EventError::InvalidVersion(version));
    }
    let discriminant: [u8; 4] = raw[1..5]
        .try_into()
        .map_err(|_| EventError::InvalidEnvelope("could not read 4-byte discriminant".into()))?;
    Ok(EventEnvelope { version, discriminant, payload: raw[5..].to_vec() })
}

// ── emit / emit_encrypted ─────────────────────────────────────────────────────
/// Emit a typed event from a LEZ program via its [`EventSchema`] implementation.
pub fn emit<E: EventSchema>(event: &E) -> Result<(), EventError> {
    let bytes = crate::codec::encode_event(event)?;
    let rc = unsafe { __sys_emit_event(bytes.as_ptr(), bytes.len()) };
    match rc {
        0  => Ok(()),
        -1 => Err(EventError::TxBudgetExceeded {
            used: 0, added: bytes.len(), limit: MAX_TX_EVENT_BYTES,
        }),
        -2 => Err(EventError::TxCountExceeded { used: 0, limit: MAX_EVENTS_PER_TX }),
        c  => Err(EventError::SyscallError(c)),
    }
}

/// Emit an AES-256-GCM encrypted event (requires `encryption` feature).
///
/// The caller provides a 32-byte key and 12-byte nonce.  The `aes-gcm`
/// dependency lives inside `lez-events`; calling crates only need the
/// `encryption` feature — no direct `aes-gcm` dependency.
#[cfg(feature = "encryption")]
pub fn emit_encrypted<E: EventSchema>(
    event:       &E,
    key_bytes:   &[u8],
    nonce_bytes: &[u8],
) -> Result<(), EventError> {
    use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit}};

    if key_bytes.len() != 32 {
        return Err(EventError::InvalidEncryptionKey { expected: 32, found: key_bytes.len() });
    }
    if nonce_bytes.len() != 12 {
        return Err(EventError::InvalidEncryptionNonce { expected: 12, found: nonce_bytes.len() });
    }

    let plaintext = crate::codec::encode_event(event)?;
    let key    = Key::<Aes256Gcm>::from_slice(key_bytes);
    let nonce  = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(key);
    let ct = cipher.encrypt(nonce, plaintext.as_ref())
        .map_err(|_| EventError::EncryptionFailed)?;

    let rc = unsafe { __sys_emit_event(ct.as_ptr(), ct.len()) };
    match rc {
        0  => Ok(()),
        -1 => Err(EventError::TxBudgetExceeded {
            used: 0, added: ct.len(), limit: MAX_TX_EVENT_BYTES,
        }),
        -2 => Err(EventError::TxCountExceeded { used: 0, limit: MAX_EVENTS_PER_TX }),
        c  => Err(EventError::SyscallError(c)),
    }
}

// ── Macros ────────────────────────────────────────────────────────────────────
/// Emit a typed event.  Accepts either:
/// - A value implementing [`EventSchema`] (recommended — delegates to [`emit`])
/// - Any `BorshSerialize` value with an explicit type name (legacy form)
#[macro_export]
macro_rules! emit_event {
    ($event:expr) => {
        $crate::emit(&$event)
    };
    ($event:expr, $name:expr) => {{
        use $crate::{encode_event_named, EventError, MAX_TX_EVENT_BYTES, MAX_EVENTS_PER_TX};
        match encode_event_named(&$event, $name) {
            Ok(bytes) => {
                let rc = unsafe { $crate::__sys_emit_event(bytes.as_ptr(), bytes.len()) };
                match rc {
                    0  => Ok(()),
                    -1 => Err(EventError::TxBudgetExceeded {
                        used: 0, added: bytes.len(), limit: MAX_TX_EVENT_BYTES,
                    }),
                    -2 => Err(EventError::TxCountExceeded { used: 0, limit: MAX_EVENTS_PER_TX }),
                    c  => Err(EventError::SyscallError(c)),
                }
            }
            Err(e) => Err(e),
        }
    }};
}

/// Emit an AES-256-GCM encrypted event (requires `encryption` feature).
#[cfg(feature = "encryption")]
#[macro_export]
macro_rules! emit_encrypted_event {
    ($event:expr, $key_bytes:expr, $nonce_bytes:expr) => {
        $crate::emit_encrypted(&$event, $key_bytes, $nonce_bytes)
    };
}

// ── Syscall shim ──────────────────────────────────────────────────────────────
#[doc(hidden)]
#[cfg(not(any(test, feature = "test-stub")))]
pub unsafe fn __sys_emit_event(ptr: *const u8, len: usize) -> i32 {
    sys_emit_event(ptr, len)
}

#[doc(hidden)]
#[cfg(any(test, feature = "test-stub"))]
pub unsafe fn __sys_emit_event(_ptr: *const u8, _len: usize) -> i32 { 0 }

// ── Prelude ───────────────────────────────────────────────────────────────────
/// Convenience re-export of the most commonly used items.
pub mod prelude {
    pub use crate::{
        decode_envelope, decode_envelope_ref, emit, emit_event,
        encode_event, encode_event_into, encode_event_named,
        event_discriminant, fnv1a_32, fnv1a_discriminant,
        human_bytes, human_error_chain, human_hex_preview,
        BatchEncoder, BatchEnvelope, DecodedEnvelopeRef, EnvelopeEncoding,
        EventEnvelope, EventError, EventResult, EventSchema,
        BackoffConfig, Backoff, RetryConfig, retry,
        EVENT_VERSION, MAX_EVENT_SIZE, MAX_EVENTS_PER_TX, MAX_TX_EVENT_BYTES,
    };
    #[cfg(feature = "encryption")]
    pub use crate::{emit_encrypted, emit_encrypted_event};
    #[cfg(feature = "std")]
    pub use crate::{
        atomic_write_json, checksum_hex,
        decode_hex_envelope, decode_hex_envelopes, decode_raw,
        validate_event_bytes, validate_hex_string,
        validate_program_id, validate_tx_hash,
        AppConfig, CliConfig, DecodedEnvelope, DecodedReceipt, DecodedEvent,
        EventFilter, EventIndex, EventIndexEntry, EventIndexQuery, OutputFormat,
        ReceiptEnvelope, ReceiptStatus,
    };
}
