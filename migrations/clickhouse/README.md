# ClickHouse Migrations

ClickHouse is the planned scale `TraceStore` backend. The schema keeps tenant
and project ids at the front of sort keys so tenant-scoped reads can be pushed
into storage instead of enforced after fetch.

The current Rust runtime does not yet include a ClickHouse store. This directory
is mounted by the optional `clickhouse` compose profile and acts as the checked
schema contract for the future `palette-store-ch` implementation.
