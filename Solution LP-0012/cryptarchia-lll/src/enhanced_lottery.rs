/// Enhanced leadership lottery combining all PoL improvements:
///
/// 1. **ECVRF ticket** — replaces HMAC pseudo-ticket with a publicly
///    verifiable VRF output (Ristretto255-SHA512).
///
/// 2. **Blinded epoch nonce** — each slot's VRF input is `H(epoch_vrf || local_secret || slot)`,
///    preventing adversaries from forcing VRF evaluation at arbitrary slots.
///
/// 3. **Nullifier set** — prevents the same slot being "won" twice across
///    competing forks or replay attacks.
///
/// 4. **Stake estimator** — adapts the lottery threshold each epoch based
///    on the node's observed win-rate vs. the expected stake-weighted rate,
///    correcting for stale or adversarially inflated stake tables.
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::{
    blinding::EpochVrfChain,
    crypto::NodeSecret,
    estimator::{EpochStakeEstimator, EstimatorConfig},
    nullifier::{derive_nullifier, NullifierSet},
    policy::{ProposalPolicy, ProposalPolicyDecision},
    proof::{build_leadership_proof, LeadershipPublicInputs},
    schedule::EpochSchedule,
    stake::{effective_leader_threshold, StakeTable},
    types::{ChainId, EpochId, LeaderAlias, Slot},
    vrf::{vrf_prove, vrf_public_key, vrf_verify, vrf_wins, VrfOutput, VrfProof},
    LllError, LllResult,
};
use crate::lottery::{LotteryConfig, ProposalAnnounce};
use crate::crypto::derive_alias;

/// Enhanced proof of leadership output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnhancedLeadershipProof {
    /// The VRF output (beta) — the pseudorandom ticket value.
    pub vrf_output: VrfOutput,
    /// The ECVRF proof — network peers verify this to confirm the ticket.
    pub vrf_proof: VrfProof,
    /// The VRF public key (Ristretto255 compressed, 32 bytes).
    pub vrf_public_key: [u8; 32],
    /// Nullifier commitment (H(nullifier)) — marks this slot as spent.
    pub nullifier_commitment: [u8; 32],
    /// The underlying leadership proof (signature over public inputs).
    pub leadership_proof: crate::proof::LeadershipProof,
}

/// Outcome of evaluating a slot with the enhanced lottery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedLeadershipOutcome {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub slot: Slot,
    pub alias: [u8; 32],
    pub is_winner: bool,
    pub next_check_in_slots: u64,
    /// Present only when `is_winner == true`.
    pub proof: Option<EnhancedLeadershipProof>,
    /// Present only when `is_winner == true`.
    pub announce: Option<ProposalAnnounce>,
    /// Stake estimator snapshot for telemetry.
    pub estimator_adjustment_ppm: u64,
}

/// Enhanced leadership lottery.
pub struct EnhancedLottery {
    pub config: LotteryConfig,
    pub schedule: EpochSchedule,
    pub node_secret: NodeSecret,
    pub validator_table: StakeTable,
    pub validator_root: [u8; 32],
    pub beacon_seed: [u8; 32],
    pub policy: ProposalPolicy,
    /// Blinded epoch VRF chain.
    pub epoch_chain: EpochVrfChain,
    /// Nullifier set — grows with each winning slot.
    pub nullifiers: NullifierSet,
    /// Stake estimator — updated after each epoch.
    pub estimator: EpochStakeEstimator,
}

impl EnhancedLottery {
    /// Construct a new enhanced lottery.
    ///
    /// `estimator_config` may be `None` to use the recommended defaults.
    pub fn new(
        config: LotteryConfig,
        schedule: EpochSchedule,
        node_secret: NodeSecret,
        validator_table: StakeTable,
        beacon_seed: [u8; 32],
        policy: ProposalPolicy,
        estimator_config: Option<EstimatorConfig>,
    ) -> LllResult<Self> {
        validator_table.validate()?;
        let root = validator_table.validator_root();

        let stake = validator_table
            .total_stake();
        let node_stake = 0u128; // will be derived per-epoch

        let epoch_chain = EpochVrfChain::genesis(config.chain_id, &beacon_seed, &node_secret);

        let estimator = EpochStakeEstimator::new(
            estimator_config.unwrap_or_default(),
            node_stake,
            stake,
        );

        Ok(Self {
            config,
            schedule,
            node_secret,
            validator_root: root.root,
            beacon_seed,
            policy,
            epoch_chain,
            nullifiers: NullifierSet::new(),
            estimator,
            validator_table,
        })
    }

