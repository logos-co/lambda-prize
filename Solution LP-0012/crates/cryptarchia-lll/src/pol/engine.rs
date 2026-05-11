use crate::crypto::{derive_alias, derive_ticket, ticket_below_threshold, NodeSecret};
use crate::pol::blind::{blind_slot_nonce, blind_ticket_nonce};
use crate::pol::commitment::{commitment_bytes, commitment_u128, hash_bytes32};
use crate::pol::difficulty::{next_threshold, DifficultyPolicy};
use crate::pol::proof::{
    proof_commitment, LeadershipBackend, SignatureTranscriptBackend,
};
use crate::pol::types::{
    LeadershipClaimStatus, LeadershipClaimV2, LeadershipPublicInputsV2, LeadershipWitnessV2,
    ProofOfLeadershipConfig,
};
use crate::pol::verifier::verify_leadership_claim;
use crate::pol::vrf::{derive_epoch_vrf_keypair, prove_vrf};
use crate::types::EpochId;
use crate::{EpochSchedule, Slot, StakeTable, LllError, LllResult};

#[derive(Clone, Debug)]
pub struct ProofOfLeadershipEngine<B = SignatureTranscriptBackend> {
    pub config: ProofOfLeadershipConfig,
    pub schedule: EpochSchedule,
    pub node_secret: NodeSecret,
    pub node_commitment: [u8; 32],
    pub previous_epoch_output: [u8; 32],
    pub validator_table: StakeTable,
    pub difficulty_policy: DifficultyPolicy,
    pub backend: B,
}

impl ProofOfLeadershipEngine<SignatureTranscriptBackend> {
    pub fn default_for_node(
        config: ProofOfLeadershipConfig,
        schedule: EpochSchedule,
        node_secret: NodeSecret,
        node_commitment: [u8; 32],
        previous_epoch_output: [u8; 32],
        validator_table: StakeTable,
    ) -> Self {
        Self {
            config,
            schedule,
            node_secret,
            node_commitment,
            previous_epoch_output,
            validator_table,
            difficulty_policy: DifficultyPolicy::default(),
            backend: SignatureTranscriptBackend,
        }
    }
}

impl<B: LeadershipBackend> ProofOfLeadershipEngine<B> {
    pub fn new(
        config: ProofOfLeadershipConfig,
        schedule: EpochSchedule,
        node_secret: NodeSecret,
        node_commitment: [u8; 32],
        previous_epoch_output: [u8; 32],
        validator_table: StakeTable,
        difficulty_policy: DifficultyPolicy,
        backend: B,
    ) -> Self {
        Self {
            config,
            schedule,
            node_secret,
            node_commitment,
            previous_epoch_output,
            validator_table,
            difficulty_policy,
            backend,
        }
    }

    pub fn epoch_id_for_slot(&self, slot: Slot) -> EpochId {
        self.schedule.slot_to_epoch(slot)
    }

    pub fn stake_for_node(&self) -> u128 {
        self.validator_table
            .validators
            .iter()
            .find(|v| v.node_commitment == self.node_commitment)
            .map(|v| v.stake)
            .unwrap_or(0)
    }

    pub fn leader_identity_commitment(
        &self,
        epoch_seed: &[u8; 32],
        epoch_id: EpochId,
    ) -> [u8; 32] {
        let alias = derive_alias(
            self.config.chain_id,
            epoch_id,
            epoch_seed,
            &self.node_secret.secret,
        );

        hash_bytes32(
            "cryptarchia/pol/v2/leader-identity",
            &[
                &alias.0,
                &self.node_commitment,
                &epoch_seed[..],
                &self.previous_epoch_output,
            ],
        )
    }

