//! # EventBus — Typed Pub/Sub Event Bus
//!
//! Routes decoded events to typed subscribers using discriminant-based dispatch.
//! Each subscriber receives a clone of matching events via an `mpsc` channel.
//!
//! ## Architecture
//! ```text
//! ReceiptStore  ──publish──►  EventBus  ──route──►  Subscriber<Transfer>
//!                                       ──route──►  Subscriber<FailureEvent>
//! ```
//!
//! ## Example
//! ```rust,ignore
//! use lez_events::bus::EventBus;
//!
//! let bus = EventBus::new();
//! let rx  = bus.subscribe::<Transfer>();
//!
//! bus.publish_receipt(&receipt);   // routes all events in the receipt
//!
//! while let Ok(env) = rx.try_recv() {
//!     println!("Transfer amount: {}", env.decoded.amount);
//! }
//! ```
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use std::sync::mpsc::{self, Receiver, SyncSender};

use borsh::BorshDeserialize;

use crate::{
    EventSchema,
    runtime::TransactionReceipt,
};

// ── Envelope ──────────────────────────────────────────────────────────────────
/// A decoded event delivered to a subscriber.
#[derive(Debug, Clone)]
pub struct Envelope<E> {
    /// The decoded event payload.
    pub decoded:    E,
    /// Transaction hash (hex) this event came from.
    pub tx_hash:    String,
    /// Block number of the transaction.
    pub block_number: u64,
    /// Index of this event within the transaction.
    pub event_index: usize,
}

// ── Internal subscriber trait ─────────────────────────────────────────────────
trait AnySubscriber: Send + Sync {
    /// Try to decode `event_wire_bytes` and send to the channel.
    /// `event_wire_bytes` is the slice *after* the 32-byte program_id prefix.
    fn dispatch(
        &self,
        event_bytes:  &[u8],
        tx_hash:      &str,
        block_number: u64,
        event_index:  usize,
    );
}

struct TypedSubscriber<E: BorshDeserialize + Clone + Send + 'static> {
    tx: SyncSender<Envelope<E>>,
}

impl<E: BorshDeserialize + Clone + Send + 'static> AnySubscriber for TypedSubscriber<E> {
    fn dispatch(
        &self,
        event_bytes:  &[u8],
        tx_hash:      &str,
        block_number: u64,
        event_index:  usize,
    ) {
        // Wire layout (after program_id): [version(1)][disc(4)][borsh_payload...]
        if event_bytes.len() < 5 { return; }
        let payload = &event_bytes[5..];
        if let Ok(decoded) = borsh::from_slice::<E>(payload) {
            let env = Envelope {
                decoded,
                tx_hash:      tx_hash.to_string(),
                block_number,
                event_index,
            };
            // Best-effort send; if the channel is full or closed, drop silently.
            let _ = self.tx.try_send(env);
        }
    }
}

type SubscriberMap = HashMap<[u8; 4], Vec<Box<dyn AnySubscriber>>>;

// ── EventBus ──────────────────────────────────────────────────────────────────
/// Thread-safe typed pub/sub event bus.
#[derive(Clone, Default)]
pub struct EventBus {
    /// discriminant → list of subscribers for that event type.
    subs: Arc<Mutex<SubscriberMap>>,
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n: usize = self.subs.lock().unwrap().values().map(|v| v.len()).sum();
        write!(f, "EventBus({n} subscribers)")
    }
}

impl EventBus {
    /// Create a new empty event bus.
    pub fn new() -> Self { Self::default() }

    /// Subscribe to all events of type `E` (must implement `EventSchema`).
    /// Returns a channel receiver; the bus holds the sender.
    /// `capacity` is the bounded channel buffer size (default: 256).
    pub fn subscribe<E>(&self) -> Receiver<Envelope<E>>
    where
        E: EventSchema + BorshDeserialize + Clone + Send + 'static,
    {
        self.subscribe_with_capacity::<E>(256)
    }

    /// Subscribe with an explicit channel buffer capacity.
    pub fn subscribe_with_capacity<E>(&self, capacity: usize) -> Receiver<Envelope<E>>
    where
        E: EventSchema + BorshDeserialize + Clone + Send + 'static,
    {
        let (tx, rx) = mpsc::sync_channel::<Envelope<E>>(capacity);
        let sub = Box::new(TypedSubscriber { tx });
        self.subs
            .lock()
            .unwrap()
            .entry(E::DISCRIMINANT)
            .or_default()
            .push(sub);
        rx
    }

    /// Publish all events in a receipt to matching subscribers.
    pub fn publish_receipt(&self, receipt: &TransactionReceipt) {
        let subs = self.subs.lock().unwrap();
        for (idx, ev_hex) in receipt.events.iter().enumerate() {
            let Ok(bytes) = hex::decode(ev_hex) else { continue };
            // bytes = [program_id(32)][version(1)][disc(4)][payload...]
            if bytes.len() < 37 { continue; }
            // version byte at index 32; discriminant at 33..37
            let disc: [u8; 4] = bytes[33..37].try_into().unwrap();
            if let Some(subscribers) = subs.get(&disc) {
                for sub in subscribers {
                    sub.dispatch(
                        &bytes[32..],          // version + disc + payload
                        &receipt.tx_hash,
                        receipt.block_number,
                        idx,
                    );
                }
            }
        }
    }

