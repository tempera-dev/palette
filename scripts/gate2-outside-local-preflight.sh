#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "Gate 2 outside-run local preflight failed: $*" >&2
  exit 1
}

require_command() {
  local name="$1"
  local reason="$2"
  if ! command -v "$name" >/dev/null 2>&1; then
    fail "missing required command '$name' ($reason)"
  fi
}

require_python3() {
  require_command python3 "proof generation and validation require python3 3.9+"
  if ! python3 - <<'PY' >/dev/null 2>&1
import sys
raise SystemExit(0 if sys.version_info >= (3, 9) else 1)
PY
  then
    local version
    version="$(python3 -c 'import sys; print(".".join(str(part) for part in sys.version_info[:3]))' 2>/dev/null || true)"
    fail "python3 must be version 3.9 or newer; got '${version:-unknown}'"
  fi
}

require_unset_or_value() {
  local name="$1"
  local expected="$2"
  local reason="$3"
  local value="${!name:-}"
  if [[ -n "$value" && "$value" != "$expected" ]]; then
    fail "$name must be unset or '$expected' for outside-person evidence ($reason); got '$value'"
  fi
}

require_unset() {
  local name="$1"
  local reason="$2"
  local value="${!name:-}"
  if [[ -n "$value" ]]; then
    fail "$name must be unset for outside-person evidence ($reason); got '$value'"
  fi
}

docker_endpoint_is_local() {
  local endpoint="$1"
  [[
    -z "$endpoint" ||
    "$endpoint" == "<no value>" ||
    "$endpoint" == unix://* ||
    "$endpoint" == npipe://* ||
    "$endpoint" == tcp://localhost:* ||
    "$endpoint" == tcp://127.* ||
    "$endpoint" == tcp://[::1]:*
  ]]
}

port_is_free() {
  if (: >/dev/tcp/127.0.0.1/"$1") >/dev/null 2>&1; then
    return 1
  fi
  return 0
}

print_port_owner() {
  local port="$1"
  if command -v lsof >/dev/null 2>&1; then
    lsof -nP -iTCP:"$port" -sTCP:LISTEN >&2 || true
    print_port_owner_details "$port"
  elif command -v ss >/dev/null 2>&1; then
    ss -ltnp "sport = :$port" >&2 || true
  fi
}

print_port_owner_details() {
  local port="$1"
  local pid
  local owner_command
  local owner_cwd
  if ! command -v lsof >/dev/null 2>&1; then
    return 0
  fi
  while IFS= read -r pid; do
    if [[ ! "$pid" =~ ^[0-9]+$ ]]; then
      continue
    fi
    owner_command=""
    if command -v ps >/dev/null 2>&1; then
      owner_command="$(ps -p "$pid" -o command= 2>/dev/null | sed -n '1p' || true)"
    fi
    if [[ -n "$owner_command" ]]; then
      echo "process $pid command: $owner_command" >&2
    fi
    owner_cwd="$(lsof -a -p "$pid" -d cwd -Fn 2>/dev/null | sed -n 's/^n//p' | head -n 1 || true)"
    if [[ -n "$owner_cwd" ]]; then
      echo "process $pid cwd: $owner_cwd" >&2
    fi
  done < <(lsof -nP -t -iTCP:"$port" -sTCP:LISTEN 2>/dev/null | awk '!seen[$0]++' || true)
}

print_stale_beater_cleanup_hint() {
  cat >&2 <<'EOF'
If this is a stale Beater Gate 2 run, clean the old Compose project before
rerunning the timed command:
  if [ -d beater ]; then (cd beater && docker compose -f docker-compose.prebuilt.yml -p beater-stopwatch down -v --remove-orphans); fi
  docker ps -aq --filter label=com.docker.compose.project=beater-stopwatch | while read -r id; do [ -z "$id" ] || docker rm -f "$id"; done
  docker volume ls -q --filter label=com.docker.compose.project=beater-stopwatch | while read -r id; do [ -z "$id" ] || docker volume rm "$id"; done
  docker network ls -q --filter label=com.docker.compose.project=beater-stopwatch | while read -r id; do [ -z "$id" ] || docker network rm "$id"; done
EOF
}

require_public_images_for_expected_commit() {
  local expected_commit="${BEATER_GATE2_EXPECTED_COMMIT:-}"
  if [[ -z "$expected_commit" ]]; then
    return 0
  fi
  python3 - "$expected_commit" <<'PY'
import json
import sys
import urllib.error
import urllib.parse
import urllib.request

expected_commit = sys.argv[1]
owner_repo = "jadenfix/beater"
images = ("beaterd", "dashboard", "dashboard-e2e", "otel-python")
expected_platforms = {("linux", "amd64"), ("linux", "arm64")}
accept = ", ".join(
    (
        "application/vnd.oci.image.index.v1+json",
        "application/vnd.docker.distribution.manifest.list.v2+json",
        "application/vnd.oci.image.manifest.v1+json",
        "application/vnd.docker.distribution.manifest.v2+json",
    )
)


def read_json(url, *, token=None, accept_header=None):
    headers = {}
    if token:
        headers["Authorization"] = f"Bearer {token}"
    if accept_header:
        headers["Accept"] = accept_header
    request = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(request, timeout=20) as response:
        return json.load(response)


def token_for(repository):
    scope = urllib.parse.quote(f"repository:{repository}:pull")
    data = read_json(f"https://ghcr.io/token?service=ghcr.io&scope={scope}")
    token = data.get("token")
    if not token:
        raise SystemExit(f"GHCR did not return an anonymous pull token for {repository}")
    return token


for image in images:
    repository = f"{owner_repo}/{image}"
    token = token_for(repository)
    manifest_url = f"https://ghcr.io/v2/{repository}/manifests/{expected_commit}"
    try:
        manifest = read_json(manifest_url, token=token, accept_header=accept)
    except urllib.error.HTTPError as error:
        raise SystemExit(
            f"missing public GHCR manifest for ghcr.io/{repository}:{expected_commit}: "
            f"HTTP {error.code}"
        ) from None
    platforms = {
        (
            (entry.get("platform") or {}).get("os", ""),
            (entry.get("platform") or {}).get("architecture", ""),
        )
        for entry in manifest.get("manifests", [])
    }
    missing = expected_platforms - platforms
    if missing:
        formatted = ", ".join(f"{os_name}/{arch}" for os_name, arch in sorted(missing))
        raise SystemExit(
            f"public GHCR manifest for ghcr.io/{repository}:{expected_commit} "
            f"is missing platform(s): {formatted}"
        )

print(f"Gate 2 public images are available for {expected_commit}.")
PY
}

