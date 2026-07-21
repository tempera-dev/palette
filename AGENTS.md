# Palette Agent Context

Use this file as startup context for Codex, Claude Code, Cursor, Copilot, and
other coding agents. It is intentionally compact. For the full plan, read
`ARCHITECTURE.md`; for contribution rules and CI gates, read `CONTRIBUTING.md`;
for the hard contract-regeneration rule, read `CLAUDE.md`.

## What Palette Is

Palette is a Rust-first open-source agent observability, replay, eval, and
recursive-improvement platform with an optional hosted edition. The product loop
is:

```text
instrument agent -> inspect trace/span tree -> promote failure to dataset
-> run evals -> compare candidate -> gate CI -> monitor production
```

The OSS edition must run locally without Palette Cloud. Hosted scale, billing,
SSO, managed cells, and enterprise governance sit outside the Apache-2.0 core.

## Repo Shape

- `Cargo.toml` is a Rust workspace. The main runtime is one `paletted` binary in
  `bins/paletted`; `bins/palettectl` is the CLI and smoke/fixture entrypoint.
- `crates/*` are modular Rust libraries for schema, ingest, OTLP, storage,
  durable bus, API, MCP, evals, replay, auth/security, datasets, experiments,
  gates, human review, alerts, usage, audit, browser capture, and `xtask`.
- `web/dashboard` is the Next.js dashboard. It consumes generated OpenAPI types
  from `web/dashboard/openapi/palette-read-api.json` and
  `web/dashboard/lib/generated/api-types.ts`.
- `sdks/openapi/palette-api.json`, `sdks/clients/*`, `sdks/{python,typescript,rust}`,
  and `sdks/semconv` are the API/SDK contract surface. Generated client output is
  intentionally committed when the contract changes.
- `migrations/{sqlite,postgres,clickhouse}` are durable schema contracts. The
  current OSS runtime uses SQLite under `--data-dir`; Postgres/ClickHouse are the
  scale path and migration contracts.
- `scripts/*` contains drift checks, SDK regen, compose/browser smoke gates, and
  release helpers. `.github/workflows/*` mirrors those gates in CI.

## Repo Nuances

- Ship one Rust binary first. Keep service splits as thin bins over shared crates
  and justify them only with measured scale pressure.
- Use standards at the edge: OTLP, W3C trace-context, OpenInference, OTel GenAI,
  and common framework exporters are first-class. Native SDKs accelerate adoption
  but are not the onboarding gate.
- Preserve both immutable raw envelopes and normalized projections. Queries/UI use
  canonical projections; lossless round-trip lives in the raw copy.
- Privacy and tenancy are design constraints: scoped keys, tenant/project/env
  isolation, redaction, audited PII unmasking, retention, crypto-shred, and
  artifact encryption matter even in OSS.
- Scope Vercel correctly. Vercel hosts the dashboard and hosted stateless
  control-plane/edge surfaces only; long-running ingest, eval, replay, queue
  consumers, and stateful workers run in `paletted` or hosted Rust cells.
- The contract is the source of truth. `/v1` handlers in `crates/palette-api`
  generate the OpenAPI spec; the spec feeds SDKs, MCP, CLI, dashboard/docs, and
  drift checks. Do not hand-edit generated clients/spec snapshots.

## Common Agent Commands

```bash
# Rust formatting and focused tests
cargo fmt --all
cargo test -p palette-api
cargo test -p palette-api --test openapi_coverage
cargo test -p palette-store-conformance
cargo run -q -p palettectl -- smoke --data-dir /tmp/palette-smoke

# Contract regeneration and drift checks
cargo xtask regen-spec
scripts/regen-sdks.sh
cargo xtask regen-semconv
scripts/check-contract-sync.sh

# Dashboard checks
cd web/dashboard
npm ci
npm run generate:api
npm test
npm run build
```

For Vercel-specific dashboard behavior, run from `web/dashboard` and keep
`PALETTE_API_BASE_URL` pointed at a running `paletted`. Vercel is not the backend
runtime for `paletted`.

## Guardrails

- Do not edit `ARCHITECTURE.md` requirements casually. Treat it as the build-ready
  plan and keep changes intentional, reviewed, and scoped.
- Do not hand-edit generated SDKs, OpenAPI snapshots, dashboard generated API
  types, or `sdks/semconv/conventions.json`; regenerate from source and commit the
  generated output in the same contract change.
- Keep PRs feature-scoped. Generated churn required by a contract change belongs
  in that PR; unrelated cleanup does not.
- Use `gh` CLI for GitHub work when asked. Do not rely on broad grep when this
  file, `CLAUDE.md`, `CONTRIBUTING.md`, or `ARCHITECTURE.md` has the answer.
- Avoid broad refactors unless they directly serve the requested change and the
  affected crate boundary.

<!-- TEMPERA-AGENT-KIT:BEGIN -->
## Tempera shared contract discipline

This managed block is copied from the reviewed Tempera agent kit. Preserve
repository-owned guidance outside the managed markers.

- Treat runtime handlers and their canonical committed contract as the source of
  truth. Skills, SDK tables, MCP catalogs, and prose cannot authorize a route or
  scope.
- Synchronize public changes in order: runtime parity, exact source lock,
  generated clients, deliberately admitted MCP capabilities, independent
  consumers, and exact-SHA integration evidence.
- Reject dirty-checkout provenance, moving-ref-only locks, one-directional drift,
  invented downstream scopes, unexplained generated diffs, and checks that did
  not execute on a runner.
- Record compatibility, owner, migration, rollout, rollback, and affected
  consumers. A breaking producer waits for review-ready consumers and a staged
  exact-SHA receipt.
- Keep MCP model-facing exposure curated. REST coverage is not an MCP target;
  every admitted tool needs explicit scope, schema, effect, and guard evidence.
- Respect repository ownership and independent Workflows/UI and Cyber program
  boundaries. Send exact contract and fixture handoffs instead of editing their
  product behavior without acceptance.
- Run the repository's agent-kit check in CI. Update generated files only through
  the synchronizer and never replace them with cross-worktree symlinks.
<!-- TEMPERA-AGENT-KIT:END -->
