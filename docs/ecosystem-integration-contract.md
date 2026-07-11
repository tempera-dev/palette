# Ecosystem Integration Contract

This is the repo-local contract for how Palette integrates with the active
neighboring projects without making them depend on hosted Palette services.

Current neighbor context checked on 2026-07-06:

- `jadenfix/palette.js` exports completed agent runs to Palette with
  `PALETTE_TRACE_EXPORT_URL`, `PALETTE_OTLP_EXPORT_URL`, or standard
  `OTEL_EXPORTER_OTLP_*` variables.
- `jadenfix/tempo` active PRs are tightening replay order, live E2E evidence,
  session MCP policy, and CI gate proof.
- `jadenfix/paletteOS` models payment authority as mandates, budgets, receipts,
  and journals.
- `jadenfix/aether` models settlement with signed agent authorization and
  `PaymentEnvelope` objects; Palette stores traces/eval evidence, not settlement
  authority.

## Boundary

Palette stays standalone:

- The default OSS build and Docker path do not require Palette Cloud.
- Billing and Stripe stay out of the Palette product API; the central Tempera
  control plane owns checkout, plans, subscriptions, invoices, and webhooks.
- No local runtime may require a license key, mandatory phone-home, or mandatory
  hosted account to ingest, inspect, replay, evaluate, or gate traces.

Hosted Tempera may bill Palette usage through the central control plane, but that
surface is outside Palette's product API. It does not become the source of
authority for local agent execution.

## Inbound Trace Surfaces

Palette accepts ecosystem traces through stable, additive ingress paths:

- Collector-compatible OTLP HTTP/JSON:
  `POST /v1/traces`
- Scoped OTLP HTTP/protobuf:
  `POST /v1/otlp/{tenant_id}/{project_id}/{environment_id}/v1/traces`
- Native canonical ingest:
  `POST /v1/traces/native`
- Importer-based source ingest:
  `POST /v1/import/{tenant_id}/{project_id}/{environment_id}`

The zero-lock-in floor is the OTLP trace data model. Collector-style OTLP/JSON
exporters may post directly to `/v1/traces`; Palette resolves tenant, project,
and environment from `x-palette-*` headers or Palette resource attributes.
Protobuf senders can use the scoped OTLP path. Any richer adapter must remain
optional and map back to canonical spans.

## Active Neighbor Repos

| Repo | Current Palette-side contract | Must not require |
| --- | --- | --- |
| Tempo | Send browser/session spans through collector-style OTLP/JSON or Palette's scoped protobuf endpoint; Palette normalizes them into canonical trace views. | Stripe config, hosted account |
| palette.js | Export agent runs and tool calls through `PALETTE_OTLP_EXPORT_URL`, `PALETTE_TRACE_EXPORT_URL`, collector-style OTLP/JSON, Palette's scoped OTLP/protobuf, or native canonical ingest; Palette displays them as `agent.run`, `llm.call`, and `tool.call` spans. | Billing feature, hosted dashboard, live model credentials |
| paletteOS | Export receipts, journals, and audit spans as traces or artifacts; Palette observes and gates outcomes. | Hosted billing as local payment authority |
| Aether | Anchor agent settlement evidence by carrying run, step, receipt, and `PaymentEnvelope` identifiers as trace attributes; Palette can evaluate and retain off-chain evidence for disputes. | Palette billing as an AIC/SWR wallet, escrow, paymaster, or settlement authority |

paletteOS owns local authority: grants, spend limits, payment mandates, receipts,
and journal verification. Central Tempera billing may meter hosted Palette usage,
but it must not authorize or block local paletteOS actions.

Aether owns settlement authority for AIC/SWR escrow, agent authorization, and
payment-envelope verification. Palette may retain OTLP/native trace evidence such
as `paletteos.payment_mandate_id`, `paletteos.receipt_requirement`,
`aether.payment_envelope_id`, and `aether.agent_payment_authorization`, but
those fields are observed metadata. They must not cause the OSS Palette runtime
to release funds, enforce payment mandates, or require Aether.

## Billing Boundary

Billing integration is control-plane-owned:

- `crates/palette-billing` may retain internal billing primitives for future
  hosted workers and migrations.
- `palette-api` exposes no billing, subscription, invoice, plan, or Stripe
  webhook routes.
- `paletted` opens no billing store and wires no Stripe webhook route.
- The usage ledger remains append-only; refunds are compensating entries that
  net down invoice quantities.
- Payment mandates, `PaymentEnvelope` signatures, AIC/SWR escrow, x402-style
  commerce flows, and paletteOS payment receipts remain external authority inputs
  that Palette may observe in traces but never treats as hosted billing state.

This preserves self-hosted operation while giving hosted deployments one
central path to metered billing.

## Verification

The static checker `scripts/check-ecosystem-contract.py` guards this document's
markers against drift from Palette-side code and governance docs. Focused runtime
coverage lives in:

- `cargo test -p palette-api --test openapi_coverage`
- `cargo test -p palette-api api_accepts_collector_otlp_json_and_reads_canonical_trace`
- `cargo test -p palette-otlp --lib`
- `cargo test -p palette-api --test route_inventory`
- `cargo test -p palette-billing`
