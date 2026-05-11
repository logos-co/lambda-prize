use alloc::string::String;
use alloc::vec::Vec;
use std::collections::HashMap;

use crate::{decoder::decode_hex_envelopes, codec::encode_event, EventSchema};

pub fn measure_encode_batch<E: EventSchema>(events: &[E]) -> usize {
    let mut total = 0usize;
    for ev in events {
        if let Ok(bytes) = encode_event(ev) {
            total += bytes.len();
        }
    }
    total
}

pub fn decode_many(
    hexes: &[String],
    idl:   Option<&HashMap<[u8; 4], String>>,
) -> Vec<Result<crate::receipt::DecodedEnvelope, crate::EventError>> {
    decode_hex_envelopes(hexes, idl)
}
