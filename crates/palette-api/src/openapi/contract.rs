//! The Tempera producer contract standard, applied to Palette's generated
//! OpenAPI document.
//!
//! `utoipa` derives the routes, parameters, and schemas from the real handlers
//! in `lib.rs`. What it cannot derive is the cross-product half of the standard
//! that the Tempera SDK generator consumes:
//!
//! * `components.schemas.Status` / `components.responses.Error` byte-identical
//!   to `tempera-sdk/contracts/status-component.json` (AIP-193), with every
//!   non-2xx resource response pointing at the shared response;
//! * an `x-tempera-*` credential declaration on every resource operation, so a
//!   generated client knows which credential and scope the route demands;
//! * `x-tempera-protocol-routes`, the honest list of routes that answer to a
//!   native protocol rather than to Google AIP.
//!
//! Applying it here — rather than hand-editing the emitted JSON — is what keeps
//! `contracts/openapi/palette.openapi.json` a pure build product of the
//! handlers, and keeps `GET /openapi.json` serving the same document. The tests
//! below fail closed when a new route arrives without a declaration.
//!
//! See `tempera-sdk/docs/CONTRACT_STANDARD.md` for the normative text.

use serde_json::{Map, Value, json};

/// A verbatim copy of `tempera-sdk/contracts/status-component.json`. The
/// producer lint compares the emitted components against that file byte for
/// byte, so this is vendored rather than re-typed.
const STATUS_COMPONENT_JSON: &str = include_str!("status-component.json");

/// Routes that answer to a native protocol and are therefore exempt from the
/// AIP path and error rules. `/healthz` is a liveness probe; the OTLP path is
/// the OpenTelemetry collector's own `/v1/traces` shape, which Palette must
/// serve verbatim for an unmodified OTLP exporter to reach it.
///
/// `POST /v1/traces` — the OTLP/JSON collector alias — is deliberately absent:
/// it already satisfies every AIP rule, so it needs no exemption, and the
/// producer lint refuses to exempt a route that does not need it.
pub(crate) const PROTOCOL_ROUTES: &[&str] = &[
    "/healthz",
    "/v1/otlp/{tenantId}/{projectId}/{environmentId}/v1/traces",
];

/// The OAuth resource indicator Palette validates every access token against
/// (`--oauth-resource`, `PALETTE_OAUTH_RESOURCE`, default `palette`).
const AUTH_AUDIENCE: &str = "palette";

// The scopes are `palette_security::ApiScope`, which is what the handlers
// actually pass to `authorize_project_route` / `authorize_tenant_route` /
// `authorize_query_scope`. They are the registered control-plane scope names,
// so a token minted by the Tempera control plane can carry them as-is.
const TRACE_READ: &str = "trace:read";
const TRACE_WRITE: &str = "trace:write";
const DATASET_WRITE: &str = "dataset:write";
const SCENARIO_READ: &str = "scenario:read";
const SCENARIO_WRITE: &str = "scenario:write";
const EVAL_RUN: &str = "eval:run";
const ADMIN: &str = "admin";

/// One row of the credential declaration.
struct OperationContract {
    method: &'static str,
    path: &'static str,
    scope: &'static str,
}

const fn op(method: &'static str, path: &'static str, scope: &'static str) -> OperationContract {
    OperationContract {
        method,
        path,
        scope,
    }
}

