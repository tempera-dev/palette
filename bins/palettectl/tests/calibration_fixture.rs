use std::process::Command;

#[test]
fn calibration_fixture_persists_judge_human_agreement_report() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let output = Command::new(env!("CARGO_BIN_EXE_palettectl"))
        .arg("calibration-fixture")
        .arg("--data-dir")
        .arg(tempdir.path())
        .output()?;

    assert!(
        output.status.success(),
        "calibration fixture stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains(r#""sampleCount": 4"#));
    assert!(stdout.contains(r#""cohenKappa": 0.5"#));
    assert!(stdout.contains(r#""brierScore": 0.140625"#));
    assert!(stdout.contains(r#""expectedCalibrationError": 0.1875"#));
    assert!(stdout.contains(r#""humanFailJudgePass": 1"#));
    assert!(stdout.contains(r#""binIndex": 7"#));
    assert!(stdout.contains(r#""calibrationGap": 0.75"#));
    assert!(!stdout.contains("sk-local-calibration-judge-secret"));
    Ok(())
}
