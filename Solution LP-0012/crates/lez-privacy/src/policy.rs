use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::PrivacyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyAction {
    Read,
    Spend,
    Transfer,
    Reveal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyRule {
    pub subject:  String,
    pub action:   PolicyAction,
    pub effect:   PolicyEffect,
    pub resource: Option<String>,
    pub note:     Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct AccessPolicySet {
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessDecision {
    Allowed,
    Denied { reason: String },
}

pub trait AccessPolicy {
    fn decide(
        &self,
        subject: &str,
        action: PolicyAction,
        resource: Option<&str>,
    ) -> AccessDecision;
}

impl AccessPolicy for AccessPolicySet {
    fn decide(
        &self,
        subject: &str,
        action: PolicyAction,
        resource: Option<&str>,
    ) -> AccessDecision {
        let mut deny_hit = None;

        for rule in &self.rules {
            if rule.subject != "*" && rule.subject != subject {
                continue;
            }
            if rule.action != action {
                continue;
            }
            if let Some(ref wanted) = rule.resource {
                if Some(wanted.as_str()) != resource {
                    continue;
                }
            }

            match rule.effect {
                PolicyEffect::Allow => return AccessDecision::Allowed,
                PolicyEffect::Deny => {
                    deny_hit = Some(
                        rule.note
                            .clone()
                            .unwrap_or_else(|| "policy denied access".to_string()),
                    )
                }
            }
        }

        if let Some(reason) = deny_hit {
            AccessDecision::Denied { reason }
        } else {
            AccessDecision::Denied { reason: "no matching allow rule".to_string() }
        }
    }
}

impl AccessPolicySet {
    pub fn allow_all_read() -> Self {
        Self {
            rules: vec![PolicyRule {
                subject:  "*".into(),
                action:   PolicyAction::Read,
                effect:   PolicyEffect::Allow,
                resource: None,
                note:     Some("default public read policy".into()),
            }],
        }
    }

    pub fn require_subject(subject: impl Into<String>, action: PolicyAction) -> Self {
        Self {
            rules: vec![PolicyRule {
                subject:  subject.into(),
                action,
                effect:   PolicyEffect::Allow,
                resource: None,
                note:     Some("explicit subject allow".into()),
            }],
        }
    }

    pub fn validate(&self) -> Result<(), PrivacyError> {
        for rule in &self.rules {
            if rule.subject.trim().is_empty() {
                return Err(PrivacyError::InvalidPolicy);
            }
        }
        Ok(())
    }
}
