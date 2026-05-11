//! # Runtime / Sequencer Glue
//! Host-side `SYS_EMIT_EVENT` handler and write-ahead event journal.
//! Compiled into the LEZ sequencer — **not** into guest programs.
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{MAX_EVENT_SIZE, MAX_EVENTS_PER_TX, MAX_TX_EVENT_BYTES};

// ── Transaction status ────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TxStatus { Success, Failed }

impl std::fmt::Display for TxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Success => write!(f, "success"), Self::Failed => write!(f, "failed") }
    }
}

// ── Transaction receipt ───────────────────────────────────────────────────────
/// Full transaction receipt.  Events are **always present** regardless of status.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionReceipt {
    pub tx_hash:      String,
    pub block_number: u64,
    pub status:       TxStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error:        Option<String>,
    /// Each entry: hex( program_id[32] ++ event_wire_bytes ).
    pub events:       Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_root:   Option<String>,
    pub timestamp:    u64,
}

impl TransactionReceipt {
    pub fn event_count(&self) -> usize { self.events.len() }

    pub fn decode_events(
        &self,
        idl: Option<&HashMap<[u8; 4], String>>,
    ) -> Vec<crate::decoder::DecodedEvent> {
        self.events.iter().filter_map(|hex_str| {
            let bytes = hex::decode(hex_str).ok()?;
            if bytes.len() < 32 { return None; }
            // decode_raw returns Result; convert to Option for filter_map.
            crate::decoder::decode_raw(&bytes[32..], idl).ok()
        }).collect()
    }
}

// ── Per-transaction write-ahead event journal ─────────────────────────────────
#[derive(Default, Debug)]
pub struct EventJournal {
    entries:     Vec<Vec<u8>>,
    total_bytes: usize,
}

impl EventJournal {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { self.entries.len() }
    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub fn total_bytes(&self) -> usize { self.total_bytes }

    pub fn append(&mut self, program_id: [u8; 32], event_bytes: &[u8]) -> i32 {
        if event_bytes.len() > MAX_EVENT_SIZE                        { return -3; }
        if self.total_bytes + event_bytes.len() > MAX_TX_EVENT_BYTES { return -1; }
        if self.entries.len() >= MAX_EVENTS_PER_TX                   { return -2; }

        let mut entry = Vec::with_capacity(32 + event_bytes.len());
        entry.extend_from_slice(&program_id);
        entry.extend_from_slice(event_bytes);
        self.total_bytes += event_bytes.len();
        self.entries.push(entry);
        0
    }

    pub fn replay_from(&mut self, other: &EventJournal) -> usize {
        let mut replayed = 0;
        for entry in &other.entries {
            if entry.len() < 32 { continue; }
            let pid: [u8; 32] = entry[..32].try_into().unwrap();
            if self.append(pid, &entry[32..]) == 0 { replayed += 1; }
        }
        replayed
    }

    fn drain(self) -> Vec<Vec<u8>> { self.entries }
}

// ── Syscall handler ───────────────────────────────────────────────────────────
pub fn handle_sys_emit_event(
    journal:    &mut EventJournal,
    program_id: [u8; 32],
    event_data: &[u8],
) -> i32 {
    journal.append(program_id, event_data)
}

// ── In-memory receipt store ───────────────────────────────────────────────────
#[derive(Clone, Default)]
pub struct ReceiptStore {
    inner: Arc<Mutex<HashMap<String, TransactionReceipt>>>,
}

impl ReceiptStore {
    pub fn new() -> Self { Self::default() }

    pub fn finalise(
        &self,
        tx_hash:      [u8; 32],
        block_number: u64,
        status:       TxStatus,
        error:        Option<String>,
        state_root:   Option<[u8; 32]>,
        journal:      EventJournal,
    ) {
        let hash_hex = hex_encode(&tx_hash);
        let events: Vec<String> = journal.drain().iter().map(|e| hex_encode(e)).collect();
        let receipt = TransactionReceipt {
            tx_hash:      hash_hex.clone(),
            block_number,
            status,
            error,
            events,
            state_root:   state_root.map(|r| hex_encode(&r)),
            timestamp:    unix_now(),
        };
        self.inner.lock().unwrap().insert(hash_hex, receipt);
    }

    pub fn get_by_hash(&self, tx_hash: &[u8; 32]) -> Option<TransactionReceipt> {
        self.inner.lock().unwrap().get(&hex_encode(tx_hash)).cloned()
    }

    pub fn get(&self, tx_hash_hex: &str) -> Option<TransactionReceipt> {
        self.inner.lock().unwrap().get(tx_hash_hex).cloned()
    }

    pub fn get_block_events(&self, tx_hashes: &[[u8; 32]]) -> Vec<TransactionReceipt> {
        let store = self.inner.lock().unwrap();
        tx_hashes.iter()
            .filter_map(|h| store.get(&hex_encode(h)).cloned())
            .collect()
    }