/// Every non-protocol operation Palette serves, with the scope its handler
/// actually requires.
///
/// Each row mirrors the `ApiScope` the handler passes to `authorize*`; an
/// OAuth or control-plane token additionally has to carry `mcp:invoke`, which
/// is a gate on the credential rather than on the route, so it is not the
/// route's `x-tempera-required-scope`.
const OPERATIONS: &[OperationContract] = &[
    op(
        "post",
        "/v1/alerts/{tenantId}/{projectId}/traces/{traceId}/webhook",
        TRACE_READ,
    ),
    op(
        "post",
        "/v1/api-keys/{tenantId}/{projectId}/{environmentId}",
        ADMIN,
    ),
    op(
        "post",
        "/v1/api-keys/{tenantId}/{projectId}/{environmentId}/{apiKeyId}/revoke",
        ADMIN,
    ),
    op(
        "get",
        "/v1/archive/{tenantId}/{projectId}/spans",
        TRACE_READ,
    ),
    op(
        "post",
        "/v1/archive/{tenantId}/{projectId}/{traceId}",
        TRACE_READ,
    ),
    op("get", "/v1/audit/{tenantId}/{projectId}", ADMIN),
    op(
        "post",
        "/v1/calibrations/{tenantId}/{projectId}/{datasetId}/versions/{versionId}",
        EVAL_RUN,
    ),
    op(
        "get",
        "/v1/connect/status/{tenantId}/{projectId}",
        TRACE_READ,
    ),
    op("get", "/v1/connectors/{tenantId}/{projectId}", TRACE_READ),
    op(
        "post",
        "/v1/connectors/{tenantId}/{projectId}/connect",
        ADMIN,
    ),
    op(
        "post",
        "/v1/connectors/{tenantId}/{projectId}/invoke",
        EVAL_RUN,
    ),
    op(
        "get",
        "/v1/connectors/{tenantId}/{projectId}/skills",
        TRACE_READ,
    ),
    op(
        "get",
        "/v1/connectors/{tenantId}/{projectId}/status",
        TRACE_READ,
    ),
    op(
        "get",
        "/v1/connectors/{tenantId}/{projectId}/tools",
        TRACE_READ,
    ),
    op("post", "/v1/datasets/{tenantId}/{projectId}", DATASET_WRITE),
    op(
        "post",
        "/v1/datasets/{tenantId}/{projectId}/{datasetId}/cases/from-trace",
        DATASET_WRITE,
    ),
    op(
        "post",
        "/v1/datasets/{tenantId}/{projectId}/{datasetId}/versions",
        DATASET_WRITE,
    ),
    op(
        "post",
        "/v1/datasets/{tenantId}/{projectId}/{datasetId}/versions/{versionId}/evals/deterministic",
        EVAL_RUN,
    ),
    op(
        "post",
        "/v1/datasets/{tenantId}/{projectId}/{datasetId}/versions/{versionId}/evals/judge",
        EVAL_RUN,
    ),
    op(
        "post",
        "/v1/eval-results/{tenantId}/{projectId}/tempera/bundles",
        EVAL_RUN,
    ),
    op(
        "post",
        "/v1/eval-results/{tenantId}/{projectId}/tempera/decisions",
        EVAL_RUN,
    ),
    op(
        "get",
        "/v1/eval-results/{tenantId}/{projectId}/tempera/{kind}/{externalId}",
        EVAL_RUN,
    ),
    op(
        "post",
        "/v1/experiments/{tenantId}/{projectId}/{datasetId}/versions/{versionId}/deterministic",
        EVAL_RUN,
    ),
    op(
        "post",
        "/v1/experiments/{tenantId}/{projectId}/{datasetId}/versions/{versionId}/judge",
        EVAL_RUN,
    ),
    op("post", "/v1/gates/{tenantId}/{projectId}", EVAL_RUN),
    op(
        "post",
        "/v1/gates/{tenantId}/{projectId}/{gateId}/run",
        EVAL_RUN,
    ),
    op(
        "post",
        "/v1/import/{tenantId}/{projectId}/{environmentId}",
        TRACE_WRITE,
    ),
    op(
        "post",
        "/v1/ingest/{tenantId}/{projectId}/dead-letters/{messageId}/replay",
        ADMIN,
    ),
    op("get", "/v1/ingest/{tenantId}/{projectId}/queue", ADMIN),
    op(
        "post",
        "/v1/ingest/{tenantId}/{projectId}/trace-ingested/drain",
        ADMIN,
    ),
    op(
        "post",
        "/v1/ingest/{tenantId}/{projectId}/trace-writes/drain",
        ADMIN,
    ),
    op(
        "post",
        "/v1/ingest/{tenantId}/{projectId}/traces/{traceId}/reconcile",
        ADMIN,
    ),
    op(
        "post",
        "/v1/judge/{tenantId}/{projectId}/evaluate",
        EVAL_RUN,
    ),
    op("get", "/v1/judge/{tenantId}/{projectId}/ledger", EVAL_RUN),
    op(
        "post",
        "/v1/online/{tenantId}/{projectId}/traces/{traceId}/sampling",
        TRACE_READ,
    ),
    op("get", "/v1/prompts/{tenantId}/{projectId}", TRACE_READ),
    op("post", "/v1/prompts/{tenantId}/{projectId}", DATASET_WRITE),
    op(
        "get",
        "/v1/prompts/{tenantId}/{projectId}/{promptId}",
        TRACE_READ,
    ),
    op(
        "get",
        "/v1/prompts/{tenantId}/{projectId}/{promptId}/diff",
        TRACE_READ,
    ),
    op(
        "get",
        "/v1/prompts/{tenantId}/{projectId}/{promptId}/versions",
        TRACE_READ,
    ),
    op(
        "post",
        "/v1/prompts/{tenantId}/{projectId}/{promptId}/versions",
        DATASET_WRITE,
    ),
    op("get", "/v1/provider-secrets/{tenantId}/{projectId}", ADMIN),
    op("post", "/v1/provider-secrets/{tenantId}/{projectId}", ADMIN),
    op(
        "post",
        "/v1/provider-secrets/{tenantId}/{projectId}/{providerSecretId}/revoke",
        ADMIN,
    ),
    op(
        "post",
        "/v1/review-queues/{tenantId}/{projectId}",
        DATASET_WRITE,
    ),
    op(
        "get",
        "/v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks",
        DATASET_WRITE,
    ),
    op(
        "post",
        "/v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/from-trace",
        DATASET_WRITE,
    ),
    op(
        "post",
        "/v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations",
        DATASET_WRITE,
    ),
    op(
        "post",
        "/v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks/{taskId}/annotations/{annotationId}/promote",
        DATASET_WRITE,
    ),
    op("get", "/v1/scenarios/{tenantId}/{projectId}", SCENARIO_READ),
    op(
        "post",
        "/v1/scenarios/{tenantId}/{projectId}",
        SCENARIO_WRITE,
    ),
    op(
        "post",
        "/v1/scenarios/{tenantId}/{projectId}/mine",
        SCENARIO_READ,
    ),
    op(
        "get",
        "/v1/scenarios/{tenantId}/{projectId}/{scenarioId}",
        SCENARIO_READ,
    ),
    op("get", "/v1/search/{tenantId}/spans", TRACE_READ),
    op("get", "/v1/spans/{tenantId}/{traceId}/{spanId}", TRACE_READ),
    op(
        "get",
        "/v1/spans/{tenantId}/{traceId}/{spanId}/io",
        TRACE_READ,
    ),
    op("post", "/v1/traces", TRACE_WRITE),
    op("post", "/v1/traces/native", TRACE_WRITE),
    op("get", "/v1/traces/{tenantId}", TRACE_READ),
    op("get", "/v1/traces/{tenantId}/{traceId}", TRACE_READ),
    op("get", "/v1/usage/{tenantId}/{projectId}", ADMIN),
];

