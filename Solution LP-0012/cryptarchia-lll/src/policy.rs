use serde::{Deserialize, Serialize};

use crate::{LllResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalVisibilityPolicy {
    HiddenByDefault,
    RevealOnWinOnly,
    RevealAfterFinality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitteePolicy {
    Open,
    Restricted,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProposalPolicy {
    pub visibility: ProposalVisibilityPolicy,
    pub committee: CommitteePolicy,
    pub require_slot_signature: bool,
    pub require_commitment: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalPolicyDecision {
    Allow,
    Deny,
}

impl ProposalPolicy {
    pub fn strict_private() -> Self {
        Self {
            visibility: ProposalVisibilityPolicy::HiddenByDefault,
            committee: CommitteePolicy::Hidden,
            require_slot_signature: true,
            require_commitment: true,
        }
    }

    pub fn validate(&self) -> LllResult<()> {
        Ok(())
    }

    pub fn decide(
        &self,
        winner_known: bool,
        has_commitment: bool,
        signed_slot: bool,
    ) -> ProposalPolicyDecision {
        if self.require_commitment && !has_commitment {
            return ProposalPolicyDecision::Deny;
        }
        if self.require_slot_signature && !signed_slot {
            return ProposalPolicyDecision::Deny;
        }
        if matches!(self.visibility, ProposalVisibilityPolicy::RevealOnWinOnly) && !winner_known {
            return ProposalPolicyDecision::Deny;
        }
        ProposalPolicyDecision::Allow
    }
}
