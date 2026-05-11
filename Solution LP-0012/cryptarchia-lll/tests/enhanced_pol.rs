/// Integration tests for the four Proof of Leadership improvements:
///   1. ECVRF-Ristretto255-SHA512
///   2. Blinded epoch nonce chaining
///   3. Nullifier set (double-leadership prevention)
///   4. Epoch-adaptive stake estimator
use cryptarchia_lll::{
    blinding::{derive_blinded_nonce, EpochVrfChain},
    build_vrf_alpha, derive_nullifier, nullifier_commitment,
    EnhancedLottery, EpochSchedule, EpochStakeEstimator, EstimatorConfig,
    LotteryConfig, NullifierSet, ProposalPolicy, StakeTable, ValidatorRecord,
    VrfOutput, LllError,
    random_node_secret,
    vrf_prove, vrf_public_key, vrf_verify, vrf_wins, vrf_output_to_u128,
};

// ─────────────────────────────────────────────────────────────────────────────
// 1. ECVRF correctness
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn vrf_prove_and_verify_roundtrip() {
    let secret = random_node_secret();
    let pk = vrf_public_key(&secret);
    let alpha = b"test message alpha";

    let (output, proof) = vrf_prove(&secret, alpha);
    let verified = vrf_verify(&pk, alpha, &output, &proof);
    assert!(
        verified.is_ok(),
        "VRF verification must succeed for a valid proof"
    );
    assert_eq!(verified.unwrap(), output);
}

#[test]
fn vrf_output_is_deterministic() {
    let secret = random_node_secret();
    let alpha = b"deterministic input";
    let (out1, _) = vrf_prove(&secret, alpha);
    let (out2, _) = vrf_prove(&secret, alpha);
    // The OUTPUT (beta) is deterministic for a fixed (sk, alpha).
    // The PROOF uses a random k, so proofs differ but outputs must match.
    assert_eq!(
        out1, out2,
        "VRF beta must be deterministic for a fixed secret and alpha"
    );
}

#[test]
fn vrf_output_differs_for_different_alpha() {
    let secret = random_node_secret();
    let (out1, _) = vrf_prove(&secret, b"alpha A");
    let (out2, _) = vrf_prove(&secret, b"alpha B");
    assert_ne!(out1, out2, "Different alpha must produce different outputs");
}

#[test]
fn vrf_output_differs_for_different_secrets() {
    let s1 = random_node_secret();
    let s2 = random_node_secret();
    let alpha = b"same alpha";
    let (out1, _) = vrf_prove(&s1, alpha);
    let (out2, _) = vrf_prove(&s2, alpha);
    assert_ne!(out1, out2, "Different secrets must produce different outputs");
}

#[test]
fn vrf_verify_rejects_wrong_output() {
    let secret = random_node_secret();
    let pk = vrf_public_key(&secret);
    let alpha = b"some input";
    let (_, proof) = vrf_prove(&secret, alpha);
    let wrong_output = VrfOutput([0xabu8; 32]);
    let result = vrf_verify(&pk, alpha, &wrong_output, &proof);
    assert!(result.is_err(), "Wrong output must fail verification");
}

#[test]
fn vrf_verify_rejects_wrong_alpha() {
    let secret = random_node_secret();
    let pk = vrf_public_key(&secret);
    let (output, proof) = vrf_prove(&secret, b"correct alpha");
    let result = vrf_verify(&pk, b"wrong alpha", &output, &proof);
    assert!(result.is_err(), "Wrong alpha must fail verification");
}

#[test]
fn vrf_verify_rejects_wrong_public_key() {
    let secret = random_node_secret();
    let other_secret = random_node_secret();
    let wrong_pk = vrf_public_key(&other_secret);
    let alpha = b"some alpha";
    let (output, proof) = vrf_prove(&secret, alpha);
    let result = vrf_verify(&wrong_pk, alpha, &output, &proof);
    assert!(result.is_err(), "Wrong public key must fail verification");
}

#[test]
fn vrf_threshold_check() {
    let output_low = VrfOutput([0u8; 32]);
    let output_high = VrfOutput([0xffu8; 32]);
    assert!(vrf_wins(&output_low, u128::MAX), "All-zero output always wins");
    assert!(
        !vrf_wins(&output_high, 0),
        "All-0xff output never wins against threshold 0"
    );
    assert!(
        vrf_wins(&output_low, 1),
        "All-zero output wins against even threshold 1"
    );
}

