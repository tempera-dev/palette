use std::collections::BTreeSet;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use beater_auth::{ApiKeyStore, CreateApiKeyRequest, SqliteApiKeyStore};
use beater_core::{EnvironmentId, ProjectId, TenantId};
use beater_security::ApiScope;
use serde_json::{json, Value};

fn run_beaterd_mcp_stdio(request: Value) -> anyhow::Result<Value> {
    let tempdir = tempfile::tempdir()?;
    run_beaterd_mcp_stdio_in(tempdir.path(), request, &[])
}

fn run_beaterd_mcp_stdio_in(
    data_dir: &Path,
    request: Value,
    envs: &[(&str, &str)],
) -> anyhow::Result<Value> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_beaterd"))
        .arg("--data-dir")
        .arg(data_dir)
        .arg("--bus-backend")
        .arg("memory")
        .arg("--trace-write-drain-interval-ms")
        .arg("0")
        .arg("--trace-ingested-drain-interval-ms")
        .arg("0")
        .arg("mcp")
        .arg("--stdio")
        .envs(envs.iter().copied())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("missing beaterd stdin"))?;
        writeln!(stdin, "{request}")?;
    }
    drop(child.stdin.take());

    let output = child.wait_with_output()?;
    assert!(
        output.status.success(),
        "beaterd mcp --stdio failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout)?;
    let mut lines = stdout.lines();
    let first = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("beaterd wrote no stdout"))?;
    assert!(
        lines.next().is_none(),
        "stdio smoke should emit exactly one JSON-RPC line, got:\n{stdout}"
    );

    serde_json::from_str(first).map_err(Into::into)
}

#[test]
fn beaterd_mcp_stdio_lists_tools() -> anyhow::Result<()> {
    let rpc = run_beaterd_mcp_stdio(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list",
        "params": {}
    }))?;
    assert_eq!(rpc["jsonrpc"], "2.0");
    assert_eq!(rpc["id"], 1);
    let tools = rpc["result"]["tools"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("tools/list result missing tools array: {rpc}"))?;
    assert_eq!(tools.len(), beater_mcp::tool_names().len() + 1);
    assert!(tools.iter().any(|tool| tool["name"] == "help"));

    Ok(())
}

#[test]
fn beaterd_mcp_stdio_calls_help_tool() -> anyhow::Result<()> {
    let rpc = run_beaterd_mcp_stdio(json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "help",
            "arguments": { "tool": "listTraces" }
        }
    }))?;

    assert_eq!(rpc["jsonrpc"], "2.0");
    assert_eq!(rpc["id"], 1);
    let result = &rpc["result"];
    assert_eq!(result["isError"], false, "help tool must not error: {rpc}");
    assert!(
        result["content"]
            .as_array()
            .is_some_and(|items| !items.is_empty()),
        "help tool should include text content: {rpc}"
    );

    let tool = &result["structuredContent"]["tool"];
    assert_eq!(tool["name"], "listTraces");
    assert_eq!(tool["method"], "GET");
    assert_eq!(tool["path"], "/v1/traces/{tenant_id}");
    assert!(
        tool["inputSchema"].is_object(),
        "help should describe listTraces input schema: {rpc}"
    );
    assert!(
        tool["outputSchema"].is_object(),
        "help should describe listTraces output schema: {rpc}"
    );

    Ok(())
}

/// e2e over the real binary in the default `--auth-mode required`: a real
/// (non-help) tool call over stdio must fail without credentials and succeed
/// when they are injected via the documented `BEATER_*` environment variables.
#[test]
fn beaterd_mcp_stdio_authenticates_real_tool_call_from_env() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;

    // Bootstrap an API key directly in the data dir, same as
    // `beaterctl api-key-create`.
    let store = SqliteApiKeyStore::open(tempdir.path().join("security.sqlite"))?;
    let created =
        tokio::runtime::Runtime::new()?.block_on(store.create_key(CreateApiKeyRequest {
            tenant_id: TenantId::new("tenant-1")?,
            project_id: ProjectId::new("proj-1")?,
            environment_id: EnvironmentId::new("env-1")?,
            scopes: BTreeSet::from([ApiScope::TraceRead]),
        }))?;

    let call = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "listTraces",
            "arguments": {
                "tenant_id": "tenant-1",
                "project_id": "proj-1",
                "environment_id": "env-1"
            }
        }
    });

    // Without credentials the call is dispatched but rejected by real auth.
    let rpc = run_beaterd_mcp_stdio_in(tempdir.path(), call.clone(), &[])?;
    assert_eq!(rpc["result"]["isError"], true, "expected 401: {rpc}");
    assert_eq!(rpc["result"]["_meta"]["httpStatus"], 401);

    // With BEATER_API_KEY + scope ids in the environment, the same call passes
    // the real `authorize()` path and returns the trace page.
    let rpc = run_beaterd_mcp_stdio_in(
        tempdir.path(),
        call,
        &[
            ("BEATER_API_KEY", created.secret.as_str()),
            ("BEATER_PROJECT_ID", "proj-1"),
            ("BEATER_ENVIRONMENT_ID", "env-1"),
        ],
    )?;
    assert_eq!(
        rpc["result"]["isError"], false,
        "env-credentialed stdio call must pass auth: {rpc}"
    );
    assert_eq!(rpc["result"]["_meta"]["httpStatus"], 200);
    assert!(
        rpc["result"]["structuredContent"]["items"].is_array(),
        "authenticated listTraces must return the trace page: {rpc}"
    );

    Ok(())
}
