use std::process::Command;

#[test]
fn smoke_fixture_emits_otlp_trace_and_drains_runtime_loop() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let output = Command::new(env!("CARGO_BIN_EXE_palettectl"))
        .arg("smoke")
        .arg("--data-dir")
        .arg(tempdir.path())
        .output()?;

    assert!(
        output.status.success(),
        "smoke stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains(r#""source": "otlp""#));
    assert!(stdout.contains(r#""accepted_spans": 1"#));
    assert!(stdout.contains(r#""trace_span_count": 1"#));
    assert!(stdout.contains(r#""normalizer_version": "palette-otlp-v1""#));
    assert!(stdout.contains(r#""completed": 1"#));
    Ok(())
}
