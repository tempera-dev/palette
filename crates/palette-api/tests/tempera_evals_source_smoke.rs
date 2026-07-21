//! Exact-source smoke for the signed Tempera Evals evidence handoff.
//!
//! The normal Palette test matrix deliberately does not need another checkout.
//! When `TEMPERA_EVALS_HANDOFF` names an ephemeral request generated from a
//! pinned Tempera Evals checkout, this test proves the real router accepts its
//! canonical bytes, preserves idempotency, and does not leak the receipt across
//! tenant scope. The fixture is protocol-only; it is not a capability claim.

use std::env;
use std::sync::Arc;

use axum::body::{Body, to_bytes};
use http::{Request, StatusCode};
use palette_api::{ApiState, router};
use palette_bus::InMemoryBus;
use palette_experiments::SqliteExperimentStore;
use palette_ingest::{IngestPolicy, IngestService};
use palette_store_obj::FsArtifactStore;
use palette_store_sql::SqliteTraceStore;
use serde_json::Value;
use tower::ServiceExt;

fn handoff_fixture() -> Option<Value> {
    let path = env::var_os("TEMPERA_EVALS_HANDOFF")?;
    let metadata = std::fs::symlink_metadata(&path)
        .unwrap_or_else(|error| panic!("read Tempera Evals handoff metadata: {error}"));
    assert!(
        metadata.file_type().is_file(),
        "handoff must be a regular file"
    );
    assert!(metadata.len() <= 1_048_576, "handoff must remain bounded");
    let bytes = std::fs::read(path)
        .unwrap_or_else(|error| panic!("read Tempera Evals handoff fixture: {error}"));
    Some(
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|error| panic!("parse Tempera Evals handoff fixture: {error}")),
    )
}

#[tokio::test]
async fn exact_tempera_evals_handoff_is_accepted_replay_safe_and_tenant_scoped() {
    let Some(handoff) = handoff_fixture() else {
        return;
    };
    let request = handoff
        .get("request")
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("handoff request must be an object"));
    assert_eq!(request.get("method").and_then(Value::as_str), Some("POST"));
    assert_eq!(
        request.get("credential_included").and_then(Value::as_bool),
        Some(false),
        "the cross-repository fixture must never carry credentials"
    );
    let path = request
        .get("path")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("handoff request path must be a string"));
    let body = request
        .get("body")
        .cloned()
        .unwrap_or_else(|| panic!("handoff request body must be present"));
    let evidence = handoff
        .get("evidence")
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("handoff evidence must be an object"));
    let trusted_key = evidence
        .get("public_key_sha256")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("handoff must identify a release public key"))
        .to_owned();
    let external_id = evidence
        .get("external_id")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("handoff must identify external evidence"));

    let temporary = tempfile::tempdir().unwrap_or_else(|error| panic!("tempdir: {error}"));
    let artifacts = Arc::new(
        FsArtifactStore::new(temporary.path().join("artifacts"))
            .unwrap_or_else(|error| panic!("artifact store: {error}")),
    );
    let traces = Arc::new(
        SqliteTraceStore::in_memory().unwrap_or_else(|error| panic!("trace store: {error}")),
    );
    let ingest = IngestService::new(
        artifacts,
        traces.clone(),
        Arc::new(InMemoryBus::new(16)),
        IngestPolicy::default(),
    );
    let experiments = Arc::new(
        SqliteExperimentStore::in_memory()
            .unwrap_or_else(|error| panic!("experiment store: {error}")),
    );
    let app = router(
        ApiState::new(ingest, traces)
            .with_experiments(experiments)
            .with_tempera_evidence_trusted_keys([trusted_key])
            .unwrap_or_else(|error| panic!("trusted release key: {error}")),
    );
    let encoded = serde_json::to_vec(&body)
        .unwrap_or_else(|error| panic!("serialize handoff request body: {error}"));

    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(encoded.clone()))
                .unwrap_or_else(|error| panic!("build import request: {error}")),
        )
        .await
        .unwrap_or_else(|error| panic!("route import request: {error}"));
    assert_eq!(first.status(), StatusCode::OK);
    let first_body = to_bytes(first.into_body(), 1_048_576)
        .await
        .unwrap_or_else(|error| panic!("read import receipt: {error}"));
    let first_receipt: Value = serde_json::from_slice(&first_body)
        .unwrap_or_else(|error| panic!("parse import receipt: {error}"));
    assert_eq!(first_receipt["created"], true);
    assert_eq!(first_receipt["external_id"], external_id);
    assert_eq!(
        first_receipt["declared_content_sha256"],
        evidence["declared_content_sha256"]
    );

    let replay = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(encoded))
                .unwrap_or_else(|error| panic!("build replay request: {error}")),
        )
        .await
        .unwrap_or_else(|error| panic!("route replay request: {error}"));
    assert_eq!(replay.status(), StatusCode::OK);
    let replay_body = to_bytes(replay.into_body(), 1_048_576)
        .await
        .unwrap_or_else(|error| panic!("read replay receipt: {error}"));
    let replay_receipt: Value = serde_json::from_slice(&replay_body)
        .unwrap_or_else(|error| panic!("parse replay receipt: {error}"));
    assert_eq!(replay_receipt["created"], false);
    assert_eq!(replay_receipt["stored_at"], first_receipt["stored_at"]);

    let receipt_path =
        format!("/v1/eval-results/tenant.alpha/project.coding/tempera/result_bundle/{external_id}");
    let receipt = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&receipt_path)
                .body(Body::empty())
                .unwrap_or_else(|error| panic!("build receipt request: {error}")),
        )
        .await
        .unwrap_or_else(|error| panic!("route receipt request: {error}"));
    assert_eq!(receipt.status(), StatusCode::OK);

    let foreign_path = receipt_path.replacen("tenant.alpha", "tenant.other", 1);
    let foreign = app
        .oneshot(
            Request::builder()
                .uri(foreign_path)
                .body(Body::empty())
                .unwrap_or_else(|error| panic!("build foreign receipt request: {error}")),
        )
        .await
        .unwrap_or_else(|error| panic!("route foreign receipt request: {error}"));
    assert_eq!(foreign.status(), StatusCode::NOT_FOUND);
}
