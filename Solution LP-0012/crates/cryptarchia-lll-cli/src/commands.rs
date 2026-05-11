use anyhow::{Context, Result};
use clap::Args;
use cryptarchia_lll::{
    random_node_secret, EpochSchedule, LocalLeadershipLottery, LotteryConfig, ProposalEnvelope,
    ProposalPolicy, StakeTable, ValidatorRecord,
};
use std::{fs, path::PathBuf};

#[derive(Args, Debug)]
pub struct SimulateArgs {
    #[arg(long, default_value_t = 64)]
    pub slots: u64,

    #[arg(long, default_value_t = 8)]
    pub validators: usize,

    #[arg(long, default_value_t = 1)]
    pub chain_id: u64,
}

#[derive(Args, Debug)]
pub struct DrawArgs {
    #[arg(long)]
    pub slot: u64,

    #[arg(long, default_value_t = 1)]
    pub chain_id: u64,
}

#[derive(Args, Debug)]
pub struct VerifyArgs {
    #[arg(long)]
    pub input: PathBuf,
}

#[derive(Args, Debug)]
pub struct ExportArgs {
    #[arg(long)]
    pub input: PathBuf,

    #[arg(long)]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct StatusArgs {
    #[arg(long)]
    pub input: Option<PathBuf>,
}

pub fn simulate(args: SimulateArgs) -> Result<()> {
    let seed = [7u8; 32];
    let mut table = StakeTable::default();

    for i in 0..args.validators {
        let mut node_commitment = [0u8; 32];
        node_commitment[0] = i as u8;
        table.validators.push(ValidatorRecord {
            node_commitment,
            stake: ((i as u128) + 1) * 1000,
            online: true,
            participating: true,
        });
    }

    let schedule = EpochSchedule {
        epoch_length: 128,
        slot_duration_ms: 2000,
        slots_per_leadership_check: 1,
    };

    let lll = LocalLeadershipLottery::new(
        LotteryConfig::strict_private(args.chain_id, 0),
        schedule,
        random_node_secret(),
        table,
        seed,
        ProposalPolicy::strict_private(),
    )?;

    let mut wins = 0u64;
    for slot in 0..args.slots {
        let outcome = lll.evaluate_slot(slot)?;
        if outcome.is_winner {
            wins += 1;
        }
        println!("{}", serde_json::to_string_pretty(&outcome)?);
    }

    println!("summary: slots={} wins={}", args.slots, wins);
    Ok(())
}

pub fn draw(args: DrawArgs) -> Result<()> {
    let seed = [9u8; 32];
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: [1u8; 32],
        stake: 10_000,
        online: true,
        participating: true,
    });
    table.validators.push(ValidatorRecord {
        node_commitment: [2u8; 32],
        stake: 7_500,
        online: true,
        participating: true,
    });

    let schedule = EpochSchedule {
        epoch_length: 128,
        slot_duration_ms: 2000,
        slots_per_leadership_check: 1,
    };

    let lll = LocalLeadershipLottery::new(
        LotteryConfig::strict_private(args.chain_id, 0),
        schedule,
        random_node_secret(),
        table,
        seed,
        ProposalPolicy::strict_private(),
    )?;

    let outcome = lll.evaluate_slot(args.slot)?;
    println!("{}", serde_json::to_string_pretty(&outcome)?);
    Ok(())
}

pub fn verify(args: VerifyArgs) -> Result<()> {
    let raw = fs::read_to_string(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    let envelope: ProposalEnvelope = serde_json::from_str(&raw)?;
    let seed = [9u8; 32];
    let mut table = StakeTable::default();
    table.validators.push(ValidatorRecord {
        node_commitment: [1u8; 32],
        stake: 10_000,
        online: true,
        participating: true,
    });

    let lll = LocalLeadershipLottery::new(
        LotteryConfig::strict_private(
            envelope.announce.chain_id,
            envelope.announce.epoch_id,
        ),
        EpochSchedule {
            epoch_length: 128,
            slot_duration_ms: 2000,
            slots_per_leadership_check: 1,
        },
        random_node_secret(),
        table,
        seed,
        ProposalPolicy::strict_private(),
    )?;

    lll.verify_envelope(&envelope)?;
    println!("proposal envelope verified");
    Ok(())
}

pub fn export(args: ExportArgs) -> Result<()> {
    let raw = fs::read_to_string(&args.input)
        .with_context(|| format!("failed to read {}", args.input.display()))?;
    fs::write(&args.output, raw)?;
    println!("exported to {}", args.output.display());
    Ok(())
}

pub fn status(args: StatusArgs) -> Result<()> {
    if let Some(input) = args.input {
        let raw = fs::read_to_string(&input)
            .with_context(|| format!("failed to read {}", input.display()))?;
        println!("{}", raw);
    } else {
        println!("cryptarchia-lll status: ready");
    }
    Ok(())
}
