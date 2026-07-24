//! End-to-end coverage for the `/v1/prompts` surface, driving the real
//! `palette-api` router with an `InMemoryPromptRegistry` wired into app state.
//!
//! This is the live consumer that makes `palette-prompts` a real feature: every
//! assertion goes through the same `router()` paletted serves.

use std::sync::Arc;

use axum::Router;
use axum::body::{Body, to_bytes};
use http::{Request, StatusCode};
use palette_api::{ApiState, router};
use palette_bus::InMemoryBus;
use palette_ingest::{IngestPolicy, IngestService};
use palette_prompts::InMemoryPromptRegistry;
use palette_store_obj::FsArtifactStore;
use palette_store_sql::SqliteTraceStore;
use serde_json::{Value, json};
use std::collections::BTreeSet;
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
    let prompts = Arc::new(InMemoryPromptRegistry::new());
    let state = ApiState::new(ingest, traces).with_prompts(prompts);
    (router(state), tempdir)
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
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or_else(|_| {
            Value::String(String::from_utf8(bytes.to_vec()).unwrap_or_else(|err| panic!("{err}")))
        })
    };
    (status, value)
}

fn str_field<'a>(value: &'a Value, pointer: &str) -> &'a str {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing string at {pointer} in {value}"))
}

async fn create_prompt(app: &Router, base: &str, name: &str) -> (String, String) {
    let (status, created) = send(
        app,
        "POST",
        base,
        Some(json!({
            "name": name,
            "template": {
                "body": format!("system\n{name}"),
                "variables": [],
                "tags": ["pagination"]
            }
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "create body: {created}");
    (
        str_field(&created, "/prompt/prompt_id").to_string(),
        str_field(&created, "/version/version_id").to_string(),
    )
}

#[tokio::test]
async fn prompts_lifecycle_create_version_list_and_diff() {
    let (app, _tempdir) = build_app();
    let base = "/v1/prompts/tenant-a/project-a";

    // create_prompt -> initial version 1
    let (status, created) = send(
        &app,
        "POST",
        base,
        Some(json!({
            "name": "answer-support-question",
            "description": "Support prompt",
            "template": {
                "body": "system\nanswer briefly",
                "variables": [{"name": "question", "required": true, "default": null, "description": null}],
                "tags": ["support"]
            },
            "created_by": "agent",
            "message": "initial"
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "create body: {created}");
    let prompt_id = str_field(&created, "/prompt/prompt_id").to_string();
    let version_one = str_field(&created, "/version/version_id").to_string();
    assert_eq!(created.pointer("/version/version_number"), Some(&json!(1)));

    // list_prompts shows the new prompt
    let (status, listed) = send(&app, "GET", base, None).await;
    assert_eq!(status, StatusCode::OK);
    let prompts = listed
        .pointer("/prompts")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("missing prompts array: {listed}"));
    assert_eq!(prompts.len(), 1);
    assert_eq!(str_field(&prompts[0], "/prompt_id"), prompt_id);

    // get_prompt by id
    let (status, prompt) = send(&app, "GET", &format!("{base}/{prompt_id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(str_field(&prompt, "/name"), "answer-support-question");

    // add_version -> version 2
    let (status, version_two_body) = send(
        &app,
        "POST",
        &format!("{base}/{prompt_id}/versions"),
        Some(json!({
            "template": {
                "body": "system\nanswer with detail",
                "variables": [],
                "tags": ["support"]
            },
            "message": "expand"
        })),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "add version body: {version_two_body}"
    );
    assert_eq!(version_two_body.pointer("/version_number"), Some(&json!(2)));
    let version_two = str_field(&version_two_body, "/version_id").to_string();

    // list_versions returns both, oldest-first
    let (status, versions_body) =
        send(&app, "GET", &format!("{base}/{prompt_id}/versions"), None).await;
    assert_eq!(status, StatusCode::OK);
    let versions = versions_body
        .pointer("/versions")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("missing versions array: {versions_body}"));
    assert_eq!(versions.len(), 2);

    // diff_versions reports the body change
    let (status, diff) = send(
        &app,
        "GET",
        &format!("{base}/{prompt_id}/diff?from={version_one}&to={version_two}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "diff body: {diff}");
    let lines = diff
        .pointer("/lines")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("missing diff lines: {diff}"));
    let texts: Vec<&str> = lines.iter().map(|line| str_field(line, "/text")).collect();
    assert!(texts.contains(&"answer briefly"), "diff texts: {texts:?}");
    assert!(
        texts.contains(&"answer with detail"),
        "diff texts: {texts:?}"
    );
}

#[tokio::test]
async fn prompt_and_version_lists_traverse_with_request_bound_aip158_tokens() {
    let (app, _tempdir) = build_app();
    let base = "/v1/prompts/tenant-a/project-a";
    let mut created_prompt_ids = BTreeSet::new();
    let mut first_prompt = None;
    for name in ["prompt-one", "prompt-two", "prompt-three"] {
        let created = create_prompt(&app, base, name).await;
        created_prompt_ids.insert(created.0.clone());
        first_prompt.get_or_insert(created);
    }

    let mut listed_prompt_ids = BTreeSet::new();
    let mut page_token = None;
    for page_index in 0..3 {
        let uri = match page_token.as_deref() {
            Some(token) => format!("{base}?pageSize=1&pageToken={token}"),
            None => format!("{base}?pageSize=1"),
        };
        let (status, body) = send(&app, "GET", &uri, None).await;
        assert_eq!(status, StatusCode::OK, "prompt page {page_index}: {body}");
        let prompts = body["prompts"]
            .as_array()
            .unwrap_or_else(|| panic!("prompt page array: {body}"));
        assert_eq!(prompts.len(), 1);
        listed_prompt_ids.insert(str_field(&prompts[0], "/prompt_id").to_string());
        page_token = body
            .get("nextPageToken")
            .and_then(Value::as_str)
            .map(str::to_string);
        if page_index < 2 {
            let token = page_token
                .as_deref()
                .unwrap_or_else(|| panic!("missing prompt page token: {body}"));
            assert!(token.starts_with("aip158_v1_"));
            assert!(!token.contains(str_field(&prompts[0], "/prompt_id")));
        } else {
            assert!(page_token.is_none(), "final prompt page: {body}");
        }
    }
    assert_eq!(listed_prompt_ids, created_prompt_ids);

    let (status, first_page) = send(&app, "GET", &format!("{base}?pageSize=1"), None).await;
    assert_eq!(status, StatusCode::OK);
    let prompt_token = str_field(&first_page, "/nextPageToken").to_string();
    let (first_prompt_id, initial_version_id) =
        first_prompt.unwrap_or_else(|| panic!("first prompt fixture"));
    for uri in [
        format!("{base}?pageSize=2&pageToken={prompt_token}"),
        format!("/v1/prompts/tenant-a/project-b?pageSize=1&pageToken={prompt_token}"),
        format!("{base}/{first_prompt_id}/versions?pageSize=1&pageToken={prompt_token}"),
        format!("{base}?pageSize=1&pageToken=not-a-token"),
        format!("{base}?limit=1"),
        format!("{base}?cursor=legacy"),
        format!("{base}?page_size=1"),
        format!("{base}?unknown=value"),
    ] {
        let (status, _) = send(&app, "GET", &uri, None).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "request must fail: {uri}");
    }

    let versions_base = format!("{base}/{first_prompt_id}/versions");
    let mut created_version_ids = BTreeSet::from([initial_version_id]);
    for number in [2, 3] {
        let (status, version) = send(
            &app,
            "POST",
            &versions_base,
            Some(json!({
                "template": {
                    "body": format!("version {number}"),
                    "variables": [],
                    "tags": ["pagination"]
                }
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "add version: {version}");
        created_version_ids.insert(str_field(&version, "/version_id").to_string());
    }

    let mut listed_version_ids = BTreeSet::new();
    let mut version_token = None;
    for page_index in 0..3 {
        let uri = match version_token.as_deref() {
            Some(token) => format!("{versions_base}?pageSize=1&pageToken={token}"),
            None => format!("{versions_base}?pageSize=1"),
        };
        let (status, body) = send(&app, "GET", &uri, None).await;
        assert_eq!(status, StatusCode::OK, "version page {page_index}: {body}");
        let versions = body["versions"]
            .as_array()
            .unwrap_or_else(|| panic!("versions page array: {body}"));
        assert_eq!(versions.len(), 1);
        listed_version_ids.insert(str_field(&versions[0], "/version_id").to_string());
        version_token = body
            .get("nextPageToken")
            .and_then(Value::as_str)
            .map(str::to_string);
        if page_index < 2 {
            assert!(version_token.is_some(), "missing version token: {body}");
        } else {
            assert!(version_token.is_none(), "final version page: {body}");
        }
    }
    assert_eq!(listed_version_ids, created_version_ids);
}

#[tokio::test]
async fn get_unknown_prompt_returns_404() {
    let (app, _tempdir) = build_app();
    let (status, body) = send(
        &app,
        "GET",
        "/v1/prompts/tenant-a/project-a/prompt_does_not_exist",
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "body: {body}");
    assert_eq!(body.pointer("/error/code"), Some(&json!(404)));
    assert_eq!(body.pointer("/error/status"), Some(&json!("NOT_FOUND")));
    assert!(
        body.pointer("/error/message")
            .and_then(|value| value.as_str())
            .is_some()
    );
}

#[tokio::test]
async fn add_version_to_unknown_prompt_returns_404() {
    let (app, _tempdir) = build_app();
    let (status, body) = send(
        &app,
        "POST",
        "/v1/prompts/tenant-a/project-a/prompt_missing/versions",
        Some(json!({"template": {"body": "x", "variables": [], "tags": []}})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND, "body: {body}");
}
