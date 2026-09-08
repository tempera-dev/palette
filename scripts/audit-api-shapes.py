#!/usr/bin/env python3
"""Audit the OpenAPI spec for shape consistency across the API surface.

Enforces the conventions that keep the API/MCP/CLI/SDKs nice and in-sync:
  - every operation has a unique dotted resource.operation operationId
  - every operation has exactly one resource tag
  - every operation documents a uniform error body for failures, referencing the
    one shared `#/components/responses/Error` (the google.rpc.Status envelope
    every Tempera producer publishes)
  - success responses reference a NAMED schema (no anonymous/inline objects)
  - public collection/search operations expose complete AIP-158 pagination
    (pageSize/pageToken/nextPageToken), with no migration exceptions
  - migrated operations reject legacy limit/cursor aliases
  - the shared error response resolves to the canonical AIP-193 `Status`
    envelope
  - ordinary JSON schema fields are lowerCamel, with zero migration debt
Exit 1 on any violation.
"""

from __future__ import annotations

from dataclasses import dataclass
import json
import re
import sys
from pathlib import Path
from typing import Any, TextIO

DEFAULT_SPEC = "contracts/openapi/palette.openapi.json"
# The one shared google.rpc.Status response every Tempera producer publishes.
SHARED_ERROR_RESPONSE = "#/components/responses/Error"
# Canonical AIP scheme (tempera-api-style-guide.md §4): `{collection}.{method}`,
# lower-camel collection, dot, lower-camel standard-method-or-custom-verb.
OPERATION_ID = re.compile(r"^[A-Za-z][A-Za-z0-9]*\.[a-z][A-Za-z0-9]*$")
# The migration is complete. This empty set is an explicit ratchet: adding any
# exception requires contract review and a focused scanner-test change.
AIP158_MIGRATION_DEBT = set()
# Custom verbs that are nevertheless public collection/search operations.
CUSTOM_COLLECTION_OPERATIONS = {"archive.querySpans", "search.spans"}
LOWER_CAMEL_PROPERTY = re.compile(r"^[a-z][A-Za-z0-9]*$")
# The migration is complete. Every ordinary component-schema property must now
# use lowerCamelCase. Raw OTLP bodies and MCP protocol payloads are not component
# schemas in this product OpenAPI document and retain their protocol spellings.
AIP127_PROPERTY_DEBT_BUDGET: dict[str, int] = {}


@dataclass(frozen=True)
class AuditResult:
    operation_count: int
    unique_operation_id_count: int
    schema_count: int
    aip127_debt_schema_count: int
    aip127_debt_field_count: int
    violations: list[str]


def load_spec(path: str | Path) -> dict[str, Any]:
    with Path(path).open(encoding="utf-8") as handle:
        return json.load(handle)


def operations(spec: dict[str, Any]) -> list[tuple[str, str, dict[str, Any]]]:
    ops = []
    for path, methods in spec["paths"].items():
        for method, op in methods.items():
            if method in ("parameters",):
                continue
            ops.append((method.upper(), path, op))
    return ops


