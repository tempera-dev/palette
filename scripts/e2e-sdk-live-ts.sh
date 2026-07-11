#!/usr/bin/env bash
# Live E2E for the TypeScript SDK. Reuses an already-built paletted binary (no
# cargo build) so it can run alongside other compilation.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

PORT="${PALETTE_E2E_PORT:-18090}"
GRPC_PORT="${PALETTE_E2E_GRPC_PORT:-14327}"
data_dir="$(mktemp -d)"
log="$(mktemp)"

cleanup() { [ -n "${pid:-}" ] && kill "$pid" 2>/dev/null || true; rm -rf "$data_dir"; }
trap cleanup EXIT

[ -x ./target/debug/paletted ] || { echo "build paletted first (run scripts/e2e-sdk-live.sh once)"; exit 1; }

./target/debug/paletted --addr "127.0.0.1:$PORT" --otlp-grpc-addr "127.0.0.1:$GRPC_PORT" \
  --data-dir "$data_dir" --auth-mode local >"$log" 2>&1 &
pid=$!

for _ in $(seq 1 60); do
  curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null 2>&1 && { ready=1; break; }
  sleep 0.5
done
[ -n "${ready:-}" ] || { echo "paletted not healthy"; cat "$log"; exit 1; }

cd sdks/typescript
[ -d node_modules ] || npm install >/dev/null 2>&1
PALETTE_BASE_URL="http://127.0.0.1:$PORT" PALETTE_TENANT_ID=demo PALETTE_PROJECT_ID=demo PALETTE_ENVIRONMENT_ID=local \
  node --import tsx tests/e2e_live.ts
