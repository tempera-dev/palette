//! Runtime trace-store backend selection (ARCHITECTURE.md §20.2 Phase 0.1).
//!
//! One `beaterd` binary serves traces from SQLite (the OSS default), Postgres,
//! or ClickHouse, selected at runtime with `--trace-store` /
//! `BEATER_TRACE_STORE`. SQLite needs no URL and keeps the exact `--data-dir`
//! layout it always had; the scale backends connect with `--trace-store-url` /
//! `BEATER_TRACE_STORE_URL` and apply their checked-in migration contracts on
//! connect (both `connect` constructors run the idempotent migration, the same
//! way the Docker-gated conformance tests in `beater-store-sql` do).

use anyhow::Context;
use beater_store::TraceStore;
use beater_store_sql::{ClickHouseTraceStore, PgTraceStore, SqliteTraceStore};
use clap::ValueEnum;
use std::path::Path;
use std::sync::Arc;

/// `--trace-store` backend choices. SQLite is the default and the only backend
/// that requires no connection URL.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum TraceStoreBackendArg {
    #[default]
    Sqlite,
    Postgres,
    Clickhouse,
}

impl TraceStoreBackendArg {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sqlite => "sqlite",
            Self::Postgres => "postgres",
            Self::Clickhouse => "clickhouse",
        }
    }
}

/// Builds the runtime [`TraceStore`] for the selected backend.
///
/// - `sqlite`: opens `traces.sqlite` under `--data-dir` (the caller's local
///   migration pass has already applied the SQLite schema contract).
///   `--trace-store-url` is ignored.
/// - `postgres`: connects with a libpq-style connection string and applies
///   `POSTGRES_TRACE_STORE_MIGRATION` (idempotent, inside
///   `PgTraceStore::connect`).
/// - `clickhouse`: connects to the ClickHouse HTTP endpoint and applies
///   `CLICKHOUSE_TRACE_STORE_MIGRATION` (idempotent, inside
///   `ClickHouseTraceStore::connect`).
///
/// The connection URL is never logged: it can embed credentials.
pub async fn build_trace_store(
    backend: TraceStoreBackendArg,
    url: Option<&str>,
    sqlite_path: &Path,
) -> anyhow::Result<Arc<dyn TraceStore>> {
    let store: Arc<dyn TraceStore> = match backend {
        TraceStoreBackendArg::Sqlite => {
            let store = SqliteTraceStore::open(sqlite_path)
                .with_context(|| format!("open sqlite trace store {}", sqlite_path.display()))?;
            crate::metrics::log_event(
                "info",
                "trace store backend selected",
                &[
                    ("backend", "sqlite"),
                    ("path", &sqlite_path.display().to_string()),
                ],
            );
            Arc::new(store)
        }
        TraceStoreBackendArg::Postgres => {
            let url = require_trace_store_url(backend, url)?;
            let store = PgTraceStore::connect(url)
                .await
                .context("connect postgres trace store (--trace-store postgres)")?;
            crate::metrics::log_event(
                "info",
                "trace store backend selected",
                &[("backend", "postgres"), ("migrations", "applied")],
            );
            Arc::new(store)
        }
        TraceStoreBackendArg::Clickhouse => {
            let url = require_trace_store_url(backend, url)?;
            let store = ClickHouseTraceStore::connect(url)
                .await
                .context("connect clickhouse trace store (--trace-store clickhouse)")?;
            crate::metrics::log_event(
                "info",
                "trace store backend selected",
                &[("backend", "clickhouse"), ("migrations", "applied")],
            );
            Arc::new(store)
        }
    };
    Ok(store)
}

/// Typed missing-URL error for the network-backed arms: fails fast (before any
/// connection attempt) and names both the flag and its env fallback.
fn require_trace_store_url(
    backend: TraceStoreBackendArg,
    url: Option<&str>,
) -> anyhow::Result<&str> {
    url.ok_or_else(|| {
        anyhow::anyhow!(
            "--trace-store {backend} requires a connection URL: pass --trace-store-url \
             or set BEATER_TRACE_STORE_URL (postgres: libpq-style string, e.g. \
             \"host=localhost user=beater dbname=beater\"; clickhouse: HTTP endpoint, \
             e.g. \"http://localhost:8123\")",
            backend = backend.as_str(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sqlite_arm_opens_a_store_under_the_data_dir() {
        let tempdir = tempfile::tempdir().unwrap_or_else(|err| panic!("{err}"));
        let path = tempdir.path().join("traces.sqlite");
        let store = build_trace_store(TraceStoreBackendArg::Sqlite, None, &path)
            .await
            .unwrap_or_else(|err| panic!("sqlite arm must build: {err}"));
        // The store is live: an empty trace lookup succeeds (SqliteTraceStore
        // returns an empty TraceView rather than an error).
        let tenant = beater_core::TenantId::new("demo").unwrap_or_else(|err| panic!("{err}"));
        let trace = beater_core::TraceId::new("t1").unwrap_or_else(|err| panic!("{err}"));
        let view = store
            .get_trace(tenant, trace)
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert!(view.spans.is_empty());
        assert!(path.exists(), "sqlite arm must create the db file");
    }

    #[tokio::test]
    async fn sqlite_arm_ignores_a_provided_url() {
        let tempdir = tempfile::tempdir().unwrap_or_else(|err| panic!("{err}"));
        let path = tempdir.path().join("traces.sqlite");
        build_trace_store(
            TraceStoreBackendArg::Sqlite,
            Some("http://localhost:8123"),
            &path,
        )
        .await
        .unwrap_or_else(|err| panic!("sqlite arm must ignore the url: {err}"));
    }

    #[tokio::test]
    async fn postgres_arm_without_url_fails_fast_with_a_clear_error() {
        let tempdir = tempfile::tempdir().unwrap_or_else(|err| panic!("{err}"));
        let err = match build_trace_store(
            TraceStoreBackendArg::Postgres,
            None,
            &tempdir.path().join("traces.sqlite"),
        )
        .await
        {
            Ok(_) => panic!("postgres without a url must fail"),
            Err(err) => err.to_string(),
        };
        assert!(err.contains("--trace-store postgres"), "{err}");
        assert!(err.contains("--trace-store-url"), "{err}");
        assert!(err.contains("BEATER_TRACE_STORE_URL"), "{err}");
    }

    #[tokio::test]
    async fn clickhouse_arm_without_url_fails_fast_with_a_clear_error() {
        let tempdir = tempfile::tempdir().unwrap_or_else(|err| panic!("{err}"));
        let err = match build_trace_store(
            TraceStoreBackendArg::Clickhouse,
            None,
            &tempdir.path().join("traces.sqlite"),
        )
        .await
        {
            Ok(_) => panic!("clickhouse without a url must fail"),
            Err(err) => err.to_string(),
        };
        assert!(err.contains("--trace-store clickhouse"), "{err}");
        assert!(err.contains("--trace-store-url"), "{err}");
        assert!(err.contains("BEATER_TRACE_STORE_URL"), "{err}");
    }

    #[test]
    fn backend_arg_value_names_are_stable() {
        assert_eq!(TraceStoreBackendArg::Sqlite.as_str(), "sqlite");
        assert_eq!(TraceStoreBackendArg::Postgres.as_str(), "postgres");
        assert_eq!(TraceStoreBackendArg::Clickhouse.as_str(), "clickhouse");
        assert_eq!(
            TraceStoreBackendArg::default(),
            TraceStoreBackendArg::Sqlite
        );
    }
}