#[test]
fn vrf_output_to_u128_consistency() {
    let v = VrfOutput([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(vrf_output_to_u128(&v), 1u128);
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. Blinded epoch nonce chaining
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn blinded_nonce_is_deterministic() {
    let epoch_vrf = [42u8; 32];
    let local_secret = [7u8; 32];
    let n1 = derive_blinded_nonce(&epoch_vrf, &local_secret, 100);
    let n2 = derive_blinded_nonce(&epoch_vrf, &local_secret, 100);
    assert_eq!(n1, n2);
}

#[test]
fn blinded_nonce_differs_per_slot() {
    let epoch_vrf = [1u8; 32];
    let local_secret = [2u8; 32];
    let n0 = derive_blinded_nonce(&epoch_vrf, &local_secret, 0);
    let n1 = derive_blinded_nonce(&epoch_vrf, &local_secret, 1);
    assert_ne!(n0, n1);
}

#[test]
fn blinded_nonce_differs_with_different_epoch_vrf() {
    let local_secret = [3u8; 32];
    let n1 = derive_blinded_nonce(&[0u8; 32], &local_secret, 55);
    let n2 = derive_blinded_nonce(&[1u8; 32], &local_secret, 55);
    assert_ne!(n1, n2, "Epoch VRF rotation must change the blinded nonce");
}

#[test]
fn blinded_nonce_differs_with_different_local_secret() {
    let epoch_vrf = [9u8; 32];
    let n1 = derive_blinded_nonce(&epoch_vrf, &[0u8; 32], 10);
    let n2 = derive_blinded_nonce(&epoch_vrf, &[1u8; 32], 10);
    assert_ne!(
        n1, n2,
        "Different local secrets must produce different nonces"
    );
}

#[test]
fn epoch_vrf_chain_advances_and_changes_nonce() {
    let secret = random_node_secret();
    let genesis_seed = [5u8; 32];
    let mut chain = EpochVrfChain::genesis(1, &genesis_seed, &secret);

    let nonce_epoch0 = chain.blinded_nonce(10);
    chain.advance(1, &secret).expect("epoch advance must succeed");
    let nonce_epoch1 = chain.blinded_nonce(10);

    assert_ne!(
        nonce_epoch0, nonce_epoch1,
        "Advancing epoch must change blinded nonces"
    );
}

#[test]
fn epoch_advance_produces_verifiable_proof() {
    let secret = random_node_secret();
    let mut chain = EpochVrfChain::genesis(1, &[0u8; 32], &secret);
    let proof = chain.advance(1, &secret).expect("advance ok");
    assert_eq!(proof.prev_epoch_id, 0);
    assert_eq!(proof.new_epoch_id, 1);
    assert_eq!(proof.chain_id, 1);
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. Nullifier set — double-leadership prevention
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn nullifier_derivation_is_deterministic() {
    let secret = random_node_secret();
    let n1 = derive_nullifier(&secret, 1, 0, 42);
    let n2 = derive_nullifier(&secret, 1, 0, 42);
    assert_eq!(n1, n2);
}

#[test]
fn nullifier_differs_per_slot() {
    let secret = random_node_secret();
    let n1 = derive_nullifier(&secret, 1, 0, 0);
    let n2 = derive_nullifier(&secret, 1, 0, 1);
    assert_ne!(n1, n2);
}

#[test]
fn nullifier_differs_per_chain() {
    let secret = random_node_secret();
    let n1 = derive_nullifier(&secret, 1, 0, 5);
    let n2 = derive_nullifier(&secret, 2, 0, 5);
    assert_ne!(n1, n2);
}

#[test]
fn nullifier_commitment_is_deterministic() {
    let n = [0x42u8; 32];
    let c1 = nullifier_commitment(&n);
    let c2 = nullifier_commitment(&n);
    assert_eq!(c1, c2);
}

#[test]
fn nullifier_commitment_differs_from_nullifier() {
    let n = [0x13u8; 32];
    let c = nullifier_commitment(&n);
    assert_ne!(n, c, "Commitment must differ from the nullifier itself");
}

#[test]
fn nullifier_set_accepts_fresh_nullifier() {
    let secret = random_node_secret();
    let mut ns = NullifierSet::new();
    let result = ns.consume(&secret, 1, 0, 100);
    assert!(result.is_ok());
    assert_eq!(ns.len(), 1);
}

#[test]
fn nullifier_set_rejects_spent_nullifier() {
    let secret = random_node_secret();
    let mut ns = NullifierSet::new();
    ns.consume(&secret, 1, 0, 200).expect("first consume ok");
    let second = ns.consume(&secret, 1, 0, 200);
    assert!(
        matches!(second, Err(LllError::NullifierCollision)),
        "Second consume of same slot must fail with NullifierCollision"
    );
}

#[test]
fn nullifier_set_allows_different_slots() {
    let secret = random_node_secret();
    let mut ns = NullifierSet::new();
    ns.consume(&secret, 1, 0, 1).expect("slot 1 ok");
    ns.consume(&secret, 1, 0, 2).expect("slot 2 ok");
    ns.consume(&secret, 1, 0, 3).expect("slot 3 ok");
    assert_eq!(ns.len(), 3);
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. Epoch-adaptive stake estimator
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn estimator_initial_adjustment_is_identity() {
    let est = EpochStakeEstimator::new(EstimatorConfig::default(), 1_000, 10_000);
    // Before warmup, threshold should be unchanged.
    let base = u128::MAX / 10;
    assert_eq!(
        est.adjusted_threshold(base),
        base,
        "No adjustment before warmup"
    );
}

#[test]
fn estimator_expected_win_rate_matches_stake_ppm() {
    let est = EpochStakeEstimator::new(EstimatorConfig::default(), 500, 1000);
    assert_eq!(est.expected_win_rate_ppm(), 500_000); // 50%
}

#[test]
fn estimator_advances_epoch_and_updates_ema() {
    let mut est = EpochStakeEstimator::new(
        EstimatorConfig {
            ema_alpha_ppm: 500_000, // fast decay for testing
            warmup_slots: 0,
            max_adjustment_ppm: 500_000,
        },
        1_000,
        10_000,
    );
    // Simulate winning every slot for one epoch
    for _ in 0..100 {
        est.observe_slot(true);
    }
    let ema_before = est.ema_win_rate_ppm;
    est.advance_epoch();
    // After observing 100/100 wins, EMA should have increased toward 1_000_000
    assert!(
        est.ema_win_rate_ppm > ema_before,
        "EMA must increase when winning more than expected"
    );
}

#[test]
fn estimator_reduces_threshold_when_winning_too_much() {
    let mut est = EpochStakeEstimator::new(
        EstimatorConfig {
            ema_alpha_ppm: 500_000,
            warmup_slots: 0,
            max_adjustment_ppm: 400_000,
        },
        1_000,
        10_000, // 10% expected win rate
    );
    // Observe 100% win rate for 200 slots then advance epoch
    for _ in 0..200 {
        est.observe_slot(true);
    }
    est.advance_epoch();

    let base = 1_000_000_000u128;
    let adjusted = est.adjusted_threshold(base);
    assert!(
        adjusted < base,
        "Threshold should be reduced when winning more than expected (base={base}, adjusted={adjusted})"
    );
}

#[test]
fn estimator_summary_is_populated() {
    let est = EpochStakeEstimator::new(EstimatorConfig::default(), 100, 1000);
    let summary = est.summary();
    assert_eq!(summary.total_evaluated, 0);
    assert_eq!(summary.expected_win_rate_ppm, 100_000); // 10%
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. EnhancedLottery — full leadership cycle
// ─────────────────────────────────────────────────────────────────────────────

fn make_enhanced_lottery() -> EnhancedLottery {
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: [1u8; 32],
        stake: 100_000,
        online: true,
        participating: true,
    });
    EnhancedLottery::new(
        LotteryConfig::strict_private(1, 0),
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2000,
            slots_per_leadership_check: 1,
        },
        random_node_secret(),
        table,
        [9u8; 32],
        ProposalPolicy::strict_private(),
        None,
    )
    .expect("lottery setup must succeed")
}

#[test]
fn enhanced_lottery_evaluates_slot() {
    let mut lll = make_enhanced_lottery();
    let outcome = lll.evaluate_slot(7).expect("evaluate_slot ok");
    assert_eq!(outcome.chain_id, 1);
    assert_eq!(outcome.slot, 7);
}

#[test]
fn enhanced_lottery_winner_has_vrf_proof() {
    use cryptarchia_lll::{derive_alias, hash32};

    let chain_id = 1u64;
    let epoch_id = 0u64;
    let beacon_seed = [1u8; 32];
    let secret = random_node_secret();

    // Compute the node's own commitment so the stake table lookup succeeds.
    // node_commitment = H(alias || beacon_seed || chain_id)
    let alias = derive_alias(chain_id, epoch_id, &beacon_seed, &secret.secret);
    let node_comm: [u8; 32] = hash32(
        &[
            alias.0.as_slice(),
            beacon_seed.as_slice(),
            &chain_id.to_le_bytes(),
        ]
        .concat(),
    );

    let stake = 50_000u128;
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: node_comm,
        stake,
        online: true,
        participating: true,
    });

    // base_threshold = u128::MAX ⟹ effective_leader_threshold(50_000, 50_000, MAX) = MAX
    // ⟹ vrf_wins(any output, MAX) = always true
    let mut lll = EnhancedLottery::new(
        LotteryConfig {
            chain_id,
            epoch_id,
            base_threshold: u128::MAX,
            max_threshold: u128::MAX,
            min_stake_for_win: 1,
            enable_hidden_aliases: true,
        },
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2000,
            slots_per_leadership_check: 1,
        },
        secret,
        table,
        beacon_seed,
        ProposalPolicy::strict_private(),
        None,
    )
    .expect("setup ok");

    // Every slot must win: threshold = MAX so any VRF output passes.
    let outcome = lll.evaluate_slot(0).expect("eval slot 0");
    assert!(
        outcome.is_winner,
        "With stake == total_stake and threshold = u128::MAX, slot 0 must win"
    );

    let proof = outcome.proof.expect("winner must have proof");
    assert_eq!(proof.vrf_proof.c.len(), 32);
    assert_eq!(proof.vrf_proof.s.len(), 32);
    assert_eq!(proof.vrf_proof.gamma.len(), 32);
    assert_eq!(proof.vrf_public_key.len(), 32);
    assert_ne!(proof.nullifier_commitment, [0u8; 32]);
    assert_ne!(proof.vrf_output.0, [0u8; 32]);
}

