use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{PrivacyError, PrivacySchema, PRIVACY_ENVELOPE_VERSION};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivatePayloadType {
    Plaintext,
    Ciphertext,
    CommitmentOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivateEnvelopeHeader {
    pub version:    u8,
    pub domain_tag: String,
    pub type_hash:  String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivateEnvelope {
    pub version:      u8,
    pub header:       PrivateEnvelopeHeader,
    pub payload_hex:  String,
    pub checksum_hex: String,
    pub payload_type: PrivatePayloadType,
}

pub fn encode_private_envelope<E: PrivacySchema>(event: &E) -> Result<Vec<u8>, PrivacyError> {
    let payload = borsh::to_vec(event).map_err(|_| PrivacyError::SerializationFailed)?;
    if payload.len() > crate::PRIVATE_MAX_PAYLOAD_SIZE {
        return Err(PrivacyError::PayloadTooLarge {
            size: payload.len(),
            limit: crate::PRIVATE_MAX_PAYLOAD_SIZE,
        });
    }

    let mut out = Vec::with_capacity(1 + 8 + 4 + payload.len());
    out.push(PRIVACY_ENVELOPE_VERSION);
    out.extend_from_slice(&E::domain());
    out.extend_from_slice(&crate::fnv1a_32(E::NAME));
    out.extend_from_slice(&payload);
    Ok(out)
}

pub fn decode_private_envelope(raw: &[u8]) -> Result<PrivateEnvelope, PrivacyError> {
    if raw.len() < 13 {
        return Err(PrivacyError::InvalidEnvelope("envelope too short".into()));
    }
    if raw[0] != PRIVACY_ENVELOPE_VERSION {
        return Err(PrivacyError::UnsupportedVersion(raw[0]));
    }

    let domain    = &raw[1..9];
    let type_hash = &raw[9..13];
    let payload   = &raw[13..];

    Ok(PrivateEnvelope {
        version: raw[0],
        header: PrivateEnvelopeHeader {
            version:    raw[0],
            domain_tag: hex::encode(domain),
            type_hash:  hex::encode(type_hash),
        },
        payload_hex:  hex::encode(payload),
        checksum_hex: crate::utils::checksum_hex(raw),
        payload_type: PrivatePayloadType::Plaintext,
    })
}

pub fn decode_private_envelope_hex(raw_hex: &str) -> Result<PrivateEnvelope, PrivacyError> {
    let clean = raw_hex.trim().trim_start_matches("0x");
    let raw = hex::decode(clean)
        .map_err(|_| PrivacyError::InvalidHex(raw_hex.to_string()))?;
    decode_private_envelope(&raw)
}
