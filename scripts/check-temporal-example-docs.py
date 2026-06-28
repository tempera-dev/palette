#!/usr/bin/env python3
"""Guard the Temporal example guide against onboarding and mapping drift."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parent.parent
README = ROOT / "examples" / "temporal" / "README.md"
ARCHITECTURE = ROOT / "ARCHITECTURE.md"
FIXTURE = ROOT / "crates" / "beater-temporal" / "tests" / "fixtures" / "order_workflow_history.json"

README_TOKENS = [
    "two language-agnostic\npaths",
    "same canonical span model",
    "You do **not** need a Beater SDK in any language.",
    "## Path A",
    "Live capture",
    "OpenTelemetry\ntracing interceptor",
    "endpoint=\"http://127.0.0.1:4317\"",
    "x-beater-tenant-id",
    "beater.framework=temporal",
    "## Path B",
    "History import",
    "temporal workflow show --workflow-id order-A-100 --output json > history.json",
    "jq '{source: \"temporal_history\", payload: .}'",
    "/v1/import/demo/demo/local",
    "source-agnostic\nimport endpoint",
    "../../crates/beater-temporal/tests/fixtures/order_workflow_history.json",
    "| `WorkflowExecutionStarted` | `agent.run`",
    "| `ActivityTaskScheduled/Started/Completed/Failed/TimedOut/Canceled` | `tool.call`",
    "| `StartChildWorkflowExecutionInitiated` + `ChildWorkflowExecution*` | nested `agent.run`",
    "| `TimerStarted/Fired/Canceled` | `agent.step`",
    "counted as unmapped, preserved in the raw envelope",
    "beater_temporal::TEMPORAL_HISTORY_CONTRACT",
    "KNOWN_EVENT_TYPES",
    "check_temporal_contract",
]

ARCHITECTURE_TOKENS = [
    "Use standards at the edge",
    "Store immutable raw data and normalized projections",
    "Temporal workflow-history →\n  canonical span normalization",
    "Framework-integration guides",
    "Temporal (sub-agent trace steps map\n  cleanly to canonical spans)",
    "zero-code OTLP bootstrap",
]


def normalize(text: str) -> str:
    return " ".join(text.split())


def main() -> int:
    readme = README.read_text(encoding="utf-8")
    architecture = ARCHITECTURE.read_text(encoding="utf-8")
    normalized_readme = normalize(readme)
    normalized_architecture = normalize(architecture)
    failures: list[str] = []

    if not FIXTURE.is_file():
        failures.append(f"missing Temporal history fixture: {FIXTURE.relative_to(ROOT)}")

    for token in README_TOKENS:
        if normalize(token) not in normalized_readme:
            failures.append(f"Temporal guide missing {token!r}")

    for token in ARCHITECTURE_TOKENS:
        if normalize(token) not in normalized_architecture:
            failures.append(f"architecture backing missing {token!r}")

    if failures:
        print("Temporal example docs drift:", file=sys.stderr)
        for failure in failures:
            print(f"  - {failure}", file=sys.stderr)
        return 1

    print("Temporal example docs cover live OTLP, history import, mapping, and drift guards.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
