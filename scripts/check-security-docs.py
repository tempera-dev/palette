#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path


REPO = Path(__file__).resolve().parent.parent

REQUIRED_FILES = [
    "SECURITY.md",
    "GOVERNANCE.md",
    "docs/security-review-2026-06-24.md",
    "docs/security-regression-fixtures.md",
]

REQUIRED_ANCHORS = {
    "ARCHITECTURE.md": [
        "Governance / SOC2 controls",
        "`SECURITY.md`",
        "A20",
        "Tenant isolation holds",
    ],
    "SECURITY.md": [
        "Reporting a vulnerability",
        "Tenant isolation",
        "PII unmask",
        "BYOK / provider secrets",
        "Self-host hardening notes",
    ],
    "GOVERNANCE.md": [
        "no-rug-pull promise",
        "Apache-2.0",
        "Self-host stays first-class",
        "The contract stays open",
    ],
    "docs/security-review-2026-06-24.md": [
        "Security Review",
        "Critical / High",
        "H1",
        "H6",
        "H7",
    ],
    "docs/security-regression-fixtures.md": [
        "Security Regression Fixture Catalog",
        "AuthZ / RBAC scope escalation",
        "Tenant cross-talk / IDOR",
        "PII redaction",
        "Summary of gaps",
        "**GAP**",
    ],
}


def fail(message: str) -> None:
    print(f"security docs check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def read(path: str) -> str:
    full_path = REPO / path
    if not full_path.is_file():
        fail(f"missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def main() -> None:
    for path in REQUIRED_FILES:
        read(path)

    for path, anchors in REQUIRED_ANCHORS.items():
        text = read(path)
        missing = [anchor for anchor in anchors if anchor not in text]
        if missing:
            fail(f"{path} missing anchor(s): {', '.join(missing)}")

    print("security docs check passed")


if __name__ == "__main__":
    main()
