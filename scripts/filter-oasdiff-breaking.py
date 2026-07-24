#!/usr/bin/env python3
"""Filter intentional pre-1.0 OpenAPI breaking changes from oasdiff output."""

from __future__ import annotations

import re
import sys
from pathlib import Path


LEGACY_SCOPE_VALUES = {
    "dataset_write",
    "eval_run",
    "pii_unmask",
    "scenario_read",
    "scenario_write",
    "trace_read",
    "trace_write",
}

AIP158_ARRAY_WRAPPER_OPERATIONS = {
    "GET /v1/audit/{tenant_id}/{project_id}",
    "GET /v1/connectors/{tenant_id}/{project_id}",
    "GET /v1/connectors/{tenant_id}/{project_id}/tools",
    "GET /v1/judge/{tenant_id}/{project_id}/ledger",
    "GET /v1/provider-secrets/{tenant_id}/{project_id}",
    "GET /v1/review-queues/{tenant_id}/{project_id}/{queue_id}/tasks",
}

AIP193_ERROR_TYPE_CHANGE = re.compile(
    r"the `error` response's property `type` changed from `string` to `object` "
    r"for status `[45]\d\d`"
)
AIP193_REMOVED_TOP_LEVEL_FIELD = re.compile(
    r"removed the required property `(message|status)` from the response "
    r"with the `[45]\d\d` status"
)


def error_blocks(text: str) -> list[str]:
    blocks: list[str] = []
    current: list[str] = []
    for line in text.splitlines():
        if line.startswith("error\t["):
            if current:
                blocks.append("\n".join(current))
            current = [line]
        elif current:
            if not line.strip():
                blocks.append("\n".join(current))
                current = []
            else:
                current.append(line)
    if current:
        blocks.append("\n".join(current))
    return blocks


def api_operation(block: str) -> str | None:
    match = re.search(r"^\s*in API ([A-Z]+ \S+)\s*$", block, re.MULTILINE)
    return match.group(1) if match is not None else None


def has_exact_detail(block: str, pattern: re.Pattern[str]) -> bool:
    return any(pattern.fullmatch(line.strip()) for line in block.splitlines())


def is_allowed_alignment_break(block: str) -> bool:
    operation = api_operation(block)

    if "[response-body-type-changed]" in block:
        return (
            operation in AIP158_ARRAY_WRAPPER_OPERATIONS
            and "the response's body `type` changed from `array<object>` to `object` "
            "for status `200`" in block
        )
    if "[response-property-type-changed]" in block:
        return has_exact_detail(block, AIP193_ERROR_TYPE_CHANGE)
    if "[response-required-property-removed]" in block:
        if has_exact_detail(block, AIP193_REMOVED_TOP_LEVEL_FIELD):
            return True
        return (
            operation == "GET /v1/traces/{tenant_id}"
            and "removed the required property `items` from the response with the `200` status"
            in block
        )
    if "[request-property-enum-value-removed]" in block:
        match = re.search(r"removed the enum value `([^`]+)` of the request property `scopes/items/`", block)
        return match is not None and match.group(1) in LEGACY_SCOPE_VALUES
    return False


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: filter-oasdiff-breaking.py OASDIFF_LOG", file=sys.stderr)
        return 2
    text = Path(sys.argv[1]).read_text(encoding="utf-8")
    blocks = error_blocks(text)
    unexpected = [block for block in blocks if not is_allowed_alignment_break(block)]
    if unexpected:
        print("Unexpected OpenAPI breaking changes:", file=sys.stderr)
        for block in unexpected:
            print(block, file=sys.stderr)
            print(file=sys.stderr)
        return 1
    if blocks:
        print(f"Allowed {len(blocks)} intentional pre-1.0 contract alignment breaks.")
    else:
        print("No OpenAPI breaking changes detected.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
