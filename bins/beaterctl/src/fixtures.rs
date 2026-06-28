//! Shared local fixture/demo helpers for `beaterctl` (issue #207).
//!
//! The fixture commands in `main.rs` repeated the same
//! "smoke trace -> dataset -> dataset version" setup before each command's
//! own eval/experiment/review logic. This module owns those stable building
//! blocks — the demo scope, the OTLP smoke-export builders, and the
//! smoke-to-dataset-version setup — while command-specific OUTPUT and ERROR
//! text stays in the command handlers in `main.rs`.
//!
//! Note on `demo_scope()`: `beaterd` and `beaterctl` are separate one-binary
//! crates. Rather than introduce a shared workspace crate just for a 3-line
//! demo scope (which would break the all-in-one operational simplicity), we
//! keep a tiny local `demo_scope()` here. `beater_ingest::smoke_trace` already
//! bakes the same demo scope in for the ingest path; this helper exists for the
//! read-back / dataset side that needs the tenant + project ids explicitly.

use anyhow::Context;
use std::path::Path;
use std::sync::Arc;

use beater_core::{EnvironmentId, ProjectId, SpanId, TenantId, TenantScope, TraceId};
use beater_datasets::{
    promote_trace_span_to_case, DatasetStore, DatasetVersionSnapshot, SqliteDatasetStore,
};
use beater_ingest::{smoke_trace, IngestPolicy, IngestService};
use beater_otlp::{encode_export_trace_request, export_to_raw_trace_ingest_request};
use beater_schema::TraceView;
use beater_store::TraceStore;
use beater_store_obj::FsArtifactStore;
use beater_store_sql::SqliteTraceStore;
use chrono::Utc;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::common::v1::{any_value, AnyValue, InstrumentationScope, KeyValue};
use opentelemetry_proto::tonic::resource::v1::Resource;
use opentelemetry_proto::tonic::trace::v1::{
    span, status, ResourceSpans, ScopeSpans, Span, Status,
};

/// The demo tenant/project/environment scope used by local fixtures and the
/// smoke trace. Matches the ids baked into `beater_ingest::smoke_trace`
/// (tenant=demo, project=demo, environment=local).
pub fn demo_scope() -> anyhow::Result<TenantScope> {
    Ok(TenantScope::new(
        TenantId::new("demo")?,
        ProjectId::new("demo")?,
        EnvironmentId::new("local")?,
    ))
}

/// The deterministic ids the smoke trace lands under: trace id `smoke-trace`,
/// root span id `smoke-root` (as produced by `beater_ingest::smoke_trace`).
pub fn smoke_trace_id() -> anyhow::Result<TraceId> {
    Ok(TraceId::new("smoke-trace")?)
}

/// The deterministic root span id of the smoke trace.
pub fn smoke_span_id() -> anyhow::Result<SpanId> {
    Ok(SpanId::new("smoke-root")?)
}

/// Open the local artifact + trace + dataset stores under `data_dir` using the
/// same file names the rest of `beaterctl` uses, returning the trace store and
/// dataset store handles after running the smoke trace through a fresh
/// [`IngestService`].
///
/// This is the shared prefix of the eval/experiment/judge fixture commands:
/// ingest the deterministic smoke trace, then read it back so it can be
/// promoted into a dataset case.
pub struct SmokeDatasetVersion {
    pub datasets: SqliteDatasetStore,
    pub tenant: TenantId,
    pub project: ProjectId,
    /// The promoted dataset case (already stored via `put_case`).
    pub case: beater_datasets::DatasetCase,
    /// The created dataset version.
    pub version: DatasetVersionSnapshot,
    /// The read-back smoke trace (one span: the promoted root). Exposed for
    /// callers/tests that assert on the promoted span; not all fixtures use it.
    #[allow(dead_code)]
    pub trace: TraceView,
}

/// Run the smoke trace and promote its root span into a one-case dataset
/// version named `dataset_name`. `reference` is the expected output stored on
/// the promoted case. The `data_dir` store layout matches the existing
/// fixtures exactly (artifacts/, traces.sqlite, datasets.sqlite, bus.sqlite).
pub async fn smoke_dataset_version(
    data_dir: &Path,
    dataset_name: &str,
    reference: serde_json::Value,
) -> anyhow::Result<SmokeDatasetVersion> {
    let artifacts = Arc::new(FsArtifactStore::new(data_dir.join("artifacts"))?);
    let traces = Arc::new(SqliteTraceStore::open(data_dir.join("traces.sqlite"))?);
    let datasets = SqliteDatasetStore::open(data_dir.join("datasets.sqlite"))?;
    let bus = Arc::new(beater_bus::SqliteDurableBus::open(
        data_dir.join("bus.sqlite"),
        128,
    )?);
    let service = IngestService::new(artifacts, traces.clone(), bus, IngestPolicy::default());
    smoke_trace(&service).await.context("run smoke trace")?;

    let tenant = TenantId::new("demo")?;
    let project = ProjectId::new("demo")?;
    let trace = traces
        .get_trace(tenant.clone(), smoke_trace_id()?)
        .await
        .context("read smoke trace")?;
    let dataset = datasets
        .create_dataset(tenant.clone(), project.clone(), dataset_name.to_string())
        .await
        .context("create fixture dataset")?;
    let case = promote_trace_span_to_case(
        tenant.clone(),
        project.clone(),
        dataset.dataset_id.clone(),
        &trace,
        Some(smoke_span_id()?),
        Some(reference),
    )
    .context("promote smoke trace to dataset case")?;
    let case = datasets
        .put_case(case)
        .await
        .context("store fixture dataset case")?;
    let version = datasets
        .create_version(tenant.clone(), project.clone(), dataset.dataset_id, None)
        .await
        .context("create fixture dataset version")?;

    Ok(SmokeDatasetVersion {
        datasets,
        tenant,
        project,
        case,
        version,
        trace,
    })
}

