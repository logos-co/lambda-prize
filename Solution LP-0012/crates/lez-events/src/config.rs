//! Configuration types for the `lez-events` CLI and SDK.
//!
//! [`AppConfig`] is serialized to / deserialized from TOML via
//! `to_toml_string()` / `from_toml_str()`.
use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};

use crate::errors::EventError;

// ── OutputFormat ──────────────────────────────────────────────────────────────
/// Output format for CLI commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Pretty-printed JSON (default).
    Pretty,
    /// Compact single-line JSON.
    Json,
    /// One JSON object per line (JSON-Lines).
    JsonLines,
}

impl Default for OutputFormat {
    fn default() -> Self { Self::Pretty }
}

impl core::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Pretty    => write!(f, "pretty"),
            Self::Json      => write!(f, "json"),
            Self::JsonLines => write!(f, "jsonl"),
        }
    }
}

// ── CliConfig ─────────────────────────────────────────────────────────────────
/// Runtime configuration for CLI operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    pub rpc_url: String,
    pub output:  OutputFormat,
    pub color:   bool,
    /// Fail immediately on any validation or decode error.
    pub strict:  bool,
    /// Keep polling for new events (live-tail mode).
    pub follow:  bool,
    /// HTTP request timeout in milliseconds.
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
    /// Number of retry attempts for RPC calls.
    #[serde(default = "default_retries")]
    pub retries: usize,
}

fn default_timeout_ms() -> u64  { 10_000 }
fn default_retries()    -> usize { 3 }

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            rpc_url:    "http://localhost:8080".to_string(),
            output:     OutputFormat::default(),
            color:      true,
            strict:     false,
            follow:     false,
            timeout_ms: default_timeout_ms(),
            retries:    default_retries(),
        }
    }
}

impl CliConfig {
    pub fn validate(&self) -> Result<(), EventError> {
        if self.rpc_url.trim().is_empty() {
            return Err(EventError::MissingField("rpc_url"));
        }
        Ok(())
    }
}

// ── AppConfig ─────────────────────────────────────────────────────────────────
/// Top-level application configuration (wraps `[cli]` table in TOML).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub cli: CliConfig,
}

impl AppConfig {
    /// Parse an [`AppConfig`] from a TOML string.
    pub fn from_toml_str(input: &str) -> Result<Self, EventError> {
        toml::from_str(input).map_err(|e| EventError::Io(e.to_string()))
    }

    /// Serialize this [`AppConfig`] to a pretty-printed TOML string.
    pub fn to_toml_string(&self) -> Result<String, EventError> {
        toml::to_string_pretty(self).map_err(|e| EventError::Io(e.to_string()))
    }

    /// Validate that required fields are populated.
    pub fn validate(&self) -> Result<(), EventError> {
        self.cli.validate()
    }
}
