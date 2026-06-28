#!/usr/bin/env bash
# report-ci-checks.sh — list every GitHub Actions workflow and its jobs so the
# CI check ledger (docs/ci-checks.md) can be kept honest.
#
# Usage:
#   scripts/report-ci-checks.sh           # print workflow + job names; exit 0
#   scripts/report-ci-checks.sh --check   # also warn about workflows not in ledger
#
# Requires: bash, grep, sed (POSIX).  No external tools needed.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
workflows_dir="$root/.github/workflows"
ledger="$root/docs/ci-checks.md"

check_mode=0
for arg in "$@"; do
  case "$arg" in
    --check) check_mode=1 ;;
    *) echo "unknown option: $arg" >&2; exit 1 ;;
  esac
done

echo "==> CI workflows and jobs in $workflows_dir"
echo ""

found_any=0
for wf in "$workflows_dir"/*.yml "$workflows_dir"/*.yaml; do
  [ -f "$wf" ] || continue
  found_any=1
  basename_wf="$(basename "$wf")"
  wf_name="$(grep -m1 '^name:' "$wf" | sed 's/^name:[[:space:]]*//' | tr -d "'\"")"
  echo "Workflow: $basename_wf  (name: ${wf_name:-<unnamed>})"
  # Extract job ids: lines like "  job-id:" at the two-space-indent level under
  # a "jobs:" block.  This is a best-effort heuristic; nested keys are ignored.
  in_jobs=0
  while IFS= read -r line; do
    if [[ "$line" =~ ^jobs:[[:space:]]*$ ]]; then
      in_jobs=1
      continue
    fi
    if [ "$in_jobs" -eq 1 ]; then
      # A new top-level key (no leading spaces) ends the jobs block.
      if [[ "$line" =~ ^[a-zA-Z] ]]; then
        in_jobs=0
        continue
      fi
      # Two-space-indented key that doesn't start with more spaces is a job id.
      if [[ "$line" =~ ^[[:space:]]{2}([a-zA-Z0-9_-]+):[[:space:]]*$ ]]; then
        job_id="${BASH_REMATCH[1]}"
        echo "  job: $job_id"
      fi
    fi
  done < "$wf"
  echo ""
done

if [ "$found_any" -eq 0 ]; then
  echo "(no workflow files found in $workflows_dir)" >&2
  exit 1
fi

if [ "$check_mode" -eq 0 ]; then
  exit 0
fi

echo "==> Checking ledger coverage ($ledger)"
echo ""

if [ ! -f "$ledger" ]; then
  echo "WARNING: ledger not found at $ledger" >&2
  exit 1
fi

warnings=0
for wf in "$workflows_dir"/*.yml "$workflows_dir"/*.yaml; do
  [ -f "$wf" ] || continue
  basename_wf="$(basename "$wf")"
  if ! grep -qF "$basename_wf" "$ledger"; then
    echo "WARNING: $basename_wf is not referenced in $ledger" >&2
    warnings=$((warnings + 1))
  fi
done

if [ "$warnings" -gt 0 ]; then
  echo ""
  echo "$warnings workflow file(s) missing from ledger." >&2
  # Advisory only; exit 0 so CI pipelines are not broken by this script.
fi

echo "Done."
exit 0
