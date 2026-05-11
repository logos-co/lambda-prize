use alloc::string::String;
use alloc::vec::Vec;

pub fn stable_ratio_u128(numerator: u128, denominator: u128) -> u128 {
    if denominator == 0 {
        return 0;
    }
    numerator.saturating_mul(u128::MAX / denominator)
}

pub fn bounded_u64(value: u64, min: u64, max: u64) -> u64 {
    value.clamp(min, max)
}

pub fn rolling_mix(previous: &[u8; 32], next: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(previous);
    data.extend_from_slice(next);
    data.extend_from_slice(b"cryptarchia/rolling-mix");
    crate::crypto::hash32(&data)
}

pub fn format_ppm(ppm: u64) -> String {
    let whole = ppm / 10_000;
    let frac = (ppm % 10_000) / 100;
    alloc::format!("{}.{:02}%", whole, frac)
}

pub fn redact_hex(bytes: &[u8]) -> String {
    if bytes.len() < 4 {
        return "****".into();
    }
    let prefix = hex::encode(&bytes[..2]);
    let suffix = hex::encode(&bytes[bytes.len() - 2..]);
    alloc::format!("{}...{}", prefix, suffix)
}

pub fn support_preview(alias: &[u8; 32]) -> String {
    redact_hex(alias)
}
