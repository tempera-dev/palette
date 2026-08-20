//! OpenAPI 3.1 contract for the Palette API.
//!
//! This is the single source of truth for the multi-language SDK pipeline. Every
//! route registered in [`crate::router`] is documented here via `#[utoipa::path]`
//! annotations placed on the real handler functions in `lib.rs`, and every schema
//! is derived from the real request/response types (no hand-maintained mirrors).

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde_json::{Map, Value};
use utoipa::OpenApi;
use utoipa::openapi::extensions::Extensions;
use utoipa::openapi::security::{
    ApiKey, ApiKeyValue, AuthorizationCode, Flow, OAuth2, Scopes, SecurityRequirement,
    SecurityScheme,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Palette API",
        version = "0.1.0",
        description = "Agent observability, evaluation, gating, and human-review APIs for Palette"
    ),
    paths(
        crate::health,
        crate::ingest_native,
        crate::ingest_otlp_json_collector,
        crate::ingest_otlp_http,
        crate::import_source_route,
        crate::create_api_key_route,
        crate::revoke_api_key_route,
        crate::list_provider_secrets_route,
        crate::create_provider_secret_route,
        crate::revoke_provider_secret_route,
        crate::list_connectors_route,
        crate::list_connector_tools_route,
        crate::connector_skills_route,
        crate::connect_connector_route,
        crate::connector_status_route,
        crate::invoke_connector_tool_route,
        crate::run_judge_eval_route,
        crate::list_judge_ledger_route,
        crate::get_usage_summary_route,
        crate::get_palette_connect_status_route,
        crate::list_audit_events_route,
        crate::get_ingest_queue_status_route,
        crate::reconcile_trace_ingested_route,
        crate::replay_dead_letter_route,
        crate::drain_trace_writes_route,
        crate::drain_trace_ingested_route,
        crate::search_spans,
        crate::list_traces,
        crate::get_trace,
        crate::get_span_route,
        crate::get_span_io_route,
        crate::archive_trace,
        crate::query_archive_spans,
        crate::create_prompt_route,
        crate::list_prompts_route,
        crate::get_prompt_route,
        crate::add_prompt_version_route,
        crate::list_prompt_versions_route,
        crate::diff_prompt_versions_route,
        crate::create_dataset,
        crate::create_scenario,
        crate::list_scenarios,
        crate::get_scenario,
        crate::mine_scenarios,
        crate::promote_dataset_case,
        crate::create_dataset_version,
        crate::run_deterministic_dataset_eval,
        crate::run_judge_dataset_eval,
        crate::run_calibration_route,
        crate::run_deterministic_experiment_route,
        crate::run_judge_experiment_route,
        crate::import_tempera_bundle_route,
        crate::record_tempera_decision_route,
        crate::get_tempera_evidence_route,
        crate::create_gate_route,
        crate::run_gate_route,
        crate::create_review_queue_route,
        crate::list_review_tasks_route,
        crate::enqueue_review_task_from_trace_route,
        crate::submit_review_annotation_route,
        crate::promote_review_annotation_route,
        crate::decide_online_sampling,
        crate::evaluate_alert,
    ),
    tags(
        (name = "health", description = "Runtime health"),
        (name = "ingest", description = "Trace ingestion, queue, and durability"),
        (name = "traces", description = "Trace read APIs"),
        (name = "spans", description = "Span read APIs"),
        (name = "search", description = "Span search"),
        (name = "archive", description = "Trace archival and archived-span queries"),
        (name = "prompts", description = "Prompt registry, versions, and diffs"),
        (name = "datasets", description = "Datasets, cases, and versions"),
        (name = "scenarios", description = "Scenario mining and replay data engine"),
        (name = "evals", description = "Dataset evaluations"),
        (name = "experiments", description = "Baseline/candidate experiments"),
        (name = "evalResults", description = "Official external evaluation evidence"),
        (name = "calibrations", description = "Evaluator calibration runs"),
        (name = "gates", description = "Release gates"),
        (name = "judge", description = "Ad-hoc judge evaluation and ledger"),
        (name = "reviews", description = "Human review queues and annotations"),
        (name = "online", description = "Online sampling decisions"),
        (name = "alerts", description = "Alert policy evaluation"),
        (name = "usage", description = "Usage summaries"),
        (name = "connect", description = "Hosted product setup and readiness"),
        (name = "audit", description = "Audit events"),
        (name = "apiKeys", description = "API key management"),
        (name = "providerSecrets", description = "Provider secret management"),
        (name = "connectors", description = "Composio-backed third-party tool connectors")
    )
)]
pub struct PaletteApi;

