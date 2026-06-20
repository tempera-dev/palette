#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
data_dir="${BEATER_GATE2_DATA_DIR:-/tmp/beater-gate2-proof}"
http_addr="${BEATER_GATE2_HTTP_ADDR:-127.0.0.1:8080}"
grpc_addr="${BEATER_GATE2_GRPC_ADDR:-127.0.0.1:4317}"
dashboard_host="${BEATER_GATE2_DASHBOARD_HOST:-127.0.0.1}"
dashboard_port="${BEATER_GATE2_DASHBOARD_PORT:-3000}"
api_url="http://$http_addr"
grpc_url="http://$grpc_addr"
dashboard_url="http://$dashboard_host:$dashboard_port"
venv_dir="${BEATER_GATE2_VENV:-/tmp/beater-gate2-otel-venv}"

server_pid=""
dashboard_pid=""

cleanup() {
  if [[ -n "$dashboard_pid" ]]; then kill "$dashboard_pid" >/dev/null 2>&1 || true; fi
  if [[ -n "$server_pid" ]]; then kill "$server_pid" >/dev/null 2>&1 || true; fi
}

wait_url() {
  local url="$1"
  local label="$2"
  local deadline=$((SECONDS + 120))
  until curl -fsS "$url" >/dev/null 2>&1; do
    if (( SECONDS >= deadline )); then
      echo "Timed out waiting for $label at $url" >&2
      return 1
    fi
    sleep 1
  done
}

require_text() {
  local url="$1"
  local needle="$2"
  local body
  body="$(curl -fsS "$url")"
  if [[ "$body" != *"$needle"* ]]; then
    echo "Expected '$needle' in $url" >&2
    return 1
  fi
}

wait_text() {
  local url="$1"
  local needle="$2"
  local label="$3"
  local deadline=$((SECONDS + 120))
  until require_text "$url" "$needle" >/dev/null 2>&1; do
    if (( SECONDS >= deadline )); then
      echo "Timed out waiting for $label to contain '$needle' at $url" >&2
      require_text "$url" "$needle"
      return 1
    fi
    sleep 1
  done
}

json_field() {
  python3 -c 'import json,sys; print(json.load(sys.stdin)[sys.argv[1]])' "$1"
}

trap cleanup EXIT

rm -rf "$data_dir"

(
  cd "$root/web/dashboard"
  npm ci
)

"$root/scripts/check-openapi-drift.sh"
cargo build -q -p beaterd -p beaterctl

"$root/target/debug/beaterd" \
  --data-dir "$data_dir" \
  --addr "$http_addr" \
  --otlp-grpc-addr "$grpc_addr" \
  --trace-write-drain-interval-ms 25 \
  --trace-ingested-drain-interval-ms 25 &
server_pid="$!"
wait_url "$api_url/health" "beaterd"

http_smoke="$("$root/target/debug/beaterctl" smoke \
  --http-url "$api_url" \
  --tenant-id demo \
  --project-id demo \
  --environment-id local \
  --timeout-ms 10000)"

grpc_smoke="$("$root/target/debug/beaterctl" smoke \
  --http-url "$api_url" \
  --otlp-grpc-url "$grpc_url" \
  --tenant-id demo \
  --project-id demo \
  --environment-id local \
  --timeout-ms 10000)"

trace_id="$(printf '%s' "$grpc_smoke" | json_field trace_id)"

python3 -m venv "$venv_dir"
"$venv_dir/bin/pip" install --quiet --upgrade pip
"$venv_dir/bin/pip" install --quiet opentelemetry-sdk opentelemetry-exporter-otlp-proto-grpc
OTEL_EXPORTER_OTLP_ENDPOINT="$grpc_url" \
BEATER_TENANT_ID=demo \
BEATER_PROJECT_ID=demo \
BEATER_ENVIRONMENT_ID=local \
  "$venv_dir/bin/python" "$root/examples/python/otel_smoke.py"

(
  cd "$root/web/dashboard"
  npm run build
)

(
  cd "$root/web/dashboard"
  BEATER_API_BASE_URL="$api_url" \
    NEXT_PUBLIC_BEATER_API_BASE_URL="$api_url" \
    HOSTNAME="$dashboard_host" \
    PORT="$dashboard_port" \
    node .next/standalone/server.js
) &
dashboard_pid="$!"

wait_url "$dashboard_url/?tenant=demo&project=demo&environment=local" "dashboard"

wait_text "$api_url/v1/traces/demo?project_id=demo&environment_id=local&kind=llm.call" "gpt-demo" "stock Python OTLP trace"
require_text "$api_url/v1/traces/demo/$trace_id" "beaterctl otlp smoke"
require_text "$api_url/openapi.json" "min_cost_micros"
wait_text "$dashboard_url/?tenant=demo&project=demo&environment=local&kind=llm.call" "Agent Trace Debugger" "dashboard"
wait_text "$dashboard_url/?tenant=demo&project=demo&environment=local&kind=llm.call" "call-policy-model" "dashboard llm.call row"

if [[ "${BEATER_GATE2_SKIP_BROWSER:-0}" != "1" ]]; then
  (
    cd "$root/web/dashboard"
    npx playwright install chromium
    PLAYWRIGHT_BASE_URL="$dashboard_url" npm run test:e2e
  )
fi

cat <<EOF
Gate 2 local proof passed.

HTTP smoke:
$http_smoke

gRPC smoke:
$grpc_smoke

Open the dashboard:
  $dashboard_url/?tenant=demo&project=demo&environment=local

Specific smoke trace:
  $dashboard_url/?tenant=demo&project=demo&environment=local&trace=$trace_id
EOF
