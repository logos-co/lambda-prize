use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{receipt::{DecodedEnvelope, ReceiptEnvelope, ReceiptStatus}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventIndexEntry {
    pub tx_hash:      String,
    pub status:       ReceiptStatus,
    pub type_name:    Option<String>,
    pub discriminant: String,
    pub payload_hex:  String,
    pub payload_size: usize,
    pub raw_size:     usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventIndex {
    pub entries: Vec<EventIndexEntry>,
    pub by_tx:   HashMap<String, Vec<usize>>,
    pub by_type: HashMap<String, Vec<usize>>,
}

#[derive(Debug, Clone, Default)]
pub struct EventIndexQuery {
    pub tx_hash:   Option<String>,
    pub type_name: Option<String>,
    pub status:    Option<ReceiptStatus>,
    pub limit:     Option<usize>,
}

impl EventIndex {
    pub fn new() -> Self { Self::default() }

    pub fn push_receipt(&mut self, receipt: &ReceiptEnvelope, decoded: &[DecodedEnvelope]) {
        for env in decoded {
            let idx = self.entries.len();
            self.entries.push(EventIndexEntry {
                tx_hash:      receipt.tx_hash.clone(),
                status:       receipt.status.clone(),
                type_name:    env.type_name.clone(),
                discriminant: env.discriminant.clone(),
                payload_hex:  env.payload_hex.clone(),
                payload_size: env.payload_size,
                raw_size:     env.raw_size,
            });
            self.by_tx.entry(receipt.tx_hash.clone()).or_default().push(idx);
            if let Some(name) = env.type_name.clone() {
                self.by_type.entry(name).or_default().push(idx);
            }
        }
    }

    pub fn query(&self, q: &EventIndexQuery) -> Vec<&EventIndexEntry> {
        let mut out = Vec::new();
        for entry in &self.entries {
            if let Some(ref tx) = q.tx_hash {
                if &entry.tx_hash != tx { continue; }
            }
            if let Some(ref ty) = q.type_name {
                if entry.type_name.as_deref() != Some(ty.as_str()) { continue; }
            }
            if let Some(ref status) = q.status {
                if &entry.status != status { continue; }
            }
            out.push(entry);
            if let Some(limit) = q.limit {
                if out.len() >= limit { break; }
            }
        }
        out
    }
}
