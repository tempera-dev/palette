# Hosting Palette (Vercel + Fly.io)

This is the deploy guide for the **hosted, auth-gated** Palette: the Next.js
dashboard on **Vercel** and the Rust backend (`paletted`) on **Fly.io**.

The repo is open source and **secret-clean** — no secrets are committed. Every
credential below is set out-of-band (Fly secrets, Vercel encrypted env, GitHub
Actions secrets). Only secret *names* appear in this repo.

## Why this split

Vercel can host the dashboard but **not** `paletted`: it's one long-lived process
(axum HTTP + tonic gRPC OTLP + background drain workers) with all state in local
SQLite + Tantivy + Parquet files. That needs a persistent process and a writable
volume — i.e. a container host. See `ARCHITECTURE.md` §3.2.

```
Browser ──▶ Vercel (web/dashboard, Next.js)
                │  server-side fetch, PALETTE_API_TOKEN
                ▼
            Fly.io (paletted) ──▶ /data persistent volume (all SQLite + index + archive)
              HTTPS :443 → :8080  (HTTP API, OTLP/HTTP ingest, /mcp)
```

## One-time backend setup (Fly.io)

Requires the `flyctl` CLI (`brew install flyctl`) and `fly auth login`.

```bash
# From the repo root (fly.toml lives here).
fly launch --no-deploy --copy-config --name palette-api --region sjc   # or `fly apps create`
fly volumes create palette_data --region sjc --size 3                   # persistent /data

# Secrets (NEVER commit these):
fly secrets set PALETTE_PROVIDER_SECRET_KEY="$(openssl rand -base64 32)"
# ^ stable keyring for provider-secret encryption; without it paletted generates
#   one under /data on first boot (fine on a volume, but explicit is safer).

fly deploy   # builds the Dockerfile `tools` stage, runs with --auth-mode required
curl -fsS https://palette-api.fly.dev/health   # -> {"ok":true}
```

### Bootstrap the first Admin API key (strict auth)

With `--auth-mode required`, the HTTP `createApiKey` route needs an existing
Admin key — a chicken-and-egg. Break it once, offline, inside the machine
(`palettectl` ships in the image because `fly.toml` builds the `tools` stage):

```bash
fly ssh console -C "palettectl api-key-create \
  --data-dir /data --tenant-id demo --project-id demo --environment-id local \
  --scopes admin,trace-read,trace-write,pii-unmask"
# prints { "api_key_id": "...", "secret": "bt_...", ... } ONCE — copy the secret.
```

Use that `bt_...` Admin secret only to mint narrower project-scoped keys. Do not
use the bootstrap Admin key as a standing Vercel credential: for the dashboard,
create a key with the minimum scopes the web tier needs (`trace-read`, plus
`pii-unmask` only if audited unmask controls are enabled; add write scopes only
for workflows that require them). After that, mint and rotate per-project/scope
keys through the API/dashboard — no more SSH.

## One-time frontend setup (Vercel)

The dashboard is in `web/dashboard`, a subdir of a Rust workspace, so set the
Vercel project **Root Directory = `web/dashboard`** (Project → Settings → General).
The repo is already linked (`.vercel/`, gitignored). Framework auto-detects
Next.js; build = `npm run build` (`web/dashboard/vercel.json`).

Set these as **encrypted, server-side** env vars (Project → Settings → Environment
Variables). Do **not** use `NEXT_PUBLIC_*` for any secret — those ship to the
browser.

| Env var | Value |
| --- | --- |
| `PALETTE_API_BASE_URL` | `https://palette-api.fly.dev` (your Fly HTTPS URL) |
| `PALETTE_API_TOKEN` | a scoped dashboard key (sent as `Authorization: Bearer`; do not use the bootstrap Admin key as a standing credential) |
| `PALETTE_GATE2_CONFIRMATION_SALT` | any long random string (`openssl rand -hex 32`) |

`PALETTE_API_KEY` is an alternative to `PALETTE_API_TOKEN` (sent as
`x-palette-api-key`). The dashboard derives `x-palette-project-id` /
`x-palette-environment-id` from the selected scope. Read in
`web/dashboard/lib/api.ts`.

## CI-driven deploys (GitHub Actions)

Two workflows deploy on push to `main`; both no-op (green) if their token secret
is absent, so forks are unaffected.

- `.github/workflows/deploy-backend.yml` → Fly. Repo secret: **`FLY_API_TOKEN`**
  (`fly tokens create deploy`).
- `.github/workflows/deploy-dashboard.yml` → Vercel. Repo secrets:
  **`VERCEL_TOKEN`**, **`VERCEL_ORG_ID`**, **`VERCEL_PROJECT_ID`**
  (org/project IDs are in `.vercel/project.json` after linking).

Set them under **Settings → Secrets and variables → Actions**.

## Required secrets at a glance (names only)

| Where | Name | Purpose |
| --- | --- | --- |
| Fly secrets | `PALETTE_PROVIDER_SECRET_KEY` | provider-secret encryption keyring |
| Vercel env | `PALETTE_API_BASE_URL` | dashboard → backend URL |
| Vercel env | `PALETTE_API_TOKEN` (or `PALETTE_API_KEY`) | dashboard → backend auth |
| Vercel env | `PALETTE_GATE2_CONFIRMATION_SALT` | gate2 confirmation HMAC |
| GH Actions | `FLY_API_TOKEN` | backend deploy |
| GH Actions | `VERCEL_TOKEN`, `VERCEL_ORG_ID`, `VERCEL_PROJECT_ID` | dashboard deploy |

## End-to-end smoke (post-deploy)

```bash
# 1. Backend health
curl -fsS https://palette-api.fly.dev/health

# 2. Seed a trace (OTLP/HTTP over HTTPS, with the bootstrap key)
cargo run -q -p palettectl -- \
  --base-url https://palette-api.fly.dev --api-key bt_... \
  smoke --http-url https://palette-api.fly.dev --otlp-grpc-url ""   # HTTP path

# 3. Open the dashboard, scoped to the seeded project
#    https://<your-app>.vercel.app/?tenant=demo&project=demo&environment=local

# 4. Browser e2e against the live dashboard
cd web/dashboard
PLAYWRIGHT_BASE_URL=https://<your-app>.vercel.app npm run test:e2e
```