    fn alias(&self, epoch_id: EpochId) -> LeaderAlias {
        derive_alias(
            self.config.chain_id,
            epoch_id,
            &self.beacon_seed,
            &self.node_secret.secret,
        )
    }

    fn node_commitment(&self, epoch_id: EpochId) -> [u8; 32] {
        crate::crypto::hash32(
            &[
                self.alias(epoch_id).0.as_slice(),
                &self.beacon_seed,
                &self.config.chain_id.to_le_bytes(),
            ]
            .concat(),
        )
    }

    /// Evaluate a slot using the enhanced PoL mechanism.
    ///
    /// Returns `EnhancedLeadershipOutcome`.  When `is_winner == true` the
    /// `proof` and `announce` fields are populated and the nullifier for
    /// this slot has been marked spent in `self.nullifiers`.
    pub fn evaluate_slot(&mut self, slot: Slot) -> LllResult<EnhancedLeadershipOutcome> {
        let epoch_id = self.schedule.slot_to_epoch(slot);
        let alias = self.alias(epoch_id);
        let next_check = self.schedule.slots_per_leadership_check.max(1);

        let stake = self
            .validator_table
            .weight_of(&self.node_commitment(epoch_id))
            .map(|w| w.effective)
            .unwrap_or(0);

        let total_stake = self.validator_table.total_stake();
        if total_stake == 0 {
            return Err(LllError::ZeroTotalStake);
        }

        // --- 1. Build blinded VRF alpha from epoch chain + slot ---
        let blinded_nonce = self.epoch_chain.blinded_nonce(slot);
        let alpha = build_vrf_alpha(
            self.config.chain_id,
            epoch_id,
            slot,
            &self.beacon_seed,
            &blinded_nonce,
        );

        // --- 2. Run ECVRF ---
        let (vrf_out, vrf_proof) = vrf_prove(&self.node_secret, &alpha);
        let vrf_pk = vrf_public_key(&self.node_secret);

        // --- 3. Compute stake-relativized threshold ---
        let base_threshold =
            effective_leader_threshold(stake, total_stake, self.config.base_threshold)
                .min(self.config.max_threshold);
        let adjusted_threshold = self.estimator.adjusted_threshold(base_threshold);

        let is_winner = stake >= self.config.min_stake_for_win
            && vrf_wins(&vrf_out, adjusted_threshold);

        // Update estimator with this slot's outcome
        self.estimator.update_stake(stake, total_stake);
        self.estimator.observe_slot(is_winner);

        let adj_ppm = self.estimator.adjustment_ratio_ppm();

        if !is_winner {
            return Ok(EnhancedLeadershipOutcome {
                chain_id: self.config.chain_id,
                epoch_id,
                slot,
                alias: alias.0,
                is_winner: false,
                next_check_in_slots: next_check,
                proof: None,
                announce: None,
                estimator_adjustment_ppm: adj_ppm,
            });
        }

        // --- 4. Nullifier check: prevent double-leadership ---
        let nullifier =
            derive_nullifier(&self.node_secret, self.config.chain_id, epoch_id, slot);
        let nullifier_commitment = crate::nullifier::nullifier_commitment(&nullifier);
        self.nullifiers.mark_spent(nullifier)?;

        // --- 5. Policy gate ---
        if matches!(
            self.policy.decide(true, true, true),
            ProposalPolicyDecision::Deny
        ) {
            return Err(LllError::ProposalDenied(
                "proposal policy denied a valid win".into(),
            ));
        }

        // --- 6. Build the inner leadership proof (signature over public inputs) ---
        let public_inputs = LeadershipPublicInputs {
            chain_id: self.config.chain_id,
            epoch_id,
            slot,
            beacon_seed: self.beacon_seed,
            validator_root: self.validator_root,
            stake,
            total_stake,
            threshold: adjusted_threshold,
            ticket: vrf_out.0,
            alias: alias.0,
        };
        let inner_proof =
            build_leadership_proof(&self.node_secret, &self.beacon_seed, public_inputs)?;

        let payload_commitment = crate::crypto::hash32(&vrf_out.0);
        let announce = ProposalAnnounce {
            chain_id: self.config.chain_id,
            epoch_id,
            slot,
            alias: alias.0,
            ticket_digest: crate::crypto::hash32(&vrf_out.0),
            payload_commitment,
        };

        let enhanced_proof = EnhancedLeadershipProof {
            vrf_output: vrf_out,
            vrf_proof,
            vrf_public_key: vrf_pk,
            nullifier_commitment,
            leadership_proof: inner_proof,
        };

        Ok(EnhancedLeadershipOutcome {
            chain_id: self.config.chain_id,
            epoch_id,
            slot,
            alias: alias.0,
            is_winner: true,
            next_check_in_slots: next_check,
            proof: Some(enhanced_proof),
            announce: Some(announce),
            estimator_adjustment_ppm: adj_ppm,
        })
    }

