# Beater Architecture

Beater is a Rust-first, open-source agent observability, replay, and eval platform.
It is designed to be credible as an OSS project, viable as a hosted API product, and
agent-native enough to compete with Arize Phoenix, Braintrust, LangSmith, Langfuse,
and Judgment-style agent debugging systems.

The core product loop is the definition of "shipped":

```text
instrument agent
  -> inspect trace/span tree
  -> promote failure to dataset
  -> run evals
  -> compare candidate change
  -> block or allow CI
  -> monitor production after deploy
```

If that loop is not excellent end to end, the rest of the platform is premature.

## 1. Non-Negotiable Principles

1. Ship one Rust binary first.
   Crates are modular, but the MVP runs as one `beaterd` process. Later service
   splits are thin bins over the same crates and are justified only by measured
   scale pressure.

2. Use standards at the edge, not proprietary lock-in.
   OTLP, W3C trace-context, OpenInference, OpenTelemetry GenAI conventions, and
   common framework exporters are first-class. The native Rust SDK is an
   accelerator, not the adoption gate.

3. Store immutable raw data and normalized projections.
   Every ingest event keeps its raw envelope, source schema URL/version, payload
   hash, normalizer version, and unmapped attributes. The normalized model is a
   canonical superset used for queries and UI. Lossless round-trip is promised
   only through the raw copy, not through lossy standards projections.

4. Design privacy and tenancy before hosted ingest.
   Tenant isolation, regional pinning, redaction, crypto-shredding, retention,
   scoped keys, audited PII access, and artifact encryption are v1 constraints.

5. Split deterministic evals from model-dependent evals.
   User-supplied deterministic evaluators run in a WASI Component Model sandbox
   via Wasmtime with no network. LLM-judge and embedding evals run through a
   judge broker with budgets, redaction, model versioning, and audit trails.

6. Be honest about replay.
   Deterministic replay requires provider, tool, memory, and clock cassettes.
   Without cassettes, the product calls it forked replay or simulation.

7. Scope Vercel correctly.
   Vercel runs the dashboard, stateless Rust functions, edge/control-plane API,
   webhooks, Blob, Queues, and Cron. Long-running ingest listeners, ClickHouse
   writers, eval pools, replay pools, and stateful workers run in hosted cells or
   in the OSS all-in-one process.

8. No cloud dependency in OSS.
   The open-source edition must run without calling Beater Cloud. Self-host
   telemetry is opt-out, and all hosted-specific APIs degrade cleanly to local
   equivalents or are absent.

## 2. Editions

| Capability | OSS self-host | Hosted |
| --- | --- | --- |
| All-in-one Rust binary | Required | Used for cells and local dev |
| OTLP and native ingest | Required | Required |
| Trace viewer and span tree | Required | Required |
| Datasets, experiments, offline evals | Required | Required |
| WASI deterministic evaluator sandbox | Required | Required |
| Judge broker with BYOK | Required | Required |
| Managed judge routing and shared judge fleet | Optional BYOK only | Required |
| Online eval sampling and alerts | OSS local/basic | Hosted scaled/commercial |
| Multi-tenant orgs, billing, quotas | Single-cell/basic | Required |
| SSO/SAML, regional pinning UI, advanced audit export | Optional enterprise | Commercial |
| No-cloud operation | Required | Not applicable |

Open-core boundary:

- Apache-2.0 core: ingest, canonical schema, local auth, trace UI, datasets,
  deterministic evals, WASI ABI, judge broker BYOK, replay cassettes, CI gate,
  import/export, Docker Compose, plugin APIs.
- Commercial/hosted: managed multi-region cells, billing, SSO/SAML, enterprise
  audit reporting, regional compliance controls, managed alerts, hosted judge
  fleet, high-scale replay/eval pools, support SLAs.
- Security-critical primitives such as audit event generation and redaction are
  not withheld from OSS. The paid boundary is managed scale and governance UX,
  not the ability to self-host safely.

## 3. Deployment Shape

### 3.1 OSS

```text
docker compose up
  beaterd       # one Rust process: API, OTLP, jobs, eval, replay
  postgres      # metadata and optional local TraceStore
  clickhouse    # optional scale TraceStore
  nats          # JetStream durable bus
  minio         # object storage
```

Local development can run with SQLite plus local filesystem object storage, but
the same code paths must work with Postgres, ClickHouse, NATS JetStream, and
S3-compatible storage.

### 3.2 Hosted

```text
Vercel
  dashboard
  Rust Functions for stateless /v1 API routes
  Vercel Queues for Vercel-native async boundaries
  Vercel Blob for small/control-plane artifacts where appropriate
  Cron for short scheduled tasks

Hosted Rust cells
  long-running OTLP gRPC/HTTP ingest
  queue consumers
  ClickHouse writers
  replay/eval/judge workers
  WebSocket/SSE fanout if needed

Managed data
  Postgres for metadata
  ClickHouse for hot trace analytics
  S3/R2/GCS for artifacts and raw envelopes
  Parquet/DataFusion for cold retention
```

Current platform constraints, verified June 19, 2026:

- Vercel's official Rust runtime is beta, runs on Fluid Compute, and uses
  `vercel_runtime` handlers in `api/`.
- Vercel Functions have request/response and body-size limits; the documented
  payload limit is 4.5 MB.
- Vercel Functions do not act as WebSocket servers.
- Vercel Queues are beta durable append-only topics with at-least-once delivery,
  idempotency keys, push consumers, and poll consumers. Poll mode is suitable for
  off-Vercel workers.
- Therefore Vercel can be the hosted control plane and stateless API surface, but
  not the only runtime for stateful ingest and long-running workers.

Primary source links:

- https://vercel.com/docs/functions/runtimes/rust
- https://vercel.com/docs/functions/limitations
- https://vercel.com/docs/queues
- https://vercel.com/docs/queues/poll-mode

## 4. Rust Workspace

The operational split is logical first, physical later.

```text
beater/
  Cargo.toml
  crates/
    beater-core/          # IDs, entity types, typed money, clocks, tenant scope
    beater-schema/        # canonical event/run/span/eval schemas, mappings, rollups
    beater-otlp/          # tonic/prost OTLP HTTP/gRPC receive and export
    beater-normalize/     # OTLP/OI/GenAI/Vercel/LangSmith/Phoenix -> canonical
    beater-ingest/        # auth, quota, raw append, normalization, sampling
    beater-store/         # TraceStore, MetadataStore, ArtifactStore, QuotaLimiter traits and StoreError
    beater-store-sql/     # SQLite TraceStore/MetadataStore/QuotaLimiter for dev/small installs
    beater-store-ch/      # ClickHouse TraceStore for scale
    beater-store-obj/     # object_store-backed artifacts and raw envelopes
    beater-bus/           # NATS JetStream default, Vercel Queues, Kafka adapter
    beater-eval/          # evaluator catalog, scoring contracts, aggregation
    beater-calibration/   # judge-vs-human agreement and kappa reports
    beater-usage/         # usage ledger, billing meters, spend summaries
    beater-audit/         # privileged access audit events and readback
    beater-sandbox/       # Wasmtime/WASI Component Model evaluator runtime
    beater-secrets/       # opaque provider-secret refs, BYOK metadata, revocation
    beater-judge/         # LLM/embedding judge broker, BYOK, calibration
    beater-replay/        # cassettes, forked replay, deterministic replay
    beater-datasets/      # datasets, versions, examples, trace promotion
    beater-experiments/   # candidate-vs-baseline comparisons and statistics
    beater-gates/         # CI/CD gates and policy evaluation
    beater-human/         # review queues, annotations, human labels
    beater-auth/          # API keys, JWT/session, RBAC, audit scopes
    beater-api/           # axum routers, OpenAPI, SSE/read APIs
    beater-sdk/           # native Rust SDK and tracing layers
    beater-telemetry/     # dogfooding, metrics, health, SLO instrumentation
  bins/
    beaterd/              # default all-in-one binary
    beaterctl/            # CLI: init, ingest test, eval run, gate, export
    beater-worker/        # later thin bin over worker modules
    beater-ingestd/       # later thin bin over ingest modules
  api/
    *.rs                  # Vercel Rust Function entrypoints where needed
  web/
    dashboard/            # Next.js dashboard consuming generated OpenAPI client
  migrations/
    postgres/
    clickhouse/
  docker-compose.yml
```

