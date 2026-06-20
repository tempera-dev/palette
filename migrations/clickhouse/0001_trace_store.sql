-- ClickHouse scale trace-store contract.
-- Tenant id leads every ORDER BY key to make tenant-scoped reads the natural
-- access path and to avoid post-fetch isolation.

CREATE DATABASE IF NOT EXISTS beater;

CREATE TABLE IF NOT EXISTS beater.raw_envelopes
(
  tenant_id String,
  project_id String,
  idempotency_key String,
  trace_id Nullable(String),
  payload_hash String,
  received_at DateTime64(6, 'UTC'),
  source LowCardinality(String),
  normalizer_version LowCardinality(String),
  raw_json String
)
ENGINE = MergeTree
PARTITION BY toYYYYMM(received_at)
ORDER BY (tenant_id, project_id, received_at, idempotency_key)
TTL received_at + INTERVAL 180 DAY
SETTINGS index_granularity = 8192;

CREATE TABLE IF NOT EXISTS beater.spans
(
  tenant_id String,
  project_id String,
  environment_id String,
  trace_id String,
  span_id String,
  parent_span_id Nullable(String),
  seq UInt64,
  kind LowCardinality(String),
  status LowCardinality(String),
  name String,
  start_time DateTime64(6, 'UTC'),
  end_time Nullable(DateTime64(6, 'UTC')),
  duration_ms Nullable(Int64),
  model_provider Nullable(String),
  model_name Nullable(String),
  cost_currency Nullable(LowCardinality(String)),
  cost_micros Nullable(Int64),
  release_id Nullable(String),
  span_json String
)
ENGINE = MergeTree
PARTITION BY toYYYYMM(start_time)
ORDER BY (tenant_id, project_id, environment_id, trace_id, start_time, span_id, seq)
TTL start_time + INTERVAL 90 DAY
SETTINGS index_granularity = 8192;

CREATE TABLE IF NOT EXISTS beater.trace_runs
(
  tenant_id String,
  project_id String,
  environment_id String,
  trace_id String,
  first_span_name String,
  span_count UInt64,
  status LowCardinality(String),
  started_at DateTime64(6, 'UTC'),
  ended_at Nullable(DateTime64(6, 'UTC')),
  duration_ms Nullable(Int64),
  total_cost_currency Nullable(LowCardinality(String)),
  total_cost_micros Nullable(Int64),
  models Array(Tuple(provider String, name String)),
  release_ids Array(String)
)
ENGINE = ReplacingMergeTree
PARTITION BY toYYYYMM(started_at)
ORDER BY (tenant_id, project_id, environment_id, started_at, trace_id)
TTL started_at + INTERVAL 90 DAY
SETTINGS index_granularity = 8192;

CREATE MATERIALIZED VIEW IF NOT EXISTS beater.trace_runs_mv
TO beater.trace_runs
AS
SELECT
  tenant_id,
  project_id,
  environment_id,
  trace_id,
  any(name) AS first_span_name,
  count() AS span_count,
  if(countIf(status = 'error') > 0, 'error', if(countIf(status = 'ok') > 0, 'ok', 'unset')) AS status,
  min(start_time) AS started_at,
  max(end_time) AS ended_at,
  dateDiff('millisecond', min(start_time), max(end_time)) AS duration_ms,
  any(cost_currency) AS total_cost_currency,
  sum(cost_micros) AS total_cost_micros,
  groupUniqArray((coalesce(model_provider, ''), coalesce(model_name, ''))) AS models,
  groupUniqArray(coalesce(release_id, '')) AS release_ids
FROM beater.spans
GROUP BY tenant_id, project_id, environment_id, trace_id;
