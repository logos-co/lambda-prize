use alloc::vec::Vec;

use crate::{codec::encode_event_into, EventError, EventSchema};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchEnvelope {
    pub count:       usize,
    pub total_bytes: usize,
    pub bytes:       Vec<u8>,
}

impl BatchEnvelope {
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone)]
pub struct BatchEncoder {
    scratch:    Vec<u8>,
    max_events: usize,
    max_bytes:  usize,
}

impl Default for BatchEncoder {
    fn default() -> Self {
        Self {
            scratch:    Vec::with_capacity(1024),
            max_events: crate::MAX_EVENTS_PER_TX,
            max_bytes:  crate::MAX_TX_EVENT_BYTES,
        }
    }
}

impl BatchEncoder {
    pub fn with_limits(max_events: usize, max_bytes: usize) -> Self {
        Self { scratch: Vec::with_capacity(1024), max_events, max_bytes }
    }

    pub fn clear(&mut self) {
        self.scratch.clear();
    }

    pub fn encode_one<E: EventSchema>(&mut self, event: &E) -> Result<&[u8], EventError> {
        encode_event_into(event, &mut self.scratch)?;
        Ok(&self.scratch)
    }

    pub fn push_encoded(&mut self, encoded: &[u8]) -> Result<(), EventError> {
        if encoded.len() > self.max_bytes {
            return Err(EventError::EventTooLarge {
                size:  encoded.len(),
                limit: self.max_bytes,
            });
        }
        if self.scratch.len() + encoded.len() > self.max_bytes {
            return Err(EventError::TxBudgetExceeded {
                used:  self.scratch.len(),
                added: encoded.len(),
                limit: self.max_bytes,
            });
        }
        self.scratch.extend_from_slice(encoded);
        Ok(())
    }

    pub fn finish(self, count: usize) -> Result<BatchEnvelope, EventError> {
        if count > self.max_events {
            return Err(EventError::TxCountExceeded { used: count, limit: self.max_events });
        }
        Ok(BatchEnvelope { count, total_bytes: self.scratch.len(), bytes: self.scratch })
    }

    pub fn encode_batch<E, I>(&mut self, events: I) -> Result<BatchEnvelope, EventError>
    where
        E: EventSchema,
        I: IntoIterator<Item = E>,
    {
        self.clear();
        let mut count = 0usize;

        for event in events {
            if count >= self.max_events {
                return Err(EventError::TxCountExceeded {
                    used:  count,
                    limit: self.max_events,
                });
            }
            let encoded = crate::codec::encode_event(&event)?;
            if self.scratch.len() + encoded.len() > self.max_bytes {
                return Err(EventError::TxBudgetExceeded {
                    used:  self.scratch.len(),
                    added: encoded.len(),
                    limit: self.max_bytes,
                });
            }
            self.scratch.extend_from_slice(&encoded);
            count += 1;
        }

        if count > self.max_events {
            return Err(EventError::TxCountExceeded { used: count, limit: self.max_events });
        }
        let bytes = core::mem::replace(&mut self.scratch, Vec::with_capacity(1024));
        Ok(BatchEnvelope { count, total_bytes: bytes.len(), bytes })
    }
}
