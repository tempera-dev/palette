# Postgres Migrations

Postgres is part of the Beater self-host topology for metadata/control-plane
state and an optional local trace-store backend. The current all-in-one
`beaterd` runtime still opens SQLite files under `--data-dir`; these migrations
are schema contracts and Docker initialization files until the Postgres stores
and migration runner are wired into the Rust runtime.

Do not treat these files as proof that Postgres is a working backend. The Gate 4
obligation still requires runtime backends plus the same `TraceStore`
conformance suite passing on SQLite and ClickHouse/Postgres.