The dashboard can use TypeScript/React for product velocity, but all platform
logic, ingestion, storage, eval, replay, API contracts, and SDK primitives remain
Rust-owned.

### 4.1 Implementation Picks

The default Rust stack should be boring and production-proven:

| Concern | Pick |
| --- | --- |
| async runtime | `tokio` |
| HTTP API | `axum`, `tower`, `hyper` |
| OTLP gRPC | `tonic`, `prost`, `opentelemetry-proto` |
| Vercel Rust Functions | `vercel_runtime` |
| serialization | `serde`, `serde_json`, `rmp-serde` where useful |
| schema/OpenAPI | `utoipa` |
| metadata DB | `sqlx` with Postgres and SQLite |
| ClickHouse | official `clickhouse` crate |
| object storage | `object_store` |
| durable bus | `async-nats` JetStream by default; Kafka adapter for enterprise |
| Vercel queue adapter | Vercel Queues HTTP API |
| WASI sandbox | `wasmtime` Component Model |
| cold analytics | `arrow`, `parquet`, `datafusion` |
| full-text search | `tantivy` |
| auth/secrets | `argon2`, `jsonwebtoken`, KMS/Vault-compatible traits |
| CLI | `clap` |
| testing | `cargo-nextest`, `testcontainers`, `insta`, `proptest` |
| build/release | `cargo-chef`, multi-stage Docker, `cargo-deny` |

The stack can change when implementation evidence demands it, but each
replacement must preserve the architecture contracts in this document.

## 5. Canonical Data Model

### 5.1 Entity Set

- `Organization`
- `User`
- `Project`
- `Environment`
- `Agent`
- `AgentRelease`
- `Run`
- `Span`
- `Event`
- `Artifact`
- `ReplayCassette`
- `Dataset`
- `DatasetVersion`
- `DatasetCase`
- `Experiment`
- `ExperimentRun`
- `Evaluator`
- `EvaluatorVersion`
- `EvalResult`
- `Gate`
- `GateRun`
- `ReviewQueue`
- `ReviewTask`
- `Annotation`
- `Prompt`
- `PromptVersion`
- `UsageLedger`
- `UsageRecord`
- `ApiKey`
- `WebhookEndpoint`
- `RedactionPolicy`
- `RetentionPolicy`
- `AuditEvent`

### 5.2 Agent Span Taxonomy

These span kinds are canonical, regardless of incoming dialect:

```text
agent.run
agent.turn
agent.plan
agent.step
llm.call
tool.call
mcp.request
retrieval.query
memory.read
memory.write
guardrail.check
human.review
evaluator.run
replay.run
```

Additional provider/framework kinds are retained as original attributes and can
be projected into OpenInference or OTel GenAI exports.

### 5.3 Raw Envelope and Normalized Projection

Every write path stores both forms:

```rust
pub struct RawEnvelope {
    pub schema_version: u32,
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub environment_id: EnvironmentId,
    pub source: SourceDialect,
    pub source_schema_url: Option<String>,
    pub source_schema_version: Option<String>,
    pub received_at: Timestamp,
    pub idempotency_key: IdempotencyKey,
    pub payload_hash: Sha256,
    pub body_ref: ArtifactRef,
    pub auth_context: AuthContext,
}

pub struct CanonicalSpan {
    pub schema_version: u32,
    pub normalizer_version: String,
    pub tenant_id: TenantId,
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub seq: u64,
    pub kind: AgentSpanKind,
    pub status: SpanStatus,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub model: Option<ModelRef>,
    pub cost: Option<Money>,
    pub tokens: Option<TokenCounts>,
    pub input_ref: Option<ArtifactRef>,
    pub output_ref: Option<ArtifactRef>,
    pub attributes: CanonicalAttrs,
    pub unmapped_attrs: serde_json::Value,
    pub raw_ref: ArtifactRef,
}
```

Idempotency key:

```text
tenant_id + project_id + trace_id + span_id + seq + payload_hash
```

Late spans are accepted. Out-of-order writes are normal. Trace completeness is a
state machine, not a boolean.

## 6. Standards and Normalization

Input dialects:

- OTLP traces over gRPC and HTTP.
- OpenInference attributes and span kinds.
- OpenTelemetry GenAI conventions.
- Vercel AI SDK telemetry shapes.
- OpenLLMetry/Traceloop-compatible attributes.
- Native Beater `/v1` JSON ingest.
- Future imports from Phoenix, LangSmith, Langfuse, and Braintrust exports.

Output dialects:

- OTLP export.
- OpenInference-compatible export.
- Parquet export.
- JSONL dataset/eval export.
- Post-training export for SFT/RL pipelines.

OpenTelemetry GenAI note:

- The old OpenTelemetry docs now point to the standalone
  `open-telemetry/semantic-conventions-genai` repository.
- The repository currently contains generated docs/model definitions for GenAI
  clients, MCP, events, spans, metrics, and provider-specific conventions.
- Pin by commit or generated schema snapshot. Do not assume stability. The
  normalizer must support both old and new forms.

Source links:

- https://opentelemetry.io/docs/specs/semconv/gen-ai/
- https://github.com/open-telemetry/semantic-conventions-genai

## 7. Storage Architecture

### 7.1 Trait Boundary

`TraceStore` exists before any ClickHouse-specific behavior leaks into product
code.

```rust
#[async_trait::async_trait]
pub trait TraceStore: Send + Sync {
    async fn write_batch(&self, batch: CanonicalTraceBatch) -> anyhow::Result<WriteAck>;
    async fn get_trace(
        &self,
        tenant: TenantId,
        trace: TraceId,
    ) -> anyhow::Result<TraceView>;
    async fn query_runs(
        &self,
        tenant: TenantId,
        filter: RunFilter,
        page: PageRequest,
    ) -> anyhow::Result<Page<RunSummary>>;
    async fn query_spans(
        &self,
        tenant: TenantId,
        filter: SpanFilter,
        page: PageRequest,
    ) -> StoreResult<Page<SpanSummary>>;
}
```

Backends:

- `SqliteTraceStore` in `beater-store-sql`: SQLite for tiny local installs and
  tests.
- `ClickHouseTraceStore`: hot analytical trace store for production scale.
- `ParquetTraceArchive`: cold tier queried through DataFusion.

Product code depends on `TraceStore`, not concrete backend crates.

SQLite and memory stores may use
`beater_store::query_runs_by_materializing_spans` as a dev/local fallback. That
helper intentionally reads matching spans and rolls up run summaries in Rust.
ClickHouse or any hosted hot-store backend must not use that fallback for normal
paths; it must aggregate run summaries, run-level filters, and pagination in the
backend over tenant-leading sort keys.

### 7.2 Data Planes

| Plane | Default OSS | Hosted scale | Purpose |
| --- | --- | --- | --- |
| Metadata | Postgres, SQLite dev | Postgres | orgs, projects, prompts, datasets, RBAC, billing metadata |
| Hot traces | Postgres/SQLite dev, ClickHouse optional | ClickHouse | runs, spans, events, scores, indexed attrs |
| Raw/artifacts | filesystem dev, MinIO/S3 | S3/R2/GCS/Vercel Blob where suitable | raw envelopes, payloads, cassettes, exports |
| Durable bus | NATS JetStream | Vercel Queues at edge, NATS or Kafka in cells | ingest buffering, eval jobs, replay jobs |
| Cold traces | Parquet + DataFusion | Parquet + DataFusion | long retention and export |
| Full text | Tantivy | Tantivy or managed equivalent | prompt/output/error search |

Redis is optional cache/pubsub, not the default durability primitive.

### 7.3 ClickHouse Rules