/// Build the OpenAPI document, stamping the live crate version at runtime.
///
/// The utoipa derive requires a literal `version`, so we override it here with
/// the actual `CARGO_PKG_VERSION` to keep the spec in lockstep with the crate.
pub fn openapi() -> utoipa::openapi::OpenApi {
    let mut doc = PaletteApi::openapi();
    doc.info.version = env!("CARGO_PKG_VERSION").to_string();
    normalize_operation_ids(&mut doc);
    apply_auth_contract(&mut doc);
    doc
}

pub fn openapi_json_pretty() -> Result<String, serde_json::Error> {
    // `utoipa::Extensions` stores extension keys in a `HashMap`. Replace the
    // three generated auth entries with one marker before serialization, then
    // expand that marker in a fixed order. This keeps the established Utoipa
    // document/schema ordering (and therefore a reviewable diff) while making
    // exact-source receipts and regen checks byte-stable across processes.
    let mut doc = openapi();
    replace_auth_extensions_with_markers(&mut doc);
    expand_auth_extension_markers(&doc.to_pretty_json()?)
}

const AUTH_EXTENSION_MARKER: &str = "x-palette-auth-contract-marker";
const AUTH_MARKER_SEPARATOR: char = '\u{1f}';

fn replace_auth_extensions_with_markers(doc: &mut utoipa::openapi::OpenApi) {
    for item in doc.paths.paths.values_mut() {
        for operation in [
            item.get.as_mut(),
            item.put.as_mut(),
            item.post.as_mut(),
            item.delete.as_mut(),
            item.options.as_mut(),
            item.head.as_mut(),
            item.patch.as_mut(),
            item.trace.as_mut(),
        ]
        .into_iter()
        .flatten()
        {
            let extensions = operation.extensions.get_or_insert_with(Extensions::default);
            let Some(kind) = extensions
                .get("x-tempera-auth-kind")
                .and_then(Value::as_str)
            else {
                panic!("Palette operation is missing x-tempera-auth-kind");
            };
            let audience = extensions
                .get("x-tempera-auth-audience")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let scope = extensions
                .get("x-tempera-required-scope")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let marker = [kind, audience, scope].join(&AUTH_MARKER_SEPARATOR.to_string());
            extensions.clear();
            extensions.insert(AUTH_EXTENSION_MARKER.to_string(), Value::String(marker));
        }
    }
}

fn expand_auth_extension_markers(rendered: &str) -> Result<String, serde_json::Error> {
    let marker_prefix = format!("\"{AUTH_EXTENSION_MARKER}\": ");
    let mut expanded = String::with_capacity(rendered.len());
    let mut marker_count = 0_usize;
    for line in rendered.lines() {
        let trimmed = line.trim_start();
        let Some(encoded_with_comma) = trimmed.strip_prefix(&marker_prefix) else {
            expanded.push_str(line);
            expanded.push('\n');
            continue;
        };
        let has_comma = encoded_with_comma.ends_with(',');
        let encoded = encoded_with_comma
            .strip_suffix(',')
            .unwrap_or(encoded_with_comma);
        let marker: String = serde_json::from_str(encoded)?;
        let values = marker.split(AUTH_MARKER_SEPARATOR).collect::<Vec<_>>();
        if values.len() != 3 {
            panic!("invalid Palette auth extension marker");
        }
        let indent = &line[..line.len() - trimmed.len()];
        let kind = serde_json::to_string(values[0])?;
        let audience = if values[1].is_empty() {
            "null".to_string()
        } else {
            serde_json::to_string(values[1])?
        };
        let scope = if values[2].is_empty() {
            "null".to_string()
        } else {
            serde_json::to_string(values[2])?
        };
        expanded.push_str(&format!("{indent}\"x-tempera-auth-kind\": {kind},\n"));
        expanded.push_str(&format!(
            "{indent}\"x-tempera-auth-audience\": {audience},\n"
        ));
        expanded.push_str(&format!(
            "{indent}\"x-tempera-required-scope\": {scope}{}\n",
            if has_comma { "," } else { "" }
        ));
        marker_count += 1;
    }
    if marker_count != 63 {
        panic!("expected 63 Palette auth extension markers, found {marker_count}");
    }
    if !rendered.ends_with('\n') {
        expanded.pop();
    }
    Ok(expanded)
}

