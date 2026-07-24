"""Regression tests for the reviewed pre-1.0 oasdiff exceptions."""

from __future__ import annotations

import importlib.util
from pathlib import Path


SCRIPT = Path(__file__).resolve().parents[1] / "filter-oasdiff-breaking.py"
SPEC = importlib.util.spec_from_file_location("filter_oasdiff_breaking", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
FILTER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(FILTER)


def block(rule: str, operation: str, detail: str) -> str:
    return f"error\t[{rule}] at /specs/new-spec.json\n\tin API {operation}\n\t\t{detail}"


def test_error_blocks_splits_oasdiff_output() -> None:
    first = block(
        "response-property-type-changed",
        "GET /v1/usage/{tenant_id}/{project_id}",
        "the `error` response's property `type` changed from `string` to `object` "
        "for status `403`",
    )
    second = block(
        "response-required-property-removed",
        "GET /v1/usage/{tenant_id}/{project_id}",
        "removed the required property `message` from the response with the `403` status",
    )
    assert FILTER.error_blocks(f"{first}\n\n{second}\n") == [first, second]


def test_allows_only_reviewed_aip193_error_envelope_changes() -> None:
    assert FILTER.is_allowed_alignment_break(
        block(
            "response-property-type-changed",
            "POST /v1/traces/native",
            "the `error` response's property `type` changed from `string` to `object` "
            "for status `429`",
        )
    )
    assert FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "POST /v1/traces/native",
            "removed the required property `message` from the response with the `429` status",
        )
    )
    assert FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "POST /v1/traces/native",
            "removed the required property `status` from the response with the `429` status",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "response-property-type-changed",
            "POST /v1/traces/native",
            "the `message` response's property `type` changed from `string` to `object` "
            "for status `429`",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "POST /v1/traces/native",
            "removed the required property `trace_id` from the response with the `429` status",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "POST /v1/traces/native",
            "removed the required property `message` from the response with the `200` status",
        )
    )


def test_allows_only_reviewed_aip158_wrapper_changes() -> None:
    for operation in FILTER.AIP158_ARRAY_WRAPPER_OPERATIONS:
        assert FILTER.is_allowed_alignment_break(
            block(
                "response-body-type-changed",
                operation,
                "the response's body `type` changed from `array<object>` to `object` "
                "for status `200`",
            )
        )

    assert FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "GET /v1/traces/{tenant_id}",
            "removed the required property `items` from the response with the `200` status",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "response-body-type-changed",
            "GET /v1/usage/{tenant_id}/{project_id}",
            "the response's body `type` changed from `array<object>` to `object` "
            "for status `200`",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "GET /v1/audit/{tenant_id}/{project_id}",
            "removed the required property `items` from the response with the `200` status",
        )
    )
