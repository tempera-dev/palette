//! Local runtime wiring extracted from `main.rs` (issue #207).
//!
//! This module owns the mechanical "where do the per-store SQLite databases
//! live under `--data-dir`" computation ([`LocalStorePaths`]) and the
//! open/migrate/construct sequence for the local all-in-one stack
//! ([`LocalStackBuilder`]). It preserves the exact file names, the exact set of
//! databases that get migrated, and the exact store constructors `main.rs`
//! previously inlined — this is an extract-and-call refactor, not a redesign.
//!
//! Migration ownership is intentionally untouched here (issue #205 owns
//! migration consolidation): the builder simply calls the same
//! `migrate_local_beaterd_sqlite` over the same deduplicated path list that
//! `main.rs` used before.

use anyhow::Context;
use std::path::{Path, PathBuf};

use crate::{AuthModeArg, BusBackendArg};

/// Per-store `*.sqlite` file paths computed under `--data-dir`.
///
/// These names are a stable on-disk contract (existing `.beater` data dirs
/// depend on them), so they must match what `main.rs` inlined exactly. The
/// `quota` path honors the `--quota-db-path` override; everything else is
/// derived from `data_dir`.
#[derive(Clone, Debug)]
pub struct LocalStorePaths {
    pub traces: PathBuf,
    pub quota: PathBuf,
    pub metadata: PathBuf,
    pub datasets: PathBuf,
    pub experiments: PathBuf,
    pub gates: PathBuf,
    pub reviews: PathBuf,
    pub calibrations: PathBuf,
    pub usage: PathBuf,
    pub audit: PathBuf,
    pub provider_secrets: PathBuf,
    pub judge: PathBuf,
    pub bus: PathBuf,
    pub security: PathBuf,
}

impl LocalStorePaths {
    /// Compute the per-store paths under `data_dir`. `quota_db_path_override`
    /// mirrors the `--quota-db-path` CLI flag: when `Some`, it replaces the
    /// default `<data_dir>/quotas.sqlite`.
    pub fn new(data_dir: &Path, quota_db_path_override: Option<&Path>) -> Self {
        Self {
            traces: data_dir.join("traces.sqlite"),
            quota: quota_db_path_override
                .map(Path::to_path_buf)
                .unwrap_or_else(|| data_dir.join("quotas.sqlite")),
            metadata: data_dir.join("metadata.sqlite"),
            datasets: data_dir.join("datasets.sqlite"),
            experiments: data_dir.join("experiments.sqlite"),
            gates: data_dir.join("gates.sqlite"),
            reviews: data_dir.join("reviews.sqlite"),
            calibrations: data_dir.join("calibrations.sqlite"),
            usage: data_dir.join("usage.sqlite"),
            audit: data_dir.join("audit.sqlite"),
            provider_secrets: data_dir.join("provider-secrets.sqlite"),
            judge: data_dir.join("judge.sqlite"),
            bus: data_dir.join("bus.sqlite"),
            security: data_dir.join("security.sqlite"),
        }
    }

    /// The set of SQLite databases beaterd migrates with its local
    /// all-tables migration, in the same order `main.rs` built the list.
    ///
    /// The bus database is only included when the SQLite bus backend is
    /// selected; the security database only when strict auth is required —
    /// matching the previous conditional pushes in `main.rs`. Stores that run
    /// their own migrations on `open()` (oauth, accounts) are intentionally
    /// excluded, exactly as before.
    pub fn migration_targets(
        &self,
        bus_backend: BusBackendArg,
        auth_mode: AuthModeArg,
    ) -> Vec<PathBuf> {
        let mut paths = vec![
            self.traces.clone(),
            self.quota.clone(),
            self.metadata.clone(),
            self.datasets.clone(),
            self.experiments.clone(),
            self.gates.clone(),
            self.reviews.clone(),
            self.calibrations.clone(),
            self.usage.clone(),
            self.audit.clone(),
            self.provider_secrets.clone(),
            self.judge.clone(),
        ];
        if matches!(bus_backend, BusBackendArg::Sqlite) {
            paths.push(self.bus.clone());
        }
        if matches!(auth_mode, AuthModeArg::Required) {
            paths.push(self.security.clone());
        }
        paths
    }
}

