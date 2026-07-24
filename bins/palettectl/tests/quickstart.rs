use std::process::Command;

#[test]
fn quickstart_reaches_scored_failure() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let output = Command::new(env!("CARGO_BIN_EXE_palettectl"))
        .arg("--base-url")
        .arg("http://127.0.0.1:8080")
        .arg("quickstart")
        .arg("--data-dir")
        .arg(tempdir.path())
        .arg("--dashboard-url")
        .arg("http://127.0.0.1:3000")
        .output()?;

    assert!(
        output.status.success(),
        "quickstart failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(stdout["command"], "quickstart");
    assert_eq!(stdout["mode"], "local");
    assert_eq!(stdout["source"], "native-smoke");
    assert_eq!(stdout["tenantId"], "demo");
    assert_eq!(stdout["projectId"], "demo");
    assert_eq!(stdout["environmentId"], "local");

    assert_eq!(stdout["trace"]["traceId"], "smoke-trace");
    assert_eq!(stdout["trace"]["spanId"], "smoke-root");
    assert_eq!(stdout["trace"]["spanCount"], 1);
    assert!(stdout["dataset"]["datasetId"].as_str().is_some());
    assert!(stdout["dataset"]["datasetVersionId"].as_str().is_some());
    assert!(stdout["dataset"]["caseId"].as_str().is_some());

    assert_eq!(stdout["scored_failure"], true);
    assert_eq!(stdout["eval"]["score"], 0.0);
    assert_eq!(stdout["eval"]["aggregateScore"], 0.0);
    assert_eq!(stdout["eval"]["label"], "fail");
    assert_eq!(stdout["eval"]["resultCount"], 1);
    assert_eq!(stdout["eval"]["evidence"]["metric"], "exact_match");
    assert_eq!(stdout["eval"]["evidence"]["pass"], false);

    assert_eq!(
        stdout["dashboard_url"],
        "http://127.0.0.1:3000/?tenant=demo&project=demo&environment=local&trace=smoke-trace&span=smoke-root"
    );
    assert_eq!(
        stdout["api_trace_url"],
        "http://127.0.0.1:8080/v1/traces/demo/smoke-trace?projectId=demo&environmentId=local"
    );
    assert_eq!(
        stdout["zero_code_env"]["OTEL_EXPORTER_OTLP_TRACES_ENDPOINT"],
        "http://127.0.0.1:8080/v1/otlp/demo/demo/local/v1/traces"
    );
    assert_eq!(
        stdout["zero_code_env"]["OTEL_EXPORTER_OTLP_HEADERS"],
        "x-palette-api-key=${PALETTE_API_KEY}"
    );
    assert_eq!(
        stdout["zero_code_env"]["PALETTE_API_KEY"],
        stdout["api_key"]["secret"]
    );
    assert!(
        stdout["api_key"]["secret"]
            .as_str()
            .is_some_and(|secret| !secret.is_empty()),
        "quickstart should provision an API key secret"
    );

    Ok(())
}
