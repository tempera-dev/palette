"""Palette semantic conventions.

This is the SINGLE place the Python SDK defines span-kind and attribute keys.
Every wrapper (`observe`, provider wrappers, and framework callbacks) imports
from here so the instrumentation can never drift within the SDK.

These strings mirror the Rust normalizer in ``crates/palette-otlp`` and the
canonical kinds in ``crates/palette-schema``. They are part of the cross-language
ingest contract; keep them in lockstep with the server. (A future codegen step
can emit this file from the Rust source -- see ``sdks/README.md``.)
"""

from __future__ import annotations


class SpanKind:
    """OpenInference ``openinference.span.kind`` values accepted by the normalizer."""

    AGENT_RUN = "agent.run"
    AGENT_TURN = "agent.turn"
    AGENT_PLAN = "agent.plan"
    AGENT_STEP = "agent.step"
    LLM_CALL = "llm.call"
    TOOL_CALL = "tool.call"
    MCP_REQUEST = "mcp.request"
    RETRIEVAL_QUERY = "retrieval.query"
    MEMORY_READ = "memory.read"
    MEMORY_WRITE = "memory.write"
    GUARDRAIL_CHECK = "guardrail.check"


#: Every accepted span kind, for validation.
SPAN_KINDS = frozenset(
    value
    for name, value in vars(SpanKind).items()
    if not name.startswith("_") and isinstance(value, str)
)


class Attr:
    """Canonical span attribute keys."""

    SPAN_KIND = "openinference.span.kind"
    SEQ = "palette.seq"
    RELEASE_ID = "agent.release_id"

    INPUT_VALUE = "input.value"
    OUTPUT_VALUE = "output.value"

    LLM_PROVIDER = "llm.provider"
    LLM_MODEL_NAME = "llm.model_name"
    LLM_TOKEN_PROMPT = "llm.token_count.prompt"
    LLM_TOKEN_COMPLETION = "llm.token_count.completion"
    LLM_TOKEN_REASONING = "llm.token_count.reasoning"
    LLM_TOKEN_CACHE_READ = "llm.token_count.cache_read"
    LLM_COST_MICROS = "llm.cost.amount_micros"
    LLM_COST_CURRENCY = "llm.cost.currency"

    DISCOVERY_CAMPAIGN_ID = "tempera.discovery.campaign_id"
    DISCOVERY_ROUND_ID = "tempera.discovery.round_id"
    DISCOVERY_STAGE = "tempera.discovery.stage"
    DISCOVERY_STATUS = "tempera.discovery.status"
    DISCOVERY_EVIDENCE_CLASS = "tempera.discovery.evidence_class"
    DISCOVERY_CLAIM_CLASS = "tempera.discovery.claim_class"
    DISCOVERY_CANDIDATE_COUNT = "tempera.discovery.candidate_count"
    DISCOVERY_SELECTED_COUNT = "tempera.discovery.selected_count"
    DISCOVERY_VERIFIED_COUNT = "tempera.discovery.verified_count"
    DISCOVERY_BUDGET_LIMIT = "tempera.discovery.budget.limit"
    DISCOVERY_BUDGET_CONSUMED = "tempera.discovery.budget.consumed"
    DISCOVERY_PROGRAM_DIGEST = "tempera.discovery.program.digest"
    DISCOVERY_PROPOSAL_DIGEST = "tempera.discovery.proposal.digest"
    DISCOVERY_PROTOCOL_DIGEST = "tempera.discovery.protocol.digest"
    DISCOVERY_PREPARE_RECEIPT_DIGEST = "tempera.discovery.receipt.prepare.digest"
    DISCOVERY_COMMIT_RECEIPT_DIGEST = "tempera.discovery.receipt.commit.digest"
    DISCOVERY_VERIFIER_RECEIPT_DIGEST = "tempera.discovery.receipt.verifier.digest"
    DISCOVERY_DECISION_RECEIPT_DIGEST = "tempera.discovery.receipt.decision.digest"
    DISCOVERY_RELEASE_DIGEST = "tempera.discovery.release.digest"


#: OTLP ingest headers used to scope traces when exporting over gRPC.
HEADER_TENANT = "x-palette-tenant-id"
HEADER_PROJECT = "x-palette-project-id"
HEADER_ENVIRONMENT = "x-palette-environment-id"
