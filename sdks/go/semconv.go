// Package palette provides ergonomic OpenTelemetry tracing helpers that emit
// spans Palette understands, mirroring the Python and TypeScript SDKs.
package palette

// Span kinds. These are the openinference.span.kind values accepted by the
// paletted OTLP normalizer. They mirror sdks/python/palette/semconv.py and the
// Rust normalizer in crates/palette-otlp -- keep them in lockstep with the
// server. This file is the single source of truth for the Go SDK.
const (
	KindAgentRun       = "agent.run"
	KindAgentTurn      = "agent.turn"
	KindAgentPlan      = "agent.plan"
	KindAgentStep      = "agent.step"
	KindLLMCall        = "llm.call"
	KindToolCall       = "tool.call"
	KindMCPRequest     = "mcp.request"
	KindRetrievalQuery = "retrieval.query"
	KindMemoryRead     = "memory.read"
	KindMemoryWrite    = "memory.write"
	KindGuardrailCheck = "guardrail.check"
)

// SpanKinds is every accepted span kind, for validation.
var SpanKinds = map[string]struct{}{
	KindAgentRun:       {},
	KindAgentTurn:      {},
	KindAgentPlan:      {},
	KindAgentStep:      {},
	KindLLMCall:        {},
	KindToolCall:       {},
	KindMCPRequest:     {},
	KindRetrievalQuery: {},
	KindMemoryRead:     {},
	KindMemoryWrite:    {},
	KindGuardrailCheck: {},
}

// Canonical span attribute keys.
const (
	AttrSpanKind  = "openinference.span.kind"
	AttrSeq       = "palette.seq"
	AttrReleaseID = "agent.release_id"

	AttrInputValue  = "input.value"
	AttrOutputValue = "output.value"

	AttrLLMProvider        = "llm.provider"
	AttrLLMModelName       = "llm.model_name"
	AttrLLMTokenPrompt     = "llm.token_count.prompt"
	AttrLLMTokenCompletion = "llm.token_count.completion"
	AttrLLMTokenReasoning  = "llm.token_count.reasoning"
	AttrLLMTokenCacheRead  = "llm.token_count.cache_read"
	AttrLLMCostMicros      = "llm.cost.amount_micros"
	AttrLLMCostCurrency    = "llm.cost.currency"

	AttrDiscoveryCampaignID            = "tempera.discovery.campaign_id"
	AttrDiscoveryRoundID               = "tempera.discovery.round_id"
	AttrDiscoveryStage                 = "tempera.discovery.stage"
	AttrDiscoveryStatus                = "tempera.discovery.status"
	AttrDiscoveryEvidenceClass         = "tempera.discovery.evidence_class"
	AttrDiscoveryClaimClass            = "tempera.discovery.claim_class"
	AttrDiscoveryCandidateCount        = "tempera.discovery.candidate_count"
	AttrDiscoverySelectedCount         = "tempera.discovery.selected_count"
	AttrDiscoveryVerifiedCount         = "tempera.discovery.verified_count"
	AttrDiscoveryBudgetLimit           = "tempera.discovery.budget.limit"
	AttrDiscoveryBudgetConsumed        = "tempera.discovery.budget.consumed"
	AttrDiscoveryProgramDigest         = "tempera.discovery.program.digest"
	AttrDiscoveryProposalDigest        = "tempera.discovery.proposal.digest"
	AttrDiscoveryProtocolDigest        = "tempera.discovery.protocol.digest"
	AttrDiscoveryPrepareReceiptDigest  = "tempera.discovery.receipt.prepare.digest"
	AttrDiscoveryCommitReceiptDigest   = "tempera.discovery.receipt.commit.digest"
	AttrDiscoveryVerifierReceiptDigest = "tempera.discovery.receipt.verifier.digest"
	AttrDiscoveryDecisionReceiptDigest = "tempera.discovery.receipt.decision.digest"
	AttrDiscoveryReleaseDigest         = "tempera.discovery.release.digest"
)

// OTLP scope headers used when exporting over gRPC (HTTP carries these in the URL path).
const (
	HeaderTenant      = "x-palette-tenant-id"
	HeaderProject     = "x-palette-project-id"
	HeaderEnvironment = "x-palette-environment-id"
)
