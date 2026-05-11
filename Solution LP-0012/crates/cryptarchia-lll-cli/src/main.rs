use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
#[command(
    name = "cryptarchia-lll",
    version,
    about = "Cryptarchia local leadership lottery"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Simulate(commands::SimulateArgs),
    Draw(commands::DrawArgs),
    Verify(commands::VerifyArgs),
    Export(commands::ExportArgs),
    Status(commands::StatusArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Simulate(args) => commands::simulate(args),
        Command::Draw(args) => commands::draw(args),
        Command::Verify(args) => commands::verify(args),
        Command::Export(args) => commands::export(args),
        Command::Status(args) => commands::status(args),
    }
}
