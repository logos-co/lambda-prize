use alloc::vec::Vec;

use crate::{EventError, EventSchema, EVENT_VERSION, MAX_EVENT_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvelopeEncoding {
    pub version:      u8,
    pub discriminant: [u8; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedEnvelopeRef<'a> {
    pub version:      u8,
    pub discriminant: [u8; 4],
    pub payload:      &'a [u8],
}

#[inline]
pub fn encode_event<E: EventSchema>(event: &E) -> Result<Vec<u8>, EventError> {
    let payload = borsh::to_vec(event).map_err(|_| EventError::SerializationFailed)?;
    encode_event_bytes(&E::DISCRIMINANT, &payload)
}

#[inline]
pub fn encode_event_into<E: EventSchema>(event: &E, out: &mut Vec<u8>) -> Result<(), EventError> {
    let payload = borsh::to_vec(event).map_err(|_| EventError::SerializationFailed)?;
    encode_event_bytes_into(&E::DISCRIMINANT, &payload, out)
}

#[inline]
pub fn encode_event_bytes(discriminant: &[u8; 4], payload: &[u8]) -> Result<Vec<u8>, EventError> {
    if payload.len() > MAX_EVENT_SIZE {
        return Err(EventError::EventTooLarge { size: payload.len(), limit: MAX_EVENT_SIZE });
    }
    let mut out = Vec::with_capacity(1 + 4 + payload.len());
    out.push(EVENT_VERSION);
    out.extend_from_slice(discriminant);
    out.extend_from_slice(payload);
    Ok(out)
}

#[inline]
pub fn encode_event_bytes_into(
    discriminant: &[u8; 4],
    payload:      &[u8],
    out:          &mut Vec<u8>,
) -> Result<(), EventError> {
    if payload.len() > MAX_EVENT_SIZE {
        return Err(EventError::EventTooLarge { size: payload.len(), limit: MAX_EVENT_SIZE });
    }
    out.clear();
    out.reserve_exact(1 + 4 + payload.len());
    out.push(EVENT_VERSION);
    out.extend_from_slice(discriminant);
    out.extend_from_slice(payload);
    Ok(())
}

#[inline]
pub fn decode_envelope_ref(raw: &[u8]) -> Result<DecodedEnvelopeRef<'_>, EventError> {
    if raw.len() < 5 {
        return Err(EventError::InvalidEnvelope(
            alloc::format!("too short: need ≥ 5 bytes, got {}", raw.len())
        ));
    }
    if raw[0] != EVENT_VERSION {
        return Err(EventError::InvalidVersion(raw[0]));
    }
    let discriminant: [u8; 4] = raw[1..5]
        .try_into()
        .map_err(|_| EventError::InvalidEnvelope("missing discriminant".into()))?;
    Ok(DecodedEnvelopeRef { version: raw[0], discriminant, payload: &raw[5..] })
}
