# Performance Budgets — Placeholder Targets

> **STATUS: PLACEHOLDER TARGETS — no runtime wiring.**
>
> Every budget below is a design-time target derived from the §16 SLO table and
> §23 performance narrative in `ARCHITECTURE.md`. None of these numbers is
> measured today; the `beater-bench` crate that will enforce them at CI time is
> `[planned]` (§20.2 #0.3, §23.10, §24.1 "beater-bench SLO gates green"). This
> document pairs with that planned crate so that when the criterion benches and
> `xtask loadgen` fixtures land, the budget table is already written and
> reviewers can verify the bench gate against it. Do not treat any row as
> measured evidence until the "Status" column reads **enforced** and points to a
> passing CI gate.

## Table of Contents

1. [Governing sources](#governing-sources)
2. [Budget table](#budget-table)
3. [Benchmark function naming conventions](#benchmark-function-naming-conventions)
4. [Promotion path (placeholder → enforced)](#promotion-path)
5. [What is explicitly out of scope](#what-is-explicitly-out-of-scope)

---

## Governing sources

All targets trace back to one of two authoritative sources:

| Source | Location |
|---|---|
| §16 Self-Observability SLOs | `ARCHITECTURE.md` §16 |
| §23 Performance, Concurrency & Scalability | `ARCHITECTURE.md` §23 |
| §23.10 Perf observability + SLO gates | `ARCHITECTURE.md` §23.10 |
| §20.2 #0.3 Measured query p95 SLOs | `ARCHITECTURE.md` §20.2 |
| §24.1 "Scale: filtered search p95" | `ARCHITECTURE.md` §24 |

The `beater-bench` crate location (once created) will be `crates/beater-bench/`.
The Roadcase xtask loadgen lives at `crates/xtask/` (§23.10, `ARCHITECTURE.md` §4).

---

## Budget table

Each row names the hot path, the placeholder p95 / p99 target, the future
Criterion benchmark function that will enforce it, and the architecture section
that defines or motivates the budget.

| Path | Budget p95 | Budget p99 | Future bench function | Architecture § | Status |
|---|---|---|---|---|---|
| **OTLP HTTP ingest** — `POST /v1/traces/:tenant` single batch, hot path | < 50 ms | < 100 ms | `bench_otlp_ingest_http_p95` | §9, §23.3, §16 (ingest-to-queryable lag ≤ 5 s) | placeholder |
| **OTLP gRPC ingest** — `ExportTraceServiceRequest` single batch, hot path | < 50 ms | < 100 ms | `bench_otlp_ingest_grpc_p95` | §9, §23.3, §3 (tonic/prost) | placeholder |
| **`write_batch` throughput** — `TraceStore::write_batch` at sustained RPS on 1M-span fixture | ≥ 5 000 spans/s | — | `bench_write_batch_throughput` | §8, §23.5, §20.2 #0.3 | placeholder |
| **Indexed 24 h span query** — `query_spans` on a 1M-span fixture, time-bounded 24 h window | < 1 s | < 2 s | `bench_query_spans_24h_p95` | §16 (query p95 ≤ 1 s), §23.5, §20.2 #0.3 | placeholder |
| **30-day filtered span query** — `query_spans` on a 10M-span fixture, 30-day window + predicate filter | < 3 s | < 6 s | `bench_query_spans_30d_p95` | §16 (query p95 ≤ 3 s), §23.5, §20.2 #0.3 | placeholder |
| **Filtered search at scale** — 10M-span seeded filtered search end-to-end in CI | < 1 s | < 2 s | `bench_filtered_search_10m_p95` | §20.2 #0.3, §24.1 "Scale: filtered search p95", §23.5 | placeholder |
| **Ingest-to-queryable lag** — wall-clock from OTLP write-ack to the span appearing in `query_spans` | < 5 s | < 10 s | `bench_ingest_to_queryable_lag_p95` | §16 (lag ≤ 5 s hosted hot path), §9, §23.5 | placeholder |
| **MCP `tools/list` dispatch** — single JSON-RPC `tools/list` call to the in-memory spec/op cache | < 5 ms | < 10 ms | `bench_mcp_tools_list_p95` | §23.2 (low single-digit ms target), §16 (planned once §20.2 #0.3 bench lands) | placeholder |
| **MCP tool dispatch-ack** — JSON-RPC tool call returning a job handle (not the long-running job itself) | < 10 ms | < 20 ms | `bench_mcp_tool_dispatch_ack_p95` | §23.2 (dispatch returns handle immediately; never blocks), §16 | placeholder |
| **Dataset case list** — `GET /v1/datasets/:id/versions/:v/cases` 100-case page | < 200 ms | < 400 ms | `bench_dataset_case_list_p95` | §23.5 (keyset pagination), §20.4 #2.1 | placeholder |
| **Eval run dispatch** — time to enqueue a deterministic WASI eval run and return a job handle | < 50 ms | < 100 ms | `bench_eval_run_dispatch_p95` | §23.4 (dispatch to worker pool, immediate handle), §21 | placeholder |
| **Judge broker round-trip** — single judge call through the pooled broker (cache miss path) | < 2 s | < 5 s | `bench_judge_broker_roundtrip_p95` | §23.4 (per-provider pools, rate-capped), §16 (judge spend attributed) | placeholder |
| **SDK span enqueue** — `observe()` / `span()` hot path enqueue to bounded in-memory buffer | < 1 µs | < 5 µs | `bench_sdk_span_enqueue_p95` | §23.3 (O(1) enqueue, no network on hot path), §15 DX SLO | placeholder |
| **`xtask loadgen` sustained OTLP** — loadgen emitting OTLP at sustained RPS; p50/p95/p99 reported | TBD by baseline | TBD by baseline | `loadgen_otlp_sustained_rps` (xtask, not criterion) | §23.10, §20.2 #0.3 | placeholder |

---

## Benchmark function naming conventions

All Criterion benchmarks live in `crates/beater-bench/benches/`. The naming
scheme is:

```
bench_<path_slug>_<metric>
```

- `<path_slug>` — snake-case description of the hot path being measured.
- `<metric>` — `p95`, `p99`, `throughput`, `roundtrip`, etc.

Benchmark groups map to test areas and CI gates (§22.5, §23.10):

| Group | Bench file (planned) | CI gate |
|---|---|---|
| ingest | `beater-bench/benches/ingest.rs` | `backend` bench gate |
| store read | `beater-bench/benches/store_read.rs` | `backend` bench gate |
| mcp dispatch | `beater-bench/benches/mcp_dispatch.rs` | `backend` bench gate (or `sdk-contract`) |
| sdk hot path | `beater-bench/benches/sdk_hotpath.rs` | `sdk-contract` gate |
| eval dispatch | `beater-bench/benches/eval_dispatch.rs` | `backend` bench gate |

The `xtask loadgen` target is a separate integration fixture, not a Criterion
bench; it runs via `cargo xtask loadgen` (the Roadcase crate at `crates/xtask/`)
and emits a load report artifact consumed by the `backend` CI gate (§23.10).

---

## Promotion path

A row moves from **placeholder** to **enforced** when:

1. `crates/beater-bench/` exists and the named bench function compiles.
2. The bench runs in CI and produces p95/p99 measurements against the
   fixture (1M / 10M span as appropriate per §20.2 #0.3).
3. The Metronome gate (§22.5) is configured to fail the `backend` bench gate
   if the p95 regresses past the budget.
4. The "Status" column in this table is updated to **enforced** with a link to
   the CI gate.

Until step 4 is complete the row is still a placeholder, even if the bench
function exists. The budget is not enforced until the gate is wired.

---

## What is explicitly out of scope

- **Judge LLM completion latency** — this is a provider SLO (OpenAI, Anthropic,
  etc.), not a Beater SLO. The judge broker round-trip budget above covers only
  the broker's own dispatch overhead (pool acquire, cache lookup, request
  serialization), not the provider's time-to-first-token.
- **Cold-start / process boot time** — `beaterd` start time is a DX concern
  (§15), not an SLO path. It is not benched here.
- **Client-side SDK latency on third-party runtimes** — the SDK enqueue bench
  covers the Rust SDK; Python/TypeScript/Go SDK enqueue is a separate concern
  per the respective client crates.
- **Dashboard render performance** — front-end Web Vitals (LCP, CLS, INP) are
  §25 Soundstage concerns tracked separately from backend SLOs.
