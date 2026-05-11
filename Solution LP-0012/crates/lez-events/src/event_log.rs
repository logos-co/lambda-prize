//! # EventLog — Append-Only Persistent Event Log
//!
//! Writes finalised [`TransactionReceipt`]s to a JSON-lines file on disk.
//! Each line is a complete, self-describing JSON receipt.  The log is
//! append-only: existing entries are never modified or deleted.
//!
//! ## Format
//! ```text
//! {"tx_hash":"0101...","block_number":1,"status":"success","events":[...],...}
//! {"tx_hash":"0202...","block_number":2,"status":"failed","error":"panic","events":[...]}
//! ```
//!
//! ## Example
//! ```rust,ignore
//! use lez_events::event_log::EventLog;
//!
//! let log = EventLog::open("/var/lez/events.jsonl")?;
//! log.append(&receipt)?;
//! let all = log.replay()?;
//! ```
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::runtime::TransactionReceipt;

// ── EventLog ──────────────────────────────────────────────────────────────────
/// Append-only JSON-lines event log backed by a file on disk.
#[derive(Clone)]
pub struct EventLog {
    path:   PathBuf,
    writer: Arc<Mutex<File>>,
}

impl std::fmt::Debug for EventLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EventLog({})", self.path.display())
    }
}

impl EventLog {
    /// Open (or create) an event log at `path`.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        Ok(Self { path, writer: Arc::new(Mutex::new(file)) })
    }

    /// Append a single receipt to the log.  Thread-safe.
    pub fn append(&self, receipt: &TransactionReceipt) -> io::Result<()> {
        let line = serde_json::to_string(receipt)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut w = self.writer.lock().unwrap();
        writeln!(w, "{line}")?;
        w.flush()
    }

    /// Read and deserialise all receipts from the log file.
    /// Malformed lines are skipped with a warning rather than aborting.
    pub fn replay(&self) -> io::Result<Vec<TransactionReceipt>> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut receipts = Vec::new();
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }
            match serde_json::from_str::<TransactionReceipt>(trimmed) {
                Ok(r)  => receipts.push(r),
                Err(e) => eprintln!("event_log: skipping malformed line {}: {e}", i + 1),
            }
        }
        Ok(receipts)
    }

    /// Return the number of receipts currently in the log.
    pub fn len(&self) -> io::Result<usize> {
        Ok(self.replay()?.len())
    }

    /// Return `true` if the log contains no receipts.
    pub fn is_empty(&self) -> io::Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Path to the underlying log file.
    pub fn path(&self) -> &Path { &self.path }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{EventJournal, ReceiptStore, TxStatus, handle_sys_emit_event};
    use tempfile::NamedTempFile;

    const PROG: [u8; 32] = [0xAB; 32];

    fn make_receipt(tx: [u8; 32], block: u64, status: TxStatus) -> TransactionReceipt {
        let store = ReceiptStore::new();
        let mut j = EventJournal::new();
        handle_sys_emit_event(&mut j, PROG, b"test_event");
        store.finalise(tx, block, status, None, None, j);
        store.get_by_hash(&tx).unwrap()
    }

    #[test]
    fn append_and_replay_round_trip() {
        let tmp  = NamedTempFile::new().unwrap();
        let log  = EventLog::open(tmp.path()).unwrap();
        let r1   = make_receipt([0x01; 32], 1, TxStatus::Success);
        let r2   = make_receipt([0x02; 32], 2, TxStatus::Failed);

        log.append(&r1).unwrap();
        log.append(&r2).unwrap();

        let replayed = log.replay().unwrap();
        assert_eq!(replayed.len(), 2);
        assert_eq!(replayed[0].tx_hash, r1.tx_hash);
        assert_eq!(replayed[1].status, TxStatus::Failed);
    }

    #[test]
    fn len_counts_entries() {
        let tmp = NamedTempFile::new().unwrap();
        let log = EventLog::open(tmp.path()).unwrap();
        assert_eq!(log.len().unwrap(), 0);
        log.append(&make_receipt([0x01; 32], 1, TxStatus::Success)).unwrap();
        assert_eq!(log.len().unwrap(), 1);
    }

    #[test]
    fn is_empty_on_new_log() {
        let tmp = NamedTempFile::new().unwrap();
        let log = EventLog::open(tmp.path()).unwrap();
        assert!(log.is_empty().unwrap());
    }

    #[test]
    fn append_is_idempotent_across_handles() {
        let tmp  = NamedTempFile::new().unwrap();
        let log1 = EventLog::open(tmp.path()).unwrap();
        let log2 = EventLog::open(tmp.path()).unwrap();
        log1.append(&make_receipt([0x01; 32], 1, TxStatus::Success)).unwrap();
        log2.append(&make_receipt([0x02; 32], 2, TxStatus::Success)).unwrap();
        assert_eq!(log1.replay().unwrap().len(), 2);
    }

    #[test]
    fn malformed_lines_are_skipped() {
        let tmp = NamedTempFile::new().unwrap();
        // Write one valid and one garbage line directly.
        {
            let mut f = std::fs::OpenOptions::new().append(true).open(tmp.path()).unwrap();
            let r = make_receipt([0x01; 32], 1, TxStatus::Success);
            writeln!(f, "{}", serde_json::to_string(&r).unwrap()).unwrap();
            writeln!(f, "{{not valid json}}").unwrap();
        }
        let log = EventLog::open(tmp.path()).unwrap();
        // Should get 1 valid receipt, not panic.
        assert_eq!(log.replay().unwrap().len(), 1);
    }

    #[test]
    fn events_survive_in_failed_tx_log() {
        let tmp = NamedTempFile::new().unwrap();
        let log = EventLog::open(tmp.path()).unwrap();
        let r   = make_receipt([0xAA; 32], 5, TxStatus::Failed);
        log.append(&r).unwrap();
        let replayed = log.replay().unwrap();
        assert_eq!(replayed[0].status, TxStatus::Failed);
        assert_eq!(replayed[0].event_count(), 1, "event must survive failure in log");
    }
}
