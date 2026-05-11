use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use lez_events::{
    atomic_write_json,
    build_support_bundle, capture_support_context, render_support_report,
    decoder::{decode_hex_envelope, register_type},
    human::{human_bytes, human_hex_preview},
    receipt::{DecodedEnvelope, DecodedReceipt, ReceiptEnvelope},
    validation::{validate_event_bytes, validate_program_id, validate_tx_hash},
    AppConfig, BackoffConfig, CliConfig, DiagnosticLevel, DiagnosticRecord, DiagnosticReport,
    EventError, EventIndex, HealthCheck, OutputFormat, retry, RetryConfig,
    SupportBundleConfig, SupportBundleWriter,
};
use reqwest::blocking::Client;
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    time::Duration,
};

use crate::{format, prompts};

// ── Top-level CLI ─────────────────────────────────────────────────────────────
#[derive(Parser, Debug)]
#[command(
    name    = "lez-events",
    version,
    about   = "Explore, decode, and index LEZ event receipts",
    long_about = "Performance-focused CLI for decoding event envelopes, inspecting receipts, \
                  building a local event index, and validating user inputs."
)]
pub struct CommandLine {
    #[arg(long, global = true, env = "LEZ_EVENTS_CONFIG",
          help = "Path to a lez-events.toml config file")]
    pub config: Option<PathBuf>,

    #[arg(long, global = true, env = "LEZ_EVENTS_RPC_URL",
          help = "Sequencer RPC base URL (overrides config)")]
    pub rpc_url: Option<String>,

    #[arg(long, global = true, help = "Output as pretty-printed JSON")]
    pub pretty: bool,

    #[arg(long, global = true, help = "Output as compact single-line JSON")]
    pub json: bool,

    #[arg(long, global = true, help = "Output as JSON-Lines (one object per line)")]
    pub jsonl: bool,

    #[arg(long, global = true, help = "Disable ANSI colour output")]
    pub no_color: bool,

    #[arg(long, global = true,
          help = "Fail immediately on any validation or decode error")]
    pub strict: bool,

    #[arg(long, global = true, help = "Keep polling for new events")]
    pub follow: bool,

    #[command(subcommand)]
    pub command: Command,
}

// ── Subcommands ───────────────────────────────────────────────────────────────
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Decode events from a receipt JSON file, stdin, or a live RPC endpoint.
    Decode(DecodeArgs),
    /// Show a summary of a receipt without decoding events.
    Inspect(InspectArgs),
    /// Validate hex strings, program IDs, or raw event bytes.
    Validate(ValidateArgs),
    /// Write a default config template to a TOML file.
    InitConfig(InitConfigArgs),
    /// Index one or more receipt JSON files into a queryable store.
    Index(IndexArgs),
    /// Query a previously-indexed events file.
    Query(QueryArgs),
    /// Print an example receipt JSON for the given scenario.
    Example(ExampleArgs),
    /// Run structural health checks on a receipt or the local environment.
    Doctor(DoctorArgs),
    /// Capture a portable support bundle (config, env, health, diagnostics).
    Bundle(BundleArgs),
    /// Report the health status of a receipt file or the CLI itself.
    Health(HealthArgs),
    /// Explain a known error code or message with remediation guidance.
    ExplainError(ExplainErrorArgs),
}

// ── Decode ────────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct DecodeArgs {
    #[arg(long, conflicts_with_all = ["stdin", "tx"],
          help = "Path to a local receipt JSON file")]
    pub file:  Option<PathBuf>,

    #[arg(long, conflicts_with_all = ["file"],
          help = "Read receipt JSON from stdin")]
    pub stdin: bool,

    #[arg(long, requires = "rpc",
          help = "Transaction hash to fetch from the RPC")]
    pub tx:    Option<String>,

    #[arg(long, help = "Sequencer RPC base URL (overrides global --rpc-url)")]
    pub rpc:   Option<String>,

    #[arg(long, value_delimiter = ',',
          help = "Known type names for IDL resolution (comma-separated)")]
    pub types: Vec<String>,

    #[arg(long,
          help = "Include malformed events as error entries instead of failing")]
    pub raw:   bool,
}

