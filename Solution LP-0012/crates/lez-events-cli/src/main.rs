mod commands;
mod config;
mod format;
mod prompts;

use anyhow::Result;
use clap::Parser;
use commands::CommandLine;
use config::resolve_config;

fn main() -> Result<()> {
    let cli = CommandLine::parse();
    let cfg = resolve_config(&cli)?;
    commands::run(cli, cfg)
}
