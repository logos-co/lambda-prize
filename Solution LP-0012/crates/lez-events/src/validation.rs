//! Input-validation helpers for hex strings, program IDs, and raw event bytes.
//! All functions return structured [`EventError`] values with helpful messages.
use crate::{errors::EventError, MAX_EVENT_SIZE};

/// Validate that `input` is a non-empty, even-length hexadecimal string.
/// Accepts an optional `0x` prefix.
pub fn validate_hex_string(input: &str) -> Result<(), EventError> {
    let s = input.trim();
    if s.is_empty() {
        return Err(EventError::InvalidHex("empty string".into()));
    }
    let s = s.trim_start_matches("0x");
    if s.len() % 2 != 0 {
        return Err(EventError::InvalidHex(
            "hex string must have an even number of digits".into(),
        ));
    }
    if hex::decode(s).is_err() {
        return Err(EventError::InvalidHex("could not decode hex digits".into()));
    }
    Ok(())
}

/// Validate that `input` is a 64-character hex-encoded program ID (32 bytes).
pub fn validate_program_id(input: &str) -> Result<(), EventError> {
    let s = input.trim();
    if s.is_empty() {
        return Err(EventError::MissingField("program_id"));
    }
    let hex_part = s.trim_start_matches("0x");
    if hex_part.len() != 64 {
        return Err(EventError::InvalidProgramId(
            format!("program id must be exactly 32 bytes (64 hex chars), got {} chars", hex_part.len()),
        ));
    }
    validate_hex_string(s)
        .map_err(|_| EventError::InvalidProgramId("contains non-hex characters".into()))?;
    Ok(())
}

/// Validate that `input` is a non-empty hex transaction hash.
pub fn validate_tx_hash(input: &str) -> Result<(), EventError> {
    let s = input.trim();
    if s.is_empty() {
        return Err(EventError::MissingField("tx_hash"));
    }
    validate_hex_string(s).map_err(|_| EventError::InvalidTxHash("invalid hex".into()))?;
    Ok(())
}

/// Validate that `bytes` is a plausible event wire-format slice.
///
/// Checks:
/// - Non-empty
/// - Total length ≤ `1 + 4 + MAX_EVENT_SIZE` (version + discriminant + payload cap)
pub fn validate_event_bytes(bytes: &[u8]) -> Result<(), EventError> {
    if bytes.is_empty() {
        return Err(EventError::InvalidEnvelope("empty event bytes".into()));
    }
    let max = 1 + 4 + MAX_EVENT_SIZE;
    if bytes.len() > max {
        return Err(EventError::EventTooLarge { size: bytes.len(), limit: max });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_empty_rejected()      { assert!(validate_hex_string("").is_err()); }
    #[test]
    fn hex_odd_rejected()        { assert!(validate_hex_string("abc").is_err()); }
    #[test]
    fn hex_valid_accepted()      { assert!(validate_hex_string("0xdeadbeef").is_ok()); }
    #[test]
    fn hex_no_prefix_accepted()  { assert!(validate_hex_string("deadbeef").is_ok()); }

    #[test]
    fn program_id_too_short()    { assert!(validate_program_id("1234").is_err()); }
    #[test]
    fn program_id_missing()      { assert!(validate_program_id("").is_err()); }
    #[test]
    fn program_id_valid() {
        let id = "ab".repeat(32);
        assert!(validate_program_id(&id).is_ok());
    }

    #[test]
    fn tx_hash_empty_rejected()  { assert!(validate_tx_hash("").is_err()); }
    #[test]
    fn tx_hash_valid_accepted()  { assert!(validate_tx_hash("0xdeadbeef").is_ok()); }

    #[test]
    fn event_bytes_empty_rejected() {
        assert!(validate_event_bytes(&[]).is_err());
    }
    #[test]
    fn event_bytes_valid_accepted() {
        assert!(validate_event_bytes(&[0u8; 10]).is_ok());
    }
}
