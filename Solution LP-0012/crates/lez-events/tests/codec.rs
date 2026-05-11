use borsh::{BorshDeserialize, BorshSerialize};
use lez_events::{decode_envelope, encode_event, encode_event_into, EventError, EventSchema};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq)]
struct ExampleEvent {
    amount: u64,
    label:  String,
}

impl EventSchema for ExampleEvent {
    const NAME: &'static str = "tests::ExampleEvent";
}

#[test]
fn encode_and_decode_round_trip_prefix() {
    let ev = ExampleEvent { amount: 42, label: "hello".to_string() };
    let bytes   = encode_event(&ev).expect("encode");
    assert_eq!(bytes[0], 0x00);
    assert_eq!(&bytes[1..5], &ExampleEvent::discriminant());
    let decoded = decode_envelope(&bytes).expect("decode");
    assert_eq!(decoded.version, 0x00);
    assert_eq!(decoded.discriminant, ExampleEvent::discriminant());
}

#[test]
fn oversize_event_is_rejected() {
    let ev  = ExampleEvent { amount: 7, label: "x".repeat(70_000) };
    let err = encode_event(&ev).unwrap_err();
    match err {
        EventError::EventTooLarge { .. } => {}
        other => panic!("expected EventTooLarge, got {other:?}"),
    }
}

#[test]
fn reusable_buffer_encoding_works() {
    let ev  = ExampleEvent { amount: 9, label: "buffer".to_string() };
    let mut buf = Vec::new();
    encode_event_into(&ev, &mut buf).expect("encode into");
    assert_eq!(buf[0], 0x00);
    assert_eq!(&buf[1..5], &ExampleEvent::discriminant());
}

#[test]
fn encode_into_reuses_buffer_without_growth() {
    let ev1 = ExampleEvent { amount: 1, label: "a".to_string() };
    let ev2 = ExampleEvent { amount: 2, label: "bb".to_string() };
    let mut buf = Vec::with_capacity(64);
    encode_event_into(&ev1, &mut buf).unwrap();
    let cap_after_first = buf.capacity();
    encode_event_into(&ev2, &mut buf).unwrap();
    assert!(buf.capacity() <= cap_after_first * 2, "buffer should not grow unreasonably");
    assert_eq!(buf[0], 0x00);
}