fn normalize_operation_ids(doc: &mut utoipa::openapi::OpenApi) {
    for item in doc.paths.paths.values_mut() {
        for operation in [
            item.get.as_mut(),
            item.put.as_mut(),
            item.post.as_mut(),
            item.delete.as_mut(),
            item.options.as_mut(),
            item.head.as_mut(),
            item.patch.as_mut(),
            item.trace.as_mut(),
        ]
        .into_iter()
        .flatten()
        {
            let Some(operation_id) = operation.operation_id.as_deref() else {
                continue;
            };
            if operation_id.contains('.') {
                continue;
            }
            let tag = operation
                .tags
                .as_ref()
                .and_then(|tags| tags.first())
                .map(String::as_str)
                .unwrap_or("api");
            // Stamp the collection from the resource tag and keep the handler's
            // lower-camel method verbatim, yielding the ecosystem AIP scheme
            // `{collection}.{method}` (tempera-api-style-guide.md §4).
            operation.operation_id = Some(format!("{}.{}", tag, operation_id));
        }
    }
}

/// Stamp the exact runtime authorization contract onto every generated operation.
///
/// Palette accepts either an Auth Hub OAuth bearer for the `palette` audience or
/// its product API key header. The runtime already enforces one exact
/// [`palette_security::ApiScope`] before doing handler work; keeping that mapping
/// in the canonical OpenAPI lets the organization SDK and caller-bound Workflows
/// enforce the same authority before making a network request. Unknown operations
/// deliberately fail generation instead of silently shipping without a scope.
fn apply_auth_contract(doc: &mut utoipa::openapi::OpenApi) {
    let components = doc.components.get_or_insert_default();
    components.add_security_scheme(
        "tempera_oauth",
        SecurityScheme::OAuth2(OAuth2::with_description(
            [Flow::AuthorizationCode(AuthorizationCode::new(
                "https://api.tempera.dev/oauth/authorize",
                "https://api.tempera.dev/oauth/token",
                palette_oauth_scopes(),
            ))],
            "Auth Hub OAuth bearer token for the `palette` resource audience.",
        )),
    );
    components.add_security_scheme(
        "palette_api_key",
        SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
            "x-palette-api-key",
            "Palette product API key scoped to one tenant, project, and environment.",
        ))),
    );

    for item in doc.paths.paths.values_mut() {
        for operation in [
            item.get.as_mut(),
            item.put.as_mut(),
            item.post.as_mut(),
            item.delete.as_mut(),
            item.options.as_mut(),
            item.head.as_mut(),
            item.patch.as_mut(),
            item.trace.as_mut(),
        ]
        .into_iter()
        .flatten()
        {
            let operation_id = operation.operation_id.as_deref().unwrap_or("<missing>");
            let extensions = operation.extensions.get_or_insert_with(Extensions::default);
            if operation_id == "health.check" {
                operation.security = Some(Vec::new());
                extensions.insert(
                    "x-tempera-auth-kind".to_string(),
                    Value::String("none".into()),
                );
                extensions.insert("x-tempera-auth-audience".to_string(), Value::Null);
                extensions.insert("x-tempera-required-scope".to_string(), Value::Null);
                continue;
            }

            let Some(scope) = operation_required_scope(operation_id) else {
                panic!("public Palette operation {operation_id:?} has no authorization scope");
            };
            operation.security = Some(vec![
                SecurityRequirement::new("tempera_oauth", [scope]),
                SecurityRequirement::new("palette_api_key", std::iter::empty::<String>()),
            ]);
            extensions.insert(
                "x-tempera-auth-kind".to_string(),
                Value::String("oauthResource".into()),
            );
            extensions.insert(
                "x-tempera-auth-audience".to_string(),
                Value::String("palette".into()),
            );
            extensions.insert(
                "x-tempera-required-scope".to_string(),
                Value::String(scope.into()),
            );
        }
    }
}

