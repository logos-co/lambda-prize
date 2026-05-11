use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyReceiptStatus {
    Success,
    Failed,
    Redacted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivacyReceipt {
    pub tx_hash:      String,
    pub status:       PrivacyReceiptStatus,
    pub error:        Option<String>,
    pub events:       Vec<String>,
    pub commitments:  Vec<String>,
    pub nullifiers:   Vec<String>,
    pub checksum_hex: Option<String>,
}

impl PrivacyReceipt {
    pub fn success(tx_hash: impl Into<String>) -> Self {
        Self {
            tx_hash:     tx_hash.into(),
            status:      PrivacyReceiptStatus::Success,
            error:       None,
            events:      Vec::new(),
            commitments: Vec::new(),
            nullifiers:  Vec::new(),
            checksum_hex: None,
        }
    }

    pub fn failed(tx_hash: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            tx_hash:     tx_hash.into(),
            status:      PrivacyReceiptStatus::Failed,
            error:       Some(error.into()),
            events:      Vec::new(),
            commitments: Vec::new(),
            nullifiers:  Vec::new(),
            checksum_hex: None,
        }
    }

    pub fn redacted(tx_hash: impl Into<String>) -> Self {
        Self {
            tx_hash:     tx_hash.into(),
            status:      PrivacyReceiptStatus::Redacted,
            error:       None,
            events:      Vec::new(),
            commitments: Vec::new(),
            nullifiers:  Vec::new(),
            checksum_hex: None,
        }
    }

    pub fn add_event(mut self, hex_event: impl Into<String>) -> Self {
        self.events.push(hex_event.into());
        self
    }

    pub fn add_commitment(mut self, commitment: impl Into<String>) -> Self {
        self.commitments.push(commitment.into());
        self
    }

    pub fn add_nullifier(mut self, nullifier: impl Into<String>) -> Self {
        self.nullifiers.push(nullifier.into());
        self
    }

    #[cfg(feature = "std")]
    pub fn finalize(mut self) -> Self {
        let joined = [
            self.tx_hash.as_str(),
            self.error.as_deref().unwrap_or(""),
            &self.events.join(""),
            &self.commitments.join(""),
            &self.nullifiers.join(""),
        ]
        .join("|");
        self.checksum_hex = Some(crate::utils::checksum_hex(joined.as_bytes()));
        self
    }
}
