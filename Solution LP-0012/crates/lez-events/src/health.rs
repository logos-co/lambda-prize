use alloc::string::String;
use serde::{Deserialize, Serialize};

/// Overall health status of a component or check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Healthy   => "healthy",
            Self::Degraded  => "degraded",
            Self::Unhealthy => "unhealthy",
        }
    }

    pub fn is_healthy(self) -> bool { self == Self::Healthy }
    pub fn is_problem(self) -> bool { self != Self::Healthy }
}

impl core::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Result of a single named health check.
///
/// Healthy checks have no remediation hint; degraded/unhealthy checks include
/// a concrete remediation step so operators know exactly what to do.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthCheck {
    pub name:        String,
    pub status:      HealthStatus,
    pub message:     String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
}

impl HealthCheck {
    pub fn healthy(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            name:        name.into(),
            status:      HealthStatus::Healthy,
            message:     message.into(),
            remediation: None,
        }
    }

    pub fn degraded(
        name:        impl Into<String>,
        message:     impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        Self {
            name:        name.into(),
            status:      HealthStatus::Degraded,
            message:     message.into(),
            remediation: Some(remediation.into()),
        }
    }

    pub fn unhealthy(
        name:        impl Into<String>,
        message:     impl Into<String>,
        remediation: impl Into<String>,
    ) -> Self {
        Self {
            name:        name.into(),
            status:      HealthStatus::Unhealthy,
            message:     message.into(),
            remediation: Some(remediation.into()),
        }
    }
}
