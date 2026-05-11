use borsh::{BorshDeserialize, BorshSerialize};
use lez_privacy::{
    commitment_bytes, decode_private_envelope, encode_private_envelope, generate_nullifier_hex,
    merkle_root, verify_commitment, CommitmentDomain, NullifierDomain, PrivacyReceipt,
    PrivacyReceiptStatus, PrivacySchema,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
struct PrivateNote {
    amount: u128,
    memo:   String,
}

impl PrivacySchema for PrivateNote {
    const NAME: &'static str = "tests::PrivateNote";
}

#[test]
fn private_envelope_round_trip() {
    let note = PrivateNote { amount: 42, memo: "hello privacy".into() };

    let raw     = encode_private_envelope(&note).expect("encode");
    let decoded = decode_private_envelope(&raw).expect("decode");
    assert_eq!(decoded.version, 1);
    assert!(!decoded.payload_hex.is_empty());
}

#[test]
fn commitments_match_expected_hex() {
    let c = commitment_bytes(CommitmentDomain::NOTE, b"abc");
    assert_eq!(c.len(), 32);
    let hexed = hex::encode(c);
    assert!(verify_commitment(CommitmentDomain::NOTE, b"abc", &hexed));
}

#[test]
fn nullifier_generation_is_deterministic() {
    let n1 = generate_nullifier_hex(b"secret", NullifierDomain::SPEND, b"commitment");
    let n2 = generate_nullifier_hex(b"secret", NullifierDomain::SPEND, b"commitment");
    assert_eq!(n1, n2);
}

#[test]
fn merkle_root_exists_for_private_notes() {
    let leaves = vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()];
    let root = merkle_root(&leaves);
    assert_eq!(root.len(), 32);
}

#[test]
fn privacy_receipt_builds_checksum() {
    let receipt = PrivacyReceipt::success("0xabc")
        .add_commitment("commitment1")
        .add_nullifier("nullifier1")
        .finalize();

    assert_eq!(receipt.status, PrivacyReceiptStatus::Success);
    assert!(receipt.checksum_hex.is_some());
}
