#!/usr/bin/env python3
"""Filter intentional pre-1.0 OpenAPI breaking changes from oasdiff output."""

from __future__ import annotations

import functools
import hashlib
import json
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

# This exception is deliberately pinned to the exact reviewed migration
# snapshot. Any later contract edit changes the digest and disables every
# AIP-127 allowance below.
AIP127_MIGRATION_SPEC_SHA256 = (
    "136074a04219ea2bb96a70674afdbad4eec142c971f7c100ae0ab9db212fc5b7"
)

# This is a one-time bridge from Palette's legacy error envelope to the
# canonical shared Status response.  It is intentionally a *pair* of reviewed
# snapshots: changing either input disables every allowance below.
CANONICAL_ERROR_MIGRATION_BASE_SHA256 = (
    "136074a04219ea2bb96a70674afdbad4eec142c971f7c100ae0ab9db212fc5b7"
)
CANONICAL_ERROR_MIGRATION_TARGET_SHA256 = (
    "2d71fbbf94e31d38386a08f6dcde201d9b8f046cf0d4a9624da9c7a2a4b2aa46"
)
CANONICAL_ERROR_STATUS_VALUES = {
    "ABORTED", "ALREADY_EXISTS", "CANCELLED", "DATA_LOSS", "DEADLINE_EXCEEDED",
    "FAILED_PRECONDITION", "INTERNAL", "INVALID_ARGUMENT", "NOT_FOUND", "OUT_OF_RANGE",
    "PERMISSION_DENIED", "RESOURCE_EXHAUSTED", "UNAUTHENTICATED", "UNAVAILABLE",
    "UNIMPLEMENTED", "UNKNOWN",
}
AIP127_EVALUATOR_KIND_OPERATIONS = {
    "POST /v1/datasets/{tenant_id}/{project_id}/{dataset_id}/versions/{version_id}/evals/deterministic",
    "POST /v1/datasets/{tenant_id}/{project_id}/{dataset_id}/versions/{version_id}/evals/judge",
    "POST /v1/experiments/{tenant_id}/{project_id}/{dataset_id}/versions/{version_id}/deterministic",
    "POST /v1/experiments/{tenant_id}/{project_id}/{dataset_id}/versions/{version_id}/judge",
    "POST /v1/judge/{tenant_id}/{project_id}/evaluate",
}
AIP127_NEW_REQUIRED_REQUEST_PROPERTY = re.compile(
    r"added the new required request property `([^`]+)`"
)
AIP127_REMOVED_REQUIRED_RESPONSE_PROPERTY = re.compile(
    r"removed the required property `([^`]+)` from the response with the `2\d\d` status"
)
AIP127_EVALUATOR_ONE_OF_REMOVAL = re.compile(
    r"removed `subschema #5, subschema #6, subschema #8, subschema #9, subschema #10` "
    r"from the `(kind|evaluator/kind)` request property `oneOf` list"
)
AIP193_ERROR_TYPE_CHANGE = re.compile(
    r"the `error` response's property `type` changed from `string` to `object` "
    r"for status `[45]\d\d`"
)
AIP193_REMOVED_TOP_LEVEL_FIELD = re.compile(
    r"removed the required property `(message|status)` from the response "
    r"with the `[45]\d\d` status"
)


@functools.lru_cache(maxsize=1)
def aip127_migration_active() -> bool:
    contract = Path(__file__).resolve().parents[1] / "contracts/openapi/palette.openapi.json"
    try:
        digest = hashlib.sha256(contract.read_bytes()).hexdigest()
    except OSError:
        return False
    return digest == AIP127_MIGRATION_SPEC_SHA256


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def has_ref(value: object, ref: str) -> bool:
    return ref in json.dumps(value, sort_keys=True, separators=(",", ":"))


