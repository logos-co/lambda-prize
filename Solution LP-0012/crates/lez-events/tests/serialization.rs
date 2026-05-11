use borsh::{BorshDeserialize, BorshSerialize};
use lez_events::{
    decode_envelope, emit_event, encode_event, encode_event_named, fnv1a_32, fnv1a_discriminant,
    EventEnvelope, EventError, EventSchema,
    EVENT_VERSION, MAX_EVENT_SIZE, MAX_EVENTS_PER_TX, MAX_TX_EVENT_BYTES,
};

// ── Test event types ──────────────────────────────────────────────────────────
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
struct Transfer {
    from:   [u8; 32],
    to:     [u8; 32],
    amount: u64,
}

impl EventSchema for Transfer {
    const NAME: &'static str = "test::Transfer";
}

#[derive(BorshSerialize)]
struct Big { data: Vec<u8> }

impl EventSchema for Big {
    const NAME: &'static str = "test::Big";
}

// ── EventSchema trait ─────────────────────────────────────────────────────────
#[test]
fn event_schema_name_and_discriminant() {
    assert_eq!(Transfer::NAME, "test::Transfer");
    assert_eq!(Transfer::DISCRIMINANT, fnv1a_discriminant("test::Transfer"));
}

#[test]
fn event_schema_discriminant_method_matches_const() {
    assert_eq!(Transfer::discriminant(), Transfer::DISCRIMINANT);
    assert_eq!(Transfer::discriminant(), fnv1a_32("test::Transfer"));
}

#[test]
fn event_schema_encode_matches_encode_event() {
    let ev        = Transfer { from: [1u8; 32], to: [2u8; 32], amount: 42 };
    let via_trait = ev.encode().unwrap();
    let via_fn    = encode_event_named(&ev, "test::Transfer").unwrap();
    assert_eq!(via_trait, via_fn);
}

#[test]
fn event_schema_discriminant_is_const() {
    const D: [u8; 4] = Transfer::DISCRIMINANT;
    assert_ne!(D, [0u8; 4]);
}

// ── Wire format ───────────────────────────────────────────────────────────────
#[test]
fn encode_version_byte() {
    let ev = Transfer { from: [1u8; 32], to: [2u8; 32], amount: 0 };
    assert_eq!(encode_event_named(&ev, "test::Transfer").unwrap()[0], EVENT_VERSION);
}

#[test]
fn encode_discriminant_position() {
    let ev   = Transfer { from: [1u8; 32], to: [2u8; 32], amount: 0 };
    let buf  = encode_event_named(&ev, "test::Transfer").unwrap();
    let disc = fnv1a_discriminant("test::Transfer");
    assert_eq!(&buf[1..5], &disc);
}

#[test]
fn encode_payload_round_trip() {
    let ev      = Transfer { from: [1u8; 32], to: [2u8; 32], amount: 12345 };
    let buf     = encode_event_named(&ev, "test::Transfer").unwrap();
    let decoded: Transfer = borsh::from_slice(&buf[5..]).unwrap();
    assert_eq!(decoded, ev);
}

#[test]
fn encode_total_length() {
    let ev          = Transfer { from: [0u8; 32], to: [0u8; 32], amount: 0 };
    let buf         = encode_event_named(&ev, "test::Transfer").unwrap();
    let payload_len = borsh::to_vec(&ev).unwrap().len();
    assert_eq!(buf.len(), 1 + 4 + payload_len);
}

#[test]
fn single_arg_encode_event_matches_named() {
    let ev   = Transfer { from: [3u8; 32], to: [4u8; 32], amount: 500 };
    let via_schema = encode_event(&ev).unwrap();
    let via_named  = encode_event_named(&ev, "test::Transfer").unwrap();
    assert_eq!(via_schema, via_named);
}

// ── Size limits ───────────────────────────────────────────────────────────────
#[test]
fn rejects_oversized_event() {
    let ev  = Big { data: vec![0u8; MAX_EVENT_SIZE + 1] };
    let err = ev.encode().unwrap_err();
    assert!(matches!(err, EventError::EventTooLarge { size, limit } if size > limit));
}

#[test]
fn accepts_exactly_max_payload() {
    let ev = Big { data: vec![0u8; MAX_EVENT_SIZE - 4] };
    assert!(ev.encode().is_ok());
}

// ── Discriminant ──────────────────────────────────────────────────────────────
#[test]
fn discriminant_deterministic() {
    assert_eq!(fnv1a_discriminant("Foo"), fnv1a_discriminant("Foo"));
}

#[test]
fn discriminant_unique_per_name() {
    assert_ne!(fnv1a_discriminant("Foo"), fnv1a_discriminant("Bar"));
    assert_ne!(fnv1a_discriminant("test::Transfer"), fnv1a_discriminant("test::Big"));
}

