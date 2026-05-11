use cryptarchia_lll::{
    random_node_secret, stake_probability_ppm, EpochSchedule, LocalLeadershipLottery,
    LotteryConfig, ProposalPolicy, StakeTable, ValidatorRecord,
};

#[test]
fn stake_probability_is_bounded() {
    assert_eq!(stake_probability_ppm(0, 100), 0);
    assert!(stake_probability_ppm(50, 100) > 0);
    assert_eq!(stake_probability_ppm(100, 100), 1_000_000);
}

#[test]
fn hidden_alias_changes_per_epoch() {
    let secret = random_node_secret();
    let a = secret.alias(1, 1, &[1u8; 32]);
    let b = secret.alias(1, 2, &[1u8; 32]);
    assert_ne!(a.0, b.0);
}

#[test]
fn hidden_alias_stable_same_epoch() {
    let secret = random_node_secret();
    let a = secret.alias(1, 5, &[9u8; 32]);
    let b = secret.alias(1, 5, &[9u8; 32]);
    assert_eq!(a.0, b.0);
}

#[test]
fn lottery_evaluates() {
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: [3u8; 32],
        stake: 10_000,
        online: true,
        participating: true,
    });

    let lottery = LocalLeadershipLottery::new(
        LotteryConfig::strict_private(1, 0),
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2000,
            slots_per_leadership_check: 1,
        },
        random_node_secret(),
        table,
        [2u8; 32],
        ProposalPolicy::strict_private(),
    )
    .unwrap();

    let out = lottery.evaluate_slot(7).unwrap();
    assert_eq!(out.chain_id, 1);
}

#[test]
fn lottery_new_rejects_empty_table() {
    let table = StakeTable::default();
    let result = LocalLeadershipLottery::new(
        LotteryConfig::strict_private(1, 0),
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2000,
            slots_per_leadership_check: 1,
        },
        random_node_secret(),
        table,
        [0u8; 32],
        ProposalPolicy::strict_private(),
    );
    assert!(result.is_err());
}

#[test]
fn ticket_threshold_all_zeros_wins() {
    use cryptarchia_lll::ticket_below_threshold;
    let ticket = [0u8; 32];
    assert!(ticket_below_threshold(&ticket, u128::MAX));
    assert!(ticket_below_threshold(&ticket, 1));
}

#[test]
fn epoch_schedule_slot_to_epoch() {
    let sched = EpochSchedule {
        epoch_length: 100,
        slot_duration_ms: 1000,
        slots_per_leadership_check: 1,
    };
    assert_eq!(sched.slot_to_epoch(0), 0);
    assert_eq!(sched.slot_to_epoch(99), 0);
    assert_eq!(sched.slot_to_epoch(100), 1);
    assert_eq!(sched.slot_to_epoch(250), 2);
}