    /// Publish all receipts from an iterator.
    pub fn publish_all<'a>(&self, receipts: impl IntoIterator<Item = &'a TransactionReceipt>) {
        for r in receipts { self.publish_receipt(r); }
    }

    /// Number of distinct event types currently subscribed.
    pub fn subscription_count(&self) -> usize {
        self.subs.lock().unwrap().len()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use crate::{
        builder::EventBuilder,
        runtime::{EventJournal, handle_sys_emit_event, ReceiptStore, TxStatus},
    };

    const PROG: [u8; 32] = [0xAB; 32];

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
    struct Transfer { amount: u64 }
    impl EventSchema for Transfer { const NAME: &'static str = "bus_test::Transfer"; }

    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
    struct Mint { amount: u64 }
    impl EventSchema for Mint { const NAME: &'static str = "bus_test::Mint"; }

    fn make_receipt(events: &[(&[u8], [u8; 32])], block: u64) -> TransactionReceipt {
        let store = ReceiptStore::new();
        let mut j = EventJournal::new();
        for (ev, prog) in events {
            handle_sys_emit_event(&mut j, *prog, ev);
        }
        let tx = [block as u8; 32];
        store.finalise(tx, block, TxStatus::Success, None, None, j);
        store.get_by_hash(&tx).unwrap()
    }

    #[test]
    fn subscriber_receives_matching_event() {
        let bus = EventBus::new();
        let rx  = bus.subscribe::<Transfer>();

        let ev_bytes = EventBuilder::from_schema::<Transfer>()
            .build(&Transfer { amount: 42 }).unwrap();
        let receipt  = make_receipt(&[(&ev_bytes, PROG)], 1);
        bus.publish_receipt(&receipt);

        let env = rx.try_recv().unwrap();
        assert_eq!(env.decoded.amount, 42);
        assert_eq!(env.block_number, 1);
        assert_eq!(env.event_index, 0);
    }

    #[test]
    fn subscriber_does_not_receive_other_type() {
        let bus     = EventBus::new();
        let rx_mint = bus.subscribe::<Mint>();

        let ev_bytes = EventBuilder::from_schema::<Transfer>()
            .build(&Transfer { amount: 7 }).unwrap();
        let receipt  = make_receipt(&[(&ev_bytes, PROG)], 1);
        bus.publish_receipt(&receipt);

        assert!(rx_mint.try_recv().is_err(), "Mint subscriber should not receive Transfer");
    }

    #[test]
    fn multiple_subscribers_same_type_both_receive() {
        let bus = EventBus::new();
        let rx1 = bus.subscribe::<Transfer>();
        let rx2 = bus.subscribe::<Transfer>();

        let ev_bytes = EventBuilder::from_schema::<Transfer>()
            .build(&Transfer { amount: 99 }).unwrap();
        let receipt  = make_receipt(&[(&ev_bytes, PROG)], 1);
        bus.publish_receipt(&receipt);

        assert_eq!(rx1.try_recv().unwrap().decoded.amount, 99);
        assert_eq!(rx2.try_recv().unwrap().decoded.amount, 99);
    }

    #[test]
    fn multiple_events_in_receipt_routed_correctly() {
        let bus      = EventBus::new();
        let rx_xfer  = bus.subscribe::<Transfer>();
        let rx_mint  = bus.subscribe::<Mint>();

        let xfer = EventBuilder::from_schema::<Transfer>().build(&Transfer { amount: 1 }).unwrap();
        let mint = EventBuilder::from_schema::<Mint>().build(&Mint { amount: 2 }).unwrap();
        let receipt  = make_receipt(&[(&xfer, PROG), (&mint, PROG)], 1);
        bus.publish_receipt(&receipt);

        assert_eq!(rx_xfer.try_recv().unwrap().decoded.amount, 1);
        assert_eq!(rx_mint.try_recv().unwrap().decoded.amount, 2);
    }

    #[test]
    fn publish_all_processes_multiple_receipts() {
        let bus = EventBus::new();
        let rx  = bus.subscribe::<Transfer>();

        let ev1 = EventBuilder::from_schema::<Transfer>().build(&Transfer { amount: 10 }).unwrap();
        let ev2 = EventBuilder::from_schema::<Transfer>().build(&Transfer { amount: 20 }).unwrap();
        let r1  = make_receipt(&[(&ev1, PROG)], 1);
        let r2  = make_receipt(&[(&ev2, PROG)], 2);
        bus.publish_all([&r1, &r2]);

        let a1 = rx.try_recv().unwrap().decoded.amount;
        let a2 = rx.try_recv().unwrap().decoded.amount;
        let mut amounts = [a1, a2];
        amounts.sort_unstable();
        assert_eq!(amounts, [10, 20]);
    }

    #[test]
    fn subscription_count_tracks_types() {
        let bus = EventBus::new();
        assert_eq!(bus.subscription_count(), 0);
        let _r1 = bus.subscribe::<Transfer>();
        assert_eq!(bus.subscription_count(), 1);
        let _r2 = bus.subscribe::<Mint>();
        assert_eq!(bus.subscription_count(), 2);
        // Second Transfer subscriber doesn't increase type count.
        let _r3 = bus.subscribe::<Transfer>();
        assert_eq!(bus.subscription_count(), 2);
    }

    #[test]
    fn failed_tx_events_still_routed() {
        let bus = EventBus::new();
        let rx  = bus.subscribe::<Transfer>();

        let store = ReceiptStore::new();
        let mut j = EventJournal::new();
        let ev = EventBuilder::from_schema::<Transfer>().build(&Transfer { amount: 55 }).unwrap();
        handle_sys_emit_event(&mut j, PROG, &ev);
        store.finalise([0x99; 32], 5, TxStatus::Failed, Some("oops".into()), None, j);
        let receipt = store.get_by_hash(&[0x99; 32]).unwrap();

        bus.publish_receipt(&receipt);
        assert_eq!(rx.try_recv().unwrap().decoded.amount, 55);
    }
}
