use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub level: AuditLevel,
    pub category: String,
    pub message: String,
    pub slot: Option<u64>,
    pub epoch_id: Option<u64>,
}

impl AuditEvent {
    pub fn new(
        level: AuditLevel,
        category: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level,
            category: category.into(),
            message: message.into(),
            slot: None,
            epoch_id: None,
        }
    }

    pub fn with_slot(mut self, slot: u64) -> Self {
        self.slot = Some(slot);
        self
    }

    pub fn with_epoch(mut self, epoch_id: u64) -> Self {
        self.epoch_id = Some(epoch_id);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LotteryTrace {
    pub events: Vec<AuditEvent>,
}

impl LotteryTrace {
    pub fn push(&mut self, event: AuditEvent) {
        self.events.push(event);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LotteryMetrics {
    pub evaluated_slots: u64,
    pub wins: u64,
    pub misses: u64,
    pub proofs_built: u64,
    pub proposals_emitted: u64,
    pub total_latency_ns: u128,
}

impl LotteryMetrics {
    pub fn record_win(&mut self) {
        self.wins += 1;
        self.evaluated_slots += 1;
    }

    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.evaluated_slots += 1;
    }

    pub fn record_proof(&mut self) {
        self.proofs_built += 1;
    }

    pub fn record_proposal(&mut self) {
        self.proposals_emitted += 1;
    }

    pub fn add_latency(&mut self, nanos: u128) {
        self.total_latency_ns = self.total_latency_ns.saturating_add(nanos);
    }

    pub fn average_latency_ns(&self) -> u128 {
        if self.evaluated_slots == 0 {
            0
        } else {
            self.total_latency_ns / self.evaluated_slots as u128
        }
    }
}
