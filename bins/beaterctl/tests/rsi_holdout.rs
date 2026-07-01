//! Binary-level E2E for `beaterctl rsi-holdout-fixture`: the held-out RSI gate
//! run over the REAL dataset substrate — ingest → promote → content-addressed
//! version → content-keyed split → `gate_candidate_on_holdout` — with both
//! decision paths proven (generalizing accepted, overfit rejected).
use std::process::Command;

#[test]
fn rsi_holdout_fixture_gates_on_the_real_dataset_substrate() -> anyhow::Result<()> {
    let data_dir = tempfile::tempdir()?;
    let fixture = Command::new(env!("CARGO_BIN_EXE_beaterctl"))
        .arg("rsi-holdout-fixture")
        .arg("--data-dir")
        .arg(data_dir.path())
        .output()?;
    assert!(
        fixture.status.success(),
        "fixture stderr: {}",
        String::from_utf8_lossy(&fixture.stderr)
    );

    let report: serde_json::Value = serde_json::from_slice(&fixture.stdout)?;

    // The dataset version is real: content-addressed corpus root over the
    // promoted cases, all 120 ingested traces present.
    assert_eq!(report["cases"], 120);
    let corpus_root = report["corpus_root"].as_str().expect("corpus_root");
    assert_eq!(corpus_root.len(), 64, "corpus root is a sha256 hex digest");

    let decisions = report["decisions"].as_array().expect("decisions");
    assert_eq!(decisions.len(), 2);

    let generalizing = &decisions[0];
    assert_eq!(generalizing["candidate"], "generalizing");
    assert_eq!(generalizing["accepted"], true);
    assert_eq!(generalizing["test_gate_decision"], "pass");
    assert_eq!(generalizing["generalization_gap"]["overfit"], false);

    let overfit = &decisions[1];
    assert_eq!(overfit["candidate"], "overfit");
    assert_eq!(overfit["accepted"], false);
    // The held-out gate alone is fooled (no regression on Test); only the
    // generalization-gap guardrail catches the overfit. Assert exactly that.
    assert_eq!(overfit["generalization_gap"]["overfit"], true);
    assert!(
        overfit["generalization_gap"]["optimize_lift"]
            .as_f64()
            .expect("optimize_lift")
            > 0.5
    );
    assert!(
        overfit["generalization_gap"]["holdout_lift"]
            .as_f64()
            .expect("holdout_lift")
            .abs()
            < 0.1
    );

    // Splits are content-keyed and powered: every case lands in exactly one
    // split and the held-out set clears the gate's min sample size.
    let splits = &generalizing["splits"];
    let (train, val, test) = (
        splits["train"].as_u64().expect("train"),
        splits["val"].as_u64().expect("val"),
        splits["test"].as_u64().expect("test"),
    );
    assert_eq!(train + val + test, 120);
    assert!(test >= 10, "held-out split must be powered, got {test}");

    Ok(())
}
