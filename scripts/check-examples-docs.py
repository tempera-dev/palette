#!/usr/bin/env python3
"""Guard examples docs against drifting from the standards-first onboarding contract."""

from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parent.parent
README = ROOT / "examples" / "README.md"
ARCHITECTURE = ROOT / "ARCHITECTURE.md"

DOC_TOKENS = [
    "zero-SDK OTLP",
    "OpenTelemetry exporter",
    "beaterd:4317",
    "demo/demo/local",
    "/v1/otlp/<tenant>/<project>/<environment>/v1/traces",
    "beaterd does not\nexpose a separate 4318 collector port",
    "OpenInference (`openinference.span.kind`)",
    "OpenLLMetry / Traceloop (`gen_ai.*`)",
    "Official OTel GenAI conventions",
    "OTLP from common web frameworks, no Beater SDK required",
    "First-class Rust adoption via the `beater` SDK",
    "cargo build --examples",
]

ARCHITECTURE_TOKENS = [
    "Use standards at the edge",
    "OpenInference, OpenTelemetry GenAI conventions",
    "zero-code env-var OTLP bootstrap (DEFAULT)",
    "no code, no SDK",
    "Python and TypeScript examples through standards-based OTLP first",
    "native Rust SDK with `tracing`, `opentelemetry-rust`, `reqwest`, `axum`,",
]

EXPECTED_EXAMPLES = [
    "examples/python/instrumentations/openinference_app.py",
    "examples/python/instrumentations/openllmetry_app.py",
    "examples/python/instrumentations/otel_genai_app.py",
    "examples/python/five_line_otel.py",
    "examples/python/frameworks/fastapi_app.py",
    "examples/python/frameworks/flask_app.py",
    "examples/typescript/frameworks/express-otlp.mjs",
    "examples/typescript/frameworks/llamaindex-otlp.mjs",
    "examples/rust/tracing_app.rs",
    "examples/rust/axum_app.rs",
    "examples/rust/tonic_app.rs",
    "examples/rust/reqwest_app.rs",
    "examples/rust/mcp_app.rs",
    "sdks/rust/examples/quickstart.rs",
]


def normalize(text: str) -> str:
    return " ".join(text.split())


def documented_paths(readme: str) -> set[str]:
    return {f"examples/{path}" for path in re.findall(r"`([^`]+\.(?:py|mjs|rs))`", readme)}


def main() -> int:
    readme = README.read_text(encoding="utf-8")
    architecture = ARCHITECTURE.read_text(encoding="utf-8")
    normalized_readme = normalize(readme)
    normalized_architecture = normalize(architecture)
    paths = documented_paths(readme)
    failures: list[str] = []

    for token in DOC_TOKENS:
        if normalize(token) not in normalized_readme:
            failures.append(f"examples README missing {token!r}")

    for token in ARCHITECTURE_TOKENS:
        if normalize(token) not in normalized_architecture:
            failures.append(f"architecture backing missing {token!r}")

    for expected in EXPECTED_EXAMPLES:
        if not (ROOT / expected).is_file():
            failures.append(f"missing documented example file: {expected}")
        elif expected.startswith("examples/") and expected not in paths:
            failures.append(f"examples README no longer lists {expected.removeprefix('examples/')}")

    if failures:
        print("Examples docs contract drift:", file=sys.stderr)
        for failure in failures:
            print(f"  - {failure}", file=sys.stderr)
        return 1

    print("Examples docs cover the standards-first onboarding inventory.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
