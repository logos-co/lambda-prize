use sha2::{Digest, Sha256};

pub fn hash_bytes32(domain: &str, parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

pub fn hash_concat32(domain: &str, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    hash_bytes32(domain, &[left.as_ref(), right.as_ref()])
}

pub fn commitment_bytes(domain: &str, bytes: &[u8]) -> [u8; 32] {
    hash_bytes32(domain, &[bytes])
}

pub fn commitment_u128(domain: &str, value: u128) -> [u8; 32] {
    commitment_bytes(domain, &value.to_le_bytes())
}

pub fn commitment_u64(domain: &str, value: u64) -> [u8; 32] {
    commitment_bytes(domain, &value.to_le_bytes())
}

#[cfg(feature = "std")]
pub fn commitment_hex(domain: &str, bytes: &[u8]) -> alloc::string::String {
    hex::encode(commitment_bytes(domain, bytes))
}

pub fn domain_hash32(domain: &str, bytes: &[u8]) -> [u8; 32] {
    hash_bytes32(domain, &[bytes])
}
