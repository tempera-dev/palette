# Palette Client Platform — Architecture & API Design

How the **API, MCP server, CLI, and 7 language SDKs** stay coherent, simple, and
impossible to drift. One contract, generated outward.

## Principle: one source of truth, generated outward

```
crates/palette-api handlers  ──#[utoipa::path] + ToSchema on the REAL types──┐
                                                                            v
                                       sdks/openapi/palette-api.json  (OpenAPI 3.1)
        +------------------+------------------+------------------+-----------------+
        v                  v                  v                  v                 v
  7 SDK clients        /mcp tools          palette CLI         docs site      conformance
  (openapi-generator)  (1 per operation)   (shared client)   (renders spec)  (live, per lang)
```

Nothing re-describes the API by hand. A handler change regenerates the spec; CI
(`sdk-contract.yml`) fails if the spec, any SDK client, or MCP tool set is stale,
and `oasdiff` blocks breaking changes. Drift is a merge-blocker, not a hope.
The dashboard projection at `web/dashboard/openapi/palette-read-api.json` is a
byte-identical generated copy of this same first-party contract, and the
contract-sync gate compares both snapshots on every change.

## Modern API design conventions (enforced)

- **Resource-tagged, action-named.** Every operation has a resource `tag`
  (`datasets`, `traces`, ...) and a globally-unique camelCase `operationId`
  (`datasets.create`, `traces.list`, `evals.runJudge`). Tags -> SDK API classes;
  operationIds -> method names. Uniqueness avoids single-package collisions
  (Go/C/Java) so names are consistent across all 7 languages without per-language
  hacks.
- **Uniform error model.** Every non-2xx is one AIP-193
  `ErrorResponse { error: { code, message, status, details } }` -> one typed
  error per SDK. Partial-success drain reports remain domain results and are
  returned with HTTP 200.
- **Typed everything.** No bare `object`/`any` responses; every response is a
  named schema. Discriminated unions use an internal `type` tag (e.g.
  `EvaluatorKind`) so they generate cleanly in strict languages.
- **AIP-158 pagination** for migrated list endpoints: lower-camel
  `pageSize`/`pageToken` requests and `nextPageToken` responses. Tokens are
  opaque and bound to the complete list request; malformed, stale, or
  cross-scope tokens fail with `400`. Scenario and control-plane collection
  families use this contract consistently.
- **Explicit tenancy.** `tenant`/`project`/`environment` are path-scoped; the SDK
  ergonomic layer binds them once at `init()` so callers never repeat them.
- **Versioned.** All routes under `/v1`; `info.version` tracks the workspace
  version; SDKs publish in lockstep.

### AIP-158 migration inventory

The following families use the standard contract above:

- `scenarios.list` — `GET /v1/scenarios/{tenantId}/{projectId}`
- `audit.list` — `GET /v1/audit/{tenantId}/{projectId}`
- `archive.querySpans` — `GET /v1/archive/{tenantId}/{projectId}/spans`
- `connectors.list` — `GET /v1/connectors/{tenantId}/{projectId}`
- `connectors.listTools` —
  `GET /v1/connectors/{tenantId}/{projectId}/tools`
- `judge.listLedger` — `GET /v1/judge/{tenantId}/{projectId}/ledger`
- `prompts.list` — `GET /v1/prompts/{tenantId}/{projectId}`
- `prompts.listVersions` —
  `GET /v1/prompts/{tenantId}/{projectId}/{promptId}/versions`
- `providerSecrets.list` —
  `GET /v1/provider-secrets/{tenantId}/{projectId}`
- `reviews.listTasks` —
  `GET /v1/review-queues/{tenantId}/{projectId}/{queueId}/tasks`
- `search.spans` — `GET /v1/search/{tenantId}/spans`
- `traces.list` — `GET /v1/traces/{tenantId}`

The migration debt ratchet is empty. Adding a public list/search operation
without the complete contract fails the API-shape audit.

Raw OTLP collection endpoints and MCP protocol pagination are protocol-native
surfaces and are not rewritten by this HTTP API migration.

### AIP-127 and AIP-193 migration boundary

The shared application-error response now uses the core AIP-193 HTTP/JSON
shape: an outer `error` object containing HTTP `code`, developer-facing
`message`, canonical RPC `status`, and `details` with `google.rpc.ErrorInfo`.
The shape audit rejects contract drift from that envelope.

The AIP-127 migration is complete: public path and query parameters and
ordinary JSON schema properties are lowerCamel, and the contract audit has a
zero-debt budget. The organization-wide route-aware checker reports zero
parameter, reachable-JSON-field, and AIP-193 response violations. The
`trace-ingested/drain` and `trace-writes/drain` operations return their domain
partial-success reports with HTTP 200, while every HTTP error response uses the
shared AIP-193 envelope.

Persisted database columns and protocol-owned payloads are not public Palette
JSON names and retain their native spellings. Axum extractor-generated errors
remain a runtime AIP-193 gap until request rejection is mapped through the
shared application envelope.

Raw OTLP request bodies and MCP protocol payloads remain governed by their
protocol schemas and are outside the ordinary JSON-field ratchet. Palette-owned
path parameters around those payloads are still lowerCamel; the protocol
exception applies only to the protocol-defined message body.

## Two SDK layers (simple by default, powerful when needed)

1. **Ergonomic (Layer 2, hand-written):** `init()` + `@observe`/`observe()` +
   `wrap_openai()/wrap_anthropic()` + LangChain/LlamaIndex callbacks. Built on
   OpenTelemetry (ingest is OTLP). One shared `semconv` module per language
   mirrors the server normalizer. This is the 5-line path most users want.
2. **Control-plane (Layer 1, generated):** typed CRUD for datasets, experiments,
   gates, evals, usage, audit, etc. -- `client.datasets.datasets.create(...)`.

The **CLI** and **Rust SDK** consume the same generated Layer-1 client, and the
**MCP** server dispatches tool calls through the same in-process router with the
same auth -- so all four surfaces are literally the same operations.

## Robust & scalable

- **Robust:** the contract is type-checked in Rust; `openapi_coverage` asserts
  spec == served routes; per-language **live conformance** drives each generated
  client against a running `paletted`; MCP has parity tests vs direct HTTP; the
  ergonomic SDKs have unit + live E2E.
- **Scalable:** adding an endpoint = annotate one handler + `regen-sdks.sh`; all
  7 SDKs, MCP tools, CLI commands, and docs update from that single change.
  Adding a language = one `sdks/config/<lang>.yaml`.

## Easy to use

- Python: `pip install palette-sdk` -> `palette.init()` -> `@palette.observe(...)`.
- TypeScript: `npm i @palette/sdk` -> `palette.init()` -> `observe(fn)`.
- CLI: `palette api <operationId> --param k=v` reaches any endpoint; typed
  sugar (`palette traces list`) for common ones.
- MCP: point any MCP client at `/mcp`; every API operation is a tool.
- Docs: `/docs` renders the live spec + tool catalog + per-language quickstarts.