/// Deterministic-ish OTLP smoke trace + span ids derived from the current
/// timestamp (used by the local and remote smoke paths).
pub fn smoke_ids() -> ([u8; 16], [u8; 8]) {
    let now = Utc::now().timestamp_nanos_opt().unwrap_or_default() as u128;
    let trace = now.to_be_bytes();
    let span = (now as u64).to_be_bytes();
    (trace, span)
}

/// Build the canonical `beaterctl` OTLP smoke export for the given ids.
pub fn otlp_smoke_export(trace_id: [u8; 16], span_id: [u8; 8]) -> ExportTraceServiceRequest {
    ExportTraceServiceRequest {
        resource_spans: vec![ResourceSpans {
            resource: Some(Resource {
                attributes: vec![otel_kv("service.name", otel_string("beaterctl-smoke"))],
                dropped_attributes_count: 0,
                entity_refs: Vec::new(),
            }),
            scope_spans: vec![ScopeSpans {
                scope: Some(InstrumentationScope {
                    name: "beaterctl".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    attributes: Vec::new(),
                    dropped_attributes_count: 0,
                }),
                spans: vec![Span {
                    trace_id: trace_id.to_vec(),
                    span_id: span_id.to_vec(),
                    trace_state: String::new(),
                    parent_span_id: Vec::new(),
                    flags: 0,
                    name: "beaterctl otlp smoke".to_string(),
                    kind: span::SpanKind::Client as i32,
                    start_time_unix_nano: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    end_time_unix_nano: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
                    attributes: vec![
                        otel_kv("openinference.span.kind", otel_string("llm")),
                        otel_kv("input.value", otel_string("hello")),
                        otel_kv("output.value", otel_string("world")),
                    ],
                    dropped_attributes_count: 0,
                    events: Vec::new(),
                    dropped_events_count: 0,
                    links: Vec::new(),
                    dropped_links_count: 0,
                    status: Some(Status {
                        message: String::new(),
                        code: status::StatusCode::Ok as i32,
                    }),
                }],
                schema_url: "https://opentelemetry.io/schemas/1.37.0".to_string(),
            }],
            schema_url: "https://opentelemetry.io/schemas/1.37.0".to_string(),
        }],
    }
}

fn otel_kv(key: &str, value: AnyValue) -> KeyValue {
    KeyValue {
        key: key.to_string(),
        key_strindex: 0,
        value: Some(value),
    }
}

fn otel_string(value: &str) -> AnyValue {
    AnyValue {
        value: Some(any_value::Value::StringValue(value.to_string())),
    }
}

/// Encode the smoke export to raw OTLP request bytes and the canonical raw
/// trace ingest request, returning both (used by the local smoke path).
pub fn smoke_raw_ingest_request(
    scope: TenantScope,
) -> anyhow::Result<beater_ingest::RawTraceIngestRequest> {
    let (trace_bytes, span_bytes) = smoke_ids();
    let export = otlp_smoke_export(trace_bytes, span_bytes);
    let raw_bytes = encode_export_trace_request(&export);
    export_to_raw_trace_ingest_request(
        scope,
        raw_bytes,
        export,
        beater_ingest::anonymous_auth_context(),
    )
    .context("build OTLP smoke ingest request")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn demo_scope_matches_smoke_trace_ids() {
        let scope = demo_scope().expect("demo scope");
        assert_eq!(scope.tenant_id.as_str(), "demo");
        assert_eq!(scope.project_id.as_str(), "demo");
        assert_eq!(scope.environment_id.as_str(), "local");
        assert_eq!(smoke_trace_id().unwrap().as_str(), "smoke-trace");
        assert_eq!(smoke_span_id().unwrap().as_str(), "smoke-root");
    }

    #[test]
    fn otlp_smoke_export_carries_one_span_with_smoke_ids() {
        let (trace, span) = smoke_ids();
        let export = otlp_smoke_export(trace, span);
        let spans = &export.resource_spans[0].scope_spans[0].spans;
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].trace_id, trace.to_vec());
        assert_eq!(spans[0].span_id, span.to_vec());
        assert_eq!(spans[0].name, "beaterctl otlp smoke");
    }

    #[tokio::test]
    async fn smoke_dataset_version_produces_one_case_version() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let setup = smoke_dataset_version(
            tempdir.path(),
            "unit-test-fixture",
            json!({ "answer": "world" }),
        )
        .await
        .expect("smoke dataset version");
        assert_eq!(setup.tenant.as_str(), "demo");
        assert_eq!(setup.project.as_str(), "demo");
        // The smoke trace has the deterministic root span we promoted.
        assert_eq!(setup.trace.spans.len(), 1);
        // The dataset version is built from the single promoted case.
        assert_eq!(setup.case.source_span_id.as_str(), "smoke-root");
    }
}
