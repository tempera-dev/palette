# SQLite Migrations

`beaterd` currently stores local OSS data in SQLite files under `--data-dir`.
This migration is embedded by `beater-store-sql` and executed at `beaterd`
startup before SQLite-backed stores are opened.

Each SQLite file records applied versions and checksums in
`_beater_schema_migrations`. Store-local `CREATE TABLE IF NOT EXISTS` blocks
remain as compatibility guards, but this file is now the versioned startup
contract and drift target for the local OSS runtime.
