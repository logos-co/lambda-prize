//! # Minimal Reference Indexer (LP-0012)
//! Reads a JSON receipt store, lists all receipts, and decodes events.
//! Demonstrates [`JsonFileReceiptStore`] and the [`EventStore`] trait.
//!
//! ## Usage
//! ```bash
//! # List receipts from an existing store
//! cargo run --example minimal_indexer -- events.json
//!
//! # Query by status
//! cargo run --example minimal_indexer -- events.json --status failed
//! ```
use lez_events::{
    fnv1a_discriminant,
    decoder::{build_idl, decode_raw},
    runtime::{EventStore, JsonFileReceiptStore},
    receipt::ReceiptEnvelope,
};
use std::{collections::HashMap, env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args   = env::args().skip(1);
    let path       = args.next().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("events.json"));
    let status_arg = args.find(|a| a != "--status")
        .and_then(|_| env::args().skip(1).collect::<Vec<_>>()
            .windows(2)
            .find(|w| w[0] == "--status")
            .map(|w| w[1].clone()));

    let store    = JsonFileReceiptStore::new(&path);
    let receipts = store.list()?;

    let idl = build_idl(&[
        ("event_demo::SuccessEvent",  fnv1a_discriminant("event_demo::SuccessEvent")),
        ("event_demo::FailureEvent",  fnv1a_discriminant("event_demo::FailureEvent")),
        ("event_demo::FriendlyEvent", fnv1a_discriminant("event_demo::FriendlyEvent")),
    ]);

    let filtered: Vec<&ReceiptEnvelope> = receipts.iter().filter(|r| {
        status_arg.as_deref().map_or(true, |s| r.status.to_string() == s)
    }).collect();

    println!("indexed {} / {} receipt(s) from {}", filtered.len(), receipts.len(), path.display());

    for r in &filtered {
        render_receipt(r, &idl);
    }

    Ok(())
}

fn render_receipt(r: &ReceiptEnvelope, idl: &HashMap<[u8; 4], String>) {
    println!(
        "─── tx={} status={} events={}",
        &r.tx_hash[..16.min(r.tx_hash.len())],
        r.status,
        r.events.len()
    );
    if let Some(err) = &r.error {
        println!("    error: {err}");
    }
    for (idx, hex_str) in r.events.iter().enumerate() {
        let bytes = match hex::decode(hex_str) {
            Ok(b) => b,
            Err(_) => { println!("    event[{idx:02}] invalid hex"); continue; }
        };

        // Strip 32-byte program_id prefix if present
        let (program_id, event_bytes) = if bytes.len() >= 32 + 5 {
            (Some(hex::encode(&bytes[..32])), &bytes[32..])
        } else {
            (None, bytes.as_slice())
        };

        match decode_raw(event_bytes, Some(idl)) {
            Ok(decoded) => {
                println!(
                    "    event[{idx:02}] type={:<32} disc={} payload={} bytes prog={}",
                    decoded.type_name.as_deref().unwrap_or("unknown"),
                    hex::encode(decoded.discriminant),
                    decoded.payload.len(),
                    program_id.as_deref()
                        .map(|p| &p[..8.min(p.len())])
                        .unwrap_or("?"),
                );
            }
            Err(e) => println!("    event[{idx:02}] decode error: {e}"),
        }
    }
}
