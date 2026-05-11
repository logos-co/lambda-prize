use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::diagnostics::{DiagnosticReport, SupportContext};
use crate::health::HealthCheck;

/// Top-level category of a support bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BundleKind {
    Support,
    Diagnostics,
    Health,
}

impl core::fmt::Display for BundleKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Support     => f.write_str("support"),
            Self::Diagnostics => f.write_str("diagnostics"),
            Self::Health      => f.write_str("health"),
        }
    }
}

/// A named, typed entry inside a [`Bundle`] (config file, receipt JSON, env var, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleEntry {
    pub name:    String,
    pub kind:    String,
    pub content: String,
}

/// A portable, self-contained support artifact.
///
/// A bundle captures everything needed to reproduce and understand a failure:
/// the command context, health-check results, structured diagnostics, and
/// arbitrary named entries (receipts, env vars, config snapshots).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bundle {
    pub kind:        BundleKind,
    pub context:     SupportContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticReport>,
    pub health:      Vec<HealthCheck>,
    pub entries:     Vec<BundleEntry>,
}

impl Bundle {
    pub fn new(kind: BundleKind, context: SupportContext) -> Self {
        Self { kind, context, diagnostics: None, health: Vec::new(), entries: Vec::new() }
    }

    pub fn push_entry(
        &mut self,
        name:    impl Into<String>,
        kind:    impl Into<String>,
        content: impl Into<String>,
    ) {
        self.entries.push(BundleEntry {
            name:    name.into(),
            kind:    kind.into(),
            content: content.into(),
        });
    }

    pub fn set_diagnostics(&mut self, diagnostics: DiagnosticReport) {
        self.diagnostics = Some(diagnostics);
    }

    pub fn push_health(&mut self, item: HealthCheck) {
        self.health.push(item);
    }

    /// Returns true if all health checks passed.
    pub fn is_healthy(&self) -> bool {
        self.health.iter().all(|h| h.status.is_healthy())
    }

    /// Returns the count of non-healthy checks.
    pub fn problem_count(&self) -> usize {
        self.health.iter().filter(|h| h.status.is_problem()).count()
    }
}
