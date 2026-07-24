# Discovery observability contract

Palette observes a Tempera Discovery campaign; it does not own biological source
data, hidden labels, experiment authorization, provider credentials, or
scientific decisions.

Discovery spans use existing client-emittable span kinds and the canonical
`tempera.discovery.*` attributes generated from
`palette_schema::conventions::attr`. A campaign should use one trace when
practical. Each round or durable workflow transition emits a child span with:

- `DISCOVERY_CAMPAIGN_ID`, `DISCOVERY_ROUND_ID`, `DISCOVERY_STAGE`, and
  `DISCOVERY_STATUS` for navigation;
- `DISCOVERY_EVIDENCE_CLASS` and `DISCOVERY_CLAIM_CLASS` so synthetic,
  public-retrospective, shadow, and prospective evidence are never presented as
  interchangeable;
- candidate, selected, and verified counts plus budget limit and consumption as
  bounded numeric attributes;
- content digests for the program, proposal, protocol, prepare/commit/verifier/
  decision receipts, and immutable release.

Allowed values remain producer-owned contract data. In particular, the
attributes do not authorize a physical action, validate a measurement, or turn
an observed status into a scientific claim. A `verified` status is meaningful
only when the corresponding content-addressed verifier receipt is retained by
its owning system.

Never put raw sequences, assay rows, hidden labels, predictions, protocol
payloads, provider secret references, credentials, personal data, or signed
result bodies in span attributes. Those remain in their authorized stores.
Palette carries identifiers, counts, classifications, and digests that let an
operator follow the campaign and locate the authoritative evidence.

## Stage guidance

The recommended stage vocabulary is:

`source`, `candidate`, `proposal`, `approval`, `submission`, `measurement`,
`verification`, `decision`, `release`, and `evaluation`.

Unknown future stages may be recorded without changing Palette, but dashboards
must display their literal value rather than silently mapping them to a known
state. Missing receipt digests, mismatched evidence classes, or an unrecognized
status should remain visible and fail any downstream release gate that requires
them.

## Claim boundary

Search, aggregation, alerts, and dashboards over these attributes are
operational evidence only. They do not establish model quality, state of the
art, prospective discovery, experimental replication, clinical validity,
therapeutic efficacy, or fitness for patient care.
