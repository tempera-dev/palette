# CI Check Ledger

Every check that runs in GitHub Actions for this repository, derived directly
from `.github/workflows/*.yml`.  Keep this table accurate by running
`scripts/report-ci-checks.sh` after adding or renaming a workflow.

| Check name | Workflow file | Job id | Required / Optional / Advisory | Trigger | What it gates | External? |
|---|---|---|---|---|---|---|
| backend tests | `backend.yml` | `backend-tests` | Required | push to `main`, PR, dispatch | Rust workspace unit/integration tests (excl. beaterd gate targets + browser crates); beaterd `sqlite_migrations` test; `sdks/rust` example compile | No |
| backend lint | `backend.yml` | `backend-lint` | Required | push to `main`, PR, dispatch | `cargo fmt --check` + `cargo clippy` (deny-lints: no unwrap/expect, forbid unsafe) for the whole workspace (excl. beaterd) | No |
| browser tests | `browser.yml` | `browser-tests` | Required | push to `main`, PR, dispatch | `beater-browser-*` crate unit/integration tests (no real browser required) | No |
| e2e tests | `browser.yml` | `e2e-tests` | Required | push to `main`, PR, dispatch | Live Chrome (CDP) + Playwright + `browser-use` (Python) + Stagehand (TS) SDK span mapping via `scripts/browser-e2e.sh` | No |
| build (amd64) | `container-images.yml` | `build` (matrix: `amd64`) | Advisory | push to `main`, dispatch | Builds and pushes `beaterd`, `dashboard`, `dashboard-e2e`, and `otel-python` amd64 images to GHCR; runs Gate 2 public-handoff verifier | No |
| build (arm64) | `container-images.yml` | `build` (matrix: `arm64`) | Advisory | push to `main`, dispatch | Same as above for arm64 on `ubuntu-24.04-arm` | No |
| publish | `container-images.yml` | `publish` | Advisory | push to `main`, dispatch (needs `build`) | Creates multi-arch manifests for the four images; runs `scripts/check-gate2-public-handoff.py` | No |
| fly deploy | `deploy-backend.yml` | `deploy` | Advisory | push to `main` (crates/bins/Dockerfile/fly.toml paths), dispatch | Deploys `beaterd` to Fly.io and smoke-checks `/health`; no-ops (green) when `FLY_API_TOKEN` secret is absent (forks) | No |
| vercel deploy (prod) | `deploy-dashboard.yml` | `deploy` | Advisory | push to `main` (`web/dashboard/**` paths), dispatch | Deploys Next.js dashboard to Vercel production; no-ops when `VERCEL_TOKEN` is absent | Yes — Vercel |
| frontend tests | `frontend.yml` | `frontend-tests` | Required | push to `main`, PR, dispatch | Dashboard render + read-client test suite (`npm test`) and production build (`npm run build`) | No |
| frontend lint | `frontend.yml` | `frontend-lint` | Required | push to `main`, PR, dispatch | TypeScript type-check (`tsc --noEmit`); generated API-client drift check (`scripts/check-openapi-drift.sh`) | No |
| live-smoke (Gate 1) | `gate1-live-smoke.yml` | `live-smoke` | Required | push to `main`, PR, dispatch | `beaterd` live runtime smoke test (`cargo test -p beaterd --test live_smoke`) | No |
| live-browser-proof (Gate 2) | `gate2-browser-proof.yml` | `live-browser-proof` | Required | push to `main`, PR, dispatch | End-to-end Gate 2 self-host browser proof (`scripts/gate2-proof.sh`) inside Playwright container | No |
| validate | `gate2-proof-contract.yml` | `validate` | Required | push to `main`, PR, dispatch | `cargo fmt --check`; bash-syntax check of all gate scripts; Gate 0 foundation contract; `beaterd` self-host contract + Gate 2 outside-validator tests | No |
| verify-contract | `release.yml` | `verify-contract` | Required (on tag) | push tag `v*`, dispatch | OpenAPI coverage test; `scripts/regen-sdks.sh --check` (spec/SDK drift) | No |
| conformance (release) | `release.yml` | `conformance` | Required (on tag) | push tag `v*`, dispatch (needs `verify-contract`) | Starts `beaterd` via docker compose and runs live SDK smoke (`scripts/smoke-compose.sh`) | No |
| publish (SDK matrix) | `release.yml` | `publish` | Advisory (on tag) | push tag `v*`, dispatch (needs `conformance`) | Publishes Rust / Python / TypeScript / Go / Java SDKs; no-ops when registry secret is absent | No |
| release-notes | `release.yml` | `release-notes` | Advisory (on tag) | push tag `v*`, dispatch (needs `publish`) | Generates oasdiff contract changelog; creates/updates the GitHub release | No |
| contract in sync | `sdk-contract.yml` | `contract` | Required | push to `main`, PR, dispatch | Semantic conventions in sync; spec + all 7 SDK clients in sync (`regen-sdks.sh --check`); additive-only contract policy (`oasdiff breaking`) | No |
| conformance (storage) | `storage-backends.yml` | `conformance` | Required | push to `main`; PR touching `crates/beater-store-sql/**`, `beater-store-conformance/**`, `beater-store/**`, `beater-schema/**`, or `migrations/**` | Pg + ClickHouse `TraceStore` conformance tests (boots throwaway containers via `testcontainers`) | No |

## Notes

- **Required / Advisory** is this doc's assessment based on what the jobs gate.
  The authoritative enforcement list is the branch-protection rule in the GitHub
  repo settings; verify there if adding a new required check.
- The Vercel deployment (`deploy-dashboard.yml`) posts a deployment status back
  to GitHub.  If the Vercel project is also connected directly to the repo,
  Vercel may post additional PR preview checks (e.g. "Vercel — Preview");
  those are controlled by Vercel project settings, not by these workflow files.
- `actions/checkout@v7` is used in several workflows but `v7` does not
  correspond to a published release of that action (current major is v4).
  See finding M6 in [`docs/security-review-2026-06-24.md`](security-review-2026-06-24.md).
- `superfly/flyctl-actions/setup-flyctl@master` in `deploy-backend.yml` is
  pinned to a mutable ref.  See finding H8 in the security review.
