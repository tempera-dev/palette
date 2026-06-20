#!/usr/bin/env bash
set -euo pipefail

proof_path="${BEATER_GATE2_OUTSIDE_PROOF:-docs/demos/gate2-outside-person-proof.md}"
allow_pending=0

if [[ "${1:-}" == "--allow-pending" ]]; then
  allow_pending=1
  shift
fi

if [[ $# -ne 0 ]]; then
  echo "usage: scripts/validate-gate2-outside-proof.sh [--allow-pending]" >&2
  exit 2
fi

python3 - "$proof_path" "$allow_pending" <<'PY'
import hashlib
import re
import sys
from pathlib import Path

proof_path = Path(sys.argv[1])
allow_pending = sys.argv[2] == "1"
repo = Path.cwd()
errors: list[str] = []

if not proof_path.exists():
    raise SystemExit(f"missing outside-person proof file: {proof_path}")

text = proof_path.read_text()


def fail(message: str) -> None:
    errors.append(message)


def require_snippet(snippet: str, description: str) -> None:
    if snippet not in text:
        fail(f"missing {description}: {snippet}")


def field_value(name: str) -> str:
    match = re.search(rf"^- {re.escape(name)}:\s*(.*)$", text, re.MULTILINE)
    if not match:
        fail(f"missing field: {name}")
        return ""
    return match.group(1).strip().strip("`")


status_match = re.search(r"^Status:\s*(.+)$", text, re.MULTILINE)
status = status_match.group(1).strip() if status_match else ""
if not status:
    fail("missing Status line")
elif status == "not yet completed." and allow_pending:
    pass
elif status != "completed.":
    fail("Status must be 'completed.' for Gate 2 closure")

for snippet, description in [
    (
        "BEATER_GATE2_WRITE_PROOF=1 BEATER_GATE2_BROWSER_PROOF=1 BEATER_GATE2_RECORD_DEMO=1 scripts/gate2-compose-stopwatch.sh",
        "canonical stopwatch command",
    ),
    ("http://127.0.0.1:3000", "default dashboard URL"),
    ("Time-to-first-trace was 300 seconds or less", "first-trace checklist item"),
    ("Time-to-quickstart-click was 300 seconds or less", "browser-click checklist item"),
    ("using only public repository instructions", "unaided-run requirement"),
]:
    require_snippet(snippet, description)

if allow_pending and status == "not yet completed.":
    if errors:
        print("Gate 2 outside-person proof template is invalid:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        raise SystemExit(1)
    print(f"Gate 2 outside-person proof is pending but structurally valid: {proof_path}")
    raise SystemExit(0)

unresolved_fields = []
for field in [
    "Name",
    "Organization or relationship to project",
    "Date",
    "Machine and OS",
    "Docker version",
    "Docker Compose version",
    "Browser",
    "Preflight status",
    "Commit SHA",
    "Branch",
    "OS/arch",
    "Beater image digest",
    "Dashboard image digest",
    "Started at",
    "Ended at",
    "Time-to-first-trace",
    "Time-to-quickstart-click",
    "Total proof duration",
    "Stopwatch proof file",
    "Screen recording SHA256",
    "Terminal output excerpt",
    "Quickstart trace ID",
    "All-kind nested trace ID",
]:
    value = field_value(field)
    if not value or value.endswith(":") or "none / describe" in value:
        unresolved_fields.append(field)
if unresolved_fields:
    fail("unresolved required fields: " + ", ".join(unresolved_fields))

if "- [ ]" in text:
    fail("all pass-checklist boxes must be checked")

for forbidden in [
    "http://127.0.0.1:13003",
    "http://127.0.0.1:13008",
    "http://127.0.0.1:13080",
    "BEATER_DASHBOARD_PORT=",
    "BEATER_HTTP_PORT=",
    "BEATER_OTLP_GRPC_PORT=",
    "BEATER_GATE2_REUSE=1",
]:
    if forbidden in text:
        fail(f"outside-person proof must not use alternate/warm-loop evidence: {forbidden}")

quickstart_url = field_value("Quickstart dashboard URL")
all_kind_url = field_value("All-kind dashboard URL")
for name, value in [
    ("Quickstart dashboard URL", quickstart_url),
    ("All-kind dashboard URL", all_kind_url),
]:
    if not value.startswith("http://127.0.0.1:3000/"):
        fail(f"{name} must use http://127.0.0.1:3000/")

recording = field_value("Screen recording")
recording_path = repo / recording if recording else Path("")
if recording and not recording_path.exists():
    fail(f"screen recording does not exist: {recording}")
notes = field_value("Screen recording notes")
notes_path = repo / notes if notes else Path("")
if notes and not notes_path.exists():
    fail(f"screen recording notes do not exist: {notes}")
sha = field_value("Screen recording SHA256")
if sha and not re.fullmatch(r"[0-9a-f]{64}", sha):
    fail("Screen recording SHA256 must be a lowercase 64-character sha256")
if recording and recording_path.exists() and re.fullmatch(r"[0-9a-f]{64}", sha):
    actual = hashlib.sha256(recording_path.read_bytes()).hexdigest()
    if actual != sha:
        fail(f"screen recording sha mismatch: expected {sha}, got {actual}")

duration_match = re.search(r"- Time-to-first-trace:\s*([0-9]+)s?", text)
if duration_match and int(duration_match.group(1)) > 300:
    fail("Time-to-first-trace exceeds 300 seconds")
click_match = re.search(r"- Time-to-quickstart-click:\s*([0-9]+)s?", text)
if click_match and int(click_match.group(1)) > 300:
    fail("Time-to-quickstart-click exceeds 300 seconds")

if errors:
    print("Gate 2 outside-person proof is not valid:", file=sys.stderr)
    for error in errors:
        print(f"- {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"Gate 2 outside-person proof is complete and valid: {proof_path}")
PY
