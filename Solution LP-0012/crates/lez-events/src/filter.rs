//! # EventFilter — Query Engine for LEZ Event Receipts
//!
//! Provides a composable, builder-style filter for querying events stored in
//! a [`ReceiptStore`](crate::runtime::ReceiptStore).
//!
//! ## Example
//! ```rust,ignore
//! use lez_events::filter::EventFilter;
//!
//! let results = EventFilter::new()
//!     .block_range(100, 200)
//!     .program([0xAB; 32])
//!     .discriminant(Transfer::DISCRIMINANT)
//!     .status_success()
//!     .apply(&store);
//! ```
use crate::runtime::{ReceiptStore, TransactionReceipt, TxStatus};

/// A composable filter for querying [`TransactionReceipt`]s from a store.
#[derive(Default, Clone, Debug)]
pub struct EventFilter {
    /// Only include receipts whose `block_number` falls in `[from, to]`.
    pub block_from:   Option<u64>,
    pub block_to:     Option<u64>,
    /// Only include receipts from this program ID (first 32 bytes of each event entry).
    pub program_id:   Option<[u8; 32]>,
    /// Only include events whose 4-byte discriminant matches.
    pub discriminant: Option<[u8; 4]>,
    /// Filter by transaction status.
    pub status:       Option<TxStatus>,
    /// Maximum number of receipts to return (applied after all other filters).
    pub limit:        Option<usize>,
}

impl EventFilter {
    /// Create a new empty filter (matches everything).
    pub fn new() -> Self { Self::default() }

    /// Restrict to a block range `[from, to]` (inclusive).
    pub fn block_range(mut self, from: u64, to: u64) -> Self {
        self.block_from = Some(from);
        self.block_to   = Some(to);
        self
    }

    /// Restrict to a specific block number.
    pub fn block(mut self, n: u64) -> Self {
        self.block_from = Some(n);
        self.block_to   = Some(n);
        self
    }

    /// Restrict to events emitted by a specific program.
    pub fn program(mut self, id: [u8; 32]) -> Self {
        self.program_id = Some(id);
        self
    }

    /// Restrict to events whose 4-byte FNV-1a discriminant matches.
    pub fn discriminant(mut self, disc: [u8; 4]) -> Self {
        self.discriminant = Some(disc);
        self
    }

    /// Only return successful transactions.
    pub fn status_success(mut self) -> Self {
        self.status = Some(TxStatus::Success);
        self
    }

    /// Only return failed transactions.
    pub fn status_failed(mut self) -> Self {
        self.status = Some(TxStatus::Failed);
        self
    }

    /// Cap the number of returned receipts.
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Apply the filter against a [`ReceiptStore`] and return matching receipts.
    /// Receipts are sorted by `block_number` ascending, then by `tx_hash`.
    pub fn apply(&self, store: &ReceiptStore) -> Vec<TransactionReceipt> {
        let mut results: Vec<TransactionReceipt> = store
            .all()
            .into_iter()
            .filter(|r| self.matches(r))
            .collect();

        // Stable sort: block_number asc, then tx_hash asc for determinism.
        results.sort_by(|a, b| {
            a.block_number.cmp(&b.block_number).then(a.tx_hash.cmp(&b.tx_hash))
        });

        if let Some(n) = self.limit {
            results.truncate(n);
        }
        results
    }

