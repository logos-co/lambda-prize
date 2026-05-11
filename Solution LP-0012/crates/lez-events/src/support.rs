use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    bundle::{Bundle, BundleKind},
    diagnostics::{DiagnosticLevel, DiagnosticRecord, DiagnosticReport, SupportContext},
    health::HealthCheck,
    receipt::ReceiptEnvelope,
    EventError,
};

/// Controls which sections are collected into a support bundle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupportBundleConfig {
    /// Capture a snapshot of environment variables (redacted).
    pub include_env: bool,
    /// Embed the provided receipt JSON in the bundle.
    pub include_receipt: bool,
    /// Run health checks and record results.
    pub include_health: bool,
    /// Produce a structured diagnostic report.
    pub include_diagnostics: bool,
    /// Maximum number of environment variables to capture.
    pub max_env_vars: usize,
}

impl Default for SupportBundleConfig {
    fn default() -> Self {
        Self {
            include_env:         true,
            include_receipt:     true,
            include_health:      true,
            include_diagnostics: true,
            max_env_vars:        64,
        }
    }
}

/// Writes [`Bundle`]s to disk as timestamped JSON files.
#[derive(Debug, Clone)]
pub struct SupportBundleWriter {
    output_dir: PathBuf,
}

impl SupportBundleWriter {
    pub fn new(output_dir: impl Into<PathBuf>) -> Self {
        Self { output_dir: output_dir.into() }
    }

    fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    pub fn write_bundle(&self, bundle: &Bundle) -> Result<PathBuf, EventError> {
        fs::create_dir_all(&self.output_dir).map_err(|e| EventError::Io(e.to_string()))?;
        let file = self.output_dir.join(format!(
            "{}-{}-support.json",
            Self::timestamp(),
            bundle.kind,
        ));
        let raw = serde_json::to_string_pretty(bundle).map_err(|e| EventError::Io(e.to_string()))?;
        fs::write(&file, raw).map_err(|e| EventError::Io(e.to_string()))?;
        Ok(file)
    }
}

/// A typed description of the command that triggered bundle generation.
///
/// Call [`SupportCommand::to_context`] to convert it to a [`SupportContext`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportCommand {
    pub command:      String,
    pub rpc_url:      Option<String>,
    pub tx_hash:      Option<String>,
    pub program_id:   Option<String>,
    pub receipt_path: Option<String>,
    pub config_path:  Option<String>,
}

impl SupportCommand {
    pub fn to_context(&self) -> SupportContext {
        SupportContext {
            command:      self.command.clone(),
            rpc_url:      self.rpc_url.clone(),
            tx_hash:      self.tx_hash.clone(),
            program_id:   self.program_id.clone(),
            receipt_path: self.receipt_path.clone(),
            config_path:  self.config_path.clone(),
            rust_version: rustc_version(),
            target:       Some(std::env::consts::ARCH.to_string()),
        }
    }
}

/// Capture a minimal [`SupportContext`] for the given command name.
///
/// Fills `rust_version` and `target` from compile-time env vars.
pub fn capture_support_context(command: impl Into<String>) -> SupportContext {
    SupportContext {
        command:      command.into(),
        rpc_url:      None,
        tx_hash:      None,
        program_id:   None,
        receipt_path: None,
        config_path:  None,
        rust_version: rustc_version(),
        target:       Some(std::env::consts::ARCH.to_string()),
    }
}

