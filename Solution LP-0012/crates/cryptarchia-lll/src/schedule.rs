use serde::{Deserialize, Serialize};

use crate::{EpochId, Slot};

pub type SlotIndex = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochSchedule {
    pub epoch_length: Slot,
    pub slot_duration_ms: u64,
    pub slots_per_leadership_check: u64,
}

impl EpochSchedule {
    pub fn slot_to_epoch(&self, slot: Slot) -> EpochId {
        if self.epoch_length == 0 {
            0
        } else {
            slot / self.epoch_length
        }
    }

    pub fn epoch_start_slot(&self, epoch: EpochId) -> Slot {
        epoch.saturating_mul(self.epoch_length)
    }

    pub fn slot_within_epoch(&self, slot: Slot) -> Slot {
        if self.epoch_length == 0 {
            slot
        } else {
            slot % self.epoch_length
        }
    }

    pub fn leadership_check_slot(&self, slot: Slot) -> bool {
        if self.slots_per_leadership_check == 0 {
            return true;
        }
        slot % self.slots_per_leadership_check == 0
    }
}
