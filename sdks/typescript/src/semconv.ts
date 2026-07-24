/**
 * Palette semantic conventions — the single place the TS SDK defines span-kind
 * and attribute keys. Every wrapper imports from here so instrumentation cannot
 * drift within the SDK. These mirror the Rust normalizer (`crates/palette-otlp`)
 * and canonical kinds (`crates/palette-schema`); keep them in lockstep.
 */

export const SpanKind = {
  AGENT_RUN: "agent.run",
  AGENT_TURN: "agent.turn",
  AGENT_PLAN: "agent.plan",
  AGENT_STEP: "agent.step",
  LLM_CALL: "llm.call",
  TOOL_CALL: "tool.call",
  MCP_REQUEST: "mcp.request",
  RETRIEVAL_QUERY: "retrieval.query",
  MEMORY_READ: "memory.read",
  MEMORY_WRITE: "memory.write",
  GUARDRAIL_CHECK: "guardrail.check",
} as const;

export type SpanKindValue = (typeof SpanKind)[keyof typeof SpanKind];

export const SPAN_KINDS: ReadonlySet<string> = new Set(Object.values(SpanKind));

export const Attr = {
  SPAN_KIND: "openinference.span.kind",
  SEQ: "palette.seq",
  RELEASE_ID: "agent.release_id",
  INPUT_VALUE: "input.value",
  OUTPUT_VALUE: "output.value",
  LLM_PROVIDER: "llm.provider",
  LLM_MODEL_NAME: "llm.model_name",
  LLM_TOKEN_PROMPT: "llm.token_count.prompt",
  LLM_TOKEN_COMPLETION: "llm.token_count.completion",
  LLM_TOKEN_REASONING: "llm.token_count.reasoning",
  LLM_TOKEN_CACHE_READ: "llm.token_count.cache_read",
  LLM_COST_MICROS: "llm.cost.amount_micros",
  LLM_COST_CURRENCY: "llm.cost.currency",
  DISCOVERY_CAMPAIGN_ID: "tempera.discovery.campaign_id",
  DISCOVERY_ROUND_ID: "tempera.discovery.round_id",
  DISCOVERY_STAGE: "tempera.discovery.stage",
  DISCOVERY_STATUS: "tempera.discovery.status",
  DISCOVERY_EVIDENCE_CLASS: "tempera.discovery.evidence_class",
  DISCOVERY_CLAIM_CLASS: "tempera.discovery.claim_class",
  DISCOVERY_CANDIDATE_COUNT: "tempera.discovery.candidate_count",
  DISCOVERY_SELECTED_COUNT: "tempera.discovery.selected_count",
  DISCOVERY_VERIFIED_COUNT: "tempera.discovery.verified_count",
  DISCOVERY_BUDGET_LIMIT: "tempera.discovery.budget.limit",
  DISCOVERY_BUDGET_CONSUMED: "tempera.discovery.budget.consumed",
  DISCOVERY_PROGRAM_DIGEST: "tempera.discovery.program.digest",
  DISCOVERY_PROPOSAL_DIGEST: "tempera.discovery.proposal.digest",
  DISCOVERY_PROTOCOL_DIGEST: "tempera.discovery.protocol.digest",
  DISCOVERY_PREPARE_RECEIPT_DIGEST: "tempera.discovery.receipt.prepare.digest",
  DISCOVERY_COMMIT_RECEIPT_DIGEST: "tempera.discovery.receipt.commit.digest",
  DISCOVERY_VERIFIER_RECEIPT_DIGEST: "tempera.discovery.receipt.verifier.digest",
  DISCOVERY_DECISION_RECEIPT_DIGEST: "tempera.discovery.receipt.decision.digest",
  DISCOVERY_RELEASE_DIGEST: "tempera.discovery.release.digest",
} as const;

export const HEADER_TENANT = "x-palette-tenant-id";
export const HEADER_PROJECT = "x-palette-project-id";
export const HEADER_ENVIRONMENT = "x-palette-environment-id";
