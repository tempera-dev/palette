use beater_audit::SqliteAuditStore;
use beater_auth::SqliteApiKeyStore;
use beater_bus::SqliteDurableBus;
use beater_calibration::SqliteCalibrationStore;
use beater_datasets::SqliteDatasetStore;
use beater_experiments::SqliteExperimentStore;
use beater_gates::SqliteGateStore;
use beater_human::SqliteHumanReviewStore;
use beater_judge::SqliteJudgeLedger;
use beater_replay::SqliteReplayStore;
use beater_secrets::{EncryptedSqliteProviderSecretStore, SecretKeyring};
use beater_store_sql::{
    migrate_local_beaterd_sqlite, SqliteMetadataStore, SqliteQuotaLimiter, SqliteTraceStore,
};
use beater_usage::SqliteUsageLedger;
use rusqlite::Connection;
use std::collections::BTreeMap;
use std::path::Path;

#[test]
fn local_sqlite_migration_bootstraps_runtime_store_schemas() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let path = tempdir.path().join("beater.sqlite");

    let report = migrate_local_beaterd_sqlite(&path)?;
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

    let second_report = migrate_local_beaterd_sqlite(&path)?;
    assert_eq!(second_report.applied, 0);
    assert_eq!(second_report.skipped, 1);

    Ok(())
}

/// Open every local store whose schema the beaterd runtime migration owns,
/// against a single SQLite database file (the same set as the e2e test above).
///
/// This is the store-owned bootstrap path: each `open()` runs an idempotent
/// `CREATE TABLE IF NOT EXISTS` block. The drift test snapshots the schema this
/// produces and compares it against the runtime migration's schema.
///
/// Stores excluded here mirror `LocalStorePaths::migration_targets`: the
/// security database (oauth/accounts) runs its OWN migrations on `open()` and is
/// not part of the `LOCAL_BEATERD_SQLITE_MIGRATIONS` contract, so it is not a
/// drift target.
fn open_all_runtime_stores(path: &Path) -> anyhow::Result<()> {
    let _traces = SqliteTraceStore::open(path)?;
    let _metadata = SqliteMetadataStore::open(path)?;
    let _quota = SqliteQuotaLimiter::open(path)?;
    let _bus = SqliteDurableBus::open(path, 16)?;
    let _api_keys = SqliteApiKeyStore::open(path)?;
    let _provider_secrets =
        EncryptedSqliteProviderSecretStore::open(path, SecretKeyring::generated_for_tests()?)?;
    let _datasets = SqliteDatasetStore::open(path)?;
    let _experiments = SqliteExperimentStore::open(path)?;
    let _gates = SqliteGateStore::open(path)?;
    let _reviews = SqliteHumanReviewStore::open(path)?;
    let _judge = SqliteJudgeLedger::open(path)?;
    let _usage = SqliteUsageLedger::open(path)?;
    let _audit = SqliteAuditStore::open(path)?;
    let _replay = SqliteReplayStore::open(path)?;
    let _calibration = SqliteCalibrationStore::open(path)?;
    Ok(())
}