require_command git "the timed run clones the public Beater repo"
require_command docker "the timed run starts the public Docker Compose topology"
require_command curl "the timed run checks the local Beater API"
require_command ffprobe "completed proof validation verifies the WebM recording"
require_python3

require_unset_or_value BEATER_GATE2_REUSE 0 "warm-loop reuse is not valid evidence"
require_unset_or_value BEATER_GATE2_LOCAL_BUILD 0 "the outside run must use prebuilt SHA-pinned images"
require_unset_or_value BEATER_GATE2_PULL_POLICY always "the outside run must pull current images"
require_unset_or_value BEATER_HTTP_PORT 8080 "the default API port is required"
require_unset_or_value BEATER_OTLP_GRPC_PORT 4317 "the default OTLP gRPC port is required"
require_unset_or_value BEATER_DASHBOARD_PORT 3000 "the default dashboard port is required"
require_unset_or_value BEATER_GATE2_WRITE_PROOF 1 "the outside run must write a stopwatch proof"
require_unset_or_value BEATER_GATE2_BROWSER_PROOF 1 "the outside run must prove the browser flow"
require_unset_or_value BEATER_GATE2_RECORD_DEMO 1 "the outside run must record the browser flow"
require_unset_or_value BEATER_GATE2_POST_SLO_TIMEOUT_SECONDS 300 "the outside run must use the documented post-SLO timeout"
require_unset_or_value KEEP_BEATER_COMPOSE 1 "the dashboard must remain running for outside-person click-through"
require_unset BEATER_GATE2_OUTSIDE_RUN_DRY_RUN "dry-run mode is not outside-person evidence"
require_unset BEATERD_IMAGE "the wrapper pins beaterd to the checked-out commit SHA"
require_unset BEATER_DASHBOARD_IMAGE "the wrapper pins dashboard to the checked-out commit SHA"
require_unset BEATER_DASHBOARD_E2E_IMAGE "the wrapper pins dashboard-e2e to the checked-out commit SHA"
require_unset BEATER_OTEL_PYTHON_IMAGE "the wrapper pins otel-python to the checked-out commit SHA"
require_unset BEATER_GATE2_RUN_ID "the stopwatch creates a fresh per-run quickstart release ID"
require_unset BEATER_GATE2_CONFIRMATION_SALT "the stopwatch creates a fresh per-run browser confirmation salt"
require_unset BEATER_GATE2_REGISTRY_FIXTURE_UNSAFE_FOR_TESTS "outside evidence must validate against public GHCR"
require_unset BEATER_GATE2_STOPWATCH_PROOF "the outside run must write docs/demos/gate2-compose-stopwatch.md"
require_unset BEATER_GATE2_RECORD_VIDEO "the outside run must write docs/demos/gate2-compose-browser-demo.webm"
require_unset BEATER_GATE2_RECORD_NOTES "the outside run must write docs/demos/gate2-compose-browser-demo.md"
require_unset BEATER_GATE2_COMPOSE_LOGS "the outside run must write docs/demos/gate2-outside-compose.log"
require_unset COMPOSE_FILE "the public command controls the Compose topology"
require_unset COMPOSE_PROJECT_NAME "the public command controls the Compose topology"
require_unset COMPOSE_PROFILES "the public command controls the Compose topology"

if [[ -e beater ]]; then
  fail "current directory already contains ./beater; run from a new or empty parent directory before cloning"
fi

if ! command -v shasum >/dev/null 2>&1 && ! command -v sha256sum >/dev/null 2>&1; then
  fail "missing required command 'shasum' or 'sha256sum' (confirmation code and recording hash proof)"
fi

if ! docker_endpoint_is_local "${DOCKER_HOST:-}"; then
  fail "DOCKER_HOST must point at a local Docker daemon because browser proof uses 127.0.0.1"
fi

docker info >/dev/null 2>&1 || fail "Docker daemon is not reachable; start Docker and rerun"
docker compose version >/dev/null 2>&1 || fail "Docker Compose v2 is required"

docker_context_host="$(docker context inspect --format '{{.Endpoints.docker.Host}}' 2>/dev/null | head -n 1 || true)"
if ! docker_endpoint_is_local "$docker_context_host"; then
  fail "Docker context must be local because browser proof uses 127.0.0.1; current endpoint is $docker_context_host"
fi

require_public_images_for_expected_commit

for port in 8080 4317 3000; do
  if ! port_is_free "$port"; then
    echo "TCP $port is already in use before the timed Gate 2 run." >&2
    print_port_owner "$port"
    print_stale_beater_cleanup_hint
    fail "free TCP $port before starting the stopwatch; do not use alternate ports for outside-person evidence"
  fi
done

echo "Gate 2 outside-run local preflight passed."