- Tenant ID leads sort keys and all query filters.
- Updates are modeled as new events or versioned replacing rows.
- Object storage holds large inputs, outputs, attachments, raw payloads, and
  cassettes. ClickHouse stores refs, hashes, sizes, MIME types, and redaction
  classifications.
- TTL moves hot rows to cold Parquet before deletion.
- Query APIs must not require `FINAL` for normal paths.

### 7.4 Queue and Job Lanes

Do not collapse every async concern into one queue. The platform has distinct
lanes with different guarantees:

| Lane | OSS default | Hosted default | Required semantics |
| --- | --- | --- | --- |
| in-process smoothing | bounded Tokio channels | bounded Tokio channels | absorbs short spikes, rejects when full |
| ingest durability | NATS JetStream | Vercel Queues at edge, NATS/Kafka in cells | at-least-once, idempotency keys, replay until retention |
| background jobs | NATS JetStream or Postgres-backed scheduler | NATS/Kafka/cell scheduler | retries, backoff, poison-message isolation |
| DLQ | NATS stream plus object refs | regional DLQ stream plus object refs | reason codes, replay tooling, alerts |
| cache/pubsub | optional Redis or in-process | Redis/managed cache where needed | never source of durability |
| enterprise bus | Kafka adapter | Kafka adapter | large customer integration and audit needs |

The current OSS all-in-one slice uses the same lane model on the SQLite durable
bus. `?durability=buffered` on native or OTLP ingest writes a canonical
`trace.write_batch` message before hot trace persistence. A scoped drain API and
the `beaterd` background worker consume only that lane, write through
`TraceStore`, publish downstream `trace.ingested` work, and move invalid or
exhausted messages to DLQ without consuming other tenants' queued work. Hosted
deployments replace the SQLite bus implementation with Vercel Queues at the edge
and NATS/Kafka in worker cells without changing the ingest contract.

Poison messages are messages that repeatedly fail for deterministic reasons.
They must be moved to DLQ with a reason, source envelope ref, attempt history,
and replay command. They must not block a partition or consumer group forever.

## 8. Ingest Pipeline

```text
receive OTLP/native request
  -> authenticate API key and project/environment
  -> enforce per-project quotas and payload limits
  -> create RawEnvelope and artifact refs
  -> normalize with pinned normalizer version
  -> enforce cardinality/payload governance
  -> buffer for tail-sampling and trace completion
  -> direct mode: write canonical projection through TraceStore
  -> buffered mode: enqueue canonical trace.write_batch for the drain worker
  -> enqueue online eval/replay/alert jobs
  -> acknowledge or DLQ
```

Required survivability behavior:

- Backpressure with bounded queues.
- At-least-once delivery reconciled by idempotency keys.
- Dead-letter queue for invalid, unauthenticated, unnormalizable, or repeatedly
  unwritable events.
- Poison-message isolation so one bad tenant payload cannot stall a shard.
- No silent drops.
- Per-project ingest quotas with explicit 429 semantics.
- Payload size caps. Oversized prompts/completions truncate to artifact refs.
- Per-attribute cardinality budgets.
- Attribute allow/deny lists at project and environment scope.
- Tail-based sampling that keeps all errors, slow traces, high-cost traces, and
  traces selected by policy while sampling routine traffic.
- Trace completion semantics based on root-span end, idle timeout, and late-span
  window.
- Clock-skew correction and out-of-order handling across distributed agents.

Trace states:

```text
open
root_ended
idle_complete
late_window_closed
complete
incomplete
```

Online evals trigger only when policy says the trace is complete enough.

## 9. Evaluator Architecture

### 9.1 Execution Lanes

Deterministic lane:

- Runtime: Wasmtime using the WASI Component Model.
- Network: none.
- Inputs: trace/case data, expected outputs, evaluator config, artifact reads
  through explicit host functions.
- Outputs: structured score, labels, evidence refs, diagnostics.
- Examples: exact match, regex, JSON schema, tool-call correctness,
  trajectory-structure checks, latency budgets, cost budgets, token budgets,
  loop detection, citation presence, PII-pattern checks.

Judge lane:

- Runtime: `judge-broker` Rust service/module.
- Network: broker only, never evaluator WASM.
- Secrets: provider keys held behind opaque `ProviderSecretId` refs. The OSS
  SQLite implementation stores ChaCha20-Poly1305 ciphertext with tenant/project/
  secret/provider associated data; hosted cells can swap the same trait for KMS
  or customer vault unwrap. API responses and ledgers expose metadata only,
  never secret material.
- Controls: redaction, preflight budget reservation, rate limit, model pinning,
  retries, request-hash caching, and append-only audit logging.
- Usage: judge calls emit idempotent `UsageRecord`s keyed by tenant/project,
  meter, source kind, and source ID. Billing summaries use charged cost, while
  cached calls remain visible as zero-cost audit records.
- Provider clients: OpenAI-compatible chat completions and Anthropic messages
  are first-class Rust judge providers behind the same broker trait, with
  retry/backoff for `429` and `5xx` responses and structured JSON score parsing.
- Dataset execution: dataset-version evals support both deterministic WASI
  scorers and judge-broker scorers. Judge-backed reports persist through the
  same `DatasetEvalReport`/`EvalResult` schema, with model/provider/request
  hashes, cache status, and charged cost pinned in reproducibility metadata.
- Experiment execution: candidate-vs-baseline gates support judge-broker
  scorers as well as deterministic scorers. Per-case experiment scores retain
  judge call IDs, charged costs, and cache status so noisy/model-dependent gates
  remain auditable. Experiment reports also retain the `GatePolicy` that
  produced the stored pass/fail/inconclusive decision.
- Examples: faithfulness, pairwise judge, trajectory quality, retrieval
  relevance, handoff quality, rubric grading, semantic safety checks.

### 9.2 EvalResult Reproducibility Contract

Every `EvalResult` pins:

- dataset version
- dataset case ID
- candidate agent release
- prompt version
- evaluator ID and version
- evaluator code hash or WASM hash
- WASI ABI version if deterministic
- judge model ID, provider, parameters, seed when available
- judge prompt/rubric version
- normalizer version
- trace schema version
- input artifact hashes
- execution timestamp
- cost and token usage
- reason if an old result cannot be reproduced

### 9.3 Statistical Rigor

Experiment comparison must include:

- candidate vs baseline deltas
- N-trial repetition for noisy evaluators
- confidence intervals
- variance by case and metric
- significance testing before declaring wins
- minimum sample-size policy per metric and gate
- explicit test choice by metric type, such as paired bootstrap, permutation
  test, McNemar, or t-test only when assumptions are met
- multiple-comparison handling when one experiment evaluates many metrics or
  cohorts
- pairwise position-bias mitigation by swapping A/B order
- judge calibration artifact with judge-vs-human agreement, confusion counts,
  and Cohen's kappa where applicable
- recalibration triggers for model deprecation, provider drift, rubric changes,
  and kappa degradation

The CI gate must be able to fail on confidence-bound regressions, not only raw
mean score deltas.

## 10. Replay and Failure Attribution

Replay substrate:

- Immutable event stream.
- Provider cassettes for LLM requests/responses, streaming chunks, tool calls,
  embeddings, retrievals, memory reads/writes, guardrail checks, random seeds,
  clock reads, and human-review stubs.
- Versioned tool schemas and MCP request/response bodies.

Replay modes:

- `deterministic_replay`: all required cassettes present and hashes match.
- `forked_replay`: starts from a captured trace, then calls live providers/tools
  after a fork point.
- `simulation`: substitutes models/tools with configured simulators.

Failure attribution pipeline:

```text
failed trace
  -> span-level deterministic checks
  -> judge or human checks where needed
  -> compare against similar passing traces
  -> replay or fork candidate corrections
  -> identify earliest span whose correction flips outcome
  -> write root-cause annotation and regression candidate
```

The product should surface:

- root-cause span
- confidence/evidence
- failed-vs-passed diff
- replay mode and guarantee level
- one-click "add to dataset"

