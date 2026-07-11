#!/usr/bin/env bash
# Run paletted on either runtime, then verify /health. Apple `container` has no
# `compose`, so this is the portable single-service launcher (Docker users can
# still use docker-compose.yml).
# Usage: scripts/run-paletted.sh [tag]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
# shellcheck source=scripts/container-runtime.sh
source scripts/container-runtime.sh

TAG="${1:-paletted:local}"
NAME="paletted-local"
PORT=8080

crt_stop "$NAME"
echo "==> Running $TAG as $NAME on $(crt_cli)"
crt_run "$NAME" "$TAG" "$PORT" >/dev/null
addr="$(crt_address "$NAME" "$PORT")"

echo "==> Waiting for health at http://$addr/health"
for _ in $(seq 1 60); do
  if curl -fsS "http://$addr/health" >/dev/null 2>&1; then
    echo "==> paletted healthy at http://$addr (runtime: $(crt_cli))"
    exit 0
  fi
  sleep 1
done
echo "paletted did not become healthy; logs:" >&2
crt_logs "$NAME" >&2
exit 1