    /// Return only the events (wire bytes) that pass all filters, across all
    /// matching receipts.  Each item is `(tx_hash, event_hex)`.
    pub fn apply_events(&self, store: &ReceiptStore) -> Vec<(String, String)> {
        self.apply(store)
            .into_iter()
            .flat_map(|r| {
                let tx = r.tx_hash.clone();
                r.events
                    .into_iter()
                    .filter(|ev_hex| self.event_matches(ev_hex))
                    .map(move |ev_hex| (tx.clone(), ev_hex))
            })
            .collect()
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    fn matches(&self, r: &TransactionReceipt) -> bool {
        if let Some(from) = self.block_from {
            if r.block_number < from { return false; }
        }
        if let Some(to) = self.block_to {
            if r.block_number > to { return false; }
        }
        if let Some(ref s) = self.status {
            if &r.status != s { return false; }
        }
        // If a program_id or discriminant filter is set, at least one event must match.
        if (self.program_id.is_some() || self.discriminant.is_some()) && !r.events.iter().any(|ev_hex| self.event_matches(ev_hex)) {
            return false;
        }
        true
    }

    fn event_matches(&self, ev_hex: &str) -> bool {
        let bytes = match hex::decode(ev_hex) {
            Ok(b) => b,
            Err(_) => return false,
        };
        // Wire layout: [program_id(32)][version(1)][discriminant(4)][payload...]
        if bytes.len() < 37 { return false; }

        if let Some(prog) = &self.program_id {
            if &bytes[..32] != prog.as_ref() { return false; }
        }
        if let Some(disc) = &self.discriminant {
            // version byte is at index 32, discriminant at 33..37
            let ev_disc: [u8; 4] = bytes[33..37].try_into().unwrap_or([0u8; 4]);
            if &ev_disc != disc { return false; }
        }
        true
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{encode_event_named, fnv1a_discriminant, runtime::{EventJournal, handle_sys_emit_event}};
    use borsh::BorshSerialize;

    const PROG_A: [u8; 32] = [0xAA; 32];
    const PROG_B: [u8; 32] = [0xBB; 32];

    #[derive(BorshSerialize)] struct Ev { v: u64 }

    fn make_store() -> ReceiptStore {
        let store = ReceiptStore::new();

        // Block 1 — PROG_A, Success, type "test::Ev"
        let mut j = EventJournal::new();
        let bytes = encode_event_named(&Ev { v: 1 }, "test::Ev").unwrap();
        handle_sys_emit_event(&mut j, PROG_A, &bytes);
        store.finalise([0x01; 32], 1, TxStatus::Success, None, None, j);

        // Block 2 — PROG_B, Failed, type "test::Ev"
        let mut j = EventJournal::new();
        let bytes = encode_event_named(&Ev { v: 2 }, "test::Ev").unwrap();
        handle_sys_emit_event(&mut j, PROG_B, &bytes);
        store.finalise([0x02; 32], 2, TxStatus::Failed, Some("err".into()), None, j);

        // Block 3 — PROG_A, Success, type "test::Other"
        let mut j = EventJournal::new();
        let bytes = encode_event_named(&Ev { v: 3 }, "test::Other").unwrap();
        handle_sys_emit_event(&mut j, PROG_A, &bytes);
        store.finalise([0x03; 32], 3, TxStatus::Success, None, None, j);

        store
    }

    #[test]
    fn no_filter_returns_all() {
        let store = make_store();
        assert_eq!(EventFilter::new().apply(&store).len(), 3);
    }

    #[test]
    fn filter_by_block_range() {
        let store = make_store();
        let r = EventFilter::new().block_range(1, 2).apply(&store);
        assert_eq!(r.len(), 2);
        assert!(r.iter().all(|x| x.block_number <= 2));
    }

    #[test]
    fn filter_by_exact_block() {
        let store = make_store();
        let r = EventFilter::new().block(2).apply(&store);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].block_number, 2);
    }

    #[test]
    fn filter_by_status_success() {
        let store = make_store();
        let r = EventFilter::new().status_success().apply(&store);
        assert_eq!(r.len(), 2);
        assert!(r.iter().all(|x| x.status == TxStatus::Success));
    }

    #[test]
    fn filter_by_status_failed() {
        let store = make_store();
        let r = EventFilter::new().status_failed().apply(&store);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].status, TxStatus::Failed);
    }

    #[test]
    fn filter_by_program_id() {
        let store = make_store();
        let r = EventFilter::new().program(PROG_A).apply(&store);
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn filter_by_discriminant() {
        let store = make_store();
        let disc = fnv1a_discriminant("test::Ev");
        let r = EventFilter::new().discriminant(disc).apply(&store);
        assert_eq!(r.len(), 2); // block 1 and block 2
    }

    #[test]
    fn filter_combined_program_and_discriminant() {
        let store = make_store();
        let disc = fnv1a_discriminant("test::Ev");
        let r = EventFilter::new().program(PROG_A).discriminant(disc).apply(&store);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].block_number, 1);
    }

    #[test]
    fn filter_limit() {
        let store = make_store();
        let r = EventFilter::new().limit(2).apply(&store);
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn results_sorted_by_block_asc() {
        let store = make_store();
        let r = EventFilter::new().apply(&store);
        let blocks: Vec<u64> = r.iter().map(|x| x.block_number).collect();
        assert_eq!(blocks, vec![1, 2, 3]);
    }

    #[test]
    fn apply_events_returns_matching_events() {
        let store = make_store();
        let disc = fnv1a_discriminant("test::Ev");
        let evs = EventFilter::new().discriminant(disc).apply_events(&store);
        assert_eq!(evs.len(), 2);
    }
}