def audit_spec(spec: dict[str, Any]) -> AuditResult:
    violations = []
    op_ids: dict[str, list[str]] = {}
    ops = operations(spec)

    for method, path, op in ops:
        where = f"{method} {path}"
        oid = op.get("operationId")
        tags = op.get("tags", [])

        if not oid:
            violations.append(f"{where}: missing operationId")
        else:
            if not OPERATION_ID.match(oid):
                violations.append(
                    f"{where}: operationId '{oid}' is not dotted resource.operation"
                )
            op_ids.setdefault(oid, []).append(where)

        if len(tags) != 1:
            violations.append(f"{where}: expected exactly 1 tag, got {tags}")

        responses = op.get("responses", {})
        # Health is the only allowed exception to the error-body rule.
        if oid != "health.check":
            err_codes = [
                c for c in responses if c == "default" or c.startswith(("4", "5"))
            ]
            if not err_codes:
                violations.append(f"{where}: no documented 4xx/5xx error response")
            for code in err_codes:
                # Every failure is the one shared google.rpc.Status response
                # component, not a per-operation copy of its schema.
                ref = responses[code].get("$ref", "")
                if ref != SHARED_ERROR_RESPONSE:
                    violations.append(
                        f"{where}: error {code} is not {SHARED_ERROR_RESPONSE} "
                        f"(got {ref or 'an inline response'})"
                    )

        # Success response must reference a named schema (no inline/anonymous object).
        for code, body in responses.items():
            if not code.startswith("2"):
                continue
            schema = body.get("content", {}).get("application/json", {}).get(
                "schema", {}
            )
            if not schema:
                continue  # empty 204-style ok
            if (
                "$ref" not in schema
                and schema.get("type") == "object"
                and "properties" in schema
            ):
                violations.append(
                    f"{where}: success {code} uses an inline anonymous object (name it)"
                )

    # operationId uniqueness
    for oid, wheres in op_ids.items():
        if len(wheres) > 1:
            violations.append(f"operationId '{oid}' is duplicated: {wheres}")

    schemas = spec["components"]["schemas"]
    error_response = spec["components"].get("responses", {}).get("Error", {})
    error_schema_ref = (
        error_response.get("content", {})
        .get("application/json", {})
        .get("schema", {})
        .get("$ref", "")
    )
    if not error_schema_ref.endswith("/Status"):
        violations.append(
            "AIP-193 components.responses.Error must reference the Status schema"
        )
    status = schemas.get("Status", {})
    error_status = status.get("properties", {}).get("error", {})
    error_status_properties = set(error_status.get("properties", {}))
    expected_error_status = {"code", "message", "status", "details"}
    if not expected_error_status.issubset(error_status_properties):
        violations.append(
            "AIP-193 Status.error lacks "
            f"{sorted(expected_error_status - error_status_properties)}"
        )
    details_items = (
        error_status.get("properties", {}).get("details", {}).get("items", {})
    )
    if details_items.get("type") != "object":
        violations.append(
            "AIP-193 Status.error.details must contain standard detail objects"
        )

    aip127_debt_by_schema: dict[str, list[str]] = {}
    for schema_name, schema in schemas.items():
        debt = sorted(
            property_name
            for property_name in schema.get("properties", {})
            if property_name != "@type"
            and not LOWER_CAMEL_PROPERTY.fullmatch(property_name)
        )
        if debt:
            aip127_debt_by_schema[schema_name] = debt
    for schema_name, debt in aip127_debt_by_schema.items():
        budget = AIP127_PROPERTY_DEBT_BUDGET.get(schema_name, 0)
        if len(debt) > budget:
            violations.append(
                f"AIP-127 schema '{schema_name}' has {len(debt)} non-lowerCamel "
                f"properties, above budget {budget}: {debt}"
            )

    # Every public list/search collection uses AIP-158. The empty debt set is
    # retained as an explicit ratchet against reintroducing an exception.
    for method, path, op in ops:
        oid = op.get("operationId", "")
        verb = oid.split(".", 1)[1] if "." in oid else oid
        # `list` or a lower-camel `list<Qualifier>` standard method.
        is_list = verb == "list" or (
            verb.startswith("list") and len(verb) > 4 and verb[4].isupper()
        )
        if is_list or oid in CUSTOM_COLLECTION_OPERATIONS:
            params = {p.get("name") for p in op.get("parameters", [])}
            aip_params = {"pageSize", "pageToken"}
            has_aip_pagination = aip_params.issubset(params)
            legacy_params = params.intersection({"cursor", "limit"})
            has_legacy_pagination = bool(legacy_params)
            if not has_aip_pagination and not has_legacy_pagination:
                violations.append(
                    f"{method} {path}: collection op '{oid}' lacks pagination params"
                )
            if params.intersection(aip_params) and not has_aip_pagination:
                missing = sorted(aip_params.difference(params))
                violations.append(
                    f"{method} {path}: AIP-158 list op '{oid}' lacks {missing}"
                )
            if oid not in AIP158_MIGRATION_DEBT and not has_aip_pagination:
                violations.append(
                    f"{method} {path}: migrated collection op '{oid}' must use AIP-158"
                )
            if oid not in AIP158_MIGRATION_DEBT and legacy_params:
                violations.append(
                    f"{method} {path}: migrated collection op '{oid}' retains legacy "
                    f"{sorted(legacy_params)}"
                )
            if has_aip_pagination:
                success_schemas = [
                    response.get("content", {})
                    .get("application/json", {})
                    .get("schema", {})
                    for code, response in op.get("responses", {}).items()
                    if code.startswith("2")
                ]
                response_has_next_token = False
                for schema in success_schemas:
                    ref = schema.get("$ref", "")
                    if ref.startswith("#/components/schemas/"):
                        schema_name = ref.rsplit("/", 1)[-1]
                        schema = spec["components"]["schemas"].get(schema_name, {})
                    if "nextPageToken" in schema.get("properties", {}):
                        response_has_next_token = True
                        break
                if not response_has_next_token:
                    violations.append(
                        f"{method} {path}: AIP-158 list op '{oid}' response lacks nextPageToken"
                    )

    return AuditResult(
        operation_count=len(ops),
        unique_operation_id_count=len(op_ids),
        schema_count=len(spec["components"]["schemas"]),
        aip127_debt_schema_count=len(aip127_debt_by_schema),
        aip127_debt_field_count=sum(map(len, aip127_debt_by_schema.values())),
        violations=violations,
    )


def report(result: AuditResult, stream: TextIO = sys.stdout) -> None:
    print(
        f"audited {result.operation_count} operations, "
        f"{result.unique_operation_id_count} unique operationIds, "
        f"{result.schema_count} schemas; AIP-127 residual debt "
        f"{result.aip127_debt_field_count} fields across "
        f"{result.aip127_debt_schema_count} schemas",
        file=stream,
    )
    if result.violations:
        print(f"\n{len(result.violations)} CONSISTENCY VIOLATIONS:", file=stream)
        for violation in result.violations:
            print(f"  - {violation}", file=stream)
        return
    print("PASS: API shapes are consistent", file=stream)


def main(argv: list[str] | None = None, stream: TextIO = sys.stdout) -> int:
    args = sys.argv[1:] if argv is None else argv
    spec_path = args[0] if args else DEFAULT_SPEC
    result = audit_spec(load_spec(spec_path))
    report(result, stream=stream)
    return 1 if result.violations else 0


if __name__ == "__main__":
    sys.exit(main())
