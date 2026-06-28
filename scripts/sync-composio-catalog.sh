#!/usr/bin/env bash
# Snapshot the Composio toolkit catalog into a committed, deterministic file so
# new/changed third-party apps surface as a reviewable git diff instead of being
# missed. Composio's own agents continuously add and update endpoints; running
# this on a schedule (or before a release) lets Beater auto-ship those updates:
# a non-empty `git diff` on the snapshot == new capabilities to expose.
#
# Usage:
#   COMPOSIO_API_KEY=ak_... scripts/sync-composio-catalog.sh          # refresh
#   COMPOSIO_API_KEY=ak_... scripts/sync-composio-catalog.sh --check  # CI: fail on drift
#
# The snapshot holds only public catalog metadata (no secrets), so it is safe to
# commit. The API key is read from the environment and never written anywhere.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
out="$root/crates/beater-composio/catalog/toolkits.json"
base="${COMPOSIO_BASE_URL:-https://backend.composio.dev/api/v3}"
limit="${COMPOSIO_CATALOG_LIMIT:-500}"

if [ -z "${COMPOSIO_API_KEY:-}" ]; then
  echo "ERROR: COMPOSIO_API_KEY is not set" >&2
  exit 2
fi

mkdir -p "$(dirname "$out")"
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

# Fetch the catalog and reduce it to the stable fields we depend on, sorted by
# slug for a deterministic, diff-friendly snapshot.
curl -fsS -m 60 -H "x-api-key: $COMPOSIO_API_KEY" "$base/toolkits?limit=$limit" \
  | python3 -c '
import json, sys
doc = json.load(sys.stdin)
items = doc.get("items", [])
rows = []
for it in items:
    meta = it.get("meta", {}) or {}
    rows.append({
        "slug": it.get("slug"),
        "name": it.get("name"),
        "no_auth": bool(it.get("no_auth", False)),
        "auth_schemes": sorted(it.get("auth_schemes", []) or []),
        "tools_count": meta.get("tools_count"),
    })
rows.sort(key=lambda r: (r["slug"] or ""))
snapshot = {"source": "composio", "count": len(rows), "toolkits": rows}
print(json.dumps(snapshot, indent=2, sort_keys=True))
' > "$tmp"

count="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["count"])' "$tmp")"

if [ "${1:-}" = "--check" ]; then
  if ! diff -q "$out" "$tmp" >/dev/null 2>&1; then
    echo "Composio catalog drift detected ($count toolkits upstream)." >&2
    echo "Run: COMPOSIO_API_KEY=... scripts/sync-composio-catalog.sh   then review + commit." >&2
    diff "$out" "$tmp" || true
    exit 1
  fi
  echo "Composio catalog snapshot is current ($count toolkits)."
else
  mv "$tmp" "$out"
  trap - EXIT
  echo "wrote $out ($count toolkits)"
fi