fn palette_oauth_scopes() -> Scopes {
    Scopes::from_iter([
        ("admin", "Administer Palette product resources."),
        ("dataset:read", "Read datasets and their versions."),
        (
            "dataset:write",
            "Create and update datasets, prompts, and reviews.",
        ),
        (
            "eval:run",
            "Run evaluations, experiments, gates, and judge operations.",
        ),
        (
            "pii:unmask",
            "Unmask sensitive trace data with an audited reason.",
        ),
        ("scenario:read", "Read and mine replay scenarios."),
        ("scenario:write", "Create replay scenarios."),
        (
            "trace:read",
            "Read traces, spans, search results, and derived state.",
        ),
        ("trace:write", "Ingest traces and source data."),
    ])
}

fn operation_required_scope(operation_id: &str) -> Option<&'static str> {
    match operation_id {
        "apiKeys.create"
        | "apiKeys.revoke"
        | "audit.list"
        | "connectors.connect"
        | "ingest.drainTraceIngested"
        | "ingest.drainTraceWrites"
        | "ingest.getQueueStatus"
        | "ingest.reconcileTrace"
        | "ingest.replayDeadLetter"
        | "providerSecrets.create"
        | "providerSecrets.list"
        | "providerSecrets.revoke"
        | "usage.getSummary" => Some("admin"),
        "datasets.create"
        | "datasets.createVersion"
        | "datasets.promoteCaseFromTrace"
        | "prompts.addVersion"
        | "prompts.create"
        | "reviews.createQueue"
        | "reviews.enqueueTaskFromTrace"
        | "reviews.listTasks"
        | "reviews.promoteAnnotation"
        | "reviews.submitAnnotation" => Some("dataset:write"),
        "calibrations.run"
        | "connectors.invokeTool"
        | "evalResults.getTemperaEvidence"
        | "evalResults.importTemperaBundle"
        | "evalResults.recordTemperaDecision"
        | "evals.runDeterministic"
        | "evals.runJudge"
        | "experiments.runDeterministic"
        | "experiments.runJudge"
        | "gates.create"
        | "gates.run"
        | "judge.evaluate"
        | "judge.listLedger" => Some("eval:run"),
        "scenarios.get" | "scenarios.list" | "scenarios.mine" => Some("scenario:read"),
        "scenarios.create" => Some("scenario:write"),
        "alerts.evaluate"
        | "archive.archiveTrace"
        | "archive.querySpans"
        | "connect.getStatus"
        | "connectors.getSkills"
        | "connectors.list"
        | "connectors.listTools"
        | "connectors.status"
        | "online.decideSampling"
        | "prompts.diffVersions"
        | "prompts.get"
        | "prompts.list"
        | "prompts.listVersions"
        | "search.spans"
        | "spans.get"
        | "spans.getIo"
        | "traces.get"
        | "traces.list" => Some("trace:read"),
        "ingest.importSource" | "ingest.native" | "ingest.otlp" | "ingest.otlpJsonCollector" => {
            Some("trace:write")
        }
        _ => None,
    }
}

/// Percent-encode a value for safe interpolation into a request path segment or
/// query-string parameter, escaping everything outside the RFC 3986 *unreserved*
/// set (`ALPHA / DIGIT / "-" / "_" / "." / "~"`).
///
/// Shared by the MCP server and CLI, which both turn a resolved spec operation
/// into a live API request by filling its path template and query string.
pub fn urlencode(value: &str) -> String {
    // RFC 3986 unreserved == NON_ALPHANUMERIC minus the four unreserved marks.
    const UNRESERVED: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    utf8_percent_encode(value, UNRESERVED).to_string()
}

/// HTTP methods that may carry an operation under a path item, per OpenAPI.
const OPERATION_METHODS: [&str; 7] = ["get", "put", "post", "delete", "options", "head", "patch"];

/// One operation discovered in the OpenAPI document, borrowing from the
/// serialized spec.
pub struct SpecOperation<'a> {
    /// The unique `operationId`.
    pub operation_id: &'a str,
    /// Lower-case HTTP method as it appears in the spec (e.g. `"get"`).
    pub method: &'a str,
    /// Path template, e.g. `/v1/traces/{tenantId}`.
    pub path: &'a str,
    /// The full Operation Object, for callers that need its parameters,
    /// request body, responses, etc.
    pub operation: &'a Map<String, Value>,
}

