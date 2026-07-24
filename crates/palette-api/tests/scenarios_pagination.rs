//! AIP-158 contract coverage for the public scenarios collection.

use std::collections::BTreeSet;
use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use http::{Request, StatusCode};
use palette_api::{ApiState, router};
use palette_bus::InMemoryBus;
use palette_ingest::{IngestPolicy, IngestService};
use palette_scenarios::SqliteScenarioStore;
use palette_store_obj::FsArtifactStore;
use palette_store_sql::SqliteTraceStore;
use serde_json::{Value, json};
use tower::ServiceExt;

fn build_app() -> (Router, tempfile::TempDir) {
    let tempdir = tempfile::tempdir().unwrap_or_else(|err| panic!("{err}"));
    let artifacts = Arc::new(
        FsArtifactStore::new(tempdir.path().join("artifacts"))
            .unwrap_or_else(|err| panic!("{err}")),
    );
    let traces = Arc::new(SqliteTraceStore::in_memory().unwrap_or_else(|err| panic!("{err}")));
    let bus = Arc::new(InMemoryBus::new(32));
    let ingest = IngestService::new(artifacts, traces.clone(), bus, IngestPolicy::default());
    let scenarios =
        Arc::new(SqliteScenarioStore::in_memory().unwrap_or_else(|err| panic!("{err}")));
    (
        router(ApiState::new(ingest, traces).with_scenarios(scenarios)),
        tempdir,
    )
}

async fn send(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let request = match body {
        Some(payload) => Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap_or_else(|err| panic!("{err}")),
        None => Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .unwrap_or_else(|err| panic!("{err}")),
    };
    let response = app
        .clone()
        .oneshot(request)
        .await
        .unwrap_or_else(|err| panic!("{err}"));
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap_or_else(|err| panic!("{err}"));
    let value = serde_json::from_slice(&bytes).unwrap_or_else(|_| {
        Value::String(String::from_utf8(bytes.to_vec()).unwrap_or_else(|err| panic!("{err}")))
    });
    (status, value)
}

async fn create_scenario(app: &Router, suffix: &str) -> String {
    let (status, body) = send(
        app,
        "POST",
        "/v1/scenarios/tenant-a/project-a",
        Some(json!({
            "title": format!("scenario-{suffix}"),
            "failureMode": "wrong_output",
            "sourceTraceIds": [format!("trace-{suffix}")],
            "expectedOutcome": "pass"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "create response: {body}");
    body.pointer("/scenarioId")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing scenarioId: {body}"))
        .to_string()
}

fn first_scenario_id(body: &Value) -> String {
    body.pointer("/scenarios/0/scenarioId")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing paged scenario: {body}"))
        .to_string()
}

#[tokio::test]
async fn scenario_pages_use_opaque_request_bound_aip158_tokens() {
    let (app, _tempdir) = build_app();
    let created: BTreeSet<String> = [
        create_scenario(&app, "one").await,
        create_scenario(&app, "two").await,
        create_scenario(&app, "three").await,
    ]
    .into_iter()
    .collect();

    let (status, first) = send(
        &app,
        "GET",
        "/v1/scenarios/tenant-a/project-a?pageSize=1",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "first page: {first}");
    assert_eq!(
        first
            .pointer("/scenarios")
            .and_then(Value::as_array)
            .map(Vec::len),
        Some(1)
    );
    assert!(first.get("next_cursor").is_none());
    let first_id = first_scenario_id(&first);
    let first_token = first
        .pointer("/nextPageToken")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing nextPageToken: {first}"));
    assert!(first_token.starts_with("aip158_v1_"));
    assert_eq!(first_token.len(), "aip158_v1_".len() + 64);
    assert!(!first_token.contains(&first_id));

    let (status, second) = send(
        &app,
        "GET",
        &format!("/v1/scenarios/tenant-a/project-a?pageSize=1&pageToken={first_token}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "second page: {second}");
    let second_id = first_scenario_id(&second);
    let second_token = second
        .pointer("/nextPageToken")
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing second nextPageToken: {second}"));

    let (status, third) = send(
        &app,
        "GET",
        &format!("/v1/scenarios/tenant-a/project-a?pageSize=1&pageToken={second_token}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "third page: {third}");
    let third_id = first_scenario_id(&third);
    assert!(third.get("nextPageToken").is_none());
    let listed: BTreeSet<String> = [first_id, second_id, third_id].into_iter().collect();
    assert_eq!(listed, created);

    for uri in [
        format!("/v1/scenarios/tenant-a/project-a?pageSize=2&pageToken={first_token}"),
        format!("/v1/scenarios/tenant-a/project-b?pageSize=1&pageToken={first_token}"),
        "/v1/scenarios/tenant-a/project-a?pageSize=1&pageToken=not-a-token".to_string(),
        "/v1/scenarios/tenant-a/project-a?limit=1".to_string(),
    ] {
        let (status, _) = send(&app, "GET", &uri, None).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "request must fail: {uri}");
    }

    let (status, zero_uses_default) = send(
        &app,
        "GET",
        "/v1/scenarios/tenant-a/project-a?pageSize=0",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        zero_uses_default
            .pointer("/scenarios")
            .and_then(Value::as_array)
            .map(Vec::len),
        Some(3)
    );
    assert!(zero_uses_default.get("nextPageToken").is_none());
}

#[tokio::test]
async fn openapi_matches_the_scenario_pagination_wire_contract() {
    let (app, _tempdir) = build_app();
    let (status, spec) = send(&app, "GET", "/openapi.json", None).await;
    assert_eq!(status, StatusCode::OK);

    let operation = &spec["paths"]["/v1/scenarios/{tenantId}/{projectId}"]["get"];
    let parameter_names: BTreeSet<&str> = operation["parameters"]
        .as_array()
        .unwrap_or_else(|| panic!("scenario parameters must be an array"))
        .iter()
        .filter_map(|parameter| parameter["name"].as_str())
        .collect();
    assert!(parameter_names.contains("pageSize"));
    assert!(parameter_names.contains("pageToken"));
    assert!(!parameter_names.contains("limit"));
    assert!(!parameter_names.contains("cursor"));

    let response_properties = &spec["components"]["schemas"]["ListScenariosResponse"]["properties"];
    assert!(response_properties.get("scenarios").is_some());
    assert!(response_properties.get("nextPageToken").is_some());
    assert!(response_properties.get("next_cursor").is_none());
}
