use alloc::string::String;
use sha2::{Digest, Sha256};

pub fn checksum_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

pub fn verify_checksum_hex(bytes: &[u8], expected: &str) -> bool {
    checksum_hex(bytes) == expected.trim().trim_start_matches("0x")
}