def reviewed_canonical_error_pairs(base_spec: Path, new_spec: Path) -> set[tuple[str, str]] | None:
    """Return reviewed error response pairs only for the exact snapshot pair."""
    try:
        if (
            sha256(base_spec) != CANONICAL_ERROR_MIGRATION_BASE_SHA256
            or sha256(new_spec) != CANONICAL_ERROR_MIGRATION_TARGET_SHA256
        ):
            return None
        base = json.loads(base_spec.read_text(encoding="utf-8"))
        new = json.loads(new_spec.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None

    pairs: set[tuple[str, str]] = set()
    for path, base_item in base.get("paths", {}).items():
        new_item = new.get("paths", {}).get(path, {})
        for method, base_operation in base_item.items():
            if method.upper() not in {"GET", "PUT", "POST", "DELETE", "PATCH", "HEAD", "OPTIONS", "TRACE"}:
                continue
            new_operation = new_item.get(method, {})
            for status, base_response in base_operation.get("responses", {}).items():
                new_response = new_operation.get("responses", {}).get(status)
                if (
                    new_response is not None
                    and has_ref(base_response, "#/components/schemas/ErrorResponse")
                    and has_ref(new_response, "#/components/responses/Error")
                ):
                    pairs.add((f"{method.upper()} {path}", str(status)))
    return pairs or None


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


def is_aip127_alignment_break(block: str, operation: str | None) -> bool:
    if not aip127_migration_active():
        return False

    if "[new-required-request-property]" in block:
        for line in block.splitlines():
            match = AIP127_NEW_REQUIRED_REQUEST_PROPERTY.fullmatch(line.strip())
            if match is None:
                continue
            leaf = match.group(1).rsplit("/", 1)[-1]
            return (
                "_" not in leaf
                and re.fullmatch(r"[a-z][A-Za-z0-9]*", leaf) is not None
                and any(character.isupper() for character in leaf)
            )
        return False

    if "[response-required-property-removed]" in block:
        for line in block.splitlines():
            match = AIP127_REMOVED_REQUIRED_RESPONSE_PROPERTY.fullmatch(line.strip())
            if match is None:
                continue
            leaf = match.group(1).rsplit("/", 1)[-1]
            return re.fullmatch(r"[a-z][a-z0-9]*(?:_[a-z0-9]+)+", leaf) is not None
        return False

    if "[request-property-one-of-removed]" in block:
        return operation in AIP127_EVALUATOR_KIND_OPERATIONS and has_exact_detail(
            block, AIP127_EVALUATOR_ONE_OF_REMOVAL
        )

    return False


def is_canonical_error_migration_break(
    block: str, pairs: set[tuple[str, str]] | None,
) -> bool:
    if not pairs:
        return False
    operation = api_operation(block)
    if operation == "GET /health" and "[api-path-removed-without-deprecation]" in block:
        return has_exact_detail(block, re.compile(r"api path removed without deprecation"))
    if operation is None:
        return False

    optional = re.compile(
        r"the response property `error/details` became optional for the status `([45]\d\d)`"
    )
    code_format = re.compile(
        r"the `error/code` response's property `format` changed from `int32` to `none` for status `([45]\d\d)`"
    )
    enum_added = re.compile(
        r"added the new `([A-Z_]+)` enum value to the `error/status` response property "
        r"for the response status `([45]\d\d)`"
    )
    for line in block.splitlines():
        detail = line.strip()
        match = optional.fullmatch(detail)
        if match is not None:
            return "[response-property-became-optional]" in block and (operation, match.group(1)) in pairs
        match = code_format.fullmatch(detail)
        if match is not None:
            return "[response-property-type-changed]" in block and (operation, match.group(1)) in pairs
        match = enum_added.fullmatch(detail)
        if match is not None:
            return (
                "[response-property-enum-value-added]" in block
                and match.group(1) in CANONICAL_ERROR_STATUS_VALUES
                and (operation, match.group(2)) in pairs
            )
    return False


def is_allowed_alignment_break(block: str, canonical_error_pairs: set[tuple[str, str]] | None = None) -> bool:
    operation = api_operation(block)

    if canonical_error_pairs is not None and is_canonical_error_migration_break(block, canonical_error_pairs):
        return True
    if is_aip127_alignment_break(block, operation):
        return True
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
    if len(sys.argv) != 4:
        print("usage: filter-oasdiff-breaking.py OASDIFF_LOG BASE_SPEC NEW_SPEC", file=sys.stderr)
        return 2
    text = Path(sys.argv[1]).read_text(encoding="utf-8")
    canonical_error_pairs = reviewed_canonical_error_pairs(Path(sys.argv[2]), Path(sys.argv[3]))
    blocks = error_blocks(text)
    if not blocks:
        print(
            "oasdiff exited nonzero without recognized breaking-change diagnostics",
            file=sys.stderr,
        )
        return 2
    unexpected = [
        block for block in blocks
        if not is_allowed_alignment_break(block, canonical_error_pairs)
    ]
    if unexpected:
        print("Unexpected OpenAPI breaking changes:", file=sys.stderr)
        for block in unexpected:
            print(block, file=sys.stderr)
            print(file=sys.stderr)
        return 1
    print(f"Allowed {len(blocks)} intentional pre-1.0 contract alignment breaks.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