## 11. Agent Harness

The harness lets the platform become an open-source eval runner, not just a
trace viewer.

```rust
#[async_trait::async_trait]
pub trait AgentAdapter {
    async fn run_case(
        &self,
        case: DatasetCase,
        ctx: HarnessContext,
    ) -> anyhow::Result<AgentRunOutput>;
}

#[async_trait::async_trait]
pub trait ToolProxy {
    async fn call_tool(&self, request: ToolRequest) -> anyhow::Result<ToolResponse>;
}

#[async_trait::async_trait]
pub trait TraceEmitter {
    async fn emit(&self, event: CanonicalEvent) -> anyhow::Result<()>;
}
```

Harness components:

- `ScenarioRunner`: runs dataset cases, seeds, timeouts, and concurrency.
- `ProviderProxy`: records provider cassettes and enforces budgets.
- `ToolProxy`: records tool/MCP cassettes and validates schemas.
- `JudgeRunner`: routes model-dependent evals through judge broker.
- `ExperimentRunner`: runs baseline and candidate releases.
- `GateRunner`: converts persisted experiment reports into CI pass/fail. A gate
  run may target an explicit experiment run ID or the latest tenant/project
  report matching the gate's dataset/evaluator selectors; the gate-run report
  snapshots the gate definition, experiment gate policy, comparison, decision,
  and pass/fail reason.
- `HumanReviewRunner`: queues trace/span review tasks, stores human annotations,
  and promotes corrected human references into dataset cases through the same
  dataset store used by automated evals.
- `TraceEmitter`: emits canonical traces and raw refs.

The same harness must run locally, in CI, and in hosted workers.

## 12. Query, UI, and Alerting

Core UI requirements:

- trace table with project/environment/release filters
- span tree and waterfall
- agent turn/plan/step view
- MCP/tool-call visibility
- prompt/input/output/artifact inspector with redaction controls
- cost/token/latency analytics
- dataset promotion from trace/span
- experiment comparison
- eval result drilldown
- replay/cassette view
- human review queues
- failure clustering and root-cause annotations

Search:

- structured filters by status, time, trace ID, span kind, model, tool, cost,
  latency, token counts, environment, agent release, evaluator, and tags
- full-text search over inputs, outputs, errors, tool names, and selected attrs
  through Tantivy or equivalent
- natural-language search is later; fast structured search is v1

Alerting:

- online eval sampling policies
- baselines by project/environment/release
- dedupe and grouping
- maintenance windows
- Slack/webhook integrations
- alert budgets and suppression
- links back to trace clusters, dataset candidates, and gates

## 13. Compliance, Security, and Data Lifecycle

Data lifecycle:

- per-project retention policies
- hot ClickHouse -> cold Parquet -> delete
- referential consistency across rows, artifacts, cassettes, indexes, and exports
- orphan artifact sweeps
- restore drills before hosted GA

Deletion:

- immutable raw rows conflict with subject deletion unless encryption is designed
  correctly
- use envelope encryption with per-tenant keys and, where enterprise requires,
  per-subject data encryption keys
- deletion can be implemented as crypto-shredding plus deletion of lookup rows
  and object refs
- deleted data must become unreadable across hot, cold, and artifact stores

Security:

- API keys are scoped, rotatable, hashed, environment-bound, and audited
- PII unmask is a separate RBAC scope from ordinary trace read
- sensitive-data access emits audit events
- BYOK for judge/model providers
- encryption at rest for metadata, artifacts, cassettes, and cold files
- key rotation and key revocation workflows
- outbound webhooks use HMAC signatures, timestamp/replay protection, retries,
  and idempotency keys
- all tenant-scoped storage APIs take `TenantId` explicitly
- hosted cells enforce region/project pinning; PII does not cross regions

## 14. Public API and DX

DX SLO:

```text
time to first trace <= 5 minutes
```

Required onboarding paths:

- zero-SDK OTLP endpoint for any OpenInference/OpenLLMetry/OTel app
- native Rust SDK with `tracing`, `opentelemetry-rust`, `reqwest`, `axum`,
  `tonic`, MCP client/server, and tool-call helpers
- Python and TypeScript examples through standards-based OTLP first
- seeded demo project
- one-command Docker Compose
- copy/paste framework quickstarts
- `beaterctl smoke` to emit a known trace and verify ingestion

Public API:

- stable `/v1`
- OpenAPI-generated clients
- deprecation policy
- request IDs and idempotency keys
- pagination and time-bounded queries
- export endpoints for OTLP, Parquet, and JSONL
- import paths from Phoenix/LangSmith/Langfuse where feasible

No lock-in:

- export traces and evals without hosted dependency
- import existing datasets/traces
- keep raw source attributes for migration and round-trip use

## 15. Self-Observability SLOs

Beater dogfoods itself. Hosted cannot launch without dashboards and alerts for:

| Signal | Initial target |
| --- | --- |
| ingest accepted write success | >= 99.9 percent over 30 days |
| ingest-to-queryable lag p95 | <= 5s hosted hot path, <= 15s OSS compose |
| DLQ age p95 | <= 5m before alert |
| query p95 for indexed 24h searches | <= 1s for normal tenant workloads |
| query p95 for 30d filtered searches | <= 3s for normal tenant workloads |
| eval queue depth | bounded by project policy and worker count |
| judge spend | attributed to org/project/experiment/evaluator |
| artifact orphan rate | zero tolerated after sweeper window |
| tail-sampling decision lag p95 | <= trace completion target |

These numbers will change with load testing, but the product must expose them
from day one.

## 16. Execution Reality

### 16.1 Minimum Shippable OSS Product

The first serious open-source release needs all of this, not a smaller demo:

- Rust all-in-one `beaterd`
- OTLP and native ingest
- canonical trace schema
- immutable raw envelopes
- `TraceStore` abstraction
- SQLite/Postgres local mode
- ClickHouse scale backend
- artifact storage
- NATS JetStream durable bus
- trace table and span tree UI
- Rust SDK and tracing layer
- dataset creation from traces
- offline deterministic evals
- WASI evaluator ABI
- judge broker with BYOK
- experiment comparison with confidence intervals
- CI gate command
- Docker Compose
- import/export for OTLP, Parquet, and JSONL

That is the smallest version that can be called a serious OSS project.

### 16.2 Team Needed for Hosted SOTA

A solo founder can ship a focused OSS MVP, but not a hosted SOTA platform quickly.
The realistic team is:

- Rust infra lead
- backend/product engineer
- frontend/product engineer
- data/observability engineer
- evals/agent-systems engineer
- infra/security engineer part-time early and full-time before hosted GA

### 16.3 Hardest Problems

The hard parts are not CRUD:

- schema evolution without breaking old traces
- multi-tenant privacy and artifact security
- accurate standards translation
- ingest survivability during storage outages and traffic spikes
- evaluator reproducibility
- judge reliability and calibration
- replay correctness
- query speed over high-cardinality span volumes
- UX that makes agent failures obvious
- open-core trust and governance

## 17. Milestones

### v0: Substrate

Acceptance:

- `beaterd` starts as one binary.
- OTLP HTTP/gRPC and native ingest accept traces.
- Raw envelopes and canonical projections are both stored.
- `TraceStore` exists with SQL and ClickHouse implementations.
- NATS JetStream buffers writes and DLQ paths are visible.
- A seeded demo trace renders in the UI.
- `beaterctl smoke` proves time-to-first-trace.

### v1: OSS Observability and Offline Evals

Acceptance:

- trace table, span tree, waterfall, and agent step views work
- dataset promotion from trace/span works
- deterministic evals run in WASI sandbox
- judge broker runs BYOK LLM evals
- experiments compare baseline vs candidate with confidence intervals
- CI gate blocks regressions
- Docker Compose is the supported self-host path

### v2: Agent-Native Debugging

Acceptance:

- replay cassettes support deterministic replay where complete
- forked replay and simulation are labeled honestly
- root-cause attribution identifies the earliest likely failing span on seeded
  multi-step agent failures