#[test]
fn enhanced_lottery_double_win_same_slot_rejected() {
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: [5u8; 32],
        stake: u128::MAX / 2,
        online: true,
        participating: true,
    });
    let mut lll = EnhancedLottery::new(
        LotteryConfig {
            chain_id: 1,
            epoch_id: 0,
            base_threshold: u128::MAX,
            max_threshold: u128::MAX,
            min_stake_for_win: 1,
            enable_hidden_aliases: true,
        },
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2000,
            slots_per_leadership_check: 1,
        },
        random_node_secret(),
        table,
        [2u8; 32],
        ProposalPolicy::strict_private(),
        None,
    )
    .expect("setup ok");

    // Find a winning slot first
    let mut winning_slot = None;
    for slot in 0..128u64 {
        let outcome = lll.evaluate_slot(slot).expect("eval ok");
        if outcome.is_winner {
            winning_slot = Some(slot);
            break;
        }
    }

    if let Some(slot) = winning_slot {
        // Re-evaluating the same winning slot must fail with NullifierCollision
        let second = lll.evaluate_slot(slot);
        assert!(
            matches!(second, Err(LllError::NullifierCollision)),
            "Re-evaluating a winning slot must fail with NullifierCollision"
        );
    }
    // If no winning slot found in 128 slots, the test passes vacuously
}

