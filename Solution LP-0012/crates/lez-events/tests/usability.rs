use lez_events::{
    human::{human_bytes, human_hex_preview},
    validation::{validate_program_id, validate_tx_hash},
    AppConfig, CliConfig, OutputFormat,
};

#[test]
fn human_bytes_formats_correctly() {
    assert_eq!(human_bytes(0),    "0 B");
    assert_eq!(human_bytes(512),  "512 B");
    assert!(human_bytes(2048).contains("KiB"));
    assert!(human_bytes(1 << 20).contains("MiB"));
}

#[test]
fn human_hex_preview_truncates() {
    let s   = "abcdef0123456789";
    let out = human_hex_preview(s, 8);
    assert!(out.starts_with("abcdef01"));
    assert!(out.contains("total"));
}

#[test]
fn human_hex_preview_passthrough_when_short() {
    assert_eq!(human_hex_preview("abcd", 8), "abcd");
}

#[test]
fn config_round_trip() {
    let cfg = AppConfig {
        cli: CliConfig {
            rpc_url:    "http://localhost:9999".into(),
            output:     OutputFormat::Pretty,
            color:      false,
            strict:     true,
            follow:     false,
            timeout_ms: 5_000,
            retries:    2,
        },
    };
    let raw    = cfg.to_toml_string().expect("serialise");
    let parsed = AppConfig::from_toml_str(&raw).expect("parse");
    assert_eq!(parsed.cli.rpc_url, "http://localhost:9999");
    assert!(parsed.cli.strict);
    assert!(!parsed.cli.color);
}

#[test]
fn config_default_values() {
    let cfg = CliConfig::default();
    assert_eq!(cfg.rpc_url,  "http://localhost:8080");
    assert_eq!(cfg.output,   OutputFormat::Pretty);
    assert!(cfg.color);
    assert!(!cfg.strict);
    assert!(!cfg.follow);
    assert_eq!(cfg.timeout_ms, 10_000);
    assert_eq!(cfg.retries,    3);
}

#[test]
fn validation_rejects_empty_tx_hash() {
    assert!(validate_tx_hash("").is_err());
}

#[test]
fn validation_accepts_valid_tx_hash() {
    assert!(validate_tx_hash("0xdeadbeef").is_ok());
    assert!(validate_tx_hash("deadbeef").is_ok());
}

#[test]
fn validation_rejects_short_program_id() {
    assert!(validate_program_id("1234").is_err());
}

#[test]
fn validation_accepts_valid_program_id() {
    let id = "ab".repeat(32);
    assert!(validate_program_id(&id).is_ok());
}