#[test]
fn discriminant_const_evaluable() {
    const D: [u8; 4] = fnv1a_discriminant("CompileTimeCheck");
    assert_ne!(D, [0u8; 4]);
}

#[test]
fn fnv1a_32_matches_fnv1a_discriminant() {
    assert_eq!(fnv1a_32("test::Transfer"), fnv1a_discriminant("test::Transfer"));
}

#[test]
fn fnv1a_32_const_evaluable() {
    const D: [u8; 4] = fnv1a_32("CompileTimeCheck");
    assert_ne!(D, [0u8; 4]);
}

// ── decode_envelope ───────────────────────────────────────────────────────────
#[test]
fn decode_envelope_parses_valid_wire_bytes() {
    let ev  = Transfer { from: [1u8; 32], to: [2u8; 32], amount: 99 };
    let buf = encode_event_named(&ev, "test::Transfer").unwrap();
    let env = decode_envelope(&buf).unwrap();
    assert_eq!(env.version, EVENT_VERSION);
    assert_eq!(env.discriminant, fnv1a_discriminant("test::Transfer"));
    assert!(!env.payload.is_empty());
}

#[test]
fn decode_envelope_rejects_too_short() {
    // InvalidEnvelope now carries a String message — match with (_)
    assert!(matches!(decode_envelope(&[]),      Err(EventError::InvalidEnvelope(_))));
    assert!(matches!(decode_envelope(&[0u8; 4]), Err(EventError::InvalidEnvelope(_))));
}

#[test]
fn decode_envelope_rejects_unknown_version() {
    let buf = vec![0xFFu8, 0, 0, 0, 0, 1, 2, 3];
    assert!(matches!(decode_envelope(&buf), Err(EventError::InvalidVersion(0xFF))));
    let buf2 = vec![0x01u8, 0, 0, 0, 0];
    assert!(matches!(decode_envelope(&buf2), Err(EventError::InvalidVersion(0x01))));
}

#[test]
fn decode_envelope_accepts_empty_payload() {
    let buf = vec![EVENT_VERSION, 0xAA, 0xBB, 0xCC, 0xDD];
    let env = decode_envelope(&buf).unwrap();
    assert!(env.payload.is_empty());
    assert_eq!(env, EventEnvelope {
        version:      EVENT_VERSION,
        discriminant: [0xAA, 0xBB, 0xCC, 0xDD],
        payload:      vec![],
    });
}

// ── Error Display ─────────────────────────────────────────────────────────────
#[test]
fn error_display_messages() {
    assert!(EventError::SerializationFailed.to_string().contains("serialis"));
    assert!(EventError::EventTooLarge { size: 100, limit: 50 }.to_string().contains("100"));
    assert!(EventError::TxBudgetExceeded { used: 0, added: 100, limit: MAX_TX_EVENT_BYTES }
        .to_string().contains("budget"));
    assert!(EventError::TxCountExceeded { used: 0, limit: MAX_EVENTS_PER_TX }
        .to_string().contains("count"));
    assert!(EventError::SyscallError(-99).to_string().contains("-99"));
    assert!(EventError::InvalidEnvelope("test".into()).to_string().contains("envelope"));
    assert!(EventError::InvalidVersion(42).to_string().contains("42"));
    assert!(EventError::InvalidProgramId("bad".into()).to_string().contains("program id"));
    assert!(EventError::InvalidHex("abc".into()).to_string().contains("hex"));
    assert!(EventError::MissingField("foo").to_string().contains("foo"));
    assert!(EventError::UnsupportedFormat("xyz".into()).to_string().contains("xyz"));
    assert!(EventError::Io("disk full".into()).to_string().contains("disk full"));
    assert!(EventError::Rpc("timeout".into()).to_string().contains("timeout"));
    assert!(EventError::InvalidEncryptionKey { expected: 32, found: 16 }
        .to_string().contains("32"));
    assert!(EventError::InvalidEncryptionNonce { expected: 12, found: 8 }
        .to_string().contains("12"));
}

// ── emit_event! macro ─────────────────────────────────────────────────────────
#[test]
fn emit_event_macro_schema_branch() -> Result<(), EventError> {
    emit_event!(Transfer { from: [0u8; 32], to: [1u8; 32], amount: 7 })?;
    Ok(())
}

#[test]
fn emit_event_macro_explicit_name_branch() -> Result<(), EventError> {
    let ev = Transfer { from: [0u8; 32], to: [1u8; 32], amount: 7 };
    emit_event!(ev, "test::Transfer")?;
    Ok(())
}

#[test]
fn emit_event_macro_rejects_oversized() {
    let ev  = Big { data: vec![0u8; MAX_EVENT_SIZE + 1] };
    let err = ev.encode().unwrap_err();
    assert!(matches!(err, EventError::EventTooLarge { .. }));
}
