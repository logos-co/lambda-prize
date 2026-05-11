use anyhow::{Context, Result};
use lez_events::{AppConfig, CliConfig, OutputFormat};
use std::{env, fs, path::PathBuf};

use crate::commands::CommandLine;

/// Resolve the effective [`CliConfig`] by merging (in priority order):
/// 1. Command-line flags
/// 2. `LEZ_EVENTS_CONFIG` env var (path to TOML file)
/// 3. Explicit `--config` flag (path to TOML file)
/// 4. Built-in defaults
pub fn resolve_config(cli: &CommandLine) -> Result<CliConfig> {
    let mut cfg = if let Some(path) = cli.config.as_ref() {
        load_config_file(path.clone())?
    } else if let Ok(path) = env::var("LEZ_EVENTS_CONFIG") {
        load_config_file(PathBuf::from(path))?
    } else {
        CliConfig::default()
    };

    if let Some(rpc) = cli.rpc_url.as_ref() {
        cfg.rpc_url = rpc.clone();
    }

    if cli.jsonl   { cfg.output = OutputFormat::JsonLines; }
    if cli.json    { cfg.output = OutputFormat::Json;      }
    if cli.pretty  { cfg.output = OutputFormat::Pretty;    }

    if cli.no_color { cfg.color  = false; }
    if cli.strict   { cfg.strict = true;  }
    if cli.follow   { cfg.follow = true;  }

    cfg.validate()
        .with_context(|| "invalid configuration: rpc_url must not be empty")?;

    Ok(cfg)
}

fn load_config_file(path: PathBuf) -> Result<CliConfig> {
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config file {}", path.display()))?;
    let app = AppConfig::from_toml_str(&raw)
        .with_context(|| format!("failed to parse config file {}", path.display()))?;
    app.validate()
        .with_context(|| format!("configuration in {} failed validation", path.display()))?;
    Ok(app.cli)
}