/// Normalize a `sqlite_master.sql` string so that DDL which differs only in
/// whitespace/formatting compares equal. SQLite stores the original CREATE text
/// verbatim, and the migration file and the store crates indent their identical
/// DDL differently, so a raw string compare would report spurious drift.
fn normalize_ddl(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The shape of one durable schema object as the drift test compares it.
///
/// Tables are compared by their *set* of columns (name, declared type,
/// not-null, primary-key membership) rather than by verbatim CREATE text: a
/// column added by `ALTER TABLE ... ADD COLUMN` is appended last, whereas the
/// same column declared inline sits wherever the author wrote it, so two
/// semantically identical tables can carry different column ORDER. Order is
/// irrelevant to compatibility here (every store addresses columns by name), so
/// comparing the order-insensitive column set is the correct "same table shape"
/// signal. Indexes are compared by their normalized DDL, which SQLite stores
/// deterministically.
#[derive(Debug, PartialEq, Eq)]
enum SchemaObject {
    Table(BTreeMap<String, String>),
    Index(String),
}

/// Describe a table as an order-insensitive map of `column -> normalized shape`.
fn table_columns(connection: &Connection, table: &str) -> BTreeMap<String, String> {
    let escaped = table.replace('"', "\"\"");
    let mut statement = connection
        .prepare(&format!("PRAGMA table_info(\"{escaped}\")"))
        .unwrap_or_else(|err| panic!("{err}"));
    let rows = statement
        .query_map([], |row| {
            // PRAGMA table_info columns: cid, name, type, notnull, dflt_value, pk
            let name: String = row.get(1)?;
            let col_type: String = row.get(2)?;
            let not_null: i64 = row.get(3)?;
            let default: Option<String> = row.get(4)?;
            let pk: i64 = row.get(5)?;
            Ok((
                name,
                format!("type={col_type} notnull={not_null} default={default:?} pk={pk}"),
            ))
        })
        .unwrap_or_else(|err| panic!("{err}"));
    let mut columns = BTreeMap::new();
    for row in rows {
        let (name, shape) = row.unwrap_or_else(|err| panic!("{err}"));
        columns.insert(name, shape);
    }
    columns
}

/// Snapshot the durable schema of a connection: every table (by column set) and
/// index (by normalized DDL), keyed by object name. Auto-generated objects
/// (`sqlite_autoindex_*`, backing inline UNIQUE/PRIMARY KEY constraints) have a
/// NULL `sql` and are skipped — they are an artifact of table DDL we already
/// compare via the column set, not independently-defined schema.
fn schema_snapshot(connection: &Connection) -> BTreeMap<String, SchemaObject> {
    let mut statement = connection
        .prepare(
            r#"
            SELECT type, name, sql
            FROM sqlite_master
            WHERE type IN ('table', 'index') AND sql IS NOT NULL
            ORDER BY name
            "#,
        )
        .unwrap_or_else(|err| panic!("{err}"));
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })
        .unwrap_or_else(|err| panic!("{err}"));
    let mut snapshot = BTreeMap::new();
    for row in rows {
        let (object_type, name, sql) = row.unwrap_or_else(|err| panic!("{err}"));
        let object = match object_type.as_str() {
            "table" => SchemaObject::Table(table_columns(connection, &name)),
            "index" => SchemaObject::Index(normalize_ddl(&sql)),
            other => panic!("unexpected sqlite_master type {other}"),
        };
        snapshot.insert(name, object);
    }
    snapshot
}

/// Migration drift guard (issue #205).
///
/// Proves that the beaterd runtime migration (`LOCAL_BEATERD_SQLITE_MIGRATIONS`,
/// the documented single source of truth for the local DB schema) is a
/// superset-or-equal of every table/index that the store `open()` bootstrap
/// paths create. If any store `open()` ever introduces a table, index, or
/// column the runtime migration does not already define — or defines it with a
/// different shape — this test fails, catching the divergence before it reaches
/// a durable on-disk schema.
///
/// Method: apply ONLY the runtime migration to one fresh DB and snapshot its
/// schema; open ALL runtime stores against a second fresh DB and snapshot that.
/// Then assert the store schema is contained in the migration schema with
/// byte-identical (normalized) DDL per object.
#[test]
fn runtime_migration_is_superset_of_store_open_schema() -> anyhow::Result<()> {
    let tempdir = tempfile::tempdir()?;

    // DB-A: runtime migration only.
    let migration_path = tempdir.path().join("migration.sqlite");
    let report = migrate_local_beaterd_sqlite(&migration_path)?;
    assert_eq!(report.applied, 1, "runtime migration should apply once");
    let migration_connection = Connection::open(&migration_path)?;
    let migration_schema = schema_snapshot(&migration_connection);

    // DB-B: store-owned bootstrap only (no runtime migration).
    let stores_path = tempdir.path().join("stores.sqlite");
    open_all_runtime_stores(&stores_path)?;
    let stores_connection = Connection::open(&stores_path)?;
    let stores_schema = schema_snapshot(&stores_connection);

    // Every object the stores create must already exist in the runtime
    // migration with the same shape (table column set / index DDL).
    let mut missing = Vec::new();
    let mut mismatched = Vec::new();
    for (name, store_object) in &stores_schema {
        match migration_schema.get(name) {
            None => missing.push(name.clone()),
            Some(migration_object) if migration_object != store_object => {
                mismatched.push(format!(
                    "{name}\n  migration: {migration_object:?}\n  store:     {store_object:?}"
                ));
            }
            Some(_) => {}
        }
    }

    assert!(
        missing.is_empty(),
        "store open() defines schema the runtime migration is missing \
         (the migration must be the single source of truth): {missing:?}"
    );
    assert!(
        mismatched.is_empty(),
        "store open() DDL diverges from the runtime migration DDL:\n{}",
        mismatched.join("\n")
    );

    Ok(())
}
