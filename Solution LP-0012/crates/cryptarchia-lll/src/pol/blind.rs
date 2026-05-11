use crate::pol::commitment::hash_bytes32;

pub fn blind_slot_nonce(
    chain_id: u64,
    epoch_id: u64,
    slot: u64,
    previous_epoch_output: &[u8; 32],
    local_secret: &[u8; 32],
) -> [u8; 32] {
    hash_bytes32(
        "cryptarchia/pol/v2/blind-slot-nonce",
        &[
            &chain_id.to_le_bytes(),
            &epoch_id.to_le_bytes(),
            &slot.to_le_bytes(),
            previous_epoch_output,
            local_secret,
        ],
    )
}

pub fn blind_ticket_nonce(
    chain_id: u64,
    epoch_id: u64,
    slot: u64,
    blinded_slot_nonce: &[u8; 32],
    vrf_output: &[u8; 32],
) -> [u8; 32] {
    hash_bytes32(
        "cryptarchia/pol/v2/blind-ticket-nonce",
        &[
            &chain_id.to_le_bytes(),
            &epoch_id.to_le_bytes(),
            &slot.to_le_bytes(),
            blinded_slot_nonce,
            vrf_output,
        ],
    )
}

pub fn blind_transcript_key(
    chain_id: u64,
    epoch_id: u64,
    previous_epoch_output: &[u8; 32],
    local_secret: &[u8; 32],
) -> [u8; 32] {
    hash_bytes32(
        "cryptarchia/pol/v2/blind-transcript-key",
        &[
            &chain_id.to_le_bytes(),
            &epoch_id.to_le_bytes(),
            previous_epoch_output,
            local_secret,
        ],
    )
}
