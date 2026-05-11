use alloc::string::String;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NullifierDomain(pub [u8; 8]);

impl NullifierDomain {
    pub const SPEND:    Self = Self(*b"SPEND___");
    pub const WITHDRAW: Self = Self(*b"WITHDRW_");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nullifier {
    pub value: [u8; 32],
}

pub fn generate_nullifier(
    secret: &[u8],
    domain: NullifierDomain,
    commitment: &[u8],
) -> Nullifier {
    let mut hasher = Sha256::new();
    hasher.update(domain.0);
    hasher.update(secret);
    hasher.update(commitment);
    Nullifier { value: hasher.finalize().into() }
}

#[cfg(feature = "std")]
pub fn generate_nullifier_hex(
    secret: &[u8],
    domain: NullifierDomain,
    commitment: &[u8],
) -> String {
    hex::encode(generate_nullifier(secret, domain, commitment).value)
}

#[cfg(feature = "std")]
pub fn verify_nullifier(
    secret: &[u8],
    domain: NullifierDomain,
    commitment: &[u8],
    expected_hex: &str,
) -> bool {
    generate_nullifier_hex(secret, domain, commitment)
        == expected_hex.trim().trim_start_matches("0x")
}
