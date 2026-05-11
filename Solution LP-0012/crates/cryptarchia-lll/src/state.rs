use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{
    lottery::{LeadershipOutcome, ProposalEnvelope},
    types::{EpochId, Slot},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProposalState {
    pub slot: Slot,
    pub epoch_id: EpochId,
    pub proposal_id: [u8; 32],
    pub alias: [u8; 32],
    pub announced: bool,
    pub verified: bool,
    pub sealed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProposalHistory {
    pub proposals: Vec<ProposalState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeadershipState {
    pub chain_id: u64,
    pub current_epoch: EpochId,
    pub last_slot: Slot,
    pub last_winner_alias: Option<[u8; 32]>,
    pub accepted: Vec<ProposalState>,
    pub missed_slots: Vec<Slot>,
}

impl LeadershipState {
    pub fn new(chain_id: u64, current_epoch: EpochId) -> Self {
        Self {
            chain_id,
            current_epoch,
            last_slot: 0,
            last_winner_alias: None,
            accepted: Vec::new(),
            missed_slots: Vec::new(),
        }
    }

    pub fn observe_outcome(&mut self, outcome: &LeadershipOutcome) {
        self.last_slot = outcome.slot;
        if outcome.is_winner {
            self.last_winner_alias = Some(outcome.alias);
        }
    }

    pub fn record_proposal(&mut self, envelope: &ProposalEnvelope) {
        let p = ProposalState {
            slot: envelope.announce.slot,
            epoch_id: envelope.announce.epoch_id,
            proposal_id: envelope.announce.payload_commitment,
            alias: envelope.announce.alias,
            announced: true,
            verified: true,
            sealed: true,
        };
        self.accepted.push(p);
    }

    pub fn record_miss(&mut self, slot: Slot) {
        self.missed_slots.push(slot);
    }

    pub fn rotate_epoch(&mut self, new_epoch: EpochId) {
        self.current_epoch = new_epoch;
    }

    pub fn audit(&self) -> ProposalHistory {
        ProposalHistory {
            proposals: self.accepted.clone(),
        }
    }
}