/// Enumerate every operation in a serialized OpenAPI document (`doc` is the
/// spec rendered to JSON, e.g. `serde_json::to_value(openapi())`), walking each
/// path item × HTTP method that carries an `operationId`.
///
/// This is the single source of truth for "how do we find operations in the
/// spec," shared by the MCP tool catalog and the CLI's operation resolver so
/// neither hand-rolls the paths × methods walk. Returns an empty list when the
/// document has no `paths` object.
pub fn operations(doc: &Value) -> Vec<SpecOperation<'_>> {
    let mut ops = Vec::new();
    let Some(paths) = doc.get("paths").and_then(Value::as_object) else {
        return ops;
    };
    for (path, item) in paths {
        let Some(item) = item.as_object() else {
            continue;
        };
        for method in OPERATION_METHODS {
            let Some(operation) = item.get(method).and_then(Value::as_object) else {
                continue;
            };
            let Some(operation_id) = operation.get("operationId").and_then(Value::as_str) else {
                continue;
            };
            ops.push(SpecOperation {
                operation_id,
                method,
                path,
                operation,
            });
        }
    }
    ops
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::json;

    use super::{operation_required_scope, operations, urlencode};

    #[test]
    fn passes_unreserved_and_escapes_the_rest() {
        // Unreserved set is left untouched.
        assert_eq!(urlencode("aZ09-_.~"), "aZ09-_.~");
        // Reserved/space/unicode are percent-escaped with uppercase hex.
        assert_eq!(urlencode("a b/c?d#e"), "a%20b%2Fc%3Fd%23e");
        assert_eq!(urlencode("café"), "caf%C3%A9");
    }

    #[test]
    fn enumerates_operations_from_the_live_spec() -> Result<(), serde_json::Error> {
        let doc = serde_json::to_value(super::openapi())?;
        let ops = operations(&doc);
        // Every advertised operation has an id, a known method, and a path.
        assert!(!ops.is_empty());
        assert!(
            ops.iter()
                .all(|op| !op.operation_id.is_empty() && op.path.starts_with('/'))
        );
        // operationIds are unique across the surface.
        let mut ids: Vec<&str> = ops.iter().map(|op| op.operation_id).collect();
        ids.sort_unstable();
        let unique = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), unique, "duplicate operationId in spec");
        Ok(())
    }

    #[test]
    fn every_non_health_operation_declares_exact_resource_authority()
    -> Result<(), serde_json::Error> {
        let doc = serde_json::to_value(super::openapi())?;
        let ops = operations(&doc);
        assert_eq!(ops.len(), 63, "unexpected public operation count");

        let oauth = &doc["components"]["securitySchemes"]["tempera_oauth"];
        assert_eq!(oauth["type"], "oauth2");
        let supported_scopes = oauth["flows"]["authorizationCode"]["scopes"]
            .as_object()
            .unwrap_or_else(|| panic!("tempera_oauth scopes are missing: {oauth}"));
        assert_eq!(
            supported_scopes
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([
                "admin",
                "dataset:read",
                "dataset:write",
                "eval:run",
                "pii:unmask",
                "scenario:read",
                "scenario:write",
                "trace:read",
                "trace:write",
            ])
        );
        let api_key = &doc["components"]["securitySchemes"]["palette_api_key"];
        assert_eq!(api_key["type"], "apiKey");
        assert_eq!(api_key["in"], "header");
        assert_eq!(api_key["name"], "x-palette-api-key");

        for op in ops {
            if op.operation_id == "health.check" {
                assert_eq!(op.operation["security"], json!([]));
                assert_eq!(op.operation["x-tempera-auth-kind"], "none");
                assert!(op.operation["x-tempera-auth-audience"].is_null());
                assert!(op.operation["x-tempera-required-scope"].is_null());
                continue;
            }

            let scope = operation_required_scope(op.operation_id)
                .unwrap_or_else(|| panic!("{} has no required-scope mapping", op.operation_id));
            assert!(supported_scopes.contains_key(scope));
            assert_eq!(op.operation["x-tempera-auth-kind"], "oauthResource");
            assert_eq!(op.operation["x-tempera-auth-audience"], "palette");
            assert_eq!(op.operation["x-tempera-required-scope"], scope);
            assert_eq!(
                op.operation["security"],
                json!([
                    {"tempera_oauth": [scope]},
                    {"palette_api_key": []},
                ]),
                "{} has the wrong security alternatives",
                op.operation_id
            );
        }
        Ok(())
    }
}
