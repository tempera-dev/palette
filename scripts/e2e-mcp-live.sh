#!/usr/bin/env bash
# Live MCP E2E: prove the MCP surface works end-to-end over both transports,
# in both auth modes, against a real beaterd — the path an agentic client
# (Claude Code, Codex, any MCP host) actually takes:
#
#   1. anon:     beaterd --auth-mode local; ingest a real trace over OTLP via
#                `beaterctl smoke`; drive POST /mcp with JSON-RPC (initialize,
#                tools/list, tools/call) and assert byte-parity with REST.
#   2. required: beaterd --auth-mode required with a bootstrapped API key;
#                assert 401 + RFC 9728 challenge without credentials, success
#                with them.
#   3. stdio:    `beaterd mcp --stdio` with credentials injected via the
#                documented BEATER_* env vars; assert an authenticated real
#                tools/call round-trips.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

PORT="${BEATER_MCP_E2E_PORT:-18091}"
GRPC_PORT="${BEATER_MCP_E2E_GRPC_PORT:-14328}"
BASE_URL="http://127.0.0.1:$PORT"
anon_dir="$(mktemp -d)"
auth_dir="$(mktemp -d)"
log="$(mktemp)"

cleanup() {
  [ -n "${beaterd_pid:-}" ] && kill "$beaterd_pid" 2>/dev/null || true
  rm -rf "$anon_dir" "$auth_dir"
}
trap cleanup EXIT

wait_health() {
  for _ in $(seq 1 120); do
    if curl -fsS "$BASE_URL/health" >/dev/null 2>&1; then return 0; fi
    sleep 0.5
  done
  echo "beaterd did not become healthy:" >&2
  cat "$log" >&2
  return 1
}

stop_beaterd() {
  if [ -n "${beaterd_pid:-}" ]; then
    kill "$beaterd_pid" 2>/dev/null || true
    wait "$beaterd_pid" 2>/dev/null || true
    beaterd_pid=""
  fi
}

echo "==> Building beaterd + beaterctl"
cargo build -q -p beaterd -p beaterctl
beaterd=./target/debug/beaterd
beaterctl=./target/debug/beaterctl

echo "==> Phase 1: HTTP /mcp, --auth-mode local (anonymous)"
"$beaterd" \
  --addr "127.0.0.1:$PORT" \
  --otlp-grpc-addr "127.0.0.1:$GRPC_PORT" \
  --data-dir "$anon_dir" \
  --auth-mode local >"$log" 2>&1 &
beaterd_pid=$!
wait_health

echo "==> Ingesting a real trace over HTTP + OTLP gRPC (beaterctl smoke)"
"$beaterctl" smoke \
  --data-dir "$anon_dir" \
  --http-url "$BASE_URL" \
  --otlp-grpc-url "http://127.0.0.1:$GRPC_PORT" >/dev/null

MODE=anon \
BEATER_MCP_URL="$BASE_URL/mcp" \
BEATER_REST_URL="$BASE_URL" \
BEATER_TENANT_ID=demo BEATER_PROJECT_ID=demo BEATER_ENVIRONMENT_ID=local \
  python3 scripts/e2e-mcp-live.py
stop_beaterd

echo "==> Phase 2: HTTP /mcp, --auth-mode required (API key)"
api_key="$("$beaterctl" api-key-create \
  --data-dir "$auth_dir" \
  --tenant-id demo --project-id demo --environment-id local \
  --scopes trace-read \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["secret"])')"

"$beaterd" \
  --addr "127.0.0.1:$PORT" \
  --otlp-grpc-addr "127.0.0.1:$GRPC_PORT" \
  --data-dir "$auth_dir" \
  --auth-mode required >"$log" 2>&1 &
beaterd_pid=$!
wait_health

MODE=required \
BEATER_MCP_URL="$BASE_URL/mcp" \
BEATER_REST_URL="$BASE_URL" \
BEATER_API_KEY="$api_key" \
BEATER_TENANT_ID=demo BEATER_PROJECT_ID=demo BEATER_ENVIRONMENT_ID=local \
  python3 scripts/e2e-mcp-live.py
stop_beaterd

echo "==> Phase 3: stdio transport with env-injected credentials"
stdio_requests='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"listTraces","arguments":{"tenant_id":"demo","project_id":"demo","environment_id":"local"}}}'

printf '%s\n' "$stdio_requests" \
  | BEATER_API_KEY="$api_key" BEATER_PROJECT_ID=demo BEATER_ENVIRONMENT_ID=local \
    "$beaterd" --data-dir "$auth_dir" --bus-backend memory mcp --stdio \
  | python3 -c '
import json, sys
lines = [json.loads(line) for line in sys.stdin if line.strip()]
assert len(lines) == 2, f"expected 2 JSON-RPC responses, got {len(lines)}"
init, call = lines
assert init["result"]["serverInfo"]["name"] == "beater-mcp", init
result = call["result"]
assert result["isError"] is False, f"stdio credentialed call failed: {result}"
assert result["_meta"]["httpStatus"] == 200, result
assert isinstance(result["structuredContent"]["items"], list), result
print("ok: stdio authenticated tools/call round-trip")
'

printf '%s\n' "$stdio_requests" \
  | "$beaterd" --data-dir "$auth_dir" --bus-backend memory mcp --stdio \
  | python3 -c '
import json, sys
lines = [json.loads(line) for line in sys.stdin if line.strip()]
result = lines[1]["result"]
assert result["isError"] is True, f"uncredentialed stdio call must fail: {result}"
assert result["_meta"]["httpStatus"] == 401, result
print("ok: stdio without credentials rejected (401)")
'

echo "==> e2e-mcp-live passed (HTTP anon + HTTP required + stdio)"