/// Apply the local all-tables migration to each unique path, deduplicating
/// (and sorting for a stable order) exactly as `main.rs` did. Migration
/// semantics are unchanged — this only owns the iteration/dedup boundary.
pub fn migrate_local_sqlite_stores(paths: &[PathBuf]) -> anyhow::Result<()> {
    let mut unique_paths = paths.to_vec();
    unique_paths.sort();
    unique_paths.dedup();
    for path in unique_paths {
        beater_store_sql::migrate_local_beaterd_sqlite(&path)
            .with_context(|| format!("migrate local sqlite schema {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_paths_use_expected_file_names_under_data_dir() {
        let data_dir = Path::new("/tmp/beater-data-207");
        let paths = LocalStorePaths::new(data_dir, None);
        assert_eq!(paths.traces, data_dir.join("traces.sqlite"));
        assert_eq!(paths.quota, data_dir.join("quotas.sqlite"));
        assert_eq!(paths.metadata, data_dir.join("metadata.sqlite"));
        assert_eq!(paths.datasets, data_dir.join("datasets.sqlite"));
        assert_eq!(paths.experiments, data_dir.join("experiments.sqlite"));
        assert_eq!(paths.gates, data_dir.join("gates.sqlite"));
        assert_eq!(paths.reviews, data_dir.join("reviews.sqlite"));
        assert_eq!(paths.calibrations, data_dir.join("calibrations.sqlite"));
        assert_eq!(paths.usage, data_dir.join("usage.sqlite"));
        assert_eq!(paths.audit, data_dir.join("audit.sqlite"));
        assert_eq!(
            paths.provider_secrets,
            data_dir.join("provider-secrets.sqlite")
        );
        assert_eq!(paths.judge, data_dir.join("judge.sqlite"));
        assert_eq!(paths.bus, data_dir.join("bus.sqlite"));
        assert_eq!(paths.security, data_dir.join("security.sqlite"));
    }

    #[test]
    fn quota_path_honors_override() {
        let data_dir = Path::new("/tmp/beater-data-207");
        let override_path = Path::new("/var/lib/beater/quota.db");
        let paths = LocalStorePaths::new(data_dir, Some(override_path));
        assert_eq!(paths.quota, override_path);
        // Everything else is still derived from data_dir.
        assert_eq!(paths.traces, data_dir.join("traces.sqlite"));
    }

    #[test]
    fn migration_targets_include_bus_only_for_sqlite_backend() {
        let paths = LocalStorePaths::new(Path::new("/tmp/d"), None);
        let with_sqlite = paths.migration_targets(BusBackendArg::Sqlite, AuthModeArg::Local);
        assert!(with_sqlite.contains(&paths.bus));
        let with_memory = paths.migration_targets(BusBackendArg::Memory, AuthModeArg::Local);
        assert!(!with_memory.contains(&paths.bus));
    }

    #[test]
    fn migration_targets_include_security_only_when_auth_required() {
        let paths = LocalStorePaths::new(Path::new("/tmp/d"), None);
        let required = paths.migration_targets(BusBackendArg::Memory, AuthModeArg::Required);
        assert!(required.contains(&paths.security));
        let local = paths.migration_targets(BusBackendArg::Memory, AuthModeArg::Local);
        assert!(!local.contains(&paths.security));
    }

    #[test]
    fn migration_targets_core_set_is_twelve_databases() {
        let paths = LocalStorePaths::new(Path::new("/tmp/d"), None);
        // The unconditional core set (no bus, no security) is 12 databases.
        let core = paths.migration_targets(BusBackendArg::Memory, AuthModeArg::Local);
        assert_eq!(core.len(), 12);
    }

    #[test]
    fn migrate_local_sqlite_stores_opens_a_temp_dir_stack_without_error() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let paths = LocalStorePaths::new(tempdir.path(), None);
        let targets = paths.migration_targets(BusBackendArg::Sqlite, AuthModeArg::Required);
        migrate_local_sqlite_stores(&targets).expect("migrate temp-dir stack");
        // The migration creates each database file on disk.
        for target in targets {
            assert!(target.exists(), "expected migrated db {target:?} to exist");
        }
    }
}