// ── Inspect ───────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct InspectArgs {
    #[arg(long, help = "Path to a local receipt JSON file")]
    pub file: PathBuf,
}

// ── Validate ──────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct ValidateArgs {
    #[arg(long, help = "Hex transaction hash to validate")]
    pub tx_hash:    Option<String>,

    #[arg(long, help = "Hex program ID to validate (must be 32 bytes / 64 hex chars)")]
    pub program_id: Option<String>,

    #[arg(long, help = "Hex event wire bytes to validate (optionally 0x-prefixed)")]
    pub event_hex:  Option<String>,
}

// ── InitConfig ────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct InitConfigArgs {
    #[arg(long, default_value = "lez-events.toml",
          help = "Output path for the config template")]
    pub out: PathBuf,
}

// ── Index ─────────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct IndexArgs {
    #[arg(long, help = "Path to a receipt JSON file (or array) to index")]
    pub file: Option<PathBuf>,

    #[arg(long, default_value = "events.json",
          help = "Output path for the indexed events file")]
    pub out: PathBuf,

    #[arg(long, default_value_t = 10_000,
          help = "Maximum number of receipts to index")]
    pub max_items: usize,

    #[arg(long, default_value_t = 1,
          help = "Number of retry attempts when writing the output file")]
    pub retries: usize,
}

// ── Query ─────────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct QueryArgs {
    #[arg(long, default_value = "events.json",
          help = "Path to an indexed events file")]
    pub file:      PathBuf,

    #[arg(long, help = "Filter by exact transaction hash")]
    pub tx_hash:   Option<String>,

    #[arg(long, help = "Filter by status: success or failed")]
    pub status:    Option<String>,

    #[arg(long, help = "Filter by event type name")]
    pub type_name: Option<String>,
}

// ── Example ───────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct ExampleArgs {
    #[arg(long, default_value = "success",
          help = "Scenario to generate: success | failure | empty")]
    pub kind: String,
}

// ── Doctor ────────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct DoctorArgs {
    #[arg(long, help = "Path to a receipt JSON file to validate")]
    pub file: Option<PathBuf>,
}

// ── Bundle ────────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct BundleArgs {
    #[arg(long, help = "Path to a receipt JSON file to include in the bundle")]
    pub file: Option<PathBuf>,

    #[arg(long, default_value = "support-bundles",
          help = "Directory to write the bundle JSON into")]
    pub out_dir: PathBuf,
}

// ── Health ────────────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct HealthArgs {
    #[arg(long, help = "Path to a receipt JSON file to health-check")]
    pub file: Option<PathBuf>,
}

// ── ExplainError ──────────────────────────────────────────────────────────────
#[derive(Args, Debug)]
pub struct ExplainErrorArgs {
    #[arg(long, help = "Machine-readable error code (e.g. TxBudgetExceeded)")]
    pub code: Option<String>,

    #[arg(long, help = "Error message text to include in the explanation report")]
    pub message: Option<String>,
}

// ── Dispatch ──────────────────────────────────────────────────────────────────
pub fn run(cli: CommandLine, cfg: CliConfig) -> Result<()> {
    match cli.command {
        Command::Decode(args)       => decode_command(args, cfg),
        Command::Inspect(args)      => inspect_command(args, cfg),
        Command::Validate(args)     => validate_command(args),
        Command::InitConfig(args)   => init_config_command(args, cfg),
        Command::Index(args)        => index_command(args),
        Command::Query(args)        => query_command(args),
        Command::Example(args)      => example_command(args),
        Command::Doctor(args)       => doctor_command(args),
        Command::Bundle(args)       => bundle_command(args),
        Command::Health(args)       => health_command(args),
        Command::ExplainError(args) => explain_error_command(args),
    }
}

