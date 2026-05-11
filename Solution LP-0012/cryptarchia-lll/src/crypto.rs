use alloc::string::String;
use alloc::vec::Vec;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use hmac::{Hmac, Mac};
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256, Sha512};

use crate::types::{ChainId, EpochId, LeaderAlias, NodePublicKey, Slot};

type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, Clone)]
pub struct NodeSecret {
    pub secret: [u8; 32],
}

impl NodeSecret {
    pub fn random() -> Self {
        let mut secret = [0u8; 32];
        OsRng.fill_bytes(&mut secret);
        Self { secret }
    }

    pub fn from_bytes(secret: [u8; 32]) -> Self {
        Self { secret }
    }

    pub fn signing_key(&self, epoch_seed: &[u8; 32]) -> SigningKey {
        let mut hasher = blake3::Hasher::new_keyed(&self.secret);
        hasher.update(epoch_seed);
        hasher.update(b"cryptarchia/epoch-key");
        let digest = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(digest.as_bytes());
        SigningKey::from_bytes(&seed)
    }

    pub fn public_key(&self, epoch_seed: &[u8; 32]) -> NodePublicKey {
        let signing_key = self.signing_key(epoch_seed);
        NodePublicKey(signing_key.verifying_key().to_bytes())
    }

    pub fn alias(&self, chain_id: ChainId, epoch_id: EpochId, epoch_seed: &[u8; 32]) -> LeaderAlias {
        derive_alias(chain_id, epoch_id, epoch_seed, &self.secret)
    }
}

pub fn random_node_secret() -> NodeSecret {
    NodeSecret::random()
}

pub fn hash32(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn hash64(data: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().into()
}

pub fn commitment_hex(label: &str, bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(label.as_bytes());
    h.update(bytes);
    hex::encode(h.finalize())
}

pub fn derive_alias(
    chain_id: ChainId,
    epoch_id: EpochId,
    epoch_seed: &[u8; 32],
    node_secret: &[u8; 32],
) -> LeaderAlias {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"cryptarchia/alias");
    hasher.update(chain_id.to_le_bytes().as_ref());
    hasher.update(epoch_id.to_le_bytes().as_ref());
    hasher.update(epoch_seed);
    hasher.update(node_secret);
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(digest.as_bytes());
    LeaderAlias(out)
}

pub fn derive_epoch_keypair(secret: &NodeSecret, epoch_seed: &[u8; 32]) -> SigningKey {
    secret.signing_key(epoch_seed)
}

fn hmac_ticket_key(node_secret: &[u8; 32], chain_id: ChainId, epoch_id: EpochId) -> [u8; 64] {
    let mut mac = HmacSha512::new_from_slice(node_secret).expect("HMAC key length valid");
    mac.update(b"cryptarchia/ticket-key");
    mac.update(&chain_id.to_le_bytes());
    mac.update(&epoch_id.to_le_bytes());
    let bytes = mac.finalize().into_bytes();
    let mut out = [0u8; 64];
    out.copy_from_slice(&bytes);
    out
}

pub fn derive_ticket(
    node_secret: &NodeSecret,
    chain_id: ChainId,
    epoch_id: EpochId,
    slot: Slot,
    beacon_seed: &[u8; 32],
    stake: u128,
    total_stake: u128,
    validator_root: &[u8; 32],
) -> [u8; 32] {
    let key = hmac_ticket_key(&node_secret.secret, chain_id, epoch_id);
    let mut mac = HmacSha512::new_from_slice(&key).expect("HMAC key length valid");
    mac.update(b"cryptarchia/lottery-ticket");
    mac.update(&chain_id.to_le_bytes());
    mac.update(&epoch_id.to_le_bytes());
    mac.update(&slot.to_le_bytes());
    mac.update(beacon_seed);
    mac.update(&stake.to_le_bytes());
    mac.update(&total_stake.to_le_bytes());
    mac.update(validator_root);
    let bytes = mac.finalize().into_bytes();
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes[..32]);
    out
}

pub fn ticket_below_threshold(ticket: &[u8; 32], threshold: u128) -> bool {
    let mut first16 = [0u8; 16];
    first16.copy_from_slice(&ticket[..16]);
    let ticket_value = u128::from_le_bytes(first16);
    ticket_value <= threshold
}

pub fn sign_challenge(
    node_secret: &NodeSecret,
    epoch_seed: &[u8; 32],
    challenge: &[u8],
) -> (NodePublicKey, Vec<u8>) {
    let signing_key = node_secret.signing_key(epoch_seed);
    let public_key = signing_key.verifying_key();
    let signature = signing_key.sign(challenge);
    (NodePublicKey(public_key.to_bytes()), signature.to_bytes().to_vec())
}

pub fn verify_signature(public_key: &NodePublicKey, challenge: &[u8], signature: &[u8]) -> bool {
    let vk = match VerifyingKey::from_bytes(&public_key.0) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let sig = match ed25519_dalek::Signature::try_from(signature) {
        Ok(s) => s,
        Err(_) => return false,
    };
    vk.verify_strict(challenge, &sig).is_ok()
}
