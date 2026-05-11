use cryptarchia_lll::{
    random_node_secret, EpochSchedule, ProofOfLeadershipConfig, ProofOfLeadershipEngine,
    StakeTable, ValidatorRecord,
};

fn make_engine() -> ProofOfLeadershipEngine {
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: [1u8; 32],
        stake: 100_000,
        online: true,
        participating: true,
    });

    ProofOfLeadershipEngine::default_for_node(
        ProofOfLeadershipConfig {
            chain_id: 1,
            epoch_id: 0,
            target_win_ppm: 25_000,
            min_stake_for_win: 1,
            max_proposal_bytes: 1_048_576,
            proof_version: 2,
            hide_total_stake: true,
        },
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2_000,
            slots_per_leadership_check: 1,
        },
        random_node_secret(),
        [1u8; 32],
        [2u8; 32],
        table,
    )
}

#[test]
fn proof_cycle_keeps_identity_and_total_stake_hidden_in_public_inputs() {
    let engine = make_engine();

    let claim = engine
        .evaluate_slot(7, [3u8; 32], [4u8; 32], [5u8; 32], [6u8; 32])
        .unwrap();

    if let Some(claim) = claim {
        assert_eq!(claim.public_inputs.version, 2);
        assert_eq!(claim.public_inputs.chain_id, 1);

        // Identity and total stake must NOT appear in public inputs directly.
        // They are commitment-based only.
        assert_ne!(claim.public_inputs.leader_identity_commitment, [0u8; 32]);
        assert_ne!(claim.public_inputs.threshold_commitment, [0u8; 32]);

        assert!(
            engine.verify_claim(&claim).is_ok(),
            "PoL claim must verify successfully"
        );
    }
}

#[test]
fn pol_claim_commitment_is_deterministic_for_same_slot() {
    let engine = make_engine();

    let c1 = engine.evaluate_slot(0, [3u8; 32], [4u8; 32], [5u8; 32], [6u8; 32]).unwrap();
    let c2 = engine.evaluate_slot(0, [3u8; 32], [4u8; 32], [5u8; 32], [6u8; 32]).unwrap();

    assert_eq!(
        c1.is_none(),
        c2.is_none(),
        "Same slot must produce consistent win/no-win"
    );

    if let (Some(a), Some(b)) = (c1, c2) {
        assert_eq!(
            a.claim_commitment, b.claim_commitment,
            "Claim commitment must be deterministic for the same inputs"
        );
    }
}

#[test]
fn pol_different_slots_produce_different_commitments() {
    let engine = make_engine();

    let mut claims = alloc::vec::Vec::new();
    for slot in 0..32u64 {
        if let Some(claim) =
            engine.evaluate_slot(slot, [3u8; 32], [4u8; 32], [5u8; 32], [6u8; 32]).unwrap()
        {
            claims.push(claim);
        }
    }

    // All collected claim commitments must be unique.
    let mut seen = std::collections::HashSet::new();
    for claim in &claims {
        let inserted = seen.insert(claim.claim_commitment);
        assert!(inserted, "Duplicate claim commitment detected across slots");
    }
}

#[test]
fn pol_verify_rejects_tampered_claim_commitment() {
    let engine = make_engine();

    for slot in 0..64u64 {
        if let Some(mut claim) =
            engine.evaluate_slot(slot, [3u8; 32], [4u8; 32], [5u8; 32], [6u8; 32]).unwrap()
        {
            claim.claim_commitment[0] ^= 0xff;
            assert!(
                engine.verify_claim(&claim).is_err(),
                "Tampered claim_commitment must fail verification"
            );
            return;
        }
    }
}

#[test]
fn pol_public_inputs_contain_no_raw_stake_or_identity() {
    let engine = make_engine();

    for slot in 0..64u64 {
        if let Some(claim) =
            engine.evaluate_slot(slot, [3u8; 32], [4u8; 32], [5u8; 32], [6u8; 32]).unwrap()
        {
            let pi = &claim.public_inputs;

            // Verify the stake (100_000u128) does not appear raw in any commitment field.
            let raw_stake = 100_000u128.to_le_bytes();
            for field in [
                pi.leader_identity_commitment,
                pi.threshold_commitment,
                pi.ticket_commitment,
                pi.proposal_commitment,
            ] {
                assert_ne!(
                    &field[..raw_stake.len()],
                    &raw_stake,
                    "Raw stake must not appear verbatim in public inputs"
                );
            }
            return;
        }
    }
}

extern crate alloc;