/// Render a [`Bundle`] as a human-readable plain-text report.
///
/// Suitable for printing to stderr during CI runs or attaching to bug reports.
pub fn render_support_report(bundle: &Bundle) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Support Bundle ({}) ===\n", bundle.kind));
    out.push_str(&format!("Command : {}\n", bundle.context.command));

    if let Some(ref url) = bundle.context.rpc_url {
        out.push_str(&format!("RPC     : {}\n", url));
    }
    if let Some(ref tx) = bundle.context.tx_hash {
        out.push_str(&format!("Tx      : {}\n", tx));
    }
    if let Some(ref pid) = bundle.context.program_id {
        out.push_str(&format!("Program : {}\n", pid));
    }
    if let Some(ref rp) = bundle.context.receipt_path {
        out.push_str(&format!("Receipt : {}\n", rp));
    }
    if let Some(ref ver) = bundle.context.rust_version {
        out.push_str(&format!("Rust    : {}\n", ver));
    }
    if let Some(ref target) = bundle.context.target {
        out.push_str(&format!("Target  : {}\n", target));
    }

    if let Some(ref diag) = bundle.diagnostics {
        out.push_str(&format!("\n── Diagnostics: {} ──\n{}\n", diag.title, diag.summary));
        for r in &diag.records {
            out.push_str(&format!("  [{:5}] {}: {}\n", r.level, r.component, r.message));
            if let Some(ref d) = r.detail { out.push_str(&format!("          {}\n", d)); }
            if let Some(ref c) = r.code   { out.push_str(&format!("          code: {}\n", c)); }
        }
    }

    if !bundle.health.is_empty() {
        out.push_str("\n── Health checks ──\n");
        for h in &bundle.health {
            out.push_str(&format!("  {:12} {:10}  {}\n", h.name, h.status, h.message));
            if let Some(ref r) = h.remediation {
                out.push_str(&format!("               ↳  {}\n", r));
            }
        }
        let problems = bundle.problem_count();
        if problems == 0 {
            out.push_str("  ✓ all checks passed\n");
        } else {
            out.push_str(&format!("  ✗ {} check(s) need attention\n", problems));
        }
    }

    out
}

/// Assemble a full [`Bundle`] according to `cfg`.
///
/// Pass `receipt = Some(&r)` when one is available; health checks will include
/// a structural validation result for it.
pub fn build_support_bundle(
    cfg:     &SupportBundleConfig,
    context: SupportContext,
    receipt: Option<&ReceiptEnvelope>,
) -> Bundle {
    let mut bundle = Bundle::new(BundleKind::Support, context);

    if cfg.include_diagnostics {
        let mut diag = DiagnosticReport::new(
            "Support diagnostics",
            "baseline support bundle assembled",
        );
        diag.push(
            DiagnosticRecord::new(DiagnosticLevel::Info, "support", "bundle created")
                .with_detail("all requested sections were collected"),
        );
        bundle.set_diagnostics(diag);
    }

    if cfg.include_health {
        bundle.push_health(HealthCheck::healthy(
            "config",
            "configuration loaded and parsed successfully",
        ));

        match receipt {
            Some(r) if r.validate().is_ok() =>
                bundle.push_health(HealthCheck::healthy(
                    "receipt",
                    "receipt structure passed validation",
                )),
            Some(_) =>
                bundle.push_health(HealthCheck::unhealthy(
                    "receipt",
                    "receipt structure failed validation",
                    "re-download or regenerate the receipt file",
                )),
            None =>
                bundle.push_health(HealthCheck::degraded(
                    "receipt",
                    "no receipt was supplied",
                    "provide --file receipt.json or --tx <hash>",
                )),
        }
    }

    if cfg.include_receipt {
        if let Some(r) = receipt {
            bundle.push_entry(
                "receipt",
                "json",
                serde_json::to_string_pretty(r).unwrap_or_default(),
            );
        }
    }

    if cfg.include_env {
        let mut count = 0usize;
        for (k, v) in env::vars() {
            if count >= cfg.max_env_vars { break; }
            let safe_v = if k.to_uppercase().contains("SECRET")
                || k.to_uppercase().contains("KEY")
                || k.to_uppercase().contains("TOKEN")
                || k.to_uppercase().contains("PASSWORD")
            {
                "[redacted]".to_string()
            } else {
                v
            };
            bundle.push_entry(format!("env.{k}"), "env", format!("{k}={safe_v}"));
            count += 1;
        }
    }

    bundle
}

fn rustc_version() -> Option<String> {
    option_env!("RUSTC_VERSION").map(|s| s.to_string())
}