- human review queues feed datasets
- online eval sampling and alert grouping work
- MCP/tool/memory spans are first-class

### v3: Hosted GA

Acceptance:

- Vercel-hosted dashboard and Rust control-plane functions
- hosted Rust cells for ingest/eval/replay
- org/project/environment isolation
- quotas, rate limits, billing ledger, and usage metering
- retention, crypto-shredding, data residency, audit, BYOK, and key rotation
- backups and restore drills
- SLO dashboards
- Slack/webhook alerts
- regional deployment story

## 18. Bar for Done

The platform is real when a team can replace ad hoc Phoenix, LangSmith,
Braintrust, and spreadsheet workflows and answer:

- What did my agent do?
- Why did it fail?
- Has this happened before?
- Can I reproduce it?
- Can I test the fix?
- Did the fix regress anything?
- Can I block deploys on that?
- Can I monitor it in production?
- Can I self-host without calling your cloud?

If any answer is no, that area is not shipped.

## 19. Planned: Execution to Parity-Grade GA

This section is the concrete, technical execution plan to take Beater from its
current state to feature parity with Arize Phoenix, Braintrust, LangSmith, and
Langfuse for deep agent evaluation. It builds on — and does not replace — the
milestones in §16–17. The milestones describe *what* must exist; this section
describes the *current measured gap* and the *specific work* to close it, at the
crate/type/endpoint level.

Every contract-touching item (new or changed `/v1` route, request/response type,
or span kind/attribute) MUST follow the §0/`CLAUDE.md` regen workflow
(`cargo xtask regen-spec` → `scripts/regen-sdks.sh` → `cargo xtask regen-semconv`
→ `scripts/check-contract-sync.sh`). Those items are tagged **[contract]** below.

### 19.1 Readiness Baseline (audited 2026-06-27)

A six-dimension audit of `main` against the parity bar. Overall readiness ≈ 33%:
strong primitives, missing product/scale/control-plane pillars.

| Dimension | Readiness | Headline gap |
| --- | --: | --- |
| Ingestion, SDKs & instrumentation | 58% | no session/thread grouping; flat scalar I/O (no message/tool-call/multimodal); no auto-instrumentation; no CrewAI/DSPy/Vercel-AI/OpenAI-Agents |
| Evaluations, datasets & reproducibility | 38% | no read APIs; no eval/dataset UI; thin scorer catalog; no prompt registry; no CI plugins |
| Security, multi-tenancy & hosted ops | 38% | no human identity/SSO; RBAC data model never enforced; audit covers one action; no deletion/retention/billing/backups |
| Experiments, statistics, online evals & alerting | 34% | one hand-rolled normal-approx; online evals sampled but never scored; alerts computed but never delivered; no Slack |
| Data model, storage, scale & query performance | 22% | SQLite-only runtime (ClickHouse/Pg unwired); full-scan queries, no LIMIT/keyset pushdown; zero benchmarks/SLOs; no runtime TTL |
| Product surface (UI, replay, annotation, prompt) | 22% | one read-only trace-waterfall page is the entire product |

Already genuinely strong (do not rebuild): OTLP HTTP+gRPC core; dual
OpenInference + OTel `gen_ai` normalizer; 4 tracing SDKs with `@observe`;
reproducibility/lineage pinning; WASI scorer sandbox; judge broker with
cost/ledger/audit; tail-sampling; crypto primitives (Argon2 keys, ChaCha20
envelope + online re-wrap, signed webhooks, BYOK); quota limiter; single-source
OpenAPI → 7 SDKs + MCP + CLI with a CI drift gate; Apache-2.0 + governance.

Biggest missing pillars: prompt management; hosted control plane
(identity/SSO/enforced RBAC); load-tested scale; product UI beyond the waterfall;
data lifecycle & compliance; online evaluation scoring; real statistics + alert
delivery; auto-instrumentation & modern-framework coverage.

### 19.2 Phase 0 — Scale & Data Plane

Goal: make a scale claim defensible. Wire the columnar store into the running
service, push filtering/pagination into the backend, prove latency, enforce TTL.

| # | Requirement | Now | Target / concrete task | Effort | Blocker |
| --- | --- | --- | --- | --- | --- |
| 0.1 | Columnar store wired into `beaterd` | `ClickHouseTraceStore`/`PgTraceStore` implemented but dead code; runtime hardcodes `SqliteTraceStore` | Add `TraceStoreBackend` env/CLI arg (`sqlite\|postgres\|clickhouse`) + `build_trace_store(cfg) -> Arc<dyn TraceStore>` in `beater-store-sql`; thread through `ApiState` and the ingest/query bins; non-ignored compose integration test booting `beaterd` on ClickHouse | L | docker |
| 0.2 | Server-side pagination + pushdown | `query_spans` appends no `LIMIT`, paginates in memory; `query_runs` materializes all spans (`limit u32::MAX`) | Push `PageRequest.limit` + time-window into SQL; keyset (seek) cursors on `(start_time, span_id)`; reimplement `query_runs` as backend `GROUP BY`; add `start_after/before` to `SpanFilter`/`RunFilter` | XL | none |
| 0.3 | Measured query p95 SLOs | no `benches/`, no criterion, no load test, no SLO evidence | New `beater-bench` crate: criterion benches for `write_batch` throughput + `query_*` latency on seeded 1M/10M/100M-span fixtures; `xtask loadgen` emitting OTLP at sustained RPS → p50/p95/p99; codify §15 SLOs + CI regression gate | XL | evidence |
| 0.4 | Runtime retention/TTL | TTL exists only as ClickHouse DDL that never runs | `RetentionPolicy{hot_days,archive_days}` in `beater-core`/`beater-schema`; retention sweeper (extend `beater-archive`) on an interval in `beaterd` demoting-then-deleting expired hot rows; `GET/PUT /v1/projects/:id/retention` **[contract]** | L | design |
| 0.5 | Automated cold-tier archival | `ParquetTraceArchive` exists, local-fs only | Write partitioned append-only Parquet (`tenant/project/yyyymm/uuid`) to object store via `beater-store-obj`; scheduled demotion job; DataFusion read path over cold files | L | design |
| 0.6 | Backend-agnostic migrations + re-normalization | versioned framework exists for SQLite only | Generalize the `SqliteMigration` version/checksum `Migrator` to ClickHouse + Postgres (`_beater_schema_migrations` on each); `xtask renormalize` reprojecting historical `RawEnvelope`s to a new canonical version | L | none |

Acceptance: `beaterd --trace-store clickhouse` boots and serves traces; a 10M-span
seeded search returns under the §15 p95 SLO in CI; expired rows are demoted then
deleted by the sweeper; benches run in CI and gate regressions.

### 19.3 Phase 1 — Agent-Native Trace Data Model

Goal: close the table-stakes agent concepts the data model lacks.

| # | Requirement | Now | Target / concrete task | Effort | Blocker |
| --- | --- | --- | --- | --- | --- |
| 1.1 | Session/thread/conversation grouping | absent from schema, normalizer, SDKs | Add `session_id/thread_id/user_id` to `CanonicalSpan`; map `session.id`/`thread.id`/`user.id` + OpenInference session attrs in `beater-otlp`; sessions index in `beater-store`; `/v1/sessions` list/get **[contract]**; `session_id` param on SDK `observe()/span()` (py/ts/go/java) | L | contract |
| 1.2 | Structured message/role/tool-call I/O | only flat `input.value/output.value` scalars | Parse OpenInference `llm.input_messages/output_messages/tool_calls` + `gen_ai.*` message events into a `CanonicalMessages` structure on `CanonicalSpan`; golden fixture tests for both dialects **[contract]** | L | contract |
| 1.3 | Multimodal (image/audio/file) I/O | stringified scalars only | `MediaArtifact{mime_type,uri-or-inline,role}` on canonical messages; parse OpenInference content-part `image_url`/`audio`; store large media via `beater-store-obj` with size caps + redaction class **[contract]** | L | design |
| 1.4 | Full-text over artifact-backed I/O | tantivy indexes only inline attrs, not artifact bodies | In `beater-search`, have the ingest processor resolve `input_ref`/`output_ref` via `ArtifactStore` and index their text into dedicated `input_body`/`output_body`/`error` fields; per-tenant shards | L | evidence |
| 1.5 | OTLP/JSON + canonical `/v1/traces` alias | OTLP HTTP is protobuf-only on a tenant-scoped path | Content-type negotiation in `ingest_otlp_http` (deserialize `ExportTraceServiceRequest` from JSON); gRPC `partial_success` population; optionally `/v1/logs` for events **[contract]** | M | contract |

