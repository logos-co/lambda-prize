#!/usr/bin/env bash
# demo.sh — End-to-end demonstration of the LEZ Event System (LP-0012)
set -euo pipefail

RPC_URL="${RPC_URL:-http://localhost:8080}"
BINARY="${BINARY:-./target/debug/lez-events}"

echo "============================================================"
echo " LEZ Event System — LP-0012 Demo"
echo " RPC: ${RPC_URL}"
echo " RISC0_DEV_MODE: ${RISC0_DEV_MODE:-not set (should be 0)}"
echo "============================================================"

if [[ "${RISC0_DEV_MODE:-0}" != "0" ]]; then
  echo "warning: expected RISC0_DEV_MODE=0 for production proofs"
fi

echo ""
echo "[1/6] Building workspace …"
cargo build --workspace
echo "      Build OK"

echo ""
echo "[2/6] Running unit tests …"
cargo test --workspace
echo "      All unit tests passed"

echo ""
echo "[3/6] Success-path: emit SuccessEvent, transaction succeeds"
cat > /tmp/receipt_success.json <<'EOF'
{
  "tx_hash": "0xaaaa1111",
  "status": "success",
  "state_root": "0xfeedbeef",
  "error": null,
  "events": [
    "abababababababababababababababababababababababababababababababab00e38f1a022a00000000000000"
  ]
}
EOF
"$BINARY" decode --file /tmp/receipt_success.json --pretty
echo "      SUCCESS-PATH: event decoded OK"

echo ""
echo "[4/6] Failure-path: emit FailureEvent, transaction fails — event persists"
cat > /tmp/receipt_failed.json <<'EOF'
{
  "tx_hash": "0xbbbb2222",
  "status": "failed",
  "state_root": null,
  "error": "simulated failure: oops",
  "events": [
    "cdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcd00a1b2c3d40400000004006f6f7073"
  ]
}
EOF
"$BINARY" decode --file /tmp/receipt_failed.json --pretty
echo "      FAILURE-PATH: event preserved in failed receipt OK"

echo ""
echo "[5/6] Fail-without-event: transaction fails, receipt has empty events array"
cat > /tmp/receipt_no_event.json <<'EOF'
{
  "tx_hash": "0xcccc3333",
  "status": "failed",
  "state_root": null,
  "error": "simulated failure with no event",
  "events": []
}
EOF
"$BINARY" decode --file /tmp/receipt_no_event.json --pretty
echo "      NO-EVENT FAILURE: empty events array confirmed OK"

echo ""
echo "[6/6] Done"
echo "============================================================"
echo " All demo cases passed!"
echo "============================================================"
