use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::{
    crypto::random_node_secret,
    lottery::{LocalLeadershipLottery, LotteryConfig},
    policy::ProposalPolicy,
    schedule::EpochSchedule,
    stake::{StakeTable, ValidatorRecord},
    telemetry::{AuditEvent, AuditLevel, LotteryMetrics, LotteryTrace},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub chain_id: u64,
    pub epoch_id: u64,
    pub slots: u64,
    pub validators: usize,
    pub seed: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStats {
    pub slots: u64,
    pub winners: u64,
    pub losers: u64,
    pub avg_winner_stake: u128,
    pub metrics: LotteryMetrics,
    pub trace: LotteryTrace,
}

pub fn build_random_validator_table(validators: usize, seed: [u8; 32]) -> StakeTable {
    let mut rng = StdRng::from_seed(seed);
    let mut table = StakeTable::default();

    for i in 0..validators {
        let mut node_commitment = [0u8; 32];
        rng.fill(&mut node_commitment);
        let stake = (rng.gen_range(1..1_000_000) as u128) * ((i as u128 % 5) + 1);
        table.validators.push(ValidatorRecord {
            node_commitment,
            stake,
            online: true,
            participating: true,
        });
    }

    table
}

pub fn run_simulation(cfg: SimulationConfig) -> SimulationStats {
    let schedule = EpochSchedule {
        epoch_length: 128,
        slot_duration_ms: 2000,
        slots_per_leadership_check: 1,
    };

    let validator_table = build_random_validator_table(cfg.validators, cfg.seed);
    let node_secret = random_node_secret();
    let lottery = LocalLeadershipLottery::new(
        LotteryConfig::strict_private(cfg.chain_id, cfg.epoch_id),
        schedule,
        node_secret,
        validator_table,
        cfg.seed,
        ProposalPolicy::strict_private(),
    )
    .expect("simulation setup must be valid");

    let mut metrics = LotteryMetrics::default();
    let mut trace = LotteryTrace::default();
    let mut winners = 0u64;
    let mut loser_slots = 0u64;
    let mut total_winner_stake = 0u128;

    for slot in 0..cfg.slots {
        let t0 = std::time::Instant::now();
        let outcome = lottery.evaluate_slot(slot).expect("slot eval");
        let elapsed = t0.elapsed().as_nanos();

        metrics.add_latency(elapsed);
        metrics.evaluated_slots += 1;

        if outcome.is_winner {
            winners += 1;
            metrics.record_win();
            metrics.record_proof();
            metrics.record_proposal();
            total_winner_stake = total_winner_stake.saturating_add(outcome.threshold);
            trace.push(
                AuditEvent::new(
                    AuditLevel::Info,
                    "simulation",
                    "slot produced a local winner",
                )
                .with_slot(slot)
                .with_epoch(cfg.epoch_id),
            );
        } else {
            loser_slots += 1;
            metrics.record_miss();
            trace.push(
                AuditEvent::new(
                    AuditLevel::Debug,
                    "simulation",
                    "slot produced no winner",
                )
                .with_slot(slot)
                .with_epoch(cfg.epoch_id),
            );
        }
    }

    SimulationStats {
        slots: cfg.slots,
        winners,
        losers: loser_slots,
        avg_winner_stake: if winners == 0 {
            0
        } else {
            total_winner_stake / winners as u128
        },
        metrics,
        trace,
    }
}