Acceptance: a multi-turn agent trace groups by session in the API; a vision LLM
call renders its image; full-text search hits prompt/output bodies stored as
artifacts; a stock OTel JSON exporter ingests with no Beater SDK.

### 19.4 Phase 2 — Read APIs & Product UI

Goal: make the eval/observability backend usable as a product, not just POST
endpoints. The dashboard today is one server-rendered trace-waterfall page.

| # | Requirement | Now | Target / concrete task | Effort | Blocker |
| --- | --- | --- | --- | --- | --- |
| 2.1 | Dataset CRUD + read APIs | create-only POST; no GET | `DatasetStore` `list_datasets/get_dataset/list_versions/update_case/delete_case/import_cases`; `GET /v1/datasets[...]`, versions, cases; CSV/JSONL import **[contract]** | M | contract |
| 2.2 | Eval-report read API | reports only readable inside POST handlers | `GET /v1/datasets/.../eval-reports/{id}`, `.../versions/{vid}/eval-reports` (list+latest), paged per-case results **[contract]** | M | contract |
| 2.3 | Experiment comparison UI (with CIs) | rich backend, no UI | `web/dashboard/app/experiments/[id]` rendering `ExperimentRunReport`: per-case score table, baseline-vs-candidate deltas with `ci_low/ci_high`, gate badge, trace deep-links | L | contract |
| 2.4 | Dataset / eval-result browse UI | none | `web/dashboard/app/datasets[...]` routes: versions, cases, eval drilldown with judge rationale | XL | contract |
| 2.5 | Human annotation queues + inline scoring UI | full `beater-human` backend, no UI | `web/dashboard/app/review` (queue + task inbox) + inline `AnnotationPanel` on span detail posting `submitReviewAnnotation`; keyboard labeling | L | none |
| 2.6 | Failed-vs-passed trace diff | none | `GET /v1/traces/:tenant/:a/diff/:b` aligning spans by name/kind/seq emitting per-span deltas **[contract]**; `web/dashboard/app/diff` side-by-side view | L | contract |
| 2.7 | Cost/latency analytics dashboard | single-run summary strip only | `GET /v1/metrics/:tenant` timeseries (p50/p95/p99, cost/token trends, model/release breakdown) **[contract]**; `web/dashboard/app/analytics` charts | L | contract |
| 2.8 | Search UI + saved views | strong filter form, no full-text UI | `web/dashboard/app/search` + `searchSpansPath()` calling `/v1/search/:tenant/spans`; attribute-predicate query bar; saved views | M | none |
| 2.9 | Client interactivity (live tail, virtualized) | fully server-rendered, GET-form nav | client components (SWR/react-query) over read APIs; SSE/websocket live-tail on `/v1/traces`; virtualized span lists | L | none |

Acceptance: a user can browse datasets, open an experiment and see per-case
deltas with CIs and a gate badge, annotate a trace in a review queue, diff a
failed vs passing trace, and watch cost/latency trends — all in the UI.

### 19.5 Phase 3 — Eval Depth & Statistics

Goal: scorer breadth and statistically defensible experiments.

| # | Requirement | Now | Target / concrete task | Effort | Blocker |
| --- | --- | --- | --- | --- | --- |
| 3.1 | Scorer catalog breadth | 10 scorers; `json_object` checks object-ness not schema | Add `FuzzyMatch{min_ratio}` (strsim), `JsonSchema{schema}`, `NumericTolerance{abs,rel}`, `EmbeddingSimilarity{model,min_cosine}` (judge lane), SQL-result match to `EvaluatorKind`/`EVALUATOR_CATALOG` **[contract]** | L | contract |
| 3.2 | Structured-rubric LLM judge | `LlmJudge{rubric:String}` free-text | `JudgeRubric{criteria:[{name,weight,scale}],reference_mode,exemplars}`; `JudgeResponse.per_criterion`; reference-guided + CoT rationale **[contract]** | L | contract |
| 3.3 | Custom scorer registry | WASI sandbox runs components, no upload/registry | `beater-scorers` (or extend `beater-eval`): `ScorerStore` (upload component bytes → `Sha256Hash`, version, list/get) on `beater-store-obj`+sqlite; `/v1/scorers` CRUD **[contract]**; resolve by `wasm_hash` into the sandbox; add memory/epoch limits to `SandboxConfig` | XL | contract |
| 3.4 | Real statistics module | single paired normal-approx, hardcoded z, Bonferroni-only | New `beater-stats` on `statrs`: paired-t / bootstrap-percentile / Wilson CIs; test selection `{PairedT, McNemarExact, WilcoxonSignedRank, Bootstrap}` with real `p_value`; Holm-Bonferroni + Benjamini-Hochberg; `power.rs` (`required_sample_size`, `achieved_power`) | L | none |
| 3.5 | Experiment depth | single metric, no segments | Multi-named-metric + segment tags on `ExperimentRunReport`; `ExperimentStore::list_runs` + `GET /v1/experiments/:tenant/:project` **[contract]**; per-slice comparison | M | contract |
| 3.6 | CI integration | none | `sdks/python/beater/pytest_plugin.py` (`@beater.eval` marker running cases through the API, asserting via `GatePolicy`); TS vitest reporter; `beater eval` gating CLI subcommand | L | contract |

Acceptance: an experiment reports a delta with a method-appropriate CI and real
p-value, FWER-corrected across metrics, refusing underpowered comparisons; a
custom uploaded scorer runs sandboxed; `pytest`/`beater eval` fails CI on
regression.

### 19.6 Phase 4 — Online Evals, Alerting & Prompt Management

Goal: production scoring, real alert delivery, and the missing prompt pillar.

| # | Requirement | Now | Target / concrete task | Effort | Blocker |
| --- | --- | --- | --- | --- | --- |
| 4.1 | Online evals that score | sampling decision only, never scored | `beater-online` worker (or `beater-temporal` workflow) consuming tail-sampled traces, running configured deterministic+judge evaluators, persisting online-tagged `EvalResult`s; `GET /v1/online/.../scores` timeseries **[contract]** | XL | design |
| 4.2 | Alert policy persistence + CRUD | policies passed inline; nothing stored | `AlertPolicyStore` (sqlite+sql) + `POST/GET/PATCH/DELETE /v1/alert-policies/...`; persist `OnlineSamplingPolicy` per project; load in `evaluate_alert`/ingest **[contract]** | L | contract |
| 4.3 | Actual webhook delivery | `WebhookDelivery` computed, never sent | delivery worker POSTing with retry/backoff + `beater-security` HMAC signature; persist attempts/status; delivery-history endpoint | M | evidence |
| 4.4 | Slack integration | zero references | `SlackChannel` formatting `AlertInput` into Block Kit (severity, score-vs-baseline, trace deep-link button); stored incoming-webhook config | M | evidence |
| 4.5 | Baseline/anomaly/drift alerting | static threshold only | `AlertCondition{AbsoluteThreshold, BaselineDeviation, Drift}` with rolling EWMA/z-score/percentile baseline over recent project scores | L | design |
| 4.6 | Durable dedupe/grouping | in-memory `AlertState` | back `AlertState` with the store so dedupe survives restarts + is shared across workers; group rollups in payload | M | none |
| 4.7 | Prompt management | `prompt_version_id` is a dangling pin, no producer | New `beater-prompts`: `PromptRegistry`, versioned `PromptTemplate`, variable schema, tags, diff; `/v1/prompts` CRUD + `runPrompt` (playground) **[contract]**; `web/dashboard/app/prompts` registry + playground + prompt-from-trace; resolve `prompt_version_id` at eval time | XL | contract |

