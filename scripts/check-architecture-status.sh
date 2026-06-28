#!/usr/bin/env bash
# Scan ARCHITECTURE.md for risky completion phrases and verify each flagged claim
# has a matching row in docs/architecture-status.md (the status ledger from PR #33).
#
# If the ledger does not yet exist, skip gracefully and exit 0 so this script is
# independently mergeable.  Once the ledger lands, re-run to enforce it.
#
# Usage:
#   scripts/check-architecture-status.sh               # normal run against repo
#   scripts/check-architecture-status.sh --self-test   # fixture smoke-test
#
# Exit non-zero on drift (un-ledgered claims).  See docs/architecture-status.md.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

step() { echo; echo "==> $1"; }

# ---------------------------------------------------------------------------
# Risky completion phrases that must be tracked in the status ledger.
# All matches are case-insensitive.
# ---------------------------------------------------------------------------
RISKY_RE='\[built\]|implemented|wired|\bgreen\b|\bdone\b|\bshipped\b'

# ---------------------------------------------------------------------------
# extract_component <line> <current_section>
#
# Returns the best "component name" token for a flagged line:
#   1. First column of a markdown table row  (strips ** backticks parens)
#   2. First **bold span** on the line
#   3. First `backtick span` on the line
#   4. The running section heading (## / ### …)
# ---------------------------------------------------------------------------
extract_component() {
  local line="$1"
  local section="$2"
  local comp=""

  # Table row — pull first column between the leading | and second |
  if [[ "$line" == \|* ]]; then
    comp=$(printf '%s' "$line" | awk -F'|' '{print $2}' \
      | sed 's/\*\*//g; s/`//g; s/(.*//; s/[[:space:]]*$//' \
      | xargs 2>/dev/null || true)
  fi

  # Bold span **...**
  if [[ -z "$comp" ]]; then
    comp=$(printf '%s' "$line" | grep -oE '\*\*[^*]+\*\*' | head -1 \
      | sed 's/\*\*//g' | xargs 2>/dev/null || true)
  fi

  # Backtick span `...`
  if [[ -z "$comp" ]]; then
    comp=$(printf '%s' "$line" | grep -oE '`[^`]+`' | head -1 \
      | tr -d '`' | xargs 2>/dev/null || true)
  fi

  # NOTE: we do NOT fall back to the section heading.  Pure prose lines that
  # lack an explicit component marker (table col / bold / backtick) are skipped
  # so as not to flood output with false positives from commentary sentences.
  # The section parameter is accepted for future use but intentionally unused.
  : "$section"  # suppress unused-variable warning

  printf '%s' "$comp"
}

# ---------------------------------------------------------------------------
# key_word <component>
#
# Returns the first "significant" word (≥4 chars, alphanumeric) from a
# component string, lowercased.  Used for fuzzy ledger lookup.
# ---------------------------------------------------------------------------
key_word() {
  printf '%s' "$1" \
    | tr -s '[:space:]()[]{}' '\n' \
    | grep -E '^[[:alnum:]_-]{4,}$' \
    | head -1 \
    | tr '[:upper:]' '[:lower:]'
}

