use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
};

use crate::{receipt::PrivacyReceipt, PrivacyError};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivacySupportBundle {
    pub title:          String,
    pub receipt_summary: Option<String>,
    pub entries:        Vec<String>,
}

impl PrivacySupportBundle {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), receipt_summary: None, entries: Vec::new() }
    }

    pub fn from_receipt(receipt: Option<&PrivacyReceipt>) -> Self {
        let summary = receipt.map(|r| {
            format!(
                "tx_hash={} status={:?} events={} commitments={} nullifiers={}",
                r.tx_hash,
                r.status,
                r.events.len(),
                r.commitments.len(),
                r.nullifiers.len()
            )
        });
        Self { title: "privacy-support".into(), receipt_summary: summary, entries: Vec::new() }
    }

    pub fn add_entry(mut self, entry: impl Into<String>) -> Self {
        self.entries.push(entry.into());
        self
    }
}

pub fn write_privacy_bundle(
    path: &Path,
    bundle: &PrivacySupportBundle,
) -> Result<(), PrivacyError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| PrivacyError::Io(e.to_string()))?;
    }
    let raw = serde_json::to_string_pretty(bundle)
        .map_err(|e| PrivacyError::Io(e.to_string()))?;
    fs::write(path, raw).map_err(|e| PrivacyError::Io(e.to_string()))?;
    Ok(())
}
