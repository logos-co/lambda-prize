use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{LllError, LllResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompactEnvelope {
    pub version: u8,
    pub kind: String,
    pub payload_hex: String,
}

pub fn encode_compact(kind: impl Into<String>, payload: &[u8]) -> LllResult<CompactEnvelope> {
    Ok(CompactEnvelope {
        version: crate::LLL_VERSION,
        kind: kind.into(),
        payload_hex: hex::encode(payload),
    })
}

pub fn decode_compact(raw: &CompactEnvelope) -> LllResult<Vec<u8>> {
    if raw.version != crate::LLL_VERSION {
        return Err(LllError::UnsupportedVersion);
    }
    hex::decode(&raw.payload_hex).map_err(|e| LllError::Serialization(e.to_string()))
}
