use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivacyDiagnostic {
    pub component:      String,
    pub message:        String,
    pub hint:           Option<String>,
    pub redacted_fields: Vec<String>,
}

impl PrivacyDiagnostic {
    pub fn new(component: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            component:       component.into(),
            message:         message.into(),
            hint:            None,
            redacted_fields: Vec::new(),
        }
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn redact(mut self, field: impl Into<String>) -> Self {
        self.redacted_fields.push(field.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivacyDiagnosticReport {
    pub title: String,
    pub items: Vec<PrivacyDiagnostic>,
}

impl PrivacyDiagnosticReport {
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), items: Vec::new() }
    }

    pub fn push(&mut self, item: PrivacyDiagnostic) {
        self.items.push(item);
    }
}

pub fn redacted_report(
    title: impl Into<String>,
    reason: impl Into<String>,
    fields: &[&str],
) -> PrivacyDiagnosticReport {
    let mut report = PrivacyDiagnosticReport::new(title);
    let mut item = PrivacyDiagnostic::new("privacy", reason);
    for field in fields {
        item = item.redact(*field);
    }
    report.push(item);
    report
}
