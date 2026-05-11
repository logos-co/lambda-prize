use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{
    beacon::EpochBeacon,
    crypto::{derive_alias, derive_ticket, ticket_below_threshold, NodeSecret},
    policy::{ProposalPolicy, ProposalPolicyDecision},
    proof::{build_leadership_proof, LeadershipProof, LeadershipPublicInputs},
    schedule::EpochSchedule,
    stake::{effective_leader_threshold, StakeTable},
    types::{ChainId, EpochId, LeaderAlias, Slot, WinnerStatus},
    LllError, LllResult,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LotteryConfig {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub base_threshold: u128,
    pub max_threshold: u128,
    pub min_stake_for_win: u128,
    pub enable_hidden_aliases: bool,
}

impl LotteryConfig {
    pub fn strict_private(chain_id: ChainId, epoch_id: EpochId) -> Self {
        Self {
            chain_id,
            epoch_id,
            base_threshold: u128::MAX / 2,
            max_threshold: u128::MAX,
            min_stake_for_win: 1,
            enable_hidden_aliases: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LotteryTicket {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub slot: Slot,
    pub alias: [u8; 32],
    pub ticket: [u8; 32],
    pub threshold: u128,
    pub stake: u128,
    pub total_stake: u128,
    pub winner: WinnerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProposalAnnounce {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub slot: Slot,
    pub alias: [u8; 32],
    pub ticket_digest: [u8; 32],
    pub payload_commitment: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProposalEnvelope {
    pub announce: ProposalAnnounce,
    pub proof: LeadershipProof,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LeadershipOutcome {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub slot: Slot,
    pub alias: [u8; 32],
    pub ticket: [u8; 32],
    pub threshold: u128,
    pub is_winner: bool,
    pub next_check_in_slots: u64,
    pub proof: Option<LeadershipProof>,
    pub announce: Option<ProposalAnnounce>,
}

#[derive(Debug, Clone)]
pub struct LocalLeadershipLottery {
    pub config: LotteryConfig,
    pub schedule: EpochSchedule,
    pub node_secret: NodeSecret,
    pub validator_root: [u8; 32],
    pub beacon_seed: [u8; 32],
    pub validator_table: StakeTable,
    pub policy: ProposalPolicy,
}

impl LocalLeadershipLottery {
    pub fn new(
        config: LotteryConfig,
        schedule: EpochSchedule,
        node_secret: NodeSecret,
        validator_table: StakeTable,
        beacon_seed: [u8; 32],
        policy: ProposalPolicy,
    ) -> LllResult<Self> {
        validator_table.validate()?;
        let root = validator_table.validator_root();
        Ok(Self {
            config,
            schedule,
            node_secret,
            validator_root: root.root,
            beacon_seed,
            validator_table,
            policy,
        })
    }

    pub fn current_epoch(&self, slot: Slot) -> EpochId {
        self.schedule.slot_to_epoch(slot)
    }

    pub fn alias(&self, epoch_id: EpochId) -> LeaderAlias {
        derive_alias(
            self.config.chain_id,
            epoch_id,
            &self.beacon_seed,
            &self.node_secret.secret,
        )
    }

    pub fn node_commitment(&self, epoch_id: EpochId) -> [u8; 32] {
        crate::crypto::hash32(
            &[
                self.alias(epoch_id).0.as_slice(),
                &self.beacon_seed,
                &self.config.chain_id.to_le_bytes(),
            ]
            .concat(),
        )
    }

    pub fn evaluate_slot(&self, slot: Slot) -> LllResult<LeadershipOutcome> {
        let epoch_id = self.current_epoch(slot);
        let _epoch_beacon = EpochBeacon {
            chain_id: self.config.chain_id,
            epoch_id,
            seed: self.beacon_seed,
        };

        let stake = self
            .validator_table
            .weight_of(&self.node_commitment(epoch_id))
            .map(|w| w.effective)
            .unwrap_or(0);

        let total_stake = self.validator_table.total_stake();
        if total_stake == 0 {
            return Err(LllError::ZeroTotalStake);
        }

        let alias = self.alias(epoch_id);
        let ticket = derive_ticket(
            &self.node_secret,
            self.config.chain_id,
            epoch_id,
            slot,
            &self.beacon_seed,
            stake,
            total_stake,
            &self.validator_root,
        );

        let threshold = effective_leader_threshold(stake, total_stake, self.config.base_threshold)
            .min(self.config.max_threshold);

        let is_winner = stake >= self.config.min_stake_for_win
            && ticket_below_threshold(&ticket, threshold);

        let next_check = self.schedule.slots_per_leadership_check.max(1);

        if !is_winner {
            return Ok(LeadershipOutcome {
                chain_id: self.config.chain_id,
                epoch_id,
                slot,
                alias: alias.0,
                ticket,
                threshold,
                is_winner: false,
                next_check_in_slots: next_check,
                proof: None,
                announce: None,
            });
        }

        let public_inputs = LeadershipPublicInputs {
            chain_id: self.config.chain_id,
            epoch_id,
            slot,
            beacon_seed: self.beacon_seed,
            validator_root: self.validator_root,
            stake,
            total_stake,
            threshold,
            ticket,
            alias: alias.0,
        };

        let proof = build_leadership_proof(&self.node_secret, &self.beacon_seed, public_inputs)?;
        let payload_commitment = crate::crypto::hash32(&proof.public_inputs.ticket);
        let announce = ProposalAnnounce {
            chain_id: self.config.chain_id,
            epoch_id,
            slot,
            alias: alias.0,
            ticket_digest: crate::crypto::hash32(&ticket),
            payload_commitment,
        };

        if matches!(
            self.policy.decide(true, true, true),
            ProposalPolicyDecision::Deny
        ) {
            return Err(LllError::ProposalDenied(
                "proposal policy denied a valid win".into(),
            ));
        }

        Ok(LeadershipOutcome {
            chain_id: self.config.chain_id,
            epoch_id,
            slot,
            alias: alias.0,
            ticket,
            threshold,
            is_winner: true,
            next_check_in_slots: next_check,
            proof: Some(proof),
            announce: Some(announce),
        })
    }

    pub fn build_envelope(&self, slot: Slot, payload: Vec<u8>) -> LllResult<ProposalEnvelope> {
        let outcome = self.evaluate_slot(slot)?;
        if !outcome.is_winner {
            return Err(LllError::ProposalDenied("not leader for this slot".into()));
        }

        let proof = outcome.proof.ok_or(LllError::UnknownProposer)?;
        let announce = outcome.announce.ok_or(LllError::UnknownProposer)?;

        Ok(ProposalEnvelope {
            announce,
            proof,
            payload,
        })
    }

    pub fn verify_envelope(&self, envelope: &ProposalEnvelope) -> LllResult<()> {
        crate::proof::verify_leadership_proof(&envelope.proof)?;
        if envelope.announce.alias != envelope.proof.header.alias.0 {
            return Err(LllError::CommitmentMismatch);
        }
        if crate::crypto::hash32(&envelope.payload) != envelope.announce.payload_commitment {
            return Err(LllError::CommitmentMismatch);
        }
        Ok(())
    }

    pub fn evaluate_slot_with_pol<B: crate::pol::LeadershipBackend>(
        &self,
        engine: &crate::pol::ProofOfLeadershipEngine<B>,
        slot: Slot,
        epoch_seed: [u8; 32],
        beacon_seed: [u8; 32],
        validator_root: [u8; 32],
        stake_root: [u8; 32],
    ) -> LllResult<Option<crate::pol::LeadershipClaimV2>> {
        engine.evaluate_slot(slot, epoch_seed, beacon_seed, validator_root, stake_root)
    }
}
