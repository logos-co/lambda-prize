use lez_events::{
    build_support_bundle, capture_support_context, render_support_report,
    DiagnosticLevel, DiagnosticRecord, DiagnosticReport,
    HealthCheck, HealthStatus,
    ReceiptEnvelope, ReceiptStatus,
    SupportBundleConfig,
};

#[test]
fn diagnostic_report_accumulates_records() {
    let mut report = DiagnosticReport::new("test report", "unit test summary");
    assert!(report.is_empty());
    report.push(DiagnosticRecord::new(DiagnosticLevel::Info, "component", "message"));
    assert_eq!(report.records.len(), 1);
    assert_eq!(report.error_count(), 0);
    assert_eq!(report.warn_count(), 0);
}

#[test]
fn diagnostic_record_builder_sets_detail_and_code() {
    let record = DiagnosticRecord::new(DiagnosticLevel::Error, "runtime", "failed to decode")
        .with_detail("invalid version byte 0xFF")
        .with_code("E0042");
    assert_eq!(record.level, DiagnosticLevel::Error);
    assert_eq!(record.detail.as_deref(), Some("invalid version byte 0xFF"));
    assert_eq!(record.code.as_deref(),   Some("E0042"));
}

#[test]
fn diagnostic_report_counts_levels_correctly() {
    let mut report = DiagnosticReport::new("t", "s");
    report.push(DiagnosticRecord::new(DiagnosticLevel::Error, "a", "err1"));
    report.push(DiagnosticRecord::new(DiagnosticLevel::Error, "a", "err2"));
    report.push(DiagnosticRecord::new(DiagnosticLevel::Warn,  "b", "warn1"));
    assert_eq!(report.error_count(), 2);
    assert_eq!(report.warn_count(),  1);
}

#[test]
fn health_check_constructors_set_status() {
    let ok   = HealthCheck::healthy("rpc", "reachable");
    let deg  = HealthCheck::degraded("rpc", "slow", "check network");
    let sick = HealthCheck::unhealthy("rpc", "unreachable", "restart node");
    assert!(ok.status.is_healthy());
    assert!(deg.status.is_problem());
    assert!(sick.status.is_problem());
    assert_eq!(sick.remediation.as_deref(), Some("restart node"));
}

#[test]
fn support_bundle_includes_health_and_diagnostics() {
    let receipt = ReceiptEnvelope {
        tx_hash:    "0xabc".into(),
        status:     ReceiptStatus::Failed,
        error:      Some("boom".into()),
        state_root: None,
        events:     vec!["00deadbeef".into()],
    };

    let bundle = build_support_bundle(
        &SupportBundleConfig::default(),
        capture_support_context("test"),
        Some(&receipt),
    );

    assert!(bundle.diagnostics.is_some());
    assert!(!bundle.health.is_empty());
}

#[test]
fn support_bundle_without_receipt_is_degraded() {
    let bundle = build_support_bundle(
        &SupportBundleConfig::default(),
        capture_support_context("test"),
        None,
    );
    let receipt_check = bundle.health.iter().find(|h| h.name == "receipt").unwrap();
    assert_eq!(receipt_check.status, HealthStatus::Degraded);
}

#[test]
fn render_support_report_contains_command_name() {
    let bundle = build_support_bundle(
        &SupportBundleConfig::default(),
        capture_support_context("doctor"),
        None,
    );
    let report = render_support_report(&bundle);
    assert!(report.contains("doctor"));
    assert!(report.contains("Support Bundle"));
}

#[test]
fn bundle_problem_count_tracks_non_healthy_checks() {
    let bundle = build_support_bundle(
        &SupportBundleConfig::default(),
        capture_support_context("test"),
        None,
    );
    assert!(bundle.problem_count() >= 1, "missing receipt should register as degraded");
}

#[test]
fn bundle_serialises_to_json_and_back() {
    let bundle = build_support_bundle(
        &SupportBundleConfig { include_env: false, ..SupportBundleConfig::default() },
        capture_support_context("roundtrip"),
        None,
    );
    let raw = serde_json::to_string(&bundle).expect("serialise");
    let back: lez_events::bundle::Bundle = serde_json::from_str(&raw).expect("deserialise");
    assert_eq!(back.context.command, "roundtrip");
}