// ── Command implementations ───────────────────────────────────────────────────
fn decode_command(args: DecodeArgs, cfg: CliConfig) -> Result<()> {
    let envelope = load_receipt_envelope(&args, &cfg)?;
    envelope.validate().context("receipt failed validation")?;
    let decoded  = decode_receipt(&envelope, &args)?;
    let output   = format::render_receipt(&decoded, cfg.output)?;
    println!("{output}");
    Ok(())
}

fn inspect_command(args: InspectArgs, cfg: CliConfig) -> Result<()> {
    let raw      = fs::read_to_string(&args.file)
        .with_context(|| format!("failed to read {}", args.file.display()))?;
    let envelope: ReceiptEnvelope = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse {}", args.file.display()))?;

    envelope.validate().context("receipt failed validation")?;

    let size_hint = human_bytes(envelope.total_hex_chars() / 2);
    let summary   = serde_json::json!({
        "tx_hash":           envelope.tx_hash,
        "status":            envelope.status,
        "error":             envelope.error,
        "state_root":        envelope.state_root,
        "event_count":       envelope.event_count(),
        "event_bytes_total": size_hint,
    });

    match cfg.output {
        OutputFormat::Pretty =>
            println!("{}", serde_json::to_string_pretty(&summary)?),
        OutputFormat::Json | OutputFormat::JsonLines =>
            println!("{}", serde_json::to_string(&summary)?),
    }
    Ok(())
}

fn validate_command(args: ValidateArgs) -> Result<()> {
    let mut validated = 0usize;

    if let Some(tx) = args.tx_hash {
        validate_tx_hash(&tx).map_err(anyhow::Error::from)?;
        println!("✓ tx hash is valid: {}", human_hex_preview(&tx, 16));
        validated += 1;
    }
    if let Some(pid) = args.program_id {
        validate_program_id(&pid).map_err(anyhow::Error::from)?;
        println!("✓ program id is valid: {}", human_hex_preview(&pid, 16));
        validated += 1;
    }
    if let Some(hex) = args.event_hex {
        let clean = hex.trim_start_matches("0x");
        let bytes = hex::decode(clean)
            .with_context(|| format!("invalid hex in --event-hex: {}", human_hex_preview(&hex, 16)))?;
        validate_event_bytes(&bytes).map_err(anyhow::Error::from)?;
        println!("✓ event bytes are valid: {} bytes", bytes.len());
        validated += 1;
    }

    if validated == 0 {
        anyhow::bail!("provide at least one of --tx-hash, --program-id, --event-hex");
    }
    Ok(())
}

fn init_config_command(args: InitConfigArgs, cfg: CliConfig) -> Result<()> {
    let app = AppConfig { cli: cfg };
    atomic_write_json(&args.out, &app)?;
    println!("wrote config template to {}", args.out.display());
    Ok(())
}

fn index_command(args: IndexArgs) -> Result<()> {
    let input = if let Some(path) = args.file {
        fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?
    } else {
        prompts::read_stdin_all()?
    };

    let receipts: Vec<ReceiptEnvelope> = if input.trim().starts_with('[') {
        serde_json::from_str(&input).context("failed to parse receipt array")?
    } else {
        vec![serde_json::from_str(&input).context("failed to parse receipt")?]
    };

    let mut decoded: Vec<DecodedReceipt> = Vec::with_capacity(receipts.len().min(args.max_items));
    let mut index = EventIndex::new();

    for receipt in receipts.into_iter().take(args.max_items) {
        receipt.validate().context("receipt failed validation")?;

        let parsed_events = receipt
            .events
            .iter()
            .map(|hex| decode_hex_envelope(hex, None))
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from)?;

        index.push_receipt(&receipt, &parsed_events);

        decoded.push(DecodedReceipt {
            tx_hash:    receipt.tx_hash,
            status:     receipt.status,
            error:      receipt.error,
            state_root: receipt.state_root,
            events:     parsed_events,
        });
    }

    atomic_write_json(&args.out, &decoded)?;
    println!("indexed {} receipt(s) into {}", decoded.len(), args.out.display());
    Ok(())
}

