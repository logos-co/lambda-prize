use alloc::string::{String, ToString};

pub fn domain_tag(name: &str) -> [u8; 8] {
    let mut out = [0u8; 8];
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in name.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    out.copy_from_slice(&hash.to_le_bytes());
    out
}

#[cfg(feature = "std")]
pub fn checksum_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

#[cfg(feature = "std")]
pub fn redacted_hex(hex_value: &str, keep: usize) -> String {
    let s = hex_value.trim().trim_start_matches("0x");
    if s.len() <= keep {
        return s.to_string();
    }
    format!("{}…", &s[..keep.min(s.len())])
}

#[cfg(feature = "std")]
pub fn support_preview(data: &[u8]) -> String {
    let checksum = checksum_hex(data);
    format!("len={} sha256={}", data.len(), redacted_hex(&checksum, 12))
}

pub fn human_amount(value: u128, decimals: u8) -> String {
    if decimals == 0 {
        return value.to_string();
    }
    let scale = 10u128.saturating_pow(decimals as u32);
    let whole = value / scale;
    let frac = value % scale;
    format!("{whole}.{:0width$}", frac, width = decimals as usize)
}
