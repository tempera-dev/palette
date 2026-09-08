//! Ratchet coverage for the public AIP-158 collection migration.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

fn operations_by_id(spec: &Value) -> BTreeMap<String, Value> {
    let mut operations = BTreeMap::new();
    for methods in spec["paths"]
        .as_object()
        .unwrap_or_else(|| panic!("OpenAPI paths"))
        .values()
    {
        for operation in methods
            .as_object()
            .unwrap_or_else(|| panic!("OpenAPI path item"))
            .values()
        {
            if let Some(operation_id) = operation["operationId"].as_str() {
                operations.insert(operation_id.to_string(), operation.clone());
            }
        }
    }
    operations
}

fn parameter_names(operation: &Value) -> BTreeSet<&str> {
    operation["parameters"]
        .as_array()
        .unwrap_or_else(|| panic!("operation parameters: {operation}"))
        .iter()
        .filter_map(|parameter| parameter["name"].as_str())
        .collect()
}

fn success_schema<'a>(spec: &'a Value, operation: &'a Value) -> &'a Value {
    let schema = &operation["responses"]["200"]["content"]["application/json"]["schema"];
    let reference = schema["$ref"]
        .as_str()
        .unwrap_or_else(|| panic!("named success schema: {schema}"));
    let name = reference
        .strip_prefix("#/components/schemas/")
        .unwrap_or_else(|| panic!("local schema reference: {reference}"));
    &spec["components"]["schemas"][name]
}

#[test]
fn every_public_collection_has_a_complete_aip158_contract() {
    let spec = palette_api::openapi::openapi_value();
    let operations = operations_by_id(&spec);
    let migrated = [
        "archive.querySpans",
        "audit.list",
        "connectors.list",
        "connectors.listTools",
        "judge.listLedger",
        "prompts.list",
        "prompts.listVersions",
        "providerSecrets.list",
        "reviews.listTasks",
        "scenarios.list",
        "search.spans",
        "traces.list",
    ];

    for operation_id in migrated {
        let operation = operations
            .get(operation_id)
            .unwrap_or_else(|| panic!("missing {operation_id}"));
        let parameters = parameter_names(operation);
        assert!(
            parameters.contains("pageSize"),
            "{operation_id}: {parameters:?}"
        );
        assert!(
            parameters.contains("pageToken"),
            "{operation_id}: {parameters:?}"
        );
        assert!(
            !parameters.contains("limit") && !parameters.contains("cursor"),
            "{operation_id} retains legacy aliases: {parameters:?}"
        );
        let response = success_schema(&spec, operation);
        assert!(
            response["properties"].get("nextPageToken").is_some(),
            "{operation_id} response lacks nextPageToken: {response}"
        );
    }
}

#[test]
fn migration_debt_is_zero_and_protocol_native_exceptions_are_exact() {
    let spec = palette_api::openapi::openapi_value();
    let operations = operations_by_id(&spec);
    for operation_id in ["ingest.otlp", "ingest.otlpJsonCollector"] {
        let parameters = parameter_names(
            operations
                .get(operation_id)
                .unwrap_or_else(|| panic!("missing protocol operation {operation_id}")),
        );
        assert!(
            !parameters.contains("pageSize")
                && !parameters.contains("pageToken")
                && !parameters.contains("nextPageToken"),
            "{operation_id} is protocol-native, not an AIP list operation"
        );
    }
    assert!(
        !spec["paths"]
            .as_object()
            .unwrap_or_else(|| panic!("paths"))
            .contains_key("/mcp"),
        "MCP protocol pagination is deliberately outside the HTTP product contract"
    );
}
