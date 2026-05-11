use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Severity level for a diagnostic record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl DiagnosticLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info  => "info",
            Self::Warn  => "warn",
            Self::Error => "error",
        }
    }
}

impl core::fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A single structured diagnostic entry.
///
/// Records carry a severity level, the component that produced them,
/// a human-readable message, optional detail text, and an optional
/// machine-readable code for tooling.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagnosticRecord {
    pub level:     DiagnosticLevel,
    pub component: String,
    pub message:   String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail:    Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code:      Option<String>,
}

impl DiagnosticRecord {
    pub fn new(
        level:     DiagnosticLevel,
        component: impl Into<String>,
        message:   impl Into<String>,
    ) -> Self {
        Self {
            level,
            component: component.into(),
            message:   message.into(),
            detail:    None,
            code:      None,
        }
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

/// A named collection of [`DiagnosticRecord`]s with a title and summary.
///
/// Suitable for writing to a support bundle, a log file, or stdout.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiagnosticReport {
    pub title:   String,
    pub summary: String,
    pub records: Vec<DiagnosticRecord>,
}

impl DiagnosticReport {
    pub fn new(title: impl Into<String>, summary: impl Into<String>) -> Self {
        Self { title: title.into(), summary: summary.into(), records: Vec::new() }
    }

    pub fn push(&mut self, record: DiagnosticRecord) {
        self.records.push(record);
    }

    pub fn is_empty(&self) -> bool { self.records.is_empty() }

    pub fn error_count(&self) -> usize {
        self.records.iter().filter(|r| r.level == DiagnosticLevel::Error).count()
    }

    pub fn warn_count(&self) -> usize {
        self.records.iter().filter(|r| r.level == DiagnosticLevel::Warn).count()
    }
}

/// Snapshot of the environment at the time a CLI command was invoked.
///
/// Included in support bundles to make failures reproducible.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupportContext {
    pub command:      String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_url:      Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash:      Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id:   Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_path:  Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target:       Option<String>,
}

impl SupportContext {
    pub fn empty(command: impl Into<String>) -> Self {
        Self {
            command:      command.into(),
            rpc_url:      None,
            tx_hash:      None,
            program_id:   None,
            receipt_path: None,
            config_path:  None,
            rust_version: None,
            target:       None,
        }
    }
}