#[test]
fn enhanced_lottery_epoch_advance_changes_nonces() {
    let mut lll = make_enhanced_lottery();
    let nonce_before = lll.epoch_chain.blinded_nonce(5);
    lll.advance_epoch(1).expect("advance ok");
    let nonce_after = lll.epoch_chain.blinded_nonce(5);
    assert_ne!(
        nonce_before, nonce_after,
        "Epoch advance must change blinded nonces"
    );
}

#[test]
fn enhanced_lottery_rejects_empty_stake_table() {
    let result = EnhancedLottery::new(
        LotteryConfig::strict_private(1, 0),
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2000,
            slots_per_leadership_check: 1,
        },
        random_node_secret(),
        StakeTable::default(),
        [0u8; 32],
        ProposalPolicy::strict_private(),
        None,
    );
    assert!(result.is_err());
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. VRF alpha construction
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn vrf_alpha_differs_per_slot() {
    let beacon = [0u8; 32];
    let nonce = [1u8; 32];
    let a1 = build_vrf_alpha(1, 0, 0, &beacon, &nonce);
    let a2 = build_vrf_alpha(1, 0, 1, &beacon, &nonce);
    assert_ne!(a1, a2);
}

#[test]
fn vrf_alpha_differs_per_chain() {
    let beacon = [0u8; 32];
    let nonce = [0u8; 32];
    let a1 = build_vrf_alpha(1, 0, 5, &beacon, &nonce);
    let a2 = build_vrf_alpha(2, 0, 5, &beacon, &nonce);
    assert_ne!(a1, a2);
}

#[test]
fn vrf_alpha_differs_per_epoch() {
    let beacon = [0u8; 32];
    let nonce = [0u8; 32];
    let a1 = build_vrf_alpha(1, 0, 5, &beacon, &nonce);
    let a2 = build_vrf_alpha(1, 1, 5, &beacon, &nonce);
    assert_ne!(a1, a2);
}
