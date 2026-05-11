//! # RPC Server
//! Minimal blocking HTTP server for the sequencer.
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::runtime::{ReceiptStore, TransactionReceipt};

/// Minimal HTTP/JSON RPC server for the sequencer.
pub fn serve(addr: &str, store: ReceiptStore) {
    let listener = TcpListener::bind(addr).expect("failed to bind RPC port");
    eprintln!("[rpc] listening on http://{addr}");
    for stream in listener.incoming().flatten() {
        let store = store.clone();
        thread::spawn(move || handle_connection(stream, store));
    }
}

fn handle_connection(mut stream: TcpStream, store: ReceiptStore) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() { return; }

    // Drain headers
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() || line == "\r\n" || line.is_empty() { break; }
    }

    let (status, body) = route(&request_line, &store);
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
}

fn route(request_line: &str, store: &ReceiptStore) -> (&'static str, String) {
    if let Some(rest) = request_line.strip_prefix("GET /getTransactionEvents") {
        if let Some(tx) = extract_param(rest, "tx") {
            return match store.get(&tx) {
                Some(r) => ("200 OK", serde_json::to_string_pretty(&r).unwrap_or_default()),
                None    => ("404 Not Found", r#"{"error":"transaction not found"}"#.into()),
            };
        }
    }

    if let Some(rest) = request_line.strip_prefix("GET /getBlockEvents") {
        if let Some(block_str) = extract_param(rest, "block") {
            if let Ok(block) = block_str.parse::<u64>() {
                let receipts = store.get_by_block(block);
                return ("200 OK", serde_json::to_string_pretty(&receipts).unwrap_or_default());
            }
        }
        if let Some(hashes_str) = extract_param(rest, "hashes") {
            if hashes_str == "latest" {
                return ("200 OK", serde_json::to_string_pretty(&store.all()).unwrap_or_default());
            }
            let receipts: Vec<TransactionReceipt> = hashes_str
                .split(',')
                .filter_map(|h| store.get(h.trim()))
                .collect();
            return ("200 OK", serde_json::to_string_pretty(&receipts).unwrap_or_default());
        }
    }

    if request_line.starts_with("GET /health") {
        return ("200 OK", r#"{"status":"ok"}"#.into());
    }

    ("404 Not Found", r#"{"error":"unknown endpoint"}"#.into())
}

fn extract_param(query: &str, key: &str) -> Option<String> {
    let needle = format!("{key}=");
    let start  = query.find(&needle)? + needle.len();
    let rest   = &query[start..];
    let end    = rest.find(|c: char| ['&', ' ', '\r'].contains(&c)).unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{EventJournal, ReceiptStore, TxStatus};

    fn make_store() -> ReceiptStore {
        let store = ReceiptStore::new();
        let mut j = EventJournal::new();
        let _ = j.append([0xAB; 32], b"test_event");
        store.finalise([0x01; 32], 1, TxStatus::Success, None, None, j);
        store
    }

    #[test]
    fn route_found() {
        let store = make_store();
        let tx_hex = "01".repeat(32);
        let req = format!("GET /getTransactionEvents?tx={tx_hex} HTTP/1.1");
        let (status, body) = route(&req, &store);
        assert_eq!(status, "200 OK");
        assert!(body.contains("success"));
    }

    #[test]
    fn route_not_found() {
        let store = make_store();
        let req = "GET /getTransactionEvents?tx=deadbeef HTTP/1.1";
        let (status, _) = route(req, &store);
        assert_eq!(status, "404 Not Found");
    }

    #[test]
    fn health_endpoint() {
        let store = ReceiptStore::new();
        let (status, body) = route("GET /health HTTP/1.1", &store);
        assert_eq!(status, "200 OK");
        assert!(body.contains("ok"));
    }
}
