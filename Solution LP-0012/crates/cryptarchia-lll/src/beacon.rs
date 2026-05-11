use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{crypto::hash32, Slot};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochBeacon {
    pub chain_id: u64,
    pub epoch_id: u64,
    pub seed: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotSeed {
    pub slot: Slot,
    pub seed: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeaconMix {
    pub previous: [u8; 32],
    pub heartbeat: [u8; 32],
    pub committee_root: [u8; 32],
}

impl EpochBeacon {
    pub fn slot_seed(&self, slot: Slot) -> SlotSeed {
        let mut data = Vec::new();
        data.extend_from_slice(&self.seed);
        data.extend_from_slice(&slot.to_le_bytes());
        data.extend_from_slice(b"cryptarchia/slot-seed");
        SlotSeed {
            slot,
            seed: hash32(&data),
        }
    }
}

impl BeaconMix {
    pub fn mix(&self) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(&self.previous);
        data.extend_from_slice(&self.heartbeat);
        data.extend_from_slice(&self.committee_root);
        data.extend_from_slice(b"cryptarchia/beacon-mix");
        hash32(&data)
    }
}
