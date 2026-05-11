use borsh::{BorshDeserialize, BorshSerialize};
use lez_events::{BatchEncoder, EventSchema};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Eq, Clone)]
struct ExampleEvent {
    amount: u64,
}

impl EventSchema for ExampleEvent {
    const NAME: &'static str = "tests::BatchExampleEvent";
}

#[test]
fn batch_encoder_collects_multiple_events() {
    let mut enc = BatchEncoder::default();
    let batch   = enc
        .encode_batch(vec![
            ExampleEvent { amount: 1 },
            ExampleEvent { amount: 2 },
            ExampleEvent { amount: 3 },
        ])
        .expect("batch");
    assert_eq!(batch.count, 3);
    assert!(!batch.bytes.is_empty());
    assert_eq!(batch.total_bytes, batch.bytes.len());
}

#[test]
fn batch_encoder_respects_event_limit() {
    let mut enc = BatchEncoder::with_limits(1, 1024 * 1024);
    let result  = enc.encode_batch(vec![
        ExampleEvent { amount: 1 },
        ExampleEvent { amount: 2 },
    ]);
    assert!(result.is_err(), "should reject more events than the limit");
}

#[test]
fn batch_encoder_respects_byte_limit() {
    let mut enc = BatchEncoder::with_limits(256, 10);
    let result  = enc.encode_batch(vec![ExampleEvent { amount: 1 }]);
    assert!(result.is_err(), "should reject batch that exceeds byte limit");
}

#[test]
fn batch_encoder_empty_batch_is_valid() {
    let mut enc   = BatchEncoder::default();
    let batch     = enc.encode_batch(vec![] as Vec<ExampleEvent>).expect("empty batch");
    assert_eq!(batch.count, 0);
    assert!(batch.bytes.is_empty());
}

#[test]
fn batch_as_slice_matches_bytes() {
    let mut enc = BatchEncoder::default();
    let batch   = enc.encode_batch(vec![ExampleEvent { amount: 99 }]).unwrap();
    assert_eq!(batch.as_slice(), batch.bytes.as_slice());
}
