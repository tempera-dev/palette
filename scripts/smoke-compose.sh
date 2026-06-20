#!/usr/bin/env bash
set -euo pipefail

project="${COMPOSE_PROJECT_NAME:-beater-smoke}"
keep="${KEEP_BEATER_COMPOSE:-0}"
all_kinds=(
  agent.run
  agent.turn
  agent.plan
  agent.step
  retrieval.query
  memory.read
  guardrail.check
  llm.call
  tool.call
  mcp.request
  memory.write
  evaluator.run
  human.review
  replay.run
)

compose() {
  docker compose -p "$project" "$@"
}

cleanup() {
  if [[ "$keep" != "1" ]]; then
    compose down -v --remove-orphans >/dev/null 2>&1 || true
  fi
}

wait_url() {
  local url="$1"
  local label="$2"
  local deadline=$((SECONDS + 120))
  until curl -fsS "$url" >/tmp/beater-smoke-response 2>/tmp/beater-smoke-error; do
    if (( SECONDS >= deadline )); then
      echo "Timed out waiting for $label at $url" >&2
      cat /tmp/beater-smoke-error >&2 || true
      return 1
    fi
    sleep 2
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
    sleep 2
  done
}

first_trace_id() {
  python3 -c 'import json,sys; print(json.load(sys.stdin)["items"][0]["trace_id"])'
}

trap cleanup EXIT

compose up -d --build postgres nats minio beaterd dashboard
wait_url "http://127.0.0.1:8080/health" "beaterd"

compose run --rm beaterctl
compose run --rm otel-python-smoke

wait_url "http://127.0.0.1:3000/?tenant=demo&project=demo&environment=local" "dashboard"
python_trace_query="http://127.0.0.1:8080/v1/traces/demo?project_id=demo&environment_id=local&kind=llm.call&model=gpt-demo&release=compose-demo"
wait_text "$python_trace_query" "gpt-demo" "stock Python OTLP trace"
python_trace_id="$(curl -fsS "$python_trace_query" | first_trace_id)"
python_trace_dashboard="http://127.0.0.1:3000/?tenant=demo&project=demo&environment=local&trace=$python_trace_id"
require_text "$python_trace_dashboard" "Agent Trace Debugger"
wait_text "$python_trace_dashboard" "call-policy-model" "dashboard llm.call row"
for kind in "${all_kinds[@]}"; do
  wait_text "$python_trace_dashboard" "$kind" "dashboard all-kind waterfall"
done
require_text "http://127.0.0.1:8080/openapi.json" "started_after"

cat <<EOF
Beater compose smoke passed.

Open the dashboard:
  $python_trace_dashboard

Set KEEP_BEATER_COMPOSE=1 to leave containers running after this script exits.
EOF
