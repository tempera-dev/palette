# Open-Issue Triage — 2026-06-28

Scope: all **171 open issues** in `jadenfix/beater`, clustered into 16 coherent themes. This is a triage map only — no code changed, no issues closed. Dispositions: **KEEP** (actionable), **DEDUP** (canonical noted), **DOC-ONLY**, **STRATEGY/DISCUSS**, **STALE/QUESTIONABLE**. Issues carrying the `bug` label (or that are de-facto bugs) are flagged ⚠ and should jump the queue.

## Summary table

| # | Cluster | Count | Priority | Notes |
|---|---------|------:|----------|-------|
| 1 | Incremental-sync / Merkle corpus sync | 11 | Medium | Umbrella #327 + 8 substeps; research/strategy heavy |
| 2 | RSI loop & optimization | 14 | **High** | Core moat; 3 bugs gate the rest |
| 3 | Live replay | 11 | Medium | 1 bug (#141); cohesive sub-program |
| 4 | MCP / connectors | 21 | Medium | Two trust-center dups; large surface |
| 5 | Billing / usage | 5 | **High** | #181+#186 in flight; #278 is a bug |
| 6 | Statistics / calibration / judge trust | 11 | **High** | 4 bugs; validity foundation for the gate |
| 7 | Agent eval features (gap-analysis) | 20 | Medium | Catalog expansion; sequence behind core |
| 8 | ARCHITECTURE.md doc cleanups | 9 | Medium | DOC-ONLY; #119/#177 phantom-spec pair |
| 9 | Storage / perf hot-path | 14 | **High** | 1 bug (#203); 2 dup pairs |
| 10 | Security / identity / privacy | 4 | **High** | #314 is a live security bug |
| 11 | Product strategy | 9 | Discuss | Mostly STRATEGY/DISCUSS |
| 12 | SDK / release | 3 | Medium | 1 bug (#284) |
| 13 | CI / build | 2 | Medium | Dev-velocity |
| 14 | Adoption / onboarding wedge | 4 | **High** | The actual go-to-market loop |
| 15 | Platform / API build-out (§20.x core) | 12 | High | Foundational read/write APIs |
| 16 | Agent product surfaces | 21 | Low | Optional/speculative; gate behind wedge |

**Total: 171.** Bug-labelled (higher priority): #141, #142, #143, #146, #147, #148, #203, #231, #263, #284, #314 (+ #278 unlabeled but de-facto bug).

---

## Cluster 1 — Incremental-sync / Merkle corpus sync (11)

**Theme:** Keep a large, constantly-changing corpus (code, docs, tool schemas, dataset rows, trace spans) synced to a remote index, transferring/recomputing only the delta — Cursor/Dolt-class problem.

Issues: #307, #317, #318, #319, #321, #322, #323, #324, #325, #327, #334.

- **#327** — Umbrella, first-principles 8-substep decomposition. **KEEP (canonical umbrella).**
- **#317–#325** (8 substeps: change-detection, content-addressed storage, incremental recompute, delta sync, Merkle/prolly-tree design, concurrency, freshness policy, vector-index maintenance). **KEEP**, but all `tier:supporting` and research-staged — sequence behind the core wedge.
- **#307** — research framing ("Merkle-backed incremental knowledge index + inconsistency eval harness"). Overlaps the umbrella's motivation. **DEDUP candidate** — keep #327 as the engineering home; fold #307's product framing in, or keep #307 as the product/strategy companion.
- **#334** — adopt prolly-tree dataset versioning ("Git for eval data"). **STRATEGY/DISCUSS** — connects #327 research to a concrete product capability on top of existing `beater-datasets`.

---

## Cluster 2 — RSI loop & optimization (14)

**Theme:** The recursive-self-improvement moat: index → propose → simulate(Train) → accept(Test) → apply. This is the project's "furthest-from-done" capability and the strategic headline (#333).

Issues: #71, #72, #92, #139, #142, #143, #184, #198, #433, #434, #435, #436, #437, #438.

- **#438** — rsi-opt umbrella. **KEEP (canonical umbrella).** Children #433 (Rust hot-path), #434 (optimizer variants), #435 (high-dim failure features), #436 (gate statistics), #437 (counterfactual/sim eval). **KEEP** all.
- **#71** — close the RSI loop end-to-end (§21). **KEEP** — the top-level gap issue.
- ⚠ **#142** [bug] — Train/Dev/Test split (Principle #12) doesn't exist in the data model, yet the gate and all RSI depend on it. **KEEP — blocker, highest priority in cluster.**
- ⚠ **#143** [bug] — RSI must not enable `apply_change` until real stats + Test split + forked replay are Done; add explicit hard gate to §21. **KEEP — safety gate; partly DOC (§21 edit) + dependency wiring.**
- **#72** — §21.4 overfit guardrail (reject overfit change on held-out OOD probe). **KEEP.**
- **#184** ≈ **#434** — *DUPLICATE.* Both implement the 5 `NotYetImplemented` `OptimizerStrategy` variants (FewShotBayesian/MIPRO/Evolutionary/GEPA/ParamSearch). **DEDUP → canonical #434** (richer, SOTA-cited, lives under the rsi-opt umbrella); close #184 as subsumed (it cites the phantom §20.10 anchor).
- **#198** — prune-or-prove high-dim RSI/OOD guardrail math before implementation. **KEEP** (math-discipline gate; pairs with #72).
- **#139** — optimization-worthiness ledger before RSI/eval budget is spent. **KEEP.**
- **#92** — §26 eval-cost optimizations (verifier-first cascade, zero-cost CI reruns, anytime-valid early-stop). **KEEP**; overlaps #437 (counterfactual eval) and #436 (anytime-valid stats).

---

## Cluster 3 — Live replay (11)

**Theme:** Cursored live-tail/replay of running agent sessions: stream contract, UI, access control, edge-cases.

Issues: #112, #141, #216, #227, #236, #239, #240, #241, #243, #244, #245.

- **#239** (cursored stream contract/backfill/idempotency), **#240** (e2e CLI+dashboard proof gate), **#243** (operator-grade session UI), **#244** (access control/redaction/share-link), **#245** (edge-case regression matrix) — cohesive `live-replay` sub-program. **KEEP** all; sequence #239→#240→{#243,#244,#245}.
- ⚠ **#141** [bug] — forked-replay attribution undesigned for the no-cassette OTLP path and **re-executes live side-effecting tools.** **KEEP — high priority**, also a hard dependency for RSI (#143) and counterfactual eval (#437).
- **#112** — replay earliest-flip bisection fast-path. **KEEP** (small, self-contained perf).
- **#216** — cross-surface session handoff + live replay links. **KEEP** (`tier:optional`).
- **#227** — terminal-agent transcript import + replay timeline. **KEEP** (`tier:optional`).
- **#236** — live trace tail for running agents + MCP calls. **KEEP** (`tier:supporting`); overlaps #239's stream contract.

---

## Cluster 4 — MCP / connectors (21)

**Theme:** MCP as the connector substrate — trust center, OAuth/token vault, policy gateway, catalog/schema lint, Composio integration.

Issues: #211, #212, #228, #229, #230, #231, #232, #233, #234, #235, #246, #249, #275, #292, #293, #295, #301, #302, #303, #304, #305.

- **#211** ≈ **#212** — *DUPLICATE.* Both propose an "MCP Trust Center" (inventory/health/auth-scope/drift vs permissions/OAuth/tool-risk). **DEDUP → canonical #212** (broader product framing); fold #211's drift/health specifics in.
- ⚠ **#231** [bug] — encode Composio connector URL params and path segments. **KEEP — small, real bug.**
- Composio/policy correctness: **#292** (OAuth grants project- vs user-scoped), **#293** (per-tool policy not broad `eval:run`), **#295** (keep Composio optional + provider-neutral contract), **#302** (skills.md risk filtering / untrusted metadata), **#303** (catalog drift misses per-tool schema/description changes), **#304** (capability discovery when provider disabled), **#305** (MCP stdio docs/status/e2e proof). **KEEP** all — these are the actionable connector-hardening set. #292/#293/#302 have security implications.
- Token/OAuth/gateway product: **#230** (OAuth token vault + account lifecycle), **#232** (MCP gateway: policy/approvals/rate-limits/audit), **#228** (one-click MCP connect + health), **#235** (dynamic tool discovery + context-budget). **KEEP**, `tier:supporting`.
- Catalog/quality: **#233** (schema lint + tool-catalog scorecards), **#249** (real pagination vs fixed 100-item caps — pairs with storage #75), **#246** (expose Beater as MCP-native surfaces), **#234** (reviewer handoff packets for agent PRs — loosely related). **KEEP.**
- **#229** (terminal/worktree black-box recorder), **#301** (browser-use/Stagehand SDK adapters — also touches SDK cluster). **KEEP.**
- **#275** — integration-surface audit ("why no first-class inbound agent-tooling story?"). **KEEP as audit/STRATEGY**; pairs with #276 (crate-consolidation audit) and #288 (§27 home).

---

## Cluster 5 — Billing / usage (5)

**Theme:** Metering, quota reservation, usage rollup, Stripe.

Issues: #181, #186, #190, #264, #278.

- **#181** ≈ **#186** — *OVERLAP (already being implemented as a pair).* #181 = `summarize_usage` is O(n)-memory, needs windowed/periodic rollup; #186 = billing hardening (reservation idempotency key **+ bounded usage rollup**). #186's item-2 restates #181. **DEDUP → canonical #186** (superset); keep #181 only if you want the rollup tracked granularly. In flight already.
- ⚠ **#278** — Stripe webhook apply is **non-atomic; partial failure permanently loses the effect.** De-facto bug (no `bug` label). **KEEP — high priority.**
- **#190** — meter actual cost into usage ledger + reconcile budget reservation on failure. **KEEP** (closes the metering loop; depends on gateway sink #182).
- **#264** — hosted billing product surface (plans/subscriptions/quotas/invoices/Stripe sync). **KEEP** (`tier:optional`, product-level).

---

## Cluster 6 — Statistics / calibration / judge trust (11)

**Theme:** Make the numbers honest — the validity foundation the gate and RSI stand on. Four bugs here are correctness-critical.

Issues: #62, #109, #111, #129, #136, #146, #147, #148, #197, #200, #263.

- **#111** — finish §10.3 statistics (power/MDE, Holm/BH multiplicity, clustered SEs, mSPRT; hardcoded-z still live). **KEEP — foundational**; #436 (rsi-opt D) is the budget-aware sibling.
- **#109** — StatisticalDesign manifests so estimators are chosen pre-results. **KEEP.**
- ⚠ **#146** [bug] — Principle #9 IPW weighting unimplemented; `sampling_weight` exists nowhere, so every roll-up is the biased average the doc forbids. **KEEP — high priority.**
- ⚠ **#147** [bug] — §20.6 online-alert "firing decisions" require mSPRT that §10.3 prohibits until it exists. **KEEP**; cross-links #83/#84 (online evals/alerts).
- ⚠ **#148** [bug] — judge cache key under-specified; a hit can serve a stale score after recalibration ("determinism = caching" broken). **KEEP — high priority.**
- ⚠ **#263** [bug] — human-review labels need selection/assignment provenance before calibration use (data-bias). **KEEP.**
- **#62** ≈ **#197** — *OVERLAP (tight pair).* #62 = surface Cohen's κ + reliability diagram/ECE **display-only**; #197 = **demote** κ and add Brier/ECE or stop claiming probability calibration. Same code (`beater-calibration`), same diagnosis. **KEEP both but combine** — #197 is the correctness fix, #62 the UI surfacing; do together. Canonical math home #197.
- **#200** — embedding-similarity scorer must be calibrated + opt-in or dropped. **KEEP** (pairs with #198's prune-or-prove discipline).
- **#129** (pass^k / N-trial variance / flakiness gates), **#136** (adaptive eval scheduling with logged inclusion probabilities). **KEEP** — stochastic-reliability stats; #136 overlaps #146's IPW weighting.

---

## Cluster 7 — Agent eval features / gap-analysis catalog (20)

**Theme:** "Add an eval for X" — broad scorer/oracle catalog expansion. Mostly `tier:supporting`; sequence behind the core gate (Cluster 6) and platform (Cluster 15).

Issues: #130, #131, #132, #134, #137, #138, #140, #158, #159, #160, #162, #163, #165, #166, #167, #168, #169, #255, #260, #265.

- `tier:core`: **#130** (state-based task oracles), **#132** (evidence-provenance/RAG/citations), **#137** (locked judge rubric bundles + drift canaries), **#140** (trajectory/process scoring over span trees), **#167** (production distribution-drift canaries), **#255** (agent-native canonical trace fields — also a data-model dependency for many others). **KEEP — prioritize within cluster.** #255 is effectively a schema prerequisite.
- Security-flavored evals: **#131** (prompt-injection / unsafe tool-use), **#160** (internal-channel privacy), **#265** (browser/computer-use visual replay + injection triage). **KEEP** (cross-link security Cluster 10).
- **#134** (contamination guards/canaries), **#138** (fault-injection evals), **#158** (human review → reusable eval intelligence), **#159** (multi-agent coordination metrics), **#162** (long-term memory lifecycle), **#163** (calibrated abstention/infeasible-task), **#165** (tool-contract conformance), **#166** (counterfactual attribution reports — overlaps replay #141/#437), **#168** (auditable routing policies), **#169** (calibrated user/env simulators — overlaps #437), **#260** (end-user feedback/escalation as evidence). **KEEP**, sequence later.

---

## Cluster 8 — ARCHITECTURE.md doc cleanups (9) — DOC-ONLY

**Theme:** Doc hygiene; no runtime change.

Issues: #65, #66, #67, #68, #119, #145, #177, #288, #296.

- **#119** ≈ **#177** — *PAIR (problem + fix).* #119 = Phase 7 / §20.10 is a phantom spec cited by #105/#107/#108; REQUIREMENTS stops at R17. #177 = write §20.10 to unblock them. **DEDUP → canonical #177** (the actionable fix); #119 is the diagnosis. **High-ish priority** because it blocks merged-but-unanchored work.
- **#65** (collapse §4 crate-enumeration surfaces), **#66** (consolidate §22.1/§22.3/§24 verification surfaces), **#67** (extract invariants into one glossary), **#68** (split the ~4.4k-line file). **KEEP — DOC-ONLY**; #68 enables the rest.
- **#145** (doc accuracy: §24.2 vs §20.7 orgs/projects/envs status), **#296** (make architecture-status generated/checked so built/planned states don't drift), **#288** (§27 numbering collision — home for "Integrations & Agent-Tooling Ecosystem" #280). **KEEP — DOC-ONLY**; #296 is the durable fix that subsumes #145-style drift.

---

## Cluster 9 — Storage / perf hot-path (14)

**Theme:** Move off SQLite-only/full-scan/synchronous patterns; wire the columnar plane; bound resource use.

Issues: #63, #74, #75, #90, #91, #164, #182, #201, #203, #205, #247, #248, #252, #257.

- **#75** ≈ **#201** — *DUPLICATE/tight pair.* #75 = server-side pagination + filter pushdown for `query_spans`/`query_runs` (§20.2 #0.2; keyset cursors). #201 = "push trace run/span queries down into storage backends" (same `query_runs_by_materializing_spans` / `limit u32::MAX` full-scan, same SQLite+PG call sites). **DEDUP → canonical #75** (more concrete task list); #201 adds PG evidence — fold in.
- **#205** ≈ **#257** — *related pair.* #205 = consolidate local SQLite migration/schema ownership; #257 = backend-agnostic runtime migrator (SQLite/PG/ClickHouse). Distinct scope but same subsystem; #205 is the local cleanup, #257 the multi-backend generalization. **KEEP both; do together.**
- ⚠ **#203** [bug] — move SQLite stores + durable bus off synchronous mutexes in async paths. **KEEP — high priority** (async correctness/perf).
- **#74** — wire columnar TraceStore (ClickHouse/PG) into beaterd (`*TraceStore` implemented but dead code). **KEEP — high** (unblocks #257, #75 pushdown payoff).
- **#164** ≈ **#182** — *DUPLICATE.* Both: gateway emits `llm.call` spans via sink but beaterd wires `NoopSpanSink`, so proxied calls aren't persisted (both from PR #150). **DEDUP → canonical #182** (concise/actionable); #164 has the fuller evidence — fold in. Depends-pair with billing #190.
- **#90** (embedded columnar HOT TraceStore via DataFusion + sync-consistency), **#91** (blake3 content-addressed artifact/cassette dedup + hot-path hashing — overlaps merkle #318), **#247** (Tantivy indexing/query tuning configurable+amortized), **#248** (expose ingest/attr/trace-completion limits as runtime config), **#252** (orphaned-artifact sweeper must page/stream vs scan u32::MAX — same anti-pattern as #75/#201), **#63** (critical-path latency on trace waterfall). **KEEP** all.

---

## Cluster 10 — Security / identity / privacy (4)

**Theme:** Enforcement and identity plane.

Issues: #80, #81, #82, #314.

- ⚠ **#314** [bug] — SECURITY: `redact_trace_view` leaks non-input/output attribute keys on Sensitive spans (residual #126 fail-open). **KEEP — top priority, live data-leak.**
- **#80** (enforced RBAC inside `authorize()` on mutating routes), **#81** (SSO/SAML/SCIM/OIDC `beater-identity`), **#82** (data deletion / crypto-shred / GDPR lifecycle). **KEEP** — `gap-analysis` identity/compliance plane. (Privacy *evals* #131/#160/#263 live in their clusters but cross-link here.)

---

## Cluster 11 — Product strategy (9) — STRATEGY/DISCUSS

**Theme:** Direction, positioning, scope-discipline. Discussion artifacts, not committed engineering.

Issues: #261, #271, #297, #309, #320, #326, #331, #332, #333.

- **#332** (honest PMF gut-check), **#333** (lead with gated RSI), **#320** (prove the Agent Release Gate wedge before broad AI-ops expansion), **#331** (keep expansion features optional without taxing the core loop). **STRATEGY/DISCUSS** — these define the wedge; #333 is the headline that #438 operationalizes.
- **#326** (customer-shareable agent release-evidence packets), **#309** (scenario mining + replay data engine), **#271** (FDE customer-knowledge packs). **KEEP** — productizable strategy bets; #326 pairs with the wedge (#320), #271 with merkle/knowledge index (#307).
- **#297** (split near-term dashboard routes from deferred studio/evolution scope) — IA discipline, executes #331. **KEEP.**
- **#261** (architecture optimality scorecard + performance gates). **KEEP** — meta/measurement; lower urgency.

---

## Cluster 12 — SDK / release (3)

Issues: #282, #283, #284.

- ⚠ **#284** [bug] — generated C client conformance bypasses the SDK for core API ops. **KEEP — high.**
- **#282** (run live SDK conformance instead of generic compose smoke), **#283** (publish the ergonomic Rust SDK, not just the generated `beater-client`). **KEEP.** (#301 browser-use/Stagehand adapters lives in MCP cluster but is a sibling.)

---

## Cluster 13 — CI / build (2)

Issues: #330, #343.

- **#330** — Cargo build-time floor (esp. macOS). **KEEP** — dev velocity.
- **#343** — prevent `main` breaking when two green PRs merge (merge queue / required up-to-date). **KEEP** — directly relevant to the worktree-per-PR workflow.

---

## Cluster 14 — Adoption / onboarding wedge (4)

**Theme:** How eval-gating actually gets adopted — the go-to-market loop. High leverage.

Issues: #152, #153, #154, #155.

- **#152** — THE BET: cassette-backed $0 / key-less / deterministic eval-CI (the capability no competitor has). **KEEP — high; the differentiator.**
- **#153** — trivial zero-code onboarding (today = Docker Compose + ffprobe + multi-line preflight). **KEEP — high.**
- **#154** — turnkey eval-CI GitHub Action. **KEEP — high** (adoption vehicle for #152).
- **#155** — OTel-native "Switzerland": lossless import-from-competitors + OTLP export-out. **KEEP.**

---

## Cluster 15 — Platform / API build-out (§20.x core) (12)

**Theme:** Foundational read/write APIs and runtime wiring the product surfaces depend on.

Issues: #73, #76, #79, #83, #84, #85, #93, #183, #202, #204, #256, #276.

- **#79** (Dataset CRUD + read APIs — create-only today), **#256** (dashboard-ready read APIs for sessions/experiments/eval-reports/analytics/prompts), **#73** (build the eval/observability product surface — today one trace-waterfall page). **KEEP — high; unblock the dashboard.**
- **#76** (beater-bench: query p95 SLOs + load test + CI regression gate — pairs with storage #75), **#83** (online evals that score sampled traces), **#84** (alert delivery: webhook worker + Slack Block Kit), **#85** (prompt-management pillar), **#183** (POST /v1/guardrails/check + span + p95<200ms bench), **#93** (stock-OTel interop completeness — gen_ai.* + OpenInference). **KEEP.**
- **#202** (agent session cockpit across CLI/IDE/web/PR/chat), **#204** (human approval cockpit with risk-scored approvals). **KEEP** — product surfaces but foundational enough to sit here.
- **#276** — crate-consolidation audit ("why N browser/store crates; whole verticals built-but-unwired"). **KEEP as audit/STRATEGY**; companion to #275 and doc #65/#68.

---

## Cluster 16 — Agent product surfaces (21) — mostly tier:optional/supporting

**Theme:** Speculative/optional product features around agent operations, collaboration, governance. Per #331, keep on roadmap but **gate behind the core wedge** to avoid product tax. Lowest aggregate priority; many are net-new surfaces.

Issues: #206, #207, #208, #210, #218, #219, #220, #222, #223, #225, #226, #237, #238, #250, #253, #259, #267, #268, #269, #270, #272.

- Refactor/foundational (not speculative): **#207** (extract local runtime wiring/fixtures out of beaterd/beaterctl mains), **#208** (split beater-api route parsing/handlers by resource), **#225** (`beaterctl agent init` zero-friction setup — adoption-adjacent, cross-links #153). **KEEP — these are real cleanups.**
- Governance/safety surfaces: **#267** (agent identity registry + capability leases + kill switches), **#269** (action receipts), **#270** (autonomy-level policy simulation), **#272** (incident-response rooms), **#268** (A2A/inter-agent delegation traces), **#226** (approval-friction analyzer + allowlist generator). **KEEP**, `tier:supporting/optional`.
- Dashboards/analytics: **#222** (work-accounting/ROI dashboard), **#253** (human-agent workflow analytics), **#250** (routine observability for scheduled agents), **#237** (trace optimization workbench), **#238** (instructions-as-code adherence reports). **KEEP**, optional.
- Collaboration/IO surfaces: **#202**→see C15; **#206** (Agent Inbox cross-surface approvals), **#210** (Beater Tap — CLI agent hooks), **#218** (agent identity + external-comms audit), **#219** (AG-UI event stream), **#220** (installable recipes/skill packs), **#223** (bring-your-agent cockpit comparing Claude/Codex/Cursor/Copilot), **#259** (voice/call-agent observability). **KEEP**, `tier:optional` — review for STALE/scope-cut in a strategy pass (#331/#297).

---

## Recommended work pairs

Tight two-issue bundles that should be designed/built as a single unit (excluding the in-flight billing pair **#181+#186**):

1. **#75 + #201 — Trace-query pushdown.** Same full-scan anti-pattern (`query_runs` materializing all spans, `limit u32::MAX`, in-memory paging) across SQLite + Postgres. #75 has the concrete keyset-cursor task list; #201 adds the Postgres call sites. One pushdown effort; #201 is effectively a duplicate. (Pull in #252's sweeper and #249's connector pagination as the same pattern.)

2. **#164 + #182 — Gateway span sink → trace store.** Literal duplicates from PR #150: production `beaterd` wires `NoopSpanSink`, so proxied `llm.call` traffic is never persisted. Build once (artifact-ref materialization + persist), then it unblocks billing metering **#190**. Canonical #182.

3. **#62 + #197 — Calibration math fix + surfacing.** Same `beater-calibration` code and diagnosis: κ is not probability calibration. #197 replaces/demotes κ with Brier/ECE + a recalibration map; #62 surfaces the reliability diagram/ECE in the "judge trust" UI. Do the math and the display together; do not ship the κ surfacing (#62) without #197's correction.

4. **#142 + #143 — RSI Train/Test split + hard gate.** #142 (bug) adds the missing Train/Dev/Test split to the data model; #143 (bug) adds the §21 hard gate that forbids `apply_change` until that split (plus real stats #111 and forked replay #141) is Done. The split is the precondition the gate enforces — design them as one safety unit. (Forms the critical path with #111 and #141.)

5. **#119 + #177 — Phantom Phase-7 spec.** #119 diagnoses that #105/#107/#108 cite a non-existent §20.10 / R18; #177 writes §20.10 into ARCHITECTURE.md to anchor them. Diagnosis + fix; close #119 when #177 lands. Quick, unblocks already-merged-but-unanchored work.

**Honorable mentions:** #205+#257 (local migration consolidation + backend-agnostic migrator — same subsystem); #211+#212 (duplicate MCP Trust Center — merge to #212); #307+#327 (Merkle research framing + engineering umbrella); #320+#326 (release-gate wedge + shareable evidence packets).
