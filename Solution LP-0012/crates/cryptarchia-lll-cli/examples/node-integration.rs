use cryptarchia_lll::{
    random_node_secret, EpochSchedule, LocalLeadershipLottery, LotteryConfig, ProposalPolicy,
    StakeTable, ValidatorRecord,
};

fn main() -> anyhow::Result<()> {
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: [1u8; 32],
        stake: 100_000,
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
        [9u8; 32],
        ProposalPolicy::strict_private(),
    )?;

    for slot in 0..32 {
        let outcome = lottery.evaluate_slot(slot)?;
        if outcome.is_winner {
            println!("slot {slot}: win with alias {}", hex::encode(outcome.alias));
        }
    }

    Ok(())
}