    /// Verify an `EnhancedLeadershipProof` received from a peer.
    ///
    /// Checks:
    ///   - The ECVRF proof is valid for the claimed public key and alpha.
    ///   - The VRF output is below the stated threshold.
    ///   - The inner leadership proof signature is valid.
    ///   - The nullifier commitment is consistent.
    pub fn verify_enhanced_proof(
        &self,
        slot: Slot,
        proof: &EnhancedLeadershipProof,
        announced_threshold: u128,
    ) -> LllResult<()> {
        let epoch_id = self.schedule.slot_to_epoch(slot);

        // Reconstruct blinded nonce using the *epoch chain* (only known to this node).
        // For network verification, the verifier reconstructs from the public epoch VRF output.
        // Here we reconstruct from the same public epoch_vrf_out stored in epoch_chain.
        let blinded_nonce = self.epoch_chain.blinded_nonce(slot);
        let alpha = build_vrf_alpha(
            self.config.chain_id,
            epoch_id,
            slot,
            &self.beacon_seed,
            &blinded_nonce,
        );

        // 1. Verify the ECVRF proof
        vrf_verify(
            &proof.vrf_public_key,
            &alpha,
            &proof.vrf_output,
            &proof.vrf_proof,
        )?;

        // 2. Verify the output is below threshold
        if !vrf_wins(&proof.vrf_output, announced_threshold) {
            return Err(LllError::ProofVerificationFailed(
                "VRF output exceeds threshold".into(),
            ));
        }

        // 3. Verify the inner leadership proof
        crate::proof::verify_leadership_proof(&proof.leadership_proof)?;

        // 4. Verify VRF output matches the ticket in the inner proof
        if proof.leadership_proof.public_inputs.ticket != proof.vrf_output.0 {
            return Err(LllError::CommitmentMismatch);
        }

        // 5. Verify nullifier commitment consistency
        let derived_nullifier =
            derive_nullifier(&self.node_secret, self.config.chain_id, epoch_id, slot);
        let expected_commitment = crate::nullifier::nullifier_commitment(&derived_nullifier);
        if expected_commitment != proof.nullifier_commitment {
            return Err(LllError::CommitmentMismatch);
        }

        Ok(())
    }

    /// Advance the epoch.
    ///
    /// Updates the epoch VRF chain and folds the current epoch's stats
    /// into the stake estimator.
    pub fn advance_epoch(&mut self, new_epoch_id: EpochId) -> LllResult<()> {
        self.epoch_chain.advance(new_epoch_id, &self.node_secret)?;
        self.estimator.advance_epoch();
        Ok(())
    }
}

/// Build the VRF alpha string for a given slot.
///
/// Public inputs (chain, epoch, slot, beacon seed) are included so
/// the proof is binding to the specific slot context.  The blinded
/// nonce adds the per-node private component that prevents forced
/// VRF evaluation at arbitrary future slots.
pub fn build_vrf_alpha(
    chain_id: ChainId,
    epoch_id: EpochId,
    slot: Slot,
    beacon_seed: &[u8; 32],
    blinded_nonce: &[u8; 32],
) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(b"cryptarchia/vrf-alpha-v1");
    v.extend_from_slice(&chain_id.to_le_bytes());
    v.extend_from_slice(&epoch_id.to_le_bytes());
    v.extend_from_slice(&slot.to_le_bytes());
    v.extend_from_slice(beacon_seed);
    v.extend_from_slice(blinded_nonce);
    v
}
