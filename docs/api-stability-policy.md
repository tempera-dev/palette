# `/v1` API stability and deprecation policy

This document defines the stability guarantees and deprecation process for the
Palette HTTP API. It satisfies requirement **R11.5** (`/v1` API is stable and
versioned) and complements the OpenAPI spec
(`contracts/openapi/palette.openapi.json`) and `CONTRIBUTING.md`.

## The contract is the single source of truth

Every `/v1` endpoint, request/response type, MCP tool, CLI command, and SDK
client is generated from one artifact: `contracts/openapi/palette.openapi.json` (itself
generated from the Rust handlers in `crates/palette-api`). The drift gate
(`scripts/check-contract-sync.sh`) and `oasdiff` in
`.github/workflows/sdk-contract.yml` block any change that is not regenerated
across the spec, all 7 SDK clients, the MCP tools, and the docs.

## Pre-1.0 caveat

Palette has not declared 1.0 yet. Until that milestone, canonical and API
schemas may evolve freely and **no wire/SDK backward-compatibility is promised
before 1.0**. That caveat does not relax the contract discipline: every `/v1`
handler change still regenerates the OpenAPI spec, generated SDK clients,
semantic conventions, MCP/CLI/docs contract surfaces, and then runs
`scripts/check-contract-sync.sh` plus the `sdk-contract` CI gate before merge.
The only pre-1.0 breaking exceptions allowed in CI are explicit contract
alignment breaks filtered by `scripts/filter-oasdiff-breaking.py`; unexpected
`oasdiff` errors still fail the merge gate. The current reviewed exceptions are
the AIP-127 migration to lowerCamel public parameters and JSON fields, the
AIP-158 migration from bare list bodies to named paginated response objects,
the AIP-193 migration to the standard application-error envelope and
HTTP-200 partial-success reports, and removal of the legacy
authorization-scope spellings. The filter matches only the specific
operations, fields, statuses, and type transitions required by those
migrations; it is not a general bypass for breaking changes. The AIP-127
exception is additionally pinned to the SHA-256 digest of the reviewed
migration spec, so any subsequent contract edit disables that exception.

## Stability guarantee for `/v1`

While the API is at `/v1`:

- **No breaking changes are made in place.** A change that removes an endpoint,
  removes a field, narrows an enum, changes a field's type, or makes an optional
  request field required is a breaking change and is **blocked by `oasdiff`** in
  CI.
- **Additive changes are allowed without a version bump.** New endpoints, new
  optional request fields, and new response fields are backward compatible and
  may ship under `/v1`. Clients must tolerate unknown response fields.
- **Error shape is stable after the reviewed pre-1.0 alignment.** Ordinary
  application errors use the shared AIP-193 `ErrorResponse` body:
  `{error: {code, message, status, details}}`. `code` is the HTTP status,
  `message` is the developer-facing detail, `status` is the canonical RPC
  status string, and `details` contains standard detail objects such as
  `google.rpc.ErrorInfo`. New detail types and error reasons may be added;
  existing meanings remain stable. Partial-success drain reports are domain
  results returned with HTTP 200, not error responses.

## Versioning model

- The path prefix (`/v1`) is the major version. After the enumerated pre-1.0
  alignment exceptions above are complete, a breaking change that cannot be
  made additive ships under a **new prefix** (`/v2`) — `/v1` is not mutated.
- During any `/v1` -> `/v2` transition, `/v1` remains served and supported for
  the deprecation window below.
- SDK client packages follow semver. A new minor adds endpoints/fields; a major
  is only released for a new API major version.

## Deprecation process

1. **Announce.** A deprecation is recorded in the changelog and the endpoint is
   marked `deprecated: true` in the OpenAPI spec (which propagates to the SDKs,
   MCP tools, and docs automatically).
2. **Warn at runtime.** Deprecated endpoints continue to function and respond
   with a `Deprecation` header (and, where applicable, a `Sunset` header per
   RFC 8594) so callers can detect usage in the wild.
3. **Provide a migration path.** The replacement endpoint/field ships and is
   documented before the deprecated surface is removed.
4. **Honor the window.** A deprecated `/v1` surface is supported for at least
   **6 months** (and at least one minor SDK release) after the announcement
   before it can be removed, and removal only happens under a new major version
   prefix.

## What this means for self-host

Self-hosted `paletted` and Palette Cloud serve the identical contract, so the same
guarantees apply to both. Because OSS runs without Palette Cloud (R1.3), the
stability of `/v1` is what your agents, SDKs, and dashboards depend on — it is
governed, versioned, and CI-enforced, not maintained by convention.

See also: [`GOVERNANCE.md`](../GOVERNANCE.md) (no-rug-pull promise) and
[`docs/feature-matrix.md`](feature-matrix.md) (open-core boundary).
