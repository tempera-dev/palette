use palette_audit::SqliteAuditStore;
use palette_auth::SqliteApiKeyStore;
use palette_bus::SqliteDurableBus;
use palette_calibration::SqliteCalibrationStore;
use palette_datasets::SqliteDatasetStore;
use palette_experiments::SqliteExperimentStore;
use palette_gates::SqliteGateStore;
use palette_human::SqliteHumanReviewStore;
use palette_judge::SqliteJudgeLedger;
use palette_replay::SqliteReplayStore;
use palette_secrets::{EncryptedSqliteProviderSecretStore, SecretKeyring};
use palette_store_sql::{
    SqliteMetadataStore, SqliteQuotaLimiter, SqliteTraceStore, migrate_local_paletted_sqlite,
};
use palette_usage::SqliteUsageLedger;

#[test]
fn local_sqlite_migration_bootstraps_runtime_store_schemas() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let path = tempdir.path().join("palette.sqlite");

    let report = migrate_local_paletted_sqlite(&path)?;
    assert_eq!(report.applied, 1);
    assert_eq!(report.skipped, 0);

    let _traces = SqliteTraceStore::open(&path)?;
    let _metadata = SqliteMetadataStore::open(&path)?;
    let _quota = SqliteQuotaLimiter::open(&path)?;
    let _bus = SqliteDurableBus::open(&path, 16)?;
    let _api_keys = SqliteApiKeyStore::open(&path)?;
    let _provider_secrets =
        EncryptedSqliteProviderSecretStore::open(&path, SecretKeyring::generated_for_tests()?)?;
    let _datasets = SqliteDatasetStore::open(&path)?;
    let _experiments = SqliteExperimentStore::open(&path)?;
    let _gates = SqliteGateStore::open(&path)?;
    let _reviews = SqliteHumanReviewStore::open(&path)?;
    let _judge = SqliteJudgeLedger::open(&path)?;
    let _usage = SqliteUsageLedger::open(&path)?;
    let _audit = SqliteAuditStore::open(&path)?;
    let _replay = SqliteReplayStore::open(&path)?;
    let _calibration = SqliteCalibrationStore::open(&path)?;

    let second_report = migrate_local_paletted_sqlite(&path)?;
    assert_eq!(second_report.applied, 0);
    assert_eq!(second_report.skipped, 1);

    Ok(())
}
