# operationId rename: `{collection}.{kebab-verb-noun}` -> `{collection}.{method}`

Ecosystem AIP scheme (tempera-api-style-guide.md §4): lower-camel collection, dot,
lower-camel standard-method or custom verb. The redundant resource-noun that merely
echoes the collection is dropped; a distinguishing qualifier is kept. All 60 new ids
are unique (verified). Collisions were resolved with qualifiers, never merged.

| # | Collection (tag) | Old operationId | New operationId |
|---|------------------|-----------------|-----------------|
| 1 | `health` | `health.health` | `health.check` |
| 2 | `ingest` | `ingest.ingest-native` | `ingest.native` |
| 3 | `apiKeys` | `apiKeys.create-api-key` | `apiKeys.create` |
| 4 | `apiKeys` | `apiKeys.revoke-api-key` | `apiKeys.revoke` |
| 5 | `providerSecrets` | `providerSecrets.create-provider-secret` | `providerSecrets.create` |
| 6 | `providerSecrets` | `providerSecrets.list-provider-secrets` | `providerSecrets.list` |
| 7 | `providerSecrets` | `providerSecrets.revoke-provider-secret` | `providerSecrets.revoke` |
| 8 | `connectors` | `connectors.list-connectors` | `connectors.list` |
| 9 | `connectors` | `connectors.list-connector-tools` | `connectors.listTools` |
| 10 | `connectors` | `connectors.get-connector-skills` | `connectors.getSkills` |
| 11 | `connectors` | `connectors.connect-connector` | `connectors.connect` |
| 12 | `connectors` | `connectors.connector-status` | `connectors.status` |
| 13 | `connectors` | `connectors.invoke-connector-tool` | `connectors.invokeTool` |
| 14 | `judge` | `judge.evaluate-judge` | `judge.evaluate` |
| 15 | `usage` | `usage.get-usage-summary` | `usage.getSummary` |
| 16 | `connect` | `connect.get-palette-connect-status` | `connect.getStatus` |
| 17 | `judge` | `judge.list-judge-ledger` | `judge.listLedger` |
| 18 | `ingest` | `ingest.get-ingest-queue-status` | `ingest.getQueueStatus` |
| 19 | `ingest` | `ingest.replay-dead-letter` | `ingest.replayDeadLetter` |
| 20 | `ingest` | `ingest.reconcile-trace` | `ingest.reconcileTrace` |
| 21 | `ingest` | `ingest.drain-trace-writes` | `ingest.drainTraceWrites` |
| 22 | `ingest` | `ingest.drain-trace-ingested` | `ingest.drainTraceIngested` |
| 23 | `ingest` | `ingest.ingest-otlp` | `ingest.otlp` |
| 24 | `ingest` | `ingest.ingest-otlp-json-collector` | `ingest.otlpJsonCollector` |
| 25 | `ingest` | `ingest.import-source` | `ingest.importSource` |
| 26 | `search` | `search.search-spans` | `search.spans` |
| 27 | `traces` | `traces.list-traces` | `traces.list` |
| 28 | `traces` | `traces.get-trace` | `traces.get` |
| 29 | `spans` | `spans.get-span` | `spans.get` |
| 30 | `spans` | `spans.get-span-io` | `spans.getIo` |
| 31 | `audit` | `audit.list-audit-events` | `audit.list` |
| 32 | `archive` | `archive.archive-trace` | `archive.archiveTrace` |
| 33 | `archive` | `archive.query-archive-spans` | `archive.querySpans` |
| 34 | `prompts` | `prompts.create-prompt` | `prompts.create` |
| 35 | `prompts` | `prompts.list-prompts` | `prompts.list` |
| 36 | `prompts` | `prompts.get-prompt` | `prompts.get` |
| 37 | `prompts` | `prompts.add-prompt-version` | `prompts.addVersion` |
| 38 | `prompts` | `prompts.list-prompt-versions` | `prompts.listVersions` |
| 39 | `prompts` | `prompts.diff-prompt-versions` | `prompts.diffVersions` |
| 40 | `datasets` | `datasets.create-dataset` | `datasets.create` |
| 41 | `scenarios` | `scenarios.create-scenario` | `scenarios.create` |
| 42 | `scenarios` | `scenarios.list-scenarios` | `scenarios.list` |
| 43 | `scenarios` | `scenarios.get-scenario` | `scenarios.get` |
| 44 | `scenarios` | `scenarios.mine-scenarios` | `scenarios.mine` |
| 45 | `datasets` | `datasets.promote-dataset-case-from-trace` | `datasets.promoteCaseFromTrace` |
| 46 | `datasets` | `datasets.create-dataset-version` | `datasets.createVersion` |
| 47 | `evals` | `evals.run-deterministic-eval` | `evals.runDeterministic` |
| 48 | `evals` | `evals.run-judge-eval` | `evals.runJudge` |
| 49 | `calibrations` | `calibrations.run-calibration` | `calibrations.run` |
| 50 | `experiments` | `experiments.run-deterministic-experiment` | `experiments.runDeterministic` |
| 51 | `experiments` | `experiments.run-judge-experiment` | `experiments.runJudge` |
| 52 | `gates` | `gates.create-gate` | `gates.create` |
| 53 | `gates` | `gates.run-gate` | `gates.run` |
| 54 | `reviews` | `reviews.create-review-queue` | `reviews.createQueue` |
| 55 | `reviews` | `reviews.list-review-tasks` | `reviews.listTasks` |
| 56 | `reviews` | `reviews.enqueue-review-task-from-trace` | `reviews.enqueueTaskFromTrace` |
| 57 | `reviews` | `reviews.submit-review-annotation` | `reviews.submitAnnotation` |
| 58 | `reviews` | `reviews.promote-review-annotation` | `reviews.promoteAnnotation` |
| 59 | `online` | `online.decide-online-sampling` | `online.decideSampling` |
| 60 | `alerts` | `alerts.evaluate-alert` | `alerts.evaluate` |

