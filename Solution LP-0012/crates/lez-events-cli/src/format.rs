use anyhow::Result;
use lez_events::{receipt::DecodedReceipt, OutputFormat};

/// Render a [`DecodedReceipt`] as a string in the requested format.
pub fn render_receipt(receipt: &DecodedReceipt, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Pretty    => Ok(serde_json::to_string_pretty(receipt)?),
        OutputFormat::Json      => Ok(serde_json::to_string(receipt)?),
        OutputFormat::JsonLines => Ok(serde_json::to_string(receipt)?),
    }
}

/// Format a `std::error::Error` chain into a readable multi-line string.
#[allow(dead_code)]
pub fn render_error(err: &dyn std::error::Error) -> String {
    let mut out    = err.to_string();
    let mut source = err.source();
    while let Some(next) = source {
        out.push_str("\n  caused by: ");
        out.push_str(&next.to_string());
        source = next.source();
    }
    out
}