    pub fn evaluate_slot(
        &self,
        slot: Slot,
        epoch_seed: [u8; 32],
        beacon_seed: [u8; 32],
        validator_root: [u8; 32],
        stake_root: [u8; 32],
    ) -> LllResult<Option<LeadershipClaimV2>> {
        let epoch_id = self.epoch_id_for_slot(slot);
        let stake = self.stake_for_node();
        let total_stake = self.validator_table.total_stake();

        if stake < self.config.min_stake_for_win || total_stake == 0 {
            return Ok(None);
        }

        let slot_nonce = blind_slot_nonce(
            self.config.chain_id,
            epoch_id,
            slot,
            &self.previous_epoch_output,
            &self.node_secret.secret,
        );

        let vrf_keypair = derive_epoch_vrf_keypair(&self.node_secret.secret, &epoch_seed);
        let vrf_message = hash_bytes32(
            "cryptarchia/pol/v2/vrf-message",
            &[
                &self.config.chain_id.to_le_bytes(),
                &epoch_id.to_le_bytes(),
                &slot.to_le_bytes(),
                &slot_nonce,
                &beacon_seed,
                &validator_root,
                &stake_root,
            ],
        );

        let vrf = prove_vrf(&vrf_keypair, &vrf_message);
        let blinded_ticket_nonce = blind_ticket_nonce(
            self.config.chain_id,
            epoch_id,
            slot,
            &slot_nonce,
            &vrf.output,
        );

        let threshold = next_threshold(
            self.difficulty_policy,
            epoch_id,
            self.config.target_win_ppm,
            self.config.target_win_ppm,
            stake,
        )
        .threshold;

        let ticket = derive_ticket(
            &self.node_secret,
            self.config.chain_id,
            epoch_id,
            slot,
            &blinded_ticket_nonce,
            stake,
            total_stake,
            &validator_root,
        );

        if !ticket_below_threshold(&ticket, threshold) {
            return Ok(None);
        }

        let leader_commitment = self.leader_identity_commitment(&epoch_seed, epoch_id);
        let stake_commitment = commitment_u128("cryptarchia/pol/v2/stake", stake);
        let total_stake_commitment =
            commitment_u128("cryptarchia/pol/v2/total-stake", total_stake);
        let threshold_commitment =
            commitment_u128("cryptarchia/pol/v2/threshold-value", threshold);
        let slot_nonce_commitment = commitment_bytes("cryptarchia/pol/v2/slot-nonce", &slot_nonce);
        let ticket_commitment = commitment_bytes("cryptarchia/pol/v2/ticket", &ticket);
        let mut proposal_data = alloc::vec::Vec::new();
        proposal_data.extend_from_slice(&self.config.chain_id.to_le_bytes());
        proposal_data.extend_from_slice(&epoch_id.to_le_bytes());
        proposal_data.extend_from_slice(&slot.to_le_bytes());
        proposal_data.extend_from_slice(&beacon_seed);
        proposal_data.extend_from_slice(&validator_root);
        proposal_data.extend_from_slice(&stake_root);
        proposal_data.extend_from_slice(&ticket);
        proposal_data.extend_from_slice(&threshold.to_le_bytes());
        let proposal_commitment =
            commitment_bytes("cryptarchia/pol/v2/proposal", &proposal_data);

        let public_inputs = LeadershipPublicInputsV2 {
            version: self.config.proof_version,
            chain_id: self.config.chain_id,
            epoch_id,
            slot,
            epoch_seed,
            previous_epoch_output_commitment: commitment_bytes(
                "cryptarchia/pol/v2/previous-epoch-output",
                &self.previous_epoch_output,
            ),
            beacon_seed,
            validator_root,
            stake_root,
            leader_identity_commitment: leader_commitment,
            slot_nonce_commitment,
            threshold_commitment,
            ticket_commitment,
            proposal_commitment,
        };

        let witness = LeadershipWitnessV2 {
            leader_secret: self.node_secret.secret,
            epoch_vrf_secret: vrf_keypair.secret,
            blinded_slot_nonce: slot_nonce,
            stake,
            total_stake,
            threshold,
            ticket,
            vrf_output: vrf.output,
            vrf_proof: vrf.proof,
            leader_identity_commitment: leader_commitment,
            stake_commitment,
            total_stake_commitment,
            threshold_commitment,
            ticket_commitment,
        };

        let proof = self.backend.prove(&public_inputs, &witness).map_err(|e| {
            LllError::ProofVerificationFailed(alloc::format!("backend prove failed: {e}"))
        })?;

        let claim_commitment = proof_commitment(
            &public_inputs,
            &proof.witness_commitment,
            &proof.challenge_digest,
        );

        Ok(Some(LeadershipClaimV2 {
            public_inputs,
            proof,
            claim_commitment,
            status: LeadershipClaimStatus::Proven,
        }))
    }

    pub fn verify_claim(&self, claim: &LeadershipClaimV2) -> LllResult<()> {
        verify_leadership_claim(&self.backend, claim).map_err(|e| {
            LllError::ProofVerificationFailed(alloc::format!("{e}"))
        })
    }
}