# ---------------------------------------------------------------------------
# check_claims <arch_file> <ledger_file_or_empty>
#
# Returns number of un-ledgered claims (0 = clean).
# ---------------------------------------------------------------------------
check_claims() {
  local arch="$1"
  local ledger="$2"

  if [[ ! -f "$arch" ]]; then
    echo "ERROR: architecture file not found: $arch" >&2
    return 1
  fi

  if [[ ! -f "$ledger" ]]; then
    echo "[SKIP] status ledger not present (pairs with PR #33) — skipping claim check"
    return 0
  fi

  local section=""
  local lineno=0
  local violations=0

  while IFS= read -r line; do
    lineno=$((lineno + 1))

    # Track current section heading
    if [[ "$line" =~ ^##+[[:space:]] ]]; then
      section="$line"
    fi

    # Test for any risky phrase (case-insensitive)
    if ! printf '%s' "$line" | grep -qiE "$RISKY_RE"; then
      continue
    fi

    # Extract component identifier
    local comp
    comp=$(extract_component "$line" "$section")

    if [[ -z "$comp" ]]; then
      continue  # can't identify a component token; skip silently
    fi

    # Derive a fuzzy lookup key
    local kw
    kw=$(key_word "$comp")

    if [[ -z "$kw" ]]; then
      continue  # component name has no 4-char word; skip
    fi

    # Check ledger (case-insensitive substring)
    if ! grep -qiF "$kw" "$ledger" 2>/dev/null; then
      printf 'DRIFT  line %4d  component="%s"  key="%s"\n' \
        "$lineno" "$comp" "$kw" >&2
      printf '       > %s\n' "$line" >&2
      violations=$((violations + 1))
    fi

  done < "$arch"

  return "$violations"
}

# ===========================================================================
# --self-test mode
# ===========================================================================
if [[ "${1:-}" == "--self-test" ]]; then
  step "self-test: running linter against fixtures"

  FIXTURES="$root/scripts/fixtures/arch-claims"
  PASS_MD="$FIXTURES/pass.md"
  FAIL_MD="$FIXTURES/fail.md"

  if [[ ! -f "$PASS_MD" ]] || [[ ! -f "$FAIL_MD" ]]; then
    echo "ERROR: fixtures not found under scripts/fixtures/arch-claims/" >&2
    exit 1
  fi

  # Build a temporary ledger that covers pass.md's components but NOT fail.md's
  # "hyperdrive-module" claim.
  tmpdir="$(mktemp -d)"
  trap 'rm -rf "$tmpdir"' EXIT

  FAKE_LEDGER="$tmpdir/ledger.md"
  cat >"$FAKE_LEDGER" <<'LEDGER'
# Architecture Status Ledger (self-test fixture)

| Component         | Status    | Notes                   |
|-------------------|-----------|-------------------------|
| beaterd server    | [built]   | passes gate2 proof      |
| Dashboard         | [built]   | Vercel deployed         |
| CLI               | [built]   | beaterctl on main       |
LEDGER

  # --- pass.md must exit 0 ---
  echo
  echo "--- pass.md (expect exit 0) ---"
  if check_claims "$PASS_MD" "$FAKE_LEDGER"; then
    echo "  PASS: pass.md returned 0"
  else
    echo "FAIL: pass.md unexpectedly returned non-zero" >&2
    exit 1
  fi

  # --- fail.md must exit non-zero ---
  echo
  echo "--- fail.md (expect non-zero) ---"
  if check_claims "$FAIL_MD" "$FAKE_LEDGER"; then
    echo "FAIL: fail.md should have returned non-zero but returned 0" >&2
    exit 1
  else
    echo "  PASS: fail.md correctly returned non-zero"
  fi

  echo
  echo "self-test PASSED"
  exit 0
fi

# ===========================================================================
# Normal run
# ===========================================================================
ARCH_FILE="${1:-ARCHITECTURE.md}"
LEDGER_FILE="${2:-docs/architecture-status.md}"

step "1/1 check architecture claims → status ledger"
echo "  arch:   $ARCH_FILE"
echo "  ledger: $LEDGER_FILE"
echo

violations=0
if ! check_claims "$ARCH_FILE" "$LEDGER_FILE"; then
  violations=$?
fi

echo
if [[ "$violations" -ne 0 ]]; then
  printf 'DRIFT: %d un-ledgered claim(s) in %s\n' "$violations" "$ARCH_FILE" >&2
  echo "Add a row for each flagged component to $LEDGER_FILE." >&2
  exit 1
fi

if [[ -f "$LEDGER_FILE" ]]; then
  echo "OK: all flagged claims have a matching row in $LEDGER_FILE"
fi
