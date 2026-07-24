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


def test_allows_only_digest_pinned_aip127_alignment_shapes() -> None:
    assert FILTER.aip127_migration_active()
    assert FILTER.is_allowed_alignment_break(
        block(
            "new-required-request-property",
            "POST /v1/datasets/{tenant_id}/{project_id}",
            "added the new required request property `datasetId`",
        )
    )
    assert FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "POST /v1/datasets/{tenant_id}/{project_id}",
            "removed the required property `dataset_id` from the response with the `200` status",
        )
    )
    assert FILTER.is_allowed_alignment_break(
        block(
            "request-property-one-of-removed",
            "POST /v1/judge/{tenant_id}/{project_id}/evaluate",
            "removed `subschema #5, subschema #6, subschema #8, subschema #9, subschema #10` "
            "from the `evaluator/kind` request property `oneOf` list",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "new-required-request-property",
            "POST /v1/datasets/{tenant_id}/{project_id}",
            "added the new required request property `unrelated`",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "response-required-property-removed",
            "POST /v1/datasets/{tenant_id}/{project_id}",
            "removed the required property `datasetId` from the response with the `200` status",
        )
    )
    assert not FILTER.is_allowed_alignment_break(
        block(
            "request-property-one-of-removed",
            "POST /v1/datasets/{tenant_id}/{project_id}",
            "removed `subschema #5, subschema #6, subschema #8, subschema #9, subschema #10` "
            "from the `kind` request property `oneOf` list",
        )
    )


def test_aip127_allowance_disables_when_contract_digest_changes() -> None:
    original = FILTER.AIP127_MIGRATION_SPEC_SHA256
    try:
        FILTER.AIP127_MIGRATION_SPEC_SHA256 = "0" * 64
        FILTER.aip127_migration_active.cache_clear()
        assert not FILTER.is_allowed_alignment_break(
            block(
                "response-required-property-removed",
                "POST /v1/datasets/{tenant_id}/{project_id}",
                "removed the required property `dataset_id` from the response with the `200` status",
            )
        )
    finally:
        FILTER.AIP127_MIGRATION_SPEC_SHA256 = original
        FILTER.aip127_migration_active.cache_clear()


if __name__ == "__main__":
    test_error_blocks_splits_oasdiff_output()
    test_allows_only_reviewed_aip193_error_envelope_changes()
    test_allows_only_reviewed_aip158_wrapper_changes()
    test_allows_only_digest_pinned_aip127_alignment_shapes()
    test_aip127_allowance_disables_when_contract_digest_changes()
    print("filter-oasdiff-breaking tests passed")
