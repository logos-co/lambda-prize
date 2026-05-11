# LEZ Event System — LP-0012

[![CI](https://github.com/your-org/lez-event-system/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/lez-event-system/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)

> **λPrize LP-0012 — Event/Log Mechanism for LEZ Program Execution**  
> **Prize:** $1,200 · **Status:** Open · **Effort:** Large

A complete, production-ready structured event system for the **Logos Execution Zone (LEZ)**. This system provides a robust mechanism for LEZ programs to emit typed, deterministically encoded events that are **guaranteed to be persisted in transaction receipts**, even if the transaction fails.

---

## 🚀 Key Features

### 🛡️ Failure-Path Persistence
The core innovation of this system is the **Write-Ahead Event Journal**. Unlike traditional state-coupled event systems, our journal captures events as they are emitted. If a program panics or fails later in execution, the events emitted *up to that point* are still flushed to the transaction receipt. This is critical for debugging and transparency.

### 🔢 Deterministic Encoding
- **Borsh Serialization:** Uses the compact and deterministic Borsh format for all event payloads.
- **FNV-1a Discriminants:** Every event type is identified by a 4-byte discriminant derived from its fully-qualified Rust type name, ensuring no ambiguity across different programs.

### 📏 Strict Resource Enforcement
The system enforces deterministic limits to prevent resource exhaustion and ensure predictable sequencer behavior:
- **Max Event Size:** 64 KiB per single event.
- **Max Transaction Budget:** 1 MiB total event data per transaction.
- **Max Event Count:** 256 events per transaction.
- **Actionable Errors:** Violations return specific `EventError` variants (e.g., `EventTooLarge`) rather than silent failures.

### 🔒 Privacy-First Design
Includes a built-in `emit_encrypted_event!` macro (optional feature) that allows programs to encrypt sensitive event data before it ever leaves the guest environment, aligning with the Logos "private-by-default" philosophy.

---

## 📂 Repository Structure

```text
lez-event-system/
├── crates/
│   ├── lez-events/             # Core SDK (no_std compatible)
│   │   ├── src/lib.rs          # emit_event! macro & EventSchema trait
│   │   ├── src/runtime.rs      # Host-side journal & receipt store
│   │   └── src/rpc.rs          # Minimal HTTP/JSON RPC server
│   └── lez-events-cli/         # CLI tool for decoding & indexing
├── programs/
│   └── event-demo/             # Reference program showing all 4 scenarios
├── docs/
│   └── event-format.md         # Detailed wire-format specification
├── examples/
│   └── minimal_indexer.rs      # Reference indexer implementation
└── scripts/
    └── demo.sh                 # End-to-end automated demo
```

---

## 🛠️ Getting Started

### Prerequisites
- **Rust:** Stable toolchain (>= 1.77)
- **CURL:** For interacting with the RPC server

### Installation & Build
```bash
# Clone and build the entire workspace
cargo build --workspace --all-features
```

### Running the Test Suite
We maintain high test coverage across the SDK, runtime, and example programs:
```bash
cargo test --workspace --all-features
```

### Live Demo
Experience the system in action with our automated demo script. It demonstrates success paths, failure paths (event preservation), and size limit enforcement:
```bash
# Run the demo (requires no special environment)
./scripts/demo.sh
```

---

## 📖 SDK Integration

### 1. Define your Event
Implement the `EventSchema` trait to give your event a stable identity.

```rust
use lez_events::{emit_event, EventSchema};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MyTransfer { 
    pub from: [u8; 32], 
    pub to: [u8; 32], 
    pub amount: u64 
}

impl EventSchema for MyTransfer {
    const NAME: &'static str = "my_app::MyTransfer";
}
```

### 2. Emit the Event
Use the `emit_event!` macro. It returns a `Result<(), EventError>`, allowing for idiomatic error handling.

```rust
fn process_transfer(amount: u64) -> Result<(), lez_events::EventError> {
    // ... logic ...
    emit_event!(MyTransfer { from: [0;32], to: [1;32], amount })
}
```

---

## 🔍 Tooling & Inspection

### Decoder CLI
The `lez-events-cli` tool can decode receipts from local files or live sequencers.

```bash
# Decode a transaction from a live RPC endpoint
cargo run -p lez-events-cli -- decode --tx <TX_HASH> --rpc http://localhost:8080 --pretty
```

### Reference Indexer
The `minimal_indexer` example shows how to poll the sequencer and build a local searchable database of events.

```bash
cargo run --example minimal_indexer -- --rpc http://localhost:8080 --db events.json
```

---

## 📜 Specification
For a deep dive into the binary layout and protocol details, refer to the [Event Format Specification](docs/event-format.md).

---

## ⚖️ License
This project is dual-licensed under the [MIT License](LICENSE-MIT) and [Apache License 2.0](LICENSE-APACHE).