    pub fn all(&self) -> Vec<TransactionReceipt> {
        self.inner.lock().unwrap().values().cloned().collect()
    }

    pub fn get_by_block(&self, block: u64) -> Vec<TransactionReceipt> {
        self.inner.lock().unwrap()
            .values()
            .filter(|r| r.block_number == block)
            .cloned()
            .collect()
    }
}

// ── EventStore trait + JsonFileReceiptStore ───────────────────────────────────
use crate::receipt::ReceiptEnvelope;

/// Trait for pluggable receipt storage backends.
///
/// The in-memory [`ReceiptStore`] struct is the primary production store.
/// [`JsonFileReceiptStore`] is provided for indexers and tests that need
/// file-backed persistence.
pub trait EventStore {
    fn put(&mut self, receipt: &ReceiptEnvelope) -> Result<(), StoreError>;
    fn get(&self, tx_hash: &str) -> Result<Option<ReceiptEnvelope>, StoreError>;
    fn list(&self) -> Result<Vec<ReceiptEnvelope>, StoreError>;
}

/// Persistent receipt store backed by a JSON file.
pub struct JsonFileReceiptStore {
    path: PathBuf,
}

impl JsonFileReceiptStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn load_all(&self) -> Result<Vec<ReceiptEnvelope>, StoreError> {
        if !self.path.exists() { return Ok(vec![]); }
        let raw = fs::read_to_string(&self.path).map_err(StoreError::Io)?;
        if raw.trim().is_empty() { return Ok(vec![]); }
        serde_json::from_str(&raw).map_err(StoreError::Serde)
    }

    fn save_all(&self, receipts: &[ReceiptEnvelope]) -> Result<(), StoreError> {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(StoreError::Io)?;
            }
        }
        let tmp = self.path.with_extension("tmp");
        {
            use std::io::{BufWriter, Write};
            let file   = fs::File::create(&tmp).map_err(StoreError::Io)?;
            let mut w  = BufWriter::new(file);
            serde_json::to_writer_pretty(&mut w, receipts).map_err(StoreError::Serde)?;
            w.flush().map_err(StoreError::Io)?;
        }
        fs::rename(&tmp, &self.path).map_err(StoreError::Io)
    }
}

impl EventStore for JsonFileReceiptStore {
    fn put(&mut self, receipt: &ReceiptEnvelope) -> Result<(), StoreError> {
        let mut all = self.load_all()?;
        if let Some(existing) = all.iter_mut().find(|r| r.tx_hash == receipt.tx_hash) {
            *existing = receipt.clone();
        } else {
            all.push(receipt.clone());
        }
        self.save_all(&all)
    }

    fn get(&self, tx_hash: &str) -> Result<Option<ReceiptEnvelope>, StoreError> {
        Ok(self.load_all()?.into_iter().find(|r| r.tx_hash == tx_hash))
    }

    fn list(&self) -> Result<Vec<ReceiptEnvelope>, StoreError> {
        self.load_all()
    }
}

// ── StoreError ────────────────────────────────────────────────────────────────
#[derive(Debug)]
pub enum StoreError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e)    => write!(f, "i/o error: {e}"),
            Self::Serde(e) => write!(f, "serialisation error: {e}"),
        }
    }
}

impl std::error::Error for StoreError {}

// ── JournalError ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalError {
    BudgetExceeded { used: usize, added: usize, limit: usize },
    CountExceeded  { used: usize, limit: usize },
}

impl std::fmt::Display for JournalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BudgetExceeded { used, added, limit } =>
                write!(f, "event budget exceeded: {used} + {added} > {limit}"),
            Self::CountExceeded { used, limit } =>
                write!(f, "event count exceeded: {used} >= {limit}"),
        }
    }
}

impl std::error::Error for JournalError {}