const HTTP_METHODS: &[&str] = &["get", "post", "put", "patch", "delete"];

fn status_components() -> &'static Value {
    use std::sync::OnceLock;
    static PARSED: OnceLock<Value> = OnceLock::new();
    PARSED.get_or_init(|| match serde_json::from_str(STATUS_COMPONENT_JSON) {
        Ok(value) => value,
        // The file is compiled in, so a parse failure is a build bug.
        Err(error) => panic!("vendored status-component.json is invalid JSON: {error}"),
    })
}

fn is_success(status: &str) -> bool {
    status.starts_with('2')
}

fn is_protocol_route(path: &str) -> bool {
    PROTOCOL_ROUTES.contains(&path)
}

/// Apply the standard to the serialized `utoipa` document, in place.
pub(crate) fn apply(document: &mut Value) {
    install_status_components(document);
    install_security_schemes(document);
    rewrite_error_responses(document);
    prune_superseded_error_schemas(document);
    declare_credentials(document);
    declare_protocol_routes(document);
}

fn components<'a>(document: &'a mut Value, section: &str) -> Option<&'a mut Map<String, Value>> {
    document
        .as_object_mut()?
        .entry("components")
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()?
        .entry(section.to_string())
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
}

/// Publish the one error envelope every Tempera producer shares.
fn install_status_components(document: &mut Value) {
    let canonical = status_components();
    let status = canonical["schemas"]["Status"].clone();
    let error = canonical["responses"]["Error"].clone();
    if let Some(schemas) = components(document, "schemas") {
        schemas.insert("Status".to_string(), status);
    }
    if let Some(responses) = components(document, "responses") {
        responses.insert("Error".to_string(), error);
    }
}

