#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use borsh::BorshSerialize;

pub mod commitment;
pub mod crypto;
pub mod errors;
pub mod nullifier;
pub mod receipt;
pub mod shielded;
pub mod tree;
pub mod utils;

#[cfg(feature = "std")]
pub mod codec;
#[cfg(feature = "std")]
pub mod diagnostics;
#[cfg(feature = "std")]
pub mod policy;
#[cfg(feature = "std")]
pub mod support;

pub use commitment::{
    commitment_bytes, commitment_with_domain, CommitmentDomain,
};
#[cfg(feature = "std")]
pub use commitment::{commitment_hex, verify_commitment};

#[cfg(feature = "std")]
pub use codec::{
    decode_private_envelope, decode_private_envelope_hex, encode_private_envelope,
    PrivateEnvelope, PrivateEnvelopeHeader, PrivatePayloadType,
};

pub use crypto::{
    decrypt_encrypted_bytes, encrypt_encrypted_bytes, generate_nonce, generate_random_key,
    CipherText, EncryptedBlob, EnvelopeKey,
};

pub use errors::{PrivacyError, PrivacyResult};

pub use nullifier::{generate_nullifier, Nullifier, NullifierDomain};
#[cfg(feature = "std")]
pub use nullifier::{generate_nullifier_hex, verify_nullifier};

#[cfg(feature = "std")]
pub use policy::{
    AccessDecision, AccessPolicy, AccessPolicySet, PolicyAction, PolicyEffect, PolicyRule,
};

pub use receipt::{PrivacyReceipt, PrivacyReceiptStatus};

pub use shielded::{
    apply_private_transfer, ShieldedAccount, ShieldedBalance, ShieldedLedger, ShieldedTransfer,
    ShieldedTransferReceipt,
};

pub use tree::{merkle_path, merkle_root, verify_merkle_path, MerkleNode, MerklePath};

#[cfg(feature = "std")]
pub use utils::{checksum_hex, human_amount, redacted_hex, support_preview};
#[cfg(not(feature = "std"))]
pub use utils::human_amount;

#[cfg(feature = "std")]
pub use diagnostics::{redacted_report, PrivacyDiagnostic, PrivacyDiagnosticReport};

#[cfg(feature = "std")]
pub use support::{write_privacy_bundle, PrivacySupportBundle};

pub const PRIVACY_ENVELOPE_VERSION: u8 = 0x01;
pub const PRIVATE_MAX_PAYLOAD_SIZE: usize = 64 * 1024;
pub const PRIVATE_MAX_NOTES_PER_TX: usize = 256;
pub const PRIVATE_MAX_TOTAL_BYTES_PER_TX: usize = 1024 * 1024;

pub trait PrivacySchema: BorshSerialize {
    const NAME: &'static str;
    fn domain() -> [u8; 8] {
        utils::domain_tag(Self::NAME)
    }
}

pub fn fnv1a_32(name: &str) -> [u8; 4] {
    let mut h: u32 = 0x811c9dc5;
    for b in name.as_bytes() {
        h ^= *b as u32;
        h = h.wrapping_mul(0x01000193);
    }
    h.to_le_bytes()
}

pub fn encode_private<E: PrivacySchema>(event: &E) -> PrivacyResult<Vec<u8>> {
    let payload = borsh::to_vec(event).map_err(|_| PrivacyError::SerializationFailed)?;
    if payload.len() > PRIVATE_MAX_PAYLOAD_SIZE {
        return Err(PrivacyError::PayloadTooLarge {
            size:  payload.len(),
            limit: PRIVATE_MAX_PAYLOAD_SIZE,
        });
    }

    let mut out = Vec::with_capacity(1 + 8 + 4 + payload.len());
    out.push(PRIVACY_ENVELOPE_VERSION);
    out.extend_from_slice(&E::domain());
    out.extend_from_slice(&fnv1a_32(E::NAME));
    out.extend_from_slice(&payload);
    Ok(out)
}

#[cfg(feature = "std")]
pub fn encode_private_hex<E: PrivacySchema>(event: &E) -> PrivacyResult<String> {
    Ok(hex::encode(encode_private(event)?))
}
