# SQLite Migrations

`beaterd` currently stores local OSS data in SQLite files under `--data-dir`.
This migration mirrors the embedded `CREATE TABLE IF NOT EXISTS` schema used by
the Rust stores today and acts as the baseline contract for future versioned
SQLite migrations.

The runtime still initializes SQLite from the store crates directly. Until a
migration runner is wired into `beaterd`, this file is a checked-in schema
contract and drift target, not an executed startup path.
