# SQLite Migrations

`beaterd` currently stores local OSS data in SQLite files under `--data-dir`.
This migration is embedded by `beater-store-sql` and executed at `beaterd`
startup before SQLite-backed stores are opened.

Each SQLite file records applied versions and checksums in
`_beater_schema_migrations`.

## Ownership model (issue #205)

The runtime migration is the **single source of truth** for the local beaterd
SQLite schema. Concretely, that source of truth is `migrate_local_beaterd_sqlite`
in `beater-store-sql`, which applies `LOCAL_BEATERD_SQLITE_MIGRATIONS` (this
directory's versioned, checksum-gated `*.sql` files) plus an idempotent
additive-column reconciliation pass (`reconcile_local_beaterd_sqlite`).

- **The migration defines the schema.** No store's `open()` may introduce a
  table, index, or column that the runtime migration does not already create.
- **Store `init()`/`open()` blocks remain as idempotent compatibility
  bootstrap** (`CREATE TABLE IF NOT EXISTS`, and a store's own `ensure_*`
  column guards). They are kept, not removed, because they let a store open a
  fresh in-memory or standalone database in unit tests and keep older on-disk
  databases self-healing. They must stay a strict subset of the migration
  schema — never a superset.
- A **drift test** enforces this invariant:
  `bins/beaterd/tests/sqlite_migrations.rs::runtime_migration_is_superset_of_store_open_schema`.
  It applies only the runtime migration to one fresh database, opens every
  runtime store against a second fresh database, and asserts the migration's
  table/index/column set is a superset-or-equal of the stores'. Any future
  store `open()` that adds schema the migration lacks fails this test.

### Why the reconciliation pass exists

The versioned `0001_local_beaterd.sql` is checksum-gated: hand-editing it would
invalidate every existing on-disk database (a "checksum mismatch" hard error at
startup). When a store later grows an additive nullable column through its own
idempotent `ALTER TABLE` bootstrap (e.g. `beater-audit`'s `previous_event_hash`
and `event_hash` hash-chain columns, added in #104 after `0001` was last
touched), the checksummed migration file cannot be amended in place. The
`reconcile_local_beaterd_sqlite` pass re-applies exactly those additive
`ALTER TABLE ... ADD COLUMN` statements, only when the column is missing, so the
migration output converges with the store schema without changing any
checksummed DDL and without breaking existing databases. The drift test proves
the convergence is exact.

Postgres and ClickHouse migrations are a separate scale-path contract and are
not governed by this model.
