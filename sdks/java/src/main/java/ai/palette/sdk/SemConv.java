package ai.palette.sdk;

import java.util.Set;

/**
 * Palette semantic conventions: the single source of truth for span-kind and
 * attribute keys in the Java SDK.
 *
 * <p>These strings mirror the Rust normalizer in {@code crates/palette-otlp} and
 * the canonical kinds in {@code crates/palette-schema}, and stay in lockstep with
 * the Python and TypeScript SDKs. They are part of the cross-language ingest
 * contract; keep them aligned with the server.
 */
public final class SemConv {

    private SemConv() {}

    // --- OpenInference span kinds (openinference.span.kind values) ---
    public static final String AGENT_RUN = "agent.run";
    public static final String AGENT_TURN = "agent.turn";
    public static final String AGENT_PLAN = "agent.plan";
    public static final String AGENT_STEP = "agent.step";
    public static final String LLM_CALL = "llm.call";
    public static final String TOOL_CALL = "tool.call";
    public static final String MCP_REQUEST = "mcp.request";
    public static final String RETRIEVAL_QUERY = "retrieval.query";
    public static final String MEMORY_READ = "memory.read";
    public static final String MEMORY_WRITE = "memory.write";
    public static final String GUARDRAIL_CHECK = "guardrail.check";

    /** Every accepted span kind, for validation. */
    public static final Set<String> SPAN_KINDS = Set.of(
            AGENT_RUN,
            AGENT_TURN,
            AGENT_PLAN,
            AGENT_STEP,
            LLM_CALL,
            TOOL_CALL,
            MCP_REQUEST,
            RETRIEVAL_QUERY,
            MEMORY_READ,
            MEMORY_WRITE,
            GUARDRAIL_CHECK);

    // --- Canonical span attribute keys ---
    public static final String SPAN_KIND = "openinference.span.kind";
    public static final String SEQ = "palette.seq";
    public static final String RELEASE_ID = "agent.release_id";

    public static final String INPUT_VALUE = "input.value";
    public static final String OUTPUT_VALUE = "output.value";

    public static final String LLM_PROVIDER = "llm.provider";
    public static final String LLM_MODEL_NAME = "llm.model_name";
    public static final String LLM_TOKEN_PROMPT = "llm.token_count.prompt";
    public static final String LLM_TOKEN_COMPLETION = "llm.token_count.completion";
    public static final String LLM_TOKEN_REASONING = "llm.token_count.reasoning";
    public static final String LLM_TOKEN_CACHE_READ = "llm.token_count.cache_read";
    public static final String LLM_COST_MICROS = "llm.cost.amount_micros";
    public static final String LLM_COST_CURRENCY = "llm.cost.currency";

    public static final String DISCOVERY_CAMPAIGN_ID = "tempera.discovery.campaign_id";
    public static final String DISCOVERY_ROUND_ID = "tempera.discovery.round_id";
    public static final String DISCOVERY_STAGE = "tempera.discovery.stage";
    public static final String DISCOVERY_STATUS = "tempera.discovery.status";
    public static final String DISCOVERY_EVIDENCE_CLASS = "tempera.discovery.evidence_class";
    public static final String DISCOVERY_CLAIM_CLASS = "tempera.discovery.claim_class";
    public static final String DISCOVERY_CANDIDATE_COUNT = "tempera.discovery.candidate_count";
    public static final String DISCOVERY_SELECTED_COUNT = "tempera.discovery.selected_count";
    public static final String DISCOVERY_VERIFIED_COUNT = "tempera.discovery.verified_count";
    public static final String DISCOVERY_BUDGET_LIMIT = "tempera.discovery.budget.limit";
    public static final String DISCOVERY_BUDGET_CONSUMED = "tempera.discovery.budget.consumed";
    public static final String DISCOVERY_PROGRAM_DIGEST = "tempera.discovery.program.digest";
    public static final String DISCOVERY_PROPOSAL_DIGEST = "tempera.discovery.proposal.digest";
    public static final String DISCOVERY_PROTOCOL_DIGEST = "tempera.discovery.protocol.digest";
    public static final String DISCOVERY_PREPARE_RECEIPT_DIGEST =
            "tempera.discovery.receipt.prepare.digest";
    public static final String DISCOVERY_COMMIT_RECEIPT_DIGEST =
            "tempera.discovery.receipt.commit.digest";
    public static final String DISCOVERY_VERIFIER_RECEIPT_DIGEST =
            "tempera.discovery.receipt.verifier.digest";
    public static final String DISCOVERY_DECISION_RECEIPT_DIGEST =
            "tempera.discovery.receipt.decision.digest";
    public static final String DISCOVERY_RELEASE_DIGEST = "tempera.discovery.release.digest";

    // --- OTLP ingest headers (used to scope traces over gRPC) ---
    public static final String HEADER_TENANT = "x-palette-tenant-id";
    public static final String HEADER_PROJECT = "x-palette-project-id";
    public static final String HEADER_ENVIRONMENT = "x-palette-environment-id";
}