// ── Helpers ───────────────────────────────────────────────────────────────────
fn hex_encode(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const PROG: [u8; 32] = [0xAB; 32];
    const TX1:  [u8; 32] = [0x01; 32];
    const TX2:  [u8; 32] = [0x02; 32];

    fn finalise(store: &ReceiptStore, tx: [u8; 32], status: TxStatus, data: &[u8]) {
        let mut j = EventJournal::new();
        assert_eq!(handle_sys_emit_event(&mut j, PROG, data), 0);
        store.finalise(tx, 1, status, None, None, j);
    }

    #[test]
    fn journal_append_and_len() {
        let mut j = EventJournal::new();
        assert!(j.is_empty());
        assert_eq!(handle_sys_emit_event(&mut j, PROG, b"hello"), 0);
        assert_eq!(j.len(), 1);
        assert_eq!(j.total_bytes(), b"hello".len());
    }

    #[test]
    fn single_event_size_limit() {
        let mut j    = EventJournal::new();
        let oversized = vec![0u8; MAX_EVENT_SIZE + 1];
        assert_eq!(handle_sys_emit_event(&mut j, PROG, &oversized), -3);
        assert_eq!(j.len(), 0);
    }

    #[test]
    fn tx_count_limit_enforced() {
        let mut j = EventJournal::new();
        for _ in 0..MAX_EVENTS_PER_TX {
            assert_eq!(handle_sys_emit_event(&mut j, PROG, b"e"), 0);
        }
        assert_eq!(handle_sys_emit_event(&mut j, PROG, b"e"), -2);
        assert_eq!(j.len(), MAX_EVENTS_PER_TX);
    }

    #[test]
    fn tx_byte_budget_enforced() {
        let mut j  = EventJournal::new();
        let chunk  = vec![0u8; 4096];
        let max_ch = MAX_TX_EVENT_BYTES / 4096;
        for _ in 0..max_ch { let _ = handle_sys_emit_event(&mut j, PROG, &chunk); }
        assert_eq!(handle_sys_emit_event(&mut j, PROG, &chunk), -1);
    }

    #[test]
    fn journal_replay_copies_entries() {
        let mut src = EventJournal::new();
        let _       = handle_sys_emit_event(&mut src, PROG, b"event_a");
        let _       = handle_sys_emit_event(&mut src, PROG, b"event_b");
        let mut dst = EventJournal::new();
        assert_eq!(dst.replay_from(&src), 2);
        assert_eq!(dst.len(), 2);
        assert_eq!(dst.total_bytes(), src.total_bytes());
    }

    #[test]
    fn success_path_persists_event() {
        let store = ReceiptStore::new();
        finalise(&store, TX1, TxStatus::Success, b"hello");
        let r = store.get_by_hash(&TX1).unwrap();
        assert_eq!(r.status, TxStatus::Success);
        assert_eq!(r.event_count(), 1);
        let entry = hex::decode(&r.events[0]).unwrap();
        assert_eq!(&entry[..32], &PROG);
        assert_eq!(&entry[32..], b"hello");
    }

    #[test]
    fn failure_path_preserves_event() {
        let store = ReceiptStore::new();
        let mut j = EventJournal::new();
        let _ = handle_sys_emit_event(&mut j, PROG, b"fail_event");
        store.finalise(TX2, 2, TxStatus::Failed, Some("panic: oops".into()), None, j);
        let r = store.get_by_hash(&TX2).unwrap();
        assert_eq!(r.status, TxStatus::Failed);
        assert_eq!(r.error.as_deref(), Some("panic: oops"));
        assert_eq!(r.event_count(), 1, "event must survive failure");
    }

    #[test]
    fn empty_journal_on_no_emit() {
        let store = ReceiptStore::new();
        store.finalise(TX1, 1, TxStatus::Failed, Some("err".into()), None, EventJournal::new());
        let r = store.get_by_hash(&TX1).unwrap();
        assert_eq!(r.event_count(), 0);
    }

    #[test]
    fn json_file_receipt_store_put_and_list() {
        let dir  = tempfile::tempdir().unwrap();
        let path = dir.path().join("receipts.json");
        let mut store = JsonFileReceiptStore::new(&path);

        let r = crate::receipt::ReceiptEnvelope {
            tx_hash:    "0xabc".into(),
            status:     crate::receipt::ReceiptStatus::Success,
            error:      None,
            state_root: Some("0xstate".into()),
            events:     vec!["00deadbeef".into()],
        };
        store.put(&r).unwrap();

        let listed = store.list().unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].tx_hash, "0xabc");
        assert_eq!(listed[0].events.len(), 1);
    }

    #[test]
    fn json_file_receipt_store_get_by_hash() {
        let dir   = tempfile::tempdir().unwrap();
        let path  = dir.path().join("r.json");
        let mut s = JsonFileReceiptStore::new(&path);

        let r = crate::receipt::ReceiptEnvelope {
            tx_hash:    "0xfeed".into(),
            status:     crate::receipt::ReceiptStatus::Failed,
            error:      Some("boom".into()),
            state_root: None,
            events:     vec![],
        };
        s.put(&r).unwrap();

        let found = s.get("0xfeed").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().error.as_deref(), Some("boom"));

        let missing = s.get("0xmissing").unwrap();
        assert!(missing.is_none());
    }

    #[test]
    fn json_file_receipt_store_update_existing() {
        let dir   = tempfile::tempdir().unwrap();
        let path  = dir.path().join("r.json");
        let mut s = JsonFileReceiptStore::new(&path);

        let r1 = crate::receipt::ReceiptEnvelope {
            tx_hash:    "0xupdate".into(),
            status:     crate::receipt::ReceiptStatus::Success,
            error:      None, state_root: None,
            events:     vec![],
        };
        s.put(&r1).unwrap();
        let mut r2 = r1.clone();
        r2.events.push("00aabbcc".into());
        s.put(&r2).unwrap();

        let all = s.list().unwrap();
        assert_eq!(all.len(), 1, "upsert must not duplicate");
        assert_eq!(all[0].events.len(), 1);
    }
}
