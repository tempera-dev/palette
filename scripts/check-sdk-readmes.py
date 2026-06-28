#!/usr/bin/env python3
"""Guard SDK README contract language against drift."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parent.parent
SDK_README = ROOT / "sdks" / "README.md"
RUST_README = ROOT / "sdks" / "rust" / "README.md"
CARGO = ROOT / "Cargo.toml"
ARCHITECTURE = ROOT / "ARCHITECTURE.md"

SDK_TOKENS = [
    "Every SDK, the MCP server, the CLI, and the docs derive from **one** artifact",
    "sdks/openapi/beater-api.json",
    "Rust API handlers",
    "7 generated control-plane clients",
    "/mcp tools",
    "beater api <op>",
    "web/dashboard /docs",
    "Layer 1 — generated control-plane clients",
    "Layer 2 — hand-written ergonomic SDKs",
    "rust, python, typescript, go, java,\nc, cpp",
    "scripts/regen-sdks.sh --check",
    "oasdiff",
]

RUST_TOKENS = [
    "Layer 2",
    "hand-written, ergonomic",
    "OpenTelemetry API",
    "init`, `observe`, `span`",
    "set_input`/`set_output",
    "shared `semconv` module kept in lockstep",
    "BeaterConfig::from_env()",
    "BEATER_BASE_URL",
    "BEATER_TENANT_ID",
    "BEATER_PROJECT_ID",
    "BEATER_ENVIRONMENT_ID",
    "{base}/v1/otlp/{tenant}/{project}/{environment}/v1/traces",
    "cargo run --example quickstart",
]

ARCHITECTURE_TOKENS = [
    "generated SDK clients plus a native Rust SDK, an MCP server, and a CLI",
    "all derive from\nthe same contract",
    "Native Rust SDK",
    "excluded from the cargo workspace",
    "scripts/check-contract-sync.sh",
    "zero drift across spec/clients/semconv/MCP/CLI/docs",
    "Cross-SDK behavioral parity conformance",
]


def normalize(text: str) -> str:
    return " ".join(text.split())


def require_tokens(label: str, text: str, tokens: list[str], failures: list[str]) -> None:
    normalized = normalize(text)
    for token in tokens:
        if normalize(token) not in normalized:
            failures.append(f"{label} missing {token!r}")


def main() -> int:
    failures: list[str] = []
    require_tokens("sdks/README.md", SDK_README.read_text(encoding="utf-8"), SDK_TOKENS, failures)
    require_tokens("sdks/rust/README.md", RUST_README.read_text(encoding="utf-8"), RUST_TOKENS, failures)
    require_tokens("ARCHITECTURE.md", ARCHITECTURE.read_text(encoding="utf-8"), ARCHITECTURE_TOKENS, failures)

    cargo = CARGO.read_text(encoding="utf-8")
    if '"sdks"' not in cargo or "exclude" not in cargo:
        failures.append("Cargo.toml must keep sdks excluded from the workspace")

    if failures:
        print("SDK README drift:", file=sys.stderr)
        for failure in failures:
            print(f"  - {failure}", file=sys.stderr)
        return 1

    print("SDK READMEs cover generated clients, Rust SDK, regen, and workspace boundaries.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
