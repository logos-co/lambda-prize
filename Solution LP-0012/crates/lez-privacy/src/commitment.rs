use alloc::string::String;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommitmentDomain(pub [u8; 8]);

impl CommitmentDomain {
    pub const BALANCE:    Self = Self(*b"BALANCE\0");
    pub const NOTE:       Self = Self(*b"NOTE____");
    pub const NULLIFIER:  Self = Self(*b"NULLIFIE");
    pub const TRANSFER:   Self = Self(*b"TRANSFER");
}

pub fn commitment_bytes(domain: CommitmentDomain, data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(domain.0);
    hasher.update(data);
    hasher.finalize().into()
}

pub fn commitment_with_domain(domain: [u8; 8], data: &[u8]) -> [u8; 32] {
    commitment_bytes(CommitmentDomain(domain), data)
}

#[cfg(feature = "std")]
pub fn commitment_hex(domain: CommitmentDomain, data: &[u8]) -> String {
    hex::encode(commitment_bytes(domain, data))
}

#[cfg(feature = "std")]
pub fn verify_commitment(domain: CommitmentDomain, data: &[u8], expected_hex: &str) -> bool {
    let got = commitment_hex(domain, data);
    got == expected_hex.trim().trim_start_matches("0x")
}