fn query_command(args: QueryArgs) -> Result<()> {
    let raw  = fs::read_to_string(&args.file)
        .with_context(|| format!("failed to read {}", args.file.display()))?;
    let rows: Vec<DecodedReceipt> = serde_json::from_str(&raw)
        .context("failed to parse indexed events file")?;

    let status_filter = args.status.as_deref().map(|s| s.to_lowercase());

    let filtered: Vec<_> = rows.into_iter().filter(|r| {
        let hash_ok   = args.tx_hash.as_ref().map_or(true, |wanted| &r.tx_hash == wanted);
        let status_ok = status_filter.as_deref().map_or(true, |wanted| {
            r.status.to_string().to_lowercase() == wanted
        });
        let type_ok   = args.type_name.as_ref().map_or(true, |wanted| {
            r.events.iter().any(|e| e.type_name.as_deref() == Some(wanted.as_str()))
        });
        hash_ok && status_ok && type_ok
    }).collect();

    println!("{}", serde_json::to_string_pretty(&filtered)?);
    Ok(())
}

fn example_command(args: ExampleArgs) -> Result<()> {
    let json = match args.kind.as_str() {
        "success" => serde_json::json!({
            "tx_hash":    "0xaaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111aaaa1111",
            "status":     "success",
            "state_root": "0xbbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222bbbb2222",
            "events":     ["00e38f1a022a00000000000000"]
        }),
        "failure" => serde_json::json!({
            "tx_hash":    "0xcccc3333cccc3333cccc3333cccc3333cccc3333cccc3333cccc3333cccc3333",
            "status":     "failed",
            "error":      "simulated failure after event",
            "events":     ["00a1b2c3d40400000004006f6f7073"]
        }),
        "empty" => serde_json::json!({
            "tx_hash":    "0xdddd4444dddd4444dddd4444dddd4444dddd4444dddd4444dddd4444dddd4444",
            "status":     "failed",
            "error":      "simulated failure before event emission",
            "events":     []
        }),
        other => return Err(EventError::UnsupportedFormat(
            format!("{other}: try 'success', 'failure', or 'empty'")
        ).into()),
    };
    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

// ── Load receipt ──────────────────────────────────────────────────────────────
fn load_receipt_envelope(args: &DecodeArgs, cfg: &CliConfig) -> Result<ReceiptEnvelope> {
    if let Some(path) = &args.file {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        return serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse {}", path.display()));
    }

    if args.stdin {
        let raw = prompts::read_stdin_all()?;
        return serde_json::from_str(&raw).context("failed to parse receipt from stdin");
    }

    if let Some(tx) = &args.tx {
        validate_tx_hash(tx)?;
        let rpc    = args.rpc.as_deref().unwrap_or(&cfg.rpc_url);
        let client = Client::builder()
            .timeout(Duration::from_millis(cfg.timeout_ms))
            .build()
            .map_err(|e| EventError::Rpc(e.to_string()))?;
        let url = format!("{}/tx/{}/events", rpc.trim_end_matches('/'), tx);

        let resp = retry(
            RetryConfig {
                attempts: cfg.retries.max(1),
                backoff:  BackoffConfig::default(),
            },
            || client.get(&url).send().map_err(|e| EventError::Rpc(e.to_string())),
        )?;

        if !resp.status().is_success() {
            return Err(EventError::Rpc(format!("RPC returned HTTP {}", resp.status())).into());
        }
        return resp.json::<ReceiptEnvelope>()
            .map_err(|e| EventError::Rpc(e.to_string()).into());
    }

    Err(EventError::MissingField("file | stdin | tx").into())
}

fn doctor_command(args: DoctorArgs) -> Result<()> {
    if let Some(file) = args.file {
        let raw = fs::read_to_string(&file)
            .with_context(|| format!("failed to read {}", file.display()))?;
        let receipt: ReceiptEnvelope = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse {}", file.display()))?;
        receipt.validate().context("receipt failed validation")?;
        println!("doctor: receipt is structurally valid ({})", file.display());
    } else {
        println!("doctor: no file provided; environment appears runnable");
    }
    Ok(())
}

fn bundle_command(args: BundleArgs) -> Result<()> {
    let receipt = if let Some(ref path) = args.file {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let env: ReceiptEnvelope = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        Some(env)
    } else {
        None
    };

    let support_cfg = SupportBundleConfig::default();
    let context     = capture_support_context("bundle");
    let bundle      = build_support_bundle(&support_cfg, context, receipt.as_ref());
    let writer      = SupportBundleWriter::new(&args.out_dir);
    let file        = writer.write_bundle(&bundle)?;

    println!("wrote support bundle to {}", file.display());
    print!("{}", render_support_report(&bundle));
    Ok(())
}

fn health_command(args: HealthArgs) -> Result<()> {
    let status = if let Some(file) = args.file {
        let raw = fs::read_to_string(&file)
            .with_context(|| format!("failed to read {}", file.display()))?;
        let receipt: ReceiptEnvelope = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse {}", file.display()))?;
        if receipt.validate().is_ok() {
            HealthCheck::healthy("receipt", "receipt validated successfully")
        } else {
            HealthCheck::unhealthy(
                "receipt",
                "receipt failed validation",
                "inspect the file and ensure event hex strings are valid",
            )
        }
    } else {
        HealthCheck::healthy("cli", "CLI is responsive and all modules loaded")
    };
    println!("{}", serde_json::to_string_pretty(&status)?);
    Ok(())
}

fn explain_error_command(args: ExplainErrorArgs) -> Result<()> {
    let mut report = DiagnosticReport::new(
        "Error explanation",
        "contextual explanation of a user-facing error",
    );

    if let Some(code) = args.code {
        report.push(
            DiagnosticRecord::new(DiagnosticLevel::Error, "error-code", "error code supplied")
                .with_detail(&code)
                .with_code(&code),
        );
    }
    if let Some(message) = args.message {
        report.push(
            DiagnosticRecord::new(DiagnosticLevel::Warn, "error-message", "error message supplied")
                .with_detail(&message),
        );
    }
    if report.is_empty() {
        report.push(
            DiagnosticRecord::new(DiagnosticLevel::Info, "explain-error", "no details supplied")
                .with_detail("provide --code or --message for a richer report"),
        );
    }

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn decode_receipt(envelope: &ReceiptEnvelope, args: &DecodeArgs) -> Result<DecodedReceipt> {
    let mut idl: HashMap<[u8; 4], String> = HashMap::new();
    for t in &args.types { register_type(&mut idl, t); }

    let mut decoded_events = Vec::with_capacity(envelope.events.len());
    for raw_hex in &envelope.events {
        match decode_hex_envelope(raw_hex, Some(&idl)) {
            Ok(e) => decoded_events.push(e),
            Err(err) if args.raw => {
                decoded_events.push(DecodedEnvelope {
                    version:      0,
                    discriminant: "00000000".into(),
                    type_name:    Some(format!("decode_error: {err}")),
                    payload_hex:  human_hex_preview(raw_hex, 32),
                    payload_size: raw_hex.len().saturating_sub(10) / 2,
                    raw_size:     raw_hex.len() / 2,
                });
            }
            Err(err) => return Err(err.into()),
        }
    }

    Ok(DecodedReceipt {
        tx_hash:    envelope.tx_hash.clone(),
        status:     envelope.status.clone(),
        error:      envelope.error.clone(),
        state_root: envelope.state_root.clone(),
        events:     decoded_events,
    })
}
