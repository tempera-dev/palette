#!/usr/bin/env python3
"""Audit the OpenAPI spec for shape consistency across the API surface.

Enforces the conventions that keep the API/MCP/CLI/SDKs nice and in-sync:
  - every operation has a unique dotted resource.operation operationId
  - every operation has exactly one resource tag
  - every operation documents a uniform error body (ApiErrorBody) for failures
  - success responses reference a NAMED schema (no anonymous/inline objects)
  - public collection/search operations expose complete AIP-158 pagination
    (pageSize/pageToken/nextPageToken), with no migration exceptions
  - migrated operations reject legacy limit/cursor aliases
  - the shared error schema has the core AIP-193 HTTP/JSON envelope
  - ordinary JSON schema fields are lowerCamel, with an explicit per-schema
    brownfield-debt budget that may only decrease
Exit 1 on any violation.
"""

from __future__ import annotations

from dataclasses import dataclass
import json
import re
import sys
from pathlib import Path
from typing import Any, TextIO

DEFAULT_SPEC = "sdks/openapi/palette-api.json"
# Canonical AIP scheme (tempera-api-style-guide.md §4): `{collection}.{method}`,
# lower-camel collection, dot, lower-camel standard-method-or-custom-verb.
OPERATION_ID = re.compile(r"^[A-Za-z][A-Za-z0-9]*\.[a-z][A-Za-z0-9]*$")
# The migration is complete. This empty set is an explicit ratchet: adding any
# exception requires contract review and a focused scanner-test change.
AIP158_MIGRATION_DEBT = set()
# Custom verbs that are nevertheless public collection/search operations.
CUSTOM_COLLECTION_OPERATIONS = {"archive.querySpans", "search.spans"}
LOWER_CAMEL_PROPERTY = re.compile(r"^[a-z][A-Za-z0-9]*$")
# Exact per-schema count of ordinary JSON properties that still use legacy
# snake_case. Every schema omitted here has a zero-debt budget. This is
# deliberately granular enough that debt cannot move to a new schema, and each
# entry may only decrease while HTTP DTOs are separated from persisted domain
# models. Raw OTLP bodies and MCP protocol payloads are not component schemas in
# this product OpenAPI document and therefore retain their protocol spellings.
AIP127_PROPERTY_DEBT_BUDGET = {
    "AddPromptVersionRequest": 1,
    "AlertDecision": 1,
    "AlertInput": 5,
    "AlertLinks": 4,
    "AlertPolicy": 6,
    "ApiKeyCreatedResponse": 5,
    "ArchiveManifest": 4,
    "ArchivedSpanRow": 20,
    "ArtifactRef": 4,
    "AuditEvent": 8,
    "AuthContext": 1,
    "BusMessage": 6,
    "CalibrationConfusion": 4,
    "CalibrationItem": 5,
    "CalibrationPolicy": 1,
    "CalibrationReport": 19,
    "CanonicalSpan": 14,
    "CaseExperimentScore": 15,
    "CaseOutputOverrideRequest": 1,
    "ConnectionLink": 3,
    "ConnectionStatus": 1,
    "ConnectorTool": 2,
    "CreateDatasetVersionRequest": 1,
    "CreateGateRequest": 4,
    "CreatePromptRequest": 1,
    "CreateProviderSecretHttpRequest": 2,
    "CreateReviewQueueHttpRequest": 2,
    "CreateScenarioRequest": 4,
    "Dataset": 4,
    "DatasetCase": 11,
    "DatasetEvalReport": 9,
    "DatasetVersionSnapshot": 6,
    "DeadLetter": 1,
    "DeadLetterReplayReport": 4,
    "DiffLine": 2,
    "EnqueueReviewTaskFromTraceHttpRequest": 5,
    "EvalReproducibility": 16,
    "EvalResult": 7,
    "ExperimentComparison": 8,
    "ExperimentRunReport": 11,
    "GateDefinition": 7,
    "GatePolicy": 3,
    "GateRunReport": 17,
    "ImportTemperaEvidenceRequest": 3,
    "IngestOutcome": 1,
    "IngestQueueStatus": 6,
    "JudgeAuditRecord": 10,
    "JudgeBrokerOutcome": 1,
    "MaintenanceWindow": 2,
    "MineScenariosRequest": 2,
    "Money": 1,
    "NativeIngestRequest": 8,
    "OnlineSamplingPolicy": 4,
    "OtlpIngestOutcome": 5,
    "PaletteConnectStatusResponse": 5,
    "PerturbationKnobs": 5,
    "PromoteReviewAnnotationHttpRequest": 1,
    "PromoteTraceCaseRequest": 2,
    "Prompt": 5,
    "PromptVersion": 5,
    "PromptVersionDiff": 2,
    "PromptVersionMetadata": 2,
    "ProviderSecretMetadata": 6,
    "PublicJudgeAuditRecord": 8,
    "QueuedTraceWork": 3,
    "ReliabilityBin": 6,
    "ReviewAnnotation": 7,
    "ReviewQueue": 5,
    "ReviewTask": 10,
    "RevokedApiKey": 2,
    "RevokedProviderSecret": 2,
    "RunCalibrationHttpRequest": 3,
    "RunDeterministicEvalRequest": 6,
    "RunExperimentRequest": 7,
    "RunGateRequest": 1,
    "RunJudgeDatasetEvalRequest": 6,
    "RunJudgeEvalHttpRequest": 2,
    "RunJudgeExperimentRequest": 8,
    "RunSummary": 10,
    "SamplingDecision": 1,
    "Scenario": 9,
    "ScenarioCluster": 3,
    "SearchHit": 5,
    "SpanIoResponse": 3,
    "SubmitReviewAnnotationHttpRequest": 2,
    "TemperaEvidenceReceipt": 10,
    "TemperaEvidenceSummary": 4,
    "TenantScope": 3,
    "TokenCounts": 1,
    "ToolExecution": 1,
    "Toolkit": 3,
    "TraceIngestedDrainReport": 4,
    "TraceIngestedReconcileReport": 7,
    "TraceView": 2,
    "TraceWriteDrainReport": 11,
    "UsageSummary": 2,
    "WebhookDelivery": 1,
    "WriteAck": 4,
}


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
        # 422 is used for partial-success (drain-with-dead-letters) and carries a
        # domain payload, not the shared error body, so it's exempt from this rule.
        if oid != "health.check":
            err_codes = [
                c for c in responses if c.startswith(("4", "5")) and c != "422"
            ]
            if not err_codes:
                violations.append(f"{where}: no documented 4xx/5xx error response")
            for code in err_codes:
                ref = (
                    responses[code]
                    .get("content", {})
                    .get("application/json", {})
                    .get("schema", {})
                    .get("$ref", "")
                )
                if not ref.endswith("/ErrorResponse") and not ref.endswith(
                    "/ApiErrorBody"
                ):
                    violations.append(
                        f"{where}: error {code} body is not the shared error schema (got {ref or 'none'})"
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
    error_response = schemas.get("ErrorResponse", {})
    error_ref = (
        error_response.get("properties", {}).get("error", {}).get("$ref", "")
    )
    if not error_ref.endswith("/ErrorStatus"):
        violations.append(
            "AIP-193 ErrorResponse.error must reference ErrorStatus"
        )
    error_status = schemas.get("ErrorStatus", {})
    error_status_properties = set(error_status.get("properties", {}))
    expected_error_status = {"code", "message", "status", "details"}
    if not expected_error_status.issubset(error_status_properties):
        violations.append(
            "AIP-193 ErrorStatus lacks "
            f"{sorted(expected_error_status - error_status_properties)}"
        )
    details_items = (
        error_status.get("properties", {}).get("details", {}).get("items", {})
    )
    if details_items.get("type") != "object":
        violations.append(
            "AIP-193 ErrorStatus.details must contain standard detail objects"
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
