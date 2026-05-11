# LEZ Event Format Specification v1.2

## 1. Overview

Events are emitted by LEZ programs via the `emit_event!` macro (or the `emit`
function) and stored in transaction receipts. The runtime preserves emitted
events even when a transaction fails — this is the write-ahead event journal
guarantee.

## 2. Wire Format

Each event stored in a receipt entry is structured as:

```
[program_id: 32 bytes] [version: 1 byte] [discriminant: 4 bytes] [payload: variable]
```

| Field | Size | Description |
|---|---|---|
| `program_id` | 32 bytes | Prepended by the runtime; identifies the emitting program |
| `version` | 1 byte | `0x00` for v1 |
| `discriminant` | 4 bytes (LE) | First 4 bytes of FNV-1a hash of the fully-qualified type name |
| `payload` | variable | Borsh-serialised event struct |

> **Note:** The `program_id` is **not** part of the guest-side event payload. It
> is prepended automatically by the runtime syscall handler.

## 3. Payload Encoding (Borsh)

All event structs are serialised using [Borsh](https://borsh.io/). Rules:

- Integers are **little-endian**.
- Strings are length-prefixed (`u32`) + UTF-8 bytes.
- Vectors are length-prefixed (`u32`) + elements.
- Struct fields are serialised in **declaration order**.
- Enums are serialised as `u8` discriminant + variant data.

## 4. Discriminant Derivation

```
discriminant = fnv1a_32(type_name_str)[0..4]  (little-endian)
```

Where `type_name_str` is the fully-qualified Rust type path (e.g.
`event_demo::SuccessEvent`). This is defined in the `EventSchema::NAME`
constant for the type.

The function `fnv1a_32` is an alias for `fnv1a_discriminant` — both compute
the same 32-bit FNV-1a hash truncated to 4 bytes.

## 5. Size Limits

| Limit | Value |
|---|---|
| Max single event payload | 64 KiB (65 536 bytes) |
| Max total event bytes per transaction | 1 MiB (1 048 576 bytes) |
| Max events per transaction | 256 |

Violations return deterministic error codes from `sys_emit_event`:

| Code | Meaning |
|---|---|
| `0` | Success |
| `-1` | Transaction byte budget exceeded |
| `-2` | Transaction event count exceeded |
| `-3` | Single event exceeds 64 KiB |

## 6. Error Behaviour

- **Too-large single event**: rejected immediately; `EventTooLarge { size, limit }`.
- **Transaction byte budget exhausted**: rejected immediately; `TxBudgetExceeded { used, added, limit }`.
- **Transaction event count exceeded**: rejected immediately; `TxCountExceeded { used, limit }`.
- **Invalid envelope**: `decode_envelope` returns `InvalidEnvelope`; decoders fail closed.
- **Unknown version byte**: `InvalidVersion(v)`; decoders must reject and not guess.

## 7. Transaction Receipt Schema

```json
{
  "tx_hash": "0x...",
  "status": "success | failed",
  "error": "optional error message",
  "state_root": "0x... (only on success)",
  "events": [
    "<hex-encoded: program_id(32) + version(1) + discriminant(4) + payload>"
  ]
}
```

Events are **always present** in the receipt regardless of transaction outcome.
This is the core guarantee of the write-ahead event journal.

## 8. Encrypted Events

LEZ programs can optionally emit encrypted events using **AES-256-GCM** via
`emit_encrypted_event!`. The `aes-gcm` dependency lives in `lez-events` —
calling crates only need to enable the `encryption` feature; no direct
`aes-gcm` dependency is required.

- **Wire format**: `[program_id:32][version:1][discriminant:4][ciphertext:variable]`
- **Decryption**: Requires the 32-byte symmetric key and 12-byte nonce.
- **Privacy**: Program ID, version, and discriminant remain public; payload is shielded.

## 9. Versioning

Version `0x00` is defined in this document. Future versions may add optional
trailing fields but must remain backward-compatible. Unknown version bytes
**must be rejected** by decoders (`InvalidVersion(v)` error) — fail closed.

## 10. Example (Hexdump)

For a `SuccessEvent { value: 42u64 }` emitted by program `0xAB...AB`:

```
program_id:    abababab...ab (32 bytes)
version:       00
discriminant:  e3 8f 1a 02   (FNV-1a of "event_demo::SuccessEvent")
payload:       2a 00 00 00 00 00 00 00  (42u64 little-endian, Borsh)
```

Full hex entry:
```
abababababababababababababababababababababababababababababababababab
00 e38f1a02 2a00000000000000
```
