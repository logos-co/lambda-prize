//! Human-friendly formatting helpers for sizes, hex previews, and error chains.
use alloc::string::{String, ToString};

/// Format a byte count as a human-readable string.
///
/// ```
/// use lez_events::human::human_bytes;
/// assert_eq!(human_bytes(512), "512 B");
/// assert!(human_bytes(2048).contains("KiB"));
/// ```
pub fn human_bytes(n: usize) -> String {
    const KB: f64 = 1_024.0;
    const MB: f64 = 1_048_576.0;
    let f = n as f64;
    if f >= MB      { format!("{:.2} MiB", f / MB) }
    else if f >= KB { format!("{:.2} KiB", f / KB) }
    else            { format!("{n} B") }
}

/// Truncate a hex string to `keep` characters and append a summary of the total.
///
/// ```
/// use lez_events::human::human_hex_preview;
/// let preview = human_hex_preview("abcdef0123456789", 8);
/// assert!(preview.starts_with("abcdef01"));
/// assert!(preview.contains("total"));
/// ```
pub fn human_hex_preview(input: &str, keep: usize) -> String {
    if input.len() <= keep {
        return input.to_string();
    }
    let prefix = &input[..keep.min(input.len())];
    format!("{prefix}… ({} chars total)", input.len())
}

/// Format a `Display` value as a plain string (convenience wrapper).
pub fn human_error_chain(err: &dyn core::fmt::Display) -> String {
    err.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_formats_correctly() {
        assert_eq!(human_bytes(0),       "0 B");
        assert_eq!(human_bytes(512),     "512 B");
        assert_eq!(human_bytes(1024),    "1.00 KiB");
        assert_eq!(human_bytes(2048),    "2.00 KiB");
        assert_eq!(human_bytes(1 << 20), "1.00 MiB");
    }

    #[test]
    fn hex_preview_truncates() {
        let long = "abcdef0123456789";
        let out  = human_hex_preview(long, 8);
        assert!(out.starts_with("abcdef01"));
        assert!(out.contains("total"));
    }

    #[test]
    fn hex_preview_passthrough_when_short() {
        let s = "abcd";
        assert_eq!(human_hex_preview(s, 8), "abcd");
    }
}