/// Name the two credentials Palette's `authorize` accepts. Both arrive at
/// `presented_api_key`: an `Authorization: Bearer` value (a `bt_` API key, a
/// `bao_` OAuth access token, or a control-plane token introspected against
/// audience `palette`), or the same secret in `x-palette-api-key`.
///
/// These are declared, not yet *required*: attaching a `security` requirement
/// to each operation changes what the seven generated clients emit, and how
/// Palette's product-native header credential should be modelled by the Tempera
/// SDK is still an open decision. `x-tempera-auth-kind` already tells the SDK
/// generator what to attach.
fn install_security_schemes(document: &mut Value) {
    let Some(schemes) = components(document, "securitySchemes") else {
        return;
    };
    schemes.insert(
        "paletteBearer".to_string(),
        json!({
            "type": "http",
            "scheme": "bearer",
            "description": "A Palette API key (bt_…), an OAuth 2.1 access token (bao_…), or a Tempera control-plane token whose audience is `palette`."
        }),
    );
    schemes.insert(
        "paletteApiKey".to_string(),
        json!({
            "type": "apiKey",
            "in": "header",
            "name": "x-palette-api-key",
            "description": "The same secret as the bearer, for callers that cannot set Authorization. Under strict auth the request must also carry x-palette-tenant-id, x-palette-project-id, and x-palette-environment-id unless the token's own claims supply them."
        }),
    );
}

/// Point every non-2xx response at `#/components/responses/Error`.
///
/// This covers the protocol routes too, because Palette does not actually
/// answer them in a foreign error shape: the OTLP endpoint's failures come out
/// of the same `ApiError` as every resource route, so describing them as
/// anything else would be a fiction. What the protocol declaration buys those
/// routes is the AIP *path* exemption, not a different envelope.
///
/// A missing error response is only *added* to resource routes. `/healthz` is
/// a liveness probe with one answer, and inventing a failure shape for it would
/// be the same fiction in the other direction.
fn rewrite_error_responses(document: &mut Value) {
    let error_ref = json!({ "$ref": "#/components/responses/Error" });
    let Some(paths) = document.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };
    for (path, path_item) in paths {
        let protocol = is_protocol_route(path);
        let Some(methods) = path_item.as_object_mut() else {
            continue;
        };
        for (method, operation) in methods {
            if !HTTP_METHODS.contains(&method.as_str()) {
                continue;
            }
            let Some(operation) = operation.as_object_mut() else {
                continue;
            };
            let Some(responses) = operation
                .entry("responses")
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
            else {
                continue;
            };
            // An operation that documents only successes tells a generated
            // client it cannot fail, which is untrue of every resource route.
            if !protocol {
                responses.entry("default").or_insert_with(|| Value::Null);
            }
            for (status, response) in responses {
                if is_success(status) {
                    continue;
                }
                *response = error_ref.clone();
            }
        }
    }
}

/// Palette's bespoke error schemas are unreachable once every resource
/// response references the shared component. Removing one can orphan the next,
/// so keep sweeping until a pass removes nothing.
fn prune_superseded_error_schemas(document: &mut Value) {
    const SUPERSEDED: &[&str] = &["ErrorResponse", "ErrorStatus"];
    loop {
        let serialized = serde_json::to_string(&document).unwrap_or_default();
        let Some(schemas) = document
            .get_mut("components")
            .and_then(|components| components.get_mut("schemas"))
            .and_then(Value::as_object_mut)
        else {
            return;
        };
        let mut removed = false;
        for name in SUPERSEDED {
            if schemas.contains_key(*name)
                && !serialized.contains(&format!("\"#/components/schemas/{name}\""))
            {
                schemas.remove(*name);
                removed = true;
            }
        }
        if !removed {
            return;
        }
    }
}

