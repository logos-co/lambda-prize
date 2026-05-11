use serde_json::json;

#[test]
fn receipt_json_round_trip() {
    let receipt = json!({
        "tx_hash": "0xabc",
        "status": "failed",
        "error": "boom",
        "state_root": null,
        "events": ["0000000000000000000000000000000000000000000000000000000000000000 00aabbccdd"]
    });

    let rendered = serde_json::to_string(&receipt).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&rendered).unwrap();

    assert_eq!(parsed["tx_hash"], "0xabc");
    assert_eq!(parsed["status"], "failed");
    assert_eq!(parsed["events"].as_array().unwrap().len(), 1);
}

#[test]
fn receipt_with_no_events_is_valid() {
    let receipt = json!({
        "tx_hash": "0xdeadbeef",
        "status": "failed",
        "error": "simulated failure",
        "state_root": null,
        "events": []
    });
    let s = serde_json::to_string(&receipt).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).unwrap();
    assert_eq!(v["events"].as_array().unwrap().len(), 0);
}
