//! # EventBuilder — Fluent API for Constructing and Emitting Events
//!
//! Provides a type-safe, ergonomic builder for creating event wire bytes and
//! optionally appending them to a host-side [`EventJournal`].
//!
//! ## Example (guest side)
//! ```rust,ignore
//! use lez_events::builder::EventBuilder;
//!
//! let bytes = EventBuilder::new("my_program::Transfer")
//!     .build(&Transfer { from: [0u8;32], to: [1u8;32], amount: 100 })?;
//! ```
//!
//! ## Example (host side — sequencer)
//! ```rust,ignore
//! use lez_events::builder::EventBuilder;
//!
//! EventBuilder::new("my_program::Transfer")
//!     .emit_to_journal(&mut journal, program_id, &transfer_event)?;
//! ```
use borsh::BorshSerialize;
use crate::{encode_event_named, EventError, EventSchema, fnv1a_discriminant, MAX_EVENTS_PER_TX, MAX_TX_EVENT_BYTES};

#[cfg(feature = "std")]
use crate::runtime::{EventJournal, handle_sys_emit_event};

// ── EventBuilder ──────────────────────────────────────────────────────────────
/// Fluent builder for constructing event wire bytes.
#[derive(Debug, Clone)]
pub struct EventBuilder {
    type_name: &'static str,
}

impl EventBuilder {
    /// Create a builder for events of the given type name.
    ///
    /// Prefer [`EventBuilder::from_schema`] when the event type implements
    /// [`EventSchema`] — the name is then resolved at compile time.
    pub fn new(type_name: &'static str) -> Self {
        Self { type_name }
    }

    /// Create a builder from a type that implements [`EventSchema`].
    /// The `NAME` constant is used — no string literal needed.
    pub fn from_schema<E: EventSchema>() -> Self {
        Self { type_name: E::NAME }
    }

    /// The type name this builder was created for.
    pub fn type_name(&self) -> &'static str { self.type_name }

    /// The 4-byte FNV-1a discriminant for this event type.
    pub fn discriminant(&self) -> [u8; 4] { fnv1a_discriminant(self.type_name) }

    /// Encode `event` to wire format and return the raw bytes.
    /// Returns `Err(EventTooLarge)` if the serialised payload exceeds the limit.
    pub fn build<E: BorshSerialize>(&self, event: &E) -> Result<alloc::vec::Vec<u8>, EventError> {
        encode_event_named(event, self.type_name)
    }

    /// Encode `event` and append it to a host-side [`EventJournal`].
    /// Returns `Ok(())` on success or a structured [`EventError`] on failure.
    #[cfg(feature = "std")]
    pub fn emit_to_journal<E: BorshSerialize>(
        &self,
        journal:    &mut EventJournal,
        program_id: [u8; 32],
        event:      &E,
    ) -> Result<(), EventError> {
        let bytes = self.build(event)?;
        let rc = handle_sys_emit_event(journal, program_id, &bytes);
        match rc {
            0  => Ok(()),
            -1 => Err(EventError::TxBudgetExceeded {
                used: 0, added: bytes.len(), limit: MAX_TX_EVENT_BYTES,
            }),
            -2 => Err(EventError::TxCountExceeded { used: 0, limit: MAX_EVENTS_PER_TX }),
            -3 => Err(EventError::EventTooLarge { size: bytes.len(), limit: crate::MAX_EVENT_SIZE }),
            c  => Err(EventError::SyscallError(c)),
        }
    }
}

extern crate alloc;

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use crate::{EVENT_VERSION, MAX_EVENT_SIZE};

    #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
    struct Transfer { amount: u64 }

    impl EventSchema for Transfer {
        const NAME: &'static str = "builder_test::Transfer";
    }

    #[test]
    fn build_produces_valid_wire_bytes() {
        let b   = EventBuilder::from_schema::<Transfer>();
        let buf = b.build(&Transfer { amount: 42 }).unwrap();
        assert_eq!(buf[0], EVENT_VERSION);
        let disc = fnv1a_discriminant("builder_test::Transfer");
        assert_eq!(&buf[1..5], &disc);
        let recovered: Transfer = borsh::from_slice(&buf[5..]).unwrap();
        assert_eq!(recovered.amount, 42);
    }

    #[test]
    fn from_schema_uses_const_name() {
        let b = EventBuilder::from_schema::<Transfer>();
        assert_eq!(b.type_name(), "builder_test::Transfer");
        assert_eq!(b.discriminant(), Transfer::DISCRIMINANT);
    }

    #[test]
    fn new_and_from_schema_produce_identical_bytes() {
        let ev   = Transfer { amount: 7 };
        let b1   = EventBuilder::new("builder_test::Transfer").build(&ev).unwrap();
        let b2   = EventBuilder::from_schema::<Transfer>().build(&ev).unwrap();
        assert_eq!(b1, b2);
    }

    #[test]
    fn build_rejects_oversized_payload() {
        #[derive(BorshSerialize)] struct Big { data: alloc::vec::Vec<u8> }
        let b   = EventBuilder::new("builder_test::Big");
        let err = b.build(&Big { data: alloc::vec![0u8; MAX_EVENT_SIZE + 1] }).unwrap_err();
        assert!(matches!(err, EventError::EventTooLarge { .. }));
    }

    #[cfg(feature = "std")]
    #[test]
    fn emit_to_journal_appends_entry() {
        use crate::runtime::EventJournal;
        let mut j = EventJournal::new();
        let prog  = [0xAB; 32];
        EventBuilder::from_schema::<Transfer>()
            .emit_to_journal(&mut j, prog, &Transfer { amount: 99 })
            .unwrap();
        assert_eq!(j.len(), 1);
    }

    #[cfg(feature = "std")]
    #[test]
    fn emit_to_journal_rejects_oversized() {
        use crate::runtime::EventJournal;
        #[derive(BorshSerialize)] struct Big { data: alloc::vec::Vec<u8> }
        let mut j = EventJournal::new();
        let err   = EventBuilder::new("builder_test::Big")
            .emit_to_journal(&mut j, [0u8; 32], &Big { data: alloc::vec![0u8; MAX_EVENT_SIZE + 1] })
            .unwrap_err();
        assert!(matches!(err, EventError::EventTooLarge { .. }));
        assert_eq!(j.len(), 0);
    }
}
