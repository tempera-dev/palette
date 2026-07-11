#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cargo run -q -p palette-api --example dump_openapi >"$tmpdir/palette-read-api.json"

(
  cd "$root/web/dashboard"
  npx openapi-typescript "$tmpdir/palette-read-api.json" -o "$tmpdir/api-types.ts" >/dev/null
)

diff -u "$root/web/dashboard/openapi/palette-read-api.json" "$tmpdir/palette-read-api.json"
diff -u "$root/web/dashboard/lib/generated/api-types.ts" "$tmpdir/api-types.ts"

echo "OpenAPI snapshot and generated dashboard client are current."
