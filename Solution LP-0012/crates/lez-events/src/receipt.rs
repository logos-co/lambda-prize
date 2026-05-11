//! Receipt types used by the CLI and decoder: [`ReceiptStatus`], [`ReceiptEnvelope`],
//! and [`DecodedReceipt`] / [`DecodedEnvelope`].
//!
//! These types provide a clean public-API layer for reading and writing receipt
//! JSON files without exposing internal runtime details.
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// ── ReceiptStatus ─────────────────────────────────────────────────────────────
/// Outcome of a transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReceiptStatus {
    Success,
    Failed,
}

impl core::fmt::Display for ReceiptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failed  => write!(f, "failed"),
        }
    }
}

// ── ReceiptEnvelope ───────────────────────────────────────────────────────────
/// Raw transaction receipt as stored in a JSON file or returned by the RPC.
///
/// The `events` field contains hex strings of event wire-format bytes.
/// Each entry may optionally include a 32-byte program ID prefix (64 hex chars)
/// before the event wire bytes — [`DecodedEnvelope`] handles both formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptEnvelope {
    pub tx_hash:    String,
    pub status:     ReceiptStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error:      Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_root: Option<String>,
    pub events:     Vec<String>,
}

impl ReceiptEnvelope {
    /// Whether the transaction succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self.status, ReceiptStatus::Success)
    }

    /// Number of events in this receipt.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Total hex characters across all event entries.
    pub fn total_hex_chars(&self) -> usize {
        self.events.iter().map(|e| e.len()).sum()
    }

    /// Validate that the tx_hash and all event hex strings are well-formed.
    pub fn validate(&self) -> Result<(), crate::errors::EventError> {
        crate::validation::validate_tx_hash(&self.tx_hash)?;
        for evt in &self.events {
            crate::validation::validate_hex_string(evt)?;
        }
        Ok(())
    }
}

// ── DecodedEnvelope ───────────────────────────────────────────────────────────
/// A fully-decoded event envelope with human-friendly fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodedEnvelope {
    pub version:      u8,
    /// 4-byte discriminant, hex-encoded.
    pub discriminant: String,
    /// Human-readable type name if an IDL map was provided.
    pub type_name:    Option<String>,
    /// Payload as a hex string.
    pub payload_hex:  String,
    /// Payload size in bytes.
    pub payload_size: usize,
    /// Total event wire size in bytes (version + discriminant + payload).
    pub raw_size:     usize,
}

// ── DecodedReceipt ────────────────────────────────────────────────────────────
/// A receipt with all events decoded into [`DecodedEnvelope`]s.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedReceipt {
    pub tx_hash:    String,
    pub status:     ReceiptStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error:      Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_root: Option<String>,
    pub events:     Vec<DecodedEnvelope>,
}

impl DecodedReceipt {
    pub fn event_count(&self) -> usize { self.events.len() }
}