## Collisions resolved with qualifiers (not merged)

- `evals.run-deterministic-eval` / `evals.run-judge-eval` -> `evals.runDeterministic` / `evals.runJudge`
- `experiments.run-deterministic-experiment` / `experiments.run-judge-experiment` -> `experiments.runDeterministic` / `experiments.runJudge`
- `datasets.create-dataset` / `datasets.create-dataset-version` -> `datasets.create` / `datasets.createVersion`
- `prompts.list-prompts` / `prompts.list-prompt-versions` -> `prompts.list` / `prompts.listVersions`
- `spans.get-span` / `spans.get-span-io` -> `spans.get` / `spans.getIo`
- `connectors.list-connectors` / `connectors.list-connector-tools` -> `connectors.list` / `connectors.listTools`
- `ingest.drain-trace-writes` / `ingest.drain-trace-ingested` -> `ingest.drainTraceWrites` / `ingest.drainTraceIngested`
- `ingest.ingest-otlp` / `ingest.ingest-otlp-json-collector` -> `ingest.otlp` / `ingest.otlpJsonCollector`

## Places where the mechanical rule needed human judgment

- `health.health` -> `health.check`: no CRUD verb and the noun echoes the collection; `health.health` is redundant, so a distinctive custom verb (`check`) was chosen.
- `archive.archive-trace` -> `archive.archiveTrace`: `archive.archive` is unclear, so the distinguishing object (`Trace`) is retained per the task guidance.
- `archive.query-archive-spans` -> `archive.querySpans`: dropped the `archive` token that echoes the collection, kept `querySpans`.
- `search.search-spans` -> `search.spans`: dropped the `search` verb that echoes the collection, leaving the distinguishing object `spans` as the method.
- `ingest.ingest-native` / `ingest.ingest-otlp` / `ingest.ingest-otlp-json-collector` -> `ingest.native` / `ingest.otlp` / `ingest.otlpJsonCollector`: dropped the `ingest` token that echoes the collection.
- `connect.get-palette-connect-status` -> `connect.getStatus` and `connectors.connector-status` -> `connectors.status`: both drop the echoed collection token; the `get` verb is kept only where it was present in the source id.
- Custom (non-CRUD) methods keep a distinctive lower-camel verb: `gates.run`, `alerts.evaluate`, `judge.evaluate`, `scenarios.mine`, `online.decideSampling`, `connectors.invokeTool`, `datasets.promoteCaseFromTrace`, `reviews.enqueueTaskFromTrace`, etc.