Acceptance: sampled production traces get scored on a schedule with a visible
trend; an alert policy persists, fires on baseline deviation, and is actually
delivered to Slack with a trace link; a prompt can be created, versioned,
diffed, run in a playground, and linked to an eval run.

### 19.7 Phase 5 — Hosted Control Plane & Compliance (Enterprise GA)

Goal: everything required before hosted multi-tenant GA can be sold (see §13, §17.3).

| # | Requirement | Now | Target / concrete task | Effort | Blocker |
| --- | --- | --- | --- | --- | --- |
| 5.1 | Orgs/projects/environments CRUD | id types + `TenantScope` only | `POST/GET/DELETE /v1/organizations\|projects\|environments` on `MetadataStore`; membership; org/project switcher in UI/SDK **[contract]** | L | contract |
| 5.2 | Human identity + enforced RBAC | `RoleBinding` data model never consulted by `authorize()` | `beater-rbac` (or extend `beater-auth`): `Role`/`Permission` + `resolve_permissions(principal, scope)` called inside `authorize()` on every mutating route; users + memberships; member/role-grant endpoints **[contract]**; conformance tests | XL | contract |
| 5.3 | SSO / SAML / SCIM / OIDC | none | `beater-identity`: OIDC (auth-code+PKCE) + SAML2 SP + SCIM 2.0 `/Users`/`/Groups`; session/JWT issuance; per-org IdP config; enforced-SSO toggle; JIT provisioning | XL | design |
| 5.4 | Storage-layer tenant isolation + secure default | app-enforced `WHERE tenant_id=?`; default auth effectively open | hosted store on Postgres with Row-Level Security keyed on per-request `SET app.tenant_id`; conformance test proving cross-tenant reads fail at the DB; make `Required` auth the default for non-localhost binds | XL | design |
| 5.5 | Data deletion / crypto-shred / GDPR | no DELETE routes, no erasure | per-tenant data-encryption keys for crypto-shred; `DELETE /v1/tenants/{id}` (key destroy + cascade), `DELETE /v1/traces/{id}`; background purge worker; deletion audit events **[contract]** | XL | contract |
| 5.6 | Data residency / regional | single-region placeholder | `region` on `OrganizationMetadata`; region-aware gateway routing to home-region backends; per-region object/DB stores; EU/US topology doc | XL | design |
| 5.7 | Comprehensive tamper-evident audit | covers exactly one action (`PiiUnmask`) | expand `AuditAction` (key/secret/role/config/export/login/auth-failure); emit from `beater-auth`/`beater-secrets`/RBAC/login; hash-chained tamper-evident column; `GET /v1/audit-events` **[contract]** | L | contract |
| 5.8 | Billing / usage ledger | idempotent ledger exists; no plans/invoicing | meters for ingest/storage/eval/judge; per-org rollups; `beater-billing` (plan/subscription + Stripe metered sync) linked to `QuotaLimiter` | L | contract |
| 5.9 | Backups + restore drills | none | hosted on Postgres+object store with PITR; `beaterctl backup`/`restore` for self-host; CI restore-drill job with documented RPO/RTO | L | evidence |
| 5.10 | SLO dashboards + dogfooding | Prometheus facade exists | Grafana dashboard JSON + Prometheus alert rules under `ops/`; self-trace OTLP exporter so `beaterd` traces into a Beater project; load test producing the §15 numbers | M | evidence |
| 5.11 | Governance / SOC2 controls | LICENSE + GOVERNANCE only | `SECURITY.md` (coordinated disclosure); `docs/compliance/` SOC2 control matrix, access-review runbook, incident-response plan, subprocessor list, DPA template | M | evidence |
| 5.12 | KMS-backed BYOK + at-rest rotation for blobs | ChaCha20 envelope for secrets only | KMS `Keyring` (AWS/GCP CMK wrap) behind `SecretKeyring`; extend envelope encryption to trace I/O blobs + PII fields; concurrency-safe rotation across stores | XL | design |

Acceptance: a non-owner is denied a mutating route by enforced RBAC; SSO login
provisions a user; a cross-tenant query fails at the database; a tenant can be
crypto-shredded and proven unreadable across hot/cold/artifact stores; billing
totals drive quota; a restore drill passes; SLO dashboards show live numbers.

### 19.8 Phase 6 — Auto-Instrumentation & Ecosystem Breadth

Goal: lower adoption friction to match the incumbents' framework coverage.

| # | Requirement | Now | Target / concrete task | Effort | Blocker |
| --- | --- | --- | --- | --- | --- |
| 6.1 | Auto-instrumentation (OpenAI/Anthropic) | one-line `wrap_*` wrappers only | `beater.auto.instrument(providers=[...])` monkeypatching `openai`/`anthropic` (incl streaming + tool calls) in py + ts | L | none |
| 6.2 | Zero-code env-var bootstrap | all paths require code | `opentelemetry-distro`/configurator (py) + TS `--require` preload reading `BEATER_*` env, setting OTLP exporter+headers, enabling installed auto-instrumentors | M | none |
| 6.3 | Modern framework coverage | LangChain (py+ts), LlamaIndex (py) only | examples + instrumentation for Vercel AI SDK (TS), OpenAI Agents SDK, CrewAI, DSPy, Pydantic AI, AutoGen, Haystack; TS LlamaIndex; token-usage extraction; 3-level span-tree integration tests | XL | evidence |
| 6.4 | `beaterctl quickstart` (TTFT) | manual compose + snippet | one command boots compose, provisions tenant/key, prints exporter snippet + dashboard URL; timed e2e asserting trace visible < SLO | M | evidence |

Acceptance: an env-var-only Python app produces traces with zero code edits;
each named framework has a working example emitting a correct agent span tree;
`beaterctl quickstart` demonstrates time-to-first-trace under the §14 SLO.

### 19.9 New Crates, Contracts & Sequencing

New crates introduced by this plan (all under the §4 workspace conventions):

- `beater-bench` — criterion benches + load-test fixtures (Phase 0).
- `beater-stats` — CIs, test selection, p-values, power, FWER/FDR (Phase 3).
- `beater-scorers` — custom-scorer registry over the WASI sandbox (Phase 3).
- `beater-online` — online-eval scoring worker (Phase 4).
- `beater-prompts` — prompt registry/versioning/playground (Phase 4).
- `beater-rbac` — role/permission resolution wired into `authorize()` (Phase 5).
- `beater-identity` — OIDC/SAML/SCIM (Phase 5).
- `beater-billing` — plans/subscriptions/Stripe metered sync (Phase 5).

Sequencing rationale (each phase unblocks the next):

```text
Phase 0  scale & data plane     -> every scale/latency claim depends on it
Phase 1  agent data model       -> sessions/messages/multimodal feed UI + evals
Phase 2  read APIs + product UI -> makes the eval/observability backend usable
Phase 3  eval depth + stats     -> defensible experiments and scorer breadth
Phase 4  online + alerts + prompts -> production loop + the prompt pillar
Phase 5  hosted control plane   -> enterprise multi-tenant GA (gates §17 hosted)
Phase 6  ecosystem breadth      -> adoption parity; can run partly in parallel
```

Cross-cutting bar for every item (no exceptions):

- Contract-touching changes regenerate spec + 7 SDKs + semconv and pass
  `scripts/check-contract-sync.sh` (CI-gated). These need Docker for
  `regen-sdks.sh`.
- Every non-trivial change lands with a runnable test; `cargo clippy
  --all-targets -D warnings` is clean (the workspace denies `unwrap`/`expect`,
  including in tests).
- New scale/perf claims ship with a benchmark or load test, never an assertion.
- Tenant isolation, redaction, and audit are never weakened to ship a feature.

Done, per §18, is when a team can replace ad-hoc Phoenix/LangSmith/Braintrust
workflows end to end. This plan is the path from 33% to that bar.

