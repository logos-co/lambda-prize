//! # event-demo
//! Example LEZ program demonstrating all event scenarios required by LP-0012:
//! 1. `Success`          — emits typed event, transaction succeeds.
//! 2. `FailAfterEvent`   — emits event then signals failure; event persists.
//! 3. `FailWithoutEvent` — signals failure immediately; empty events array.
//! 4. `SizeLimit`        — attempts to emit oversized event; returns error.
//! 5. `FriendlyMessage`  — emits a human-readable message event.
//! 6. `Private`          — emits an AES-256-GCM encrypted event.
#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use borsh::{BorshDeserialize, BorshSerialize};
use lez_events::{emit_encrypted_event, emit_event, EventError, EventSchema, MAX_EVENT_SIZE};

// ── Instruction enum ──────────────────────────────────────────────────────────
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instruction {
    Success         { value: u64 },
    FailAfterEvent  { reason: String },
    FailWithoutEvent,
    SizeLimit,
    FriendlyMessage { message: String },
    Private         { key: [u8; 32], nonce: [u8; 12], secret: u64 },
}

// ── Event structs ─────────────────────────────────────────────────────────────
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct SuccessEvent   { pub value: u64 }

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct FailureEvent   { pub reason: String }

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct FriendlyEvent  { pub message: String }

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq, Clone)]
pub struct SecretEvent    { pub secret: u64 }

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OversizedEvent { pub data: Vec<u8> }

// ── EventSchema impls ─────────────────────────────────────────────────────────
impl EventSchema for SuccessEvent   { const NAME: &'static str = "event_demo::SuccessEvent";   }
impl EventSchema for FailureEvent   { const NAME: &'static str = "event_demo::FailureEvent";   }
impl EventSchema for FriendlyEvent  { const NAME: &'static str = "event_demo::FriendlyEvent";  }
impl EventSchema for SecretEvent    { const NAME: &'static str = "event_demo::SecretEvent";    }
impl EventSchema for OversizedEvent { const NAME: &'static str = "event_demo::OversizedEvent"; }

// ── ProgramError ──────────────────────────────────────────────────────────────
#[derive(Debug)]
pub enum ProgramError {
    Event(EventError),
    OversizeRequested { requested: usize, limit: usize },
    PanicSimulated(&'static str),
}

impl core::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Event(e) =>
                write!(f, "{e}"),
            Self::OversizeRequested { requested, limit } =>
                write!(f, "requested event size {requested} B exceeds limit {limit} B"),
            Self::PanicSimulated(msg) =>
                write!(f, "simulated failure: {msg}"),
        }
    }
}

impl From<EventError> for ProgramError {
    fn from(e: EventError) -> Self { Self::Event(e) }
}

// ── Entry point ───────────────────────────────────────────────────────────────
pub fn process_instruction(instruction: Instruction) -> Result<(), ProgramError> {
    match instruction {
        Instruction::Success { value } => {
            emit_event!(SuccessEvent { value })?;
            Ok(())
        }

        Instruction::FailAfterEvent { reason } => {
            emit_event!(FailureEvent { reason })?;
            Err(ProgramError::PanicSimulated("intentional failure after event"))
        }

        Instruction::FailWithoutEvent => {
            Err(ProgramError::PanicSimulated("intentional failure before event emission"))
        }

        Instruction::SizeLimit => {
            let ev = OversizedEvent { data: vec![0u8; MAX_EVENT_SIZE + 1] };
            match emit_event!(ev) {
                Ok(())                                        => Ok(()),
                Err(EventError::EventTooLarge { size, limit }) =>
                    Err(ProgramError::OversizeRequested { requested: size, limit }),
                Err(e) => Err(ProgramError::Event(e)),
            }
        }

        Instruction::FriendlyMessage { message } => {
            emit_event!(FriendlyEvent { message })?;
            Ok(())
        }

        Instruction::Private { key, nonce, secret } => {
            emit_encrypted_event!(SecretEvent { secret }, &key, &nonce)
                .map_err(ProgramError::Event)
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn success_path()          { assert!(process_instruction(Instruction::Success { value: 42 }).is_ok()); }
    #[test] fn fail_after_event()      { assert!(matches!(process_instruction(Instruction::FailAfterEvent  { reason: "oops".into() }), Err(ProgramError::PanicSimulated(_)))); }
    #[test] fn fail_without_event()    { assert!(matches!(process_instruction(Instruction::FailWithoutEvent), Err(ProgramError::PanicSimulated(_)))); }
    #[test] fn size_limit()            { assert!(matches!(process_instruction(Instruction::SizeLimit), Err(ProgramError::OversizeRequested { .. }))); }
    #[test] fn friendly_message()      { assert!(process_instruction(Instruction::FriendlyMessage { message: "hello".into() }).is_ok()); }
    #[test] fn private_encrypted()     { assert!(process_instruction(Instruction::Private { key: [0u8; 32], nonce: [0u8; 12], secret: 1337 }).is_ok()); }

    #[test]
    fn discriminant_stable() {
        let d = lez_events::fnv1a_discriminant("event_demo::SuccessEvent");
        assert_eq!(d, SuccessEvent::DISCRIMINANT);
        assert_eq!(d, lez_events::fnv1a_32("event_demo::SuccessEvent"));
    }

    #[test]
    fn schema_names_round_trip() {
        assert_eq!(SuccessEvent::NAME,  "event_demo::SuccessEvent");
        assert_eq!(FailureEvent::NAME,  "event_demo::FailureEvent");
        assert_eq!(FriendlyEvent::NAME, "event_demo::FriendlyEvent");
        assert_eq!(SecretEvent::NAME,   "event_demo::SecretEvent");
    }
}
