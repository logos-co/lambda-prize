//! Event decoder: converts raw hex-encoded event entries into human-readable types.
//! Requires the `std` feature.
use std::collections::HashMap;

pub use crate::receipt::DecodedEnvelope;

use crate::{
    decode_envelope as parse_envelope,
    errors::EventError,
    fnv1a_discriminant,
    EventSchema, EVENT_VERSION,
};

extern crate alloc;
use alloc::{string::String, vec::Vec};

// ── DecodedEvent (internal, backward-compat) ──────────────────────────────────
/// Decoded event.  Fields are lower-level than [`DecodedEnvelope`]; used
/// internally by the runtime's `decode_events` helper.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DecodedEvent {
    pub version:      u8,
    pub discriminant: [u8; 4],
    pub payload:      Vec<u8>,
    pub payload_hex:  String,
    pub type_name:    Option<String>,
    /// Hex-encoded 32-byte program ID, populated by the caller when available.
    pub program_id:   Option<String>,
}

/// Decode raw event wire bytes (version + discriminant + payload) into a
/// [`DecodedEvent`], failing closed on any structural problem.
pub fn decode_raw(
    bytes: &[u8],
    idl:   Option<&HashMap<[u8; 4], String>>,
) -> Result<DecodedEvent, EventError> {
    let env       = parse_envelope(bytes)?;
    let type_name = idl.and_then(|m| m.get(&env.discriminant)).cloned();
    let hex: String = env.payload.iter().map(|b| format!("{b:02x}")).collect();
    Ok(DecodedEvent {
        version:      env.version,
        discriminant: env.discriminant,
        payload_hex:  hex,
        payload:      env.payload,
        type_name,
        program_id:   None,
    })
}

/// Decode raw event wire bytes into a [`DecodedEnvelope`] (the richer,
/// CLI-facing type).
///
/// The `raw` slice must start at the version byte — do **not** include the
/// 32-byte program ID prefix here.
pub fn decode_envelope_to_env(
    raw: &[u8],
    idl: Option<&HashMap<[u8; 4], String>>,
) -> Result<DecodedEnvelope, EventError> {
    crate::validation::validate_event_bytes(raw)?;
    let env       = parse_envelope(raw)?;
    let type_name = idl.and_then(|m| m.get(&env.discriminant)).cloned();
    Ok(DecodedEnvelope {
        version:      env.version,
        discriminant: hex::encode(env.discriminant),
        type_name,
        payload_hex:  hex::encode(&env.payload),
        payload_size: env.payload.len(),
        raw_size:     raw.len(),
    })
}

/// Decode a hex-encoded event entry into a [`DecodedEnvelope`].
///
/// Handles two formats:
/// 1. **Event wire bytes only** — hex string starts with version byte `00`.
/// 2. **Full receipt entry** — 32-byte program ID prefix + event wire bytes
///    (64 hex chars of program ID, then `00` version byte).
///
/// In strict mode you should ensure the caller strips the program ID prefix
/// before calling this function to avoid the heuristic.
pub fn decode_hex_envelope(
    raw_hex: &str,
    idl:     Option<&HashMap<[u8; 4], String>>,
) -> Result<DecodedEnvelope, EventError> {
    let clean = raw_hex.trim().trim_start_matches("0x");
    let bytes = hex::decode(clean)
        .map_err(|_| EventError::InvalidHex(raw_hex.to_string()))?;

    // Detect and strip 32-byte program_id prefix heuristically:
    // if bytes[32] is EVENT_VERSION and the total length is ≥ 32+5, strip it.
    let event_bytes: &[u8] = if bytes.len() >= 32 + 5 && bytes[32] == EVENT_VERSION {
        &bytes[32..]
    } else {
        &bytes
    };

    decode_envelope_to_env(event_bytes, idl)
}

/// Batch-decode a slice of hex-encoded event strings.
///
/// Returns one `Result` per entry, preserving order.  Partial failures do not
/// abort the whole batch — callers can filter on `Ok`/`Err` as needed.
#[inline]
pub fn decode_hex_envelopes(
    hexes: &[String],
    idl:   Option<&HashMap<[u8; 4], String>>,
) -> Vec<Result<DecodedEnvelope, EventError>> {
    let mut out = Vec::with_capacity(hexes.len());
    for h in hexes {
        out.push(decode_hex_envelope(h, idl));
    }
    out
}

// ── IDL helpers ───────────────────────────────────────────────────────────────
/// Build an IDL map from a slice of `(type_name, discriminant)` pairs.
pub fn build_idl(entries: &[(&str, [u8; 4])]) -> HashMap<[u8; 4], String> {
    entries.iter().map(|(n, d)| (*d, (*n).to_string())).collect()
}

/// Register a type name in an IDL map using its FNV-1a discriminant.
pub fn register_type(idl: &mut HashMap<[u8; 4], String>, type_name: &str) {
    idl.insert(fnv1a_discriminant(type_name), type_name.to_string());
}

/// Register a type that implements [`EventSchema`] — discriminant computed from
/// the `NAME` constant, no string needed.
pub fn register_schema<E: EventSchema>(idl: &mut HashMap<[u8; 4], String>) {
    idl.insert(E::DISCRIMINANT, E::NAME.to_string());
}