/// Stamp the credential declaration onto every operation in [`OPERATIONS`].
fn declare_credentials(document: &mut Value) {
    let Some(paths) = document.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };
    for contract in OPERATIONS {
        let Some(operation) = paths
            .get_mut(contract.path)
            .and_then(Value::as_object_mut)
            .and_then(|item| item.get_mut(contract.method))
            .and_then(Value::as_object_mut)
        else {
            panic!(
                "x-tempera declaration names {} {}, which Palette does not serve",
                contract.method.to_uppercase(),
                contract.path
            );
        };
        operation.insert("x-tempera-auth-kind".to_string(), json!("oauthResource"));
        operation.insert("x-tempera-auth-audience".to_string(), json!(AUTH_AUDIENCE));
        operation.insert(
            "x-tempera-required-scope".to_string(),
            json!(contract.scope),
        );
    }
}

fn declare_protocol_routes(document: &mut Value) {
    let Some(root) = document.as_object_mut() else {
        return;
    };
    root.insert(
        "x-tempera-protocol-routes".to_string(),
        Value::Array(PROTOCOL_ROUTES.iter().map(|route| json!(route)).collect()),
    );
}

#[cfg(test)]
mod contract_standard_tests {
    use super::*;
    use crate::openapi::openapi_value;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn object<'a>(value: &'a Value, what: &str) -> Result<&'a Map<String, Value>, String> {
        value
            .as_object()
            .ok_or_else(|| format!("expected {what} to be an object"))
    }

    /// A new route must not be able to ship without saying which credential it
    /// demands: the SDK generator would otherwise emit an unauthenticated
    /// client method for it.
    #[test]
    fn every_operation_is_declared() -> TestResult {
        let document = openapi_value();
        let mut undeclared = Vec::new();
        for (path, path_item) in object(&document["paths"], "paths")? {
            if is_protocol_route(path) {
                continue;
            }
            for method in object(path_item, path)?.keys() {
                if !HTTP_METHODS.contains(&method.as_str()) {
                    continue;
                }
                if !OPERATIONS
                    .iter()
                    .any(|entry| entry.method == method && entry.path == path)
                {
                    undeclared.push(format!("{} {path}", method.to_uppercase()));
                }
            }
        }
        assert!(
            undeclared.is_empty(),
            "these operations carry no x-tempera credential declaration; \
             add them to OPERATIONS in src/openapi/contract.rs: {undeclared:?}"
        );
        Ok(())
    }

    /// A declaration that names a route Palette does not serve is a lie the
    /// producer lint rejects, so catch it here first.
    #[test]
    fn declared_operations_and_protocol_routes_are_served() -> TestResult {
        let document = openapi_value();
        let paths = object(&document["paths"], "paths")?;
        for route in PROTOCOL_ROUTES {
            assert!(
                paths.contains_key(*route),
                "x-tempera-protocol-routes declares {route}, which Palette does not serve"
            );
        }
        for contract in OPERATIONS {
            assert!(
                paths
                    .get(contract.path)
                    .and_then(Value::as_object)
                    .is_some_and(|item| item.contains_key(contract.method)),
                "OPERATIONS declares {} {}, which Palette does not serve",
                contract.method.to_uppercase(),
                contract.path
            );
        }
        Ok(())
    }

    #[test]
    fn status_components_are_canonical() {
        let document = openapi_value();
        let canonical = status_components();
        assert_eq!(
            document["components"]["schemas"]["Status"],
            canonical["schemas"]["Status"]
        );
        assert_eq!(
            document["components"]["responses"]["Error"],
            canonical["responses"]["Error"]
        );
    }

    #[test]
    fn resource_errors_reference_the_shared_response() -> TestResult {
        let document = openapi_value();
        for (path, path_item) in object(&document["paths"], "paths")? {
            if is_protocol_route(path) {
                continue;
            }
            for (method, operation) in object(path_item, path)? {
                if !HTTP_METHODS.contains(&method.as_str()) {
                    continue;
                }
                let responses = object(&operation["responses"], path)?;
                let errors: Vec<_> = responses
                    .iter()
                    .filter(|(status, _)| !is_success(status))
                    .collect();
                assert!(
                    !errors.is_empty(),
                    "{} {path} documents no error response",
                    method.to_uppercase()
                );
                for (status, response) in errors {
                    assert_eq!(
                        response["$ref"],
                        "#/components/responses/Error",
                        "{} {path} {status} does not use the shared error response",
                        method.to_uppercase()
                    );
                }
            }
        }
        Ok(())
    }
}
