#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::types::{ChainId, EpochId, Slot};

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProofOfLeadershipConfig {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub target_win_ppm: u64,
    pub min_stake_for_win: u128,
    pub max_proposal_bytes: usize,
    pub proof_version: u8,
    pub hide_total_stake: bool,
}

impl Default for ProofOfLeadershipConfig {
    fn default() -> Self {
        Self {
            chain_id: 0,
            epoch_id: 0,
            target_win_ppm: 25_000,
            min_stake_for_win: 1,
            max_proposal_bytes: 1_048_576,
            proof_version: 2,
            hide_total_stake: true,
        }
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaderIdentityCommitment(pub [u8; 32]);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaderSlotCommitment(pub [u8; 32]);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProposalCommitment(pub [u8; 32]);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofOfLeadershipWitnessDigest(pub [u8; 32]);

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeadershipClaimStatus {
    Pending,
    Proven,
    Verified,
    Rejected,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpochLeadershipState {
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub previous_epoch_output: [u8; 32],
    pub beacon_seed: [u8; 32],
    pub validator_root: [u8; 32],
    pub stake_root: [u8; 32],
    pub total_stake_commitment: [u8; 32],
    pub slot_nonce_seed: [u8; 32],
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaderSlotContext {
    pub slot: Slot,
    pub slot_nonce_commitment: [u8; 32],
    pub blinded_slot_nonce: [u8; 32],
    pub leader_identity_commitment: [u8; 32],
    pub stake_commitment: [u8; 32],
    pub threshold_commitment: [u8; 32],
    pub ticket_commitment: [u8; 32],
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeadershipPublicInputsV2 {
    pub version: u8,
    pub chain_id: ChainId,
    pub epoch_id: EpochId,
    pub slot: Slot,
    pub epoch_seed: [u8; 32],
    pub previous_epoch_output_commitment: [u8; 32],
    pub beacon_seed: [u8; 32],
    pub validator_root: [u8; 32],
    pub stake_root: [u8; 32],
    pub leader_identity_commitment: [u8; 32],
    pub slot_nonce_commitment: [u8; 32],
    pub threshold_commitment: [u8; 32],
    pub ticket_commitment: [u8; 32],
    pub proposal_commitment: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeadershipWitnessV2 {
    pub leader_secret: [u8; 32],
    pub epoch_vrf_secret: [u8; 32],
    pub blinded_slot_nonce: [u8; 32],
    pub stake: u128,
    pub total_stake: u128,
    pub threshold: u128,
    pub ticket: [u8; 32],
    pub vrf_output: [u8; 32],
    pub vrf_proof: [u8; 64],
    pub leader_identity_commitment: [u8; 32],
    pub stake_commitment: [u8; 32],
    pub total_stake_commitment: [u8; 32],
    pub threshold_commitment: [u8; 32],
    pub ticket_commitment: [u8; 32],
}

impl LeadershipWitnessV2 {
    pub fn is_valid_shape(&self) -> bool {
        self.stake > 0 && self.total_stake > 0 && self.threshold > 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeadershipProofV2 {
    pub version: u8,
    pub public_key: [u8; 32],
    pub witness_commitment: [u8; 32],
    pub challenge_digest: [u8; 32],
    pub signature: [u8; 64],
}

#[cfg(feature = "std")]
impl serde::Serialize for LeadershipProofV2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("LeadershipProofV2", 5)?;
        st.serialize_field("version", &self.version)?;
        st.serialize_field("public_key", &self.public_key)?;
        st.serialize_field("witness_commitment", &self.witness_commitment)?;
        st.serialize_field("challenge_digest", &self.challenge_digest)?;
        st.serialize_field("signature", &self.signature.as_ref())?;
        st.end()
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for LeadershipProofV2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Raw {
            version: u8,
            public_key: [u8; 32],
            witness_commitment: [u8; 32],
            challenge_digest: [u8; 32],
            signature: alloc::vec::Vec<u8>,
        }
        let r = Raw::deserialize(d)?;
        if r.signature.len() != 64 {
            return Err(serde::de::Error::custom("signature must be 64 bytes"));
        }
        let mut sig = [0u8; 64];
        sig.copy_from_slice(&r.signature);
        Ok(LeadershipProofV2 {
            version: r.version,
            public_key: r.public_key,
            witness_commitment: r.witness_commitment,
            challenge_digest: r.challenge_digest,
            signature: sig,
        })
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeadershipClaimV2 {
    pub public_inputs: LeadershipPublicInputsV2,
    pub proof: LeadershipProofV2,
    pub claim_commitment: [u8; 32],
    pub status: LeadershipClaimStatus,
}