## 20. Planned: Recursive Self-Improvement MCP & Agent Studio

This is a second product surface layered on the Beater eval/judge/trace/dataset/
replay primitives (§9–§12, §19): a single MCP server that gives an agent — driven
by Claude Code or any MCP client — a recursive self-improvement loop, plus a
visual Studio to design, watch, and edit agents. The thesis: **"a tool belt that
generates tool belts."** The MCP reuses Beater for traces, evals, judges,
datasets, and replay; it does not reinvent them.

Design invariants (carried from §1):

- **Human-in-the-loop by default.** The loop runs as plan → approve → execute:
  the MCP indexes the agent, reports what it found ("is this correct? which of
  1–6 are you OK changing?"), and only then iterates. Full autonomy is opt-in.
- **Generalize, do not overfit.** Improvements must target the stated goal and
  generalize across inputs, not curve-fit the current trace/dataset. The loop is
  policy-aware: load-bearing prompts/tools are not changed unless contradictory.
- **Standards + reuse at the edge.** Scoring is Beater's existing LLM-judge +
  deterministic WASI evals; memory/tools are provisioned, not hand-rolled.
- **MCP-first, SDK-second.** Recommend the MCP to learn the workflow, then expose
  a deterministic SDK for repeatable monitoring/improvement pipelines.

### 20.1 The MCP Server (`beater-mcp-improve`)

A single MCP exposing a tool-belt for recursive self-improvement. Every tool call
is a metered self-improvement action (see §20.6). Core tools:

- `index_agent` — discover the agent's code, config, system/UI/customer prompts,
  policy, tools, and runtime (localhost, API logs, browser) and build a map from
  symptom → corresponding code/prompt/data.
- `propose_change` — given a goal + traces + evals, propose a typed change:
  `{prompt | system_prompt | customer_prompt | code | tool_add | tool_remove |
  data_label | memory_config}`, with rationale and the exact file/symbol/span it
  targets. Returns a plan, never a silent edit.
- `simulate` — run N candidate iterations through Beater's harness (§11) with
  judge + simulation loops; report the score gradient (LLM-judge + deterministic)
  and whether a change is promising before it touches the repo.
- `apply_change` — wire the approved change at a chosen integration depth
  (suggest-only → wire a Studio node → edit repo code), collaborating with Claude
  Code for the actual code write.
- `track_evolution` — record the agent's version history (tools added/removed,
  prompts rewritten, labels challenged) so the loop can see its own trajectory.
- `challenge_labels` — flag dataset labels the evidence contradicts; route to the
  human grader (§20.5).

### 20.2 Auto-Provisioned Tool-Belt (`beater-toolbelt`)

OAuth in, and the platform auto-provisions agent capabilities on demand — the
"pop-up" experience:

- **Vector memory** — one-click managed vector DB; the loop can propose "this
  agent would benefit from vector-search memory" and simulate a few iterations
  before committing.
- **SQL store**, **web search**, **scrapers**, and common agent tooling as
  built-ins, auto-wired into the agent and addressable by the improvement loop.
- Tools are discoverable, versioned, and applied/removed by `propose_change`/
  `apply_change`; provisioning is metered.

### 20.3 The Self-Improvement Loop

```text
goal + params + few examples
  -> index_agent (code + prompts + policy + runtime)
  -> collect traces/evals (Beater) + classify failures
  -> propose_change (typed, goal-targeted, generalizable)
  -> simulate (judge loops + simulation loops -> score gradient)
  -> human approve (which changes; depth)
  -> apply_change (Claude Code / hosted writer) + re-eval
  -> track_evolution -> repeat
```

The gradient is "where is the best performance" by LLM-judge **and** deterministic
eval (the latter trusted only where state is known-correct). Anti-overfit and
policy-awareness gate every accepted change.

### 20.4 Integrations & Code-Awareness

- **Runtime introspection:** aware of where localhost runs; can open the browser,
  read API logs from the user's codebase, and locate the responsible stack layer.
- **Frameworks:** direct link to browser-use; Temporal (sub-agent trace steps map
  cleanly to canonical spans); LangChain / LangGraph. Auto-discover internal
  workflows and classify their traces into improvement candidates.
- **Integration depths:** (1) suggest-only, (2) wire a Studio node, (3) change
  actual repo code — chosen per change.

### 20.5 Agent Studio (`beater-studio`)

A visual surface that maps front-end ↔ back-end:

- **Canvas** (Excalidraw-style, mostly native): agent design auto-drawn as nodes,
  **topologically sorted left→right**, with explicit visualization of recursive
  self-improvement loops.
- **JSON-schema-first:** every node/edge is backed by JSON schema stored in the
  backend; Claude Code assists with the schema via the MCP. A canonical
  "good workflow" example + a skills doc the MCP/Claude Code pull from.
- **Studio mode:** watch the agent run, see traces live, drag tools in; Claude
  Code wires them (AI tier: a hosted agent wires them).
- **Human grading:** an expert feedback area to grade right/wrong inline, feeding
  `challenge_labels` and calibration (§9.3).

### 20.6 Commercial Model & Metering (DRAFT — not for public publish until confirmed)

Metering counts MCP tool calls / endpoint calls as "requests"; AI credits meter
model spend (judge/code-writer). Margin target: large; the $20 plan is roughly
break-even at full utilization.

| Plan | Price | Requests/mo | Included AI credits | Overage |
| --- | --- | --- | --- | --- |
| Free | $0 | 5,000 | $5 | — |
| Starter | $8/mo | 8,000 | — | — |
| Pro / AI | $20/mo | 50,000 | $40 | pay-as-you-go credits |
| Usage (AI) | metered | — | per plan above | pay-as-you-go |

Two metered dimensions:

- **Requests** — MCP tool calls / endpoint calls, capped per plan per month.
- **AI credits** — model spend (judge + code-writer); Free includes $5/mo, Pro
  includes $40/mo, beyond which it is pay-as-you-go.

**Rolling-window rate limiting (Claude-Code/Codex-style).** On top of the monthly
caps, both tiers enforce **rolling 5-hour and weekly windows** computed from a
multi-factor cost (tool-call count, tokens, model tier, simulation depth), so
bursty usage is smoothed and abuse is bounded without a hard monthly cliff. The
windows reset continuously (seek-based), not on calendar boundaries.

Margin target: large; the $20 Pro plan is roughly break-even at full utilization.

Requires: a metering/credits service (`beater-credits`) over the existing
`beater-usage` ledger (§9 usage records) + `QuotaLimiter` (§7.4) with rolling
5h/weekly windows, plan tiers, and Stripe metered billing (ties into §19.7 5.8).

### 20.7 New Crates & SDK

- `beater-mcp-improve` — the self-improvement MCP server + tool-belt protocol.
- `beater-toolbelt` — auto-provisioned vector/SQL/web/scraper tools.
- `beater-studio` — Studio canvas UI (Next.js) + JSON-schema store.
- `beater-credits` — request + AI-credit metering, plan tiers, Stripe sync.
- Deterministic **improvement SDK** (py/ts) over the same endpoints for repeatable
  monitoring/improvement pipelines.

### 20.8 Phasing & Acceptance

- **MVP:** `beater-mcp-improve` with `index_agent`/`propose_change`/`simulate`/
  `apply_change`, wired to Beater evals/judge/harness, plan→approve→execute,
  metering on tool calls. Acceptance: from a goal + a small agent (system prompt +
  policy), the MCP indexes it, proposes a generalizable change, simulates a score
  gain, and applies it via Claude Code with human approval.
- **+1:** auto-provisioned tool-belt (vector/SQL/web) + browser-use/Temporal
  integration.
- **+2:** Studio canvas (topo-sorted nodes, JSON schema, live traces, drag-to-add)
  + human grading.
- **+3:** deterministic SDK, LangGraph integration, credits/billing tiers GA.

This product depends on Phases 0–4 of §19 (scale, data model, read APIs, evals/
stats, online evals) being far enough along that traces and evals are real inputs
to the loop.
