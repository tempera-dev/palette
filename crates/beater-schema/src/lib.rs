use beater_core::{
    AgentReleaseId, ApiKeyId, ArtifactId, Clock, DatasetCaseId, DatasetId, DatasetVersionId,
    EnvironmentId, EvalResultId, EvaluatorId, EvaluatorVersionId, ExperimentId, GateId,
    IdempotencyKey, Money, Page, PageRequest, ProjectId, PromptId, PromptVersionId, RunId,
    Sha256Hash, SpanId, TenantId, TenantScope, Timestamp, TokenCounts, TraceId, WebhookEndpointId,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const RAW_SCHEMA_VERSION: u32 = 1;
pub const CANONICAL_SCHEMA_VERSION: u32 = 1;

pub type CanonicalAttrs = BTreeMap<String, Value>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceDialect {
    Native,
    Otlp,
    OpenInference,
    OpenTelemetryGenAi,
    VercelAiSdk,
    OpenLlmetry,
    PhoenixImport,
    LangSmithImport,
    LangfuseImport,
}

impl SourceDialect {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Otlp => "otlp",
            Self::OpenInference => "open_inference",
            Self::OpenTelemetryGenAi => "open_telemetry_gen_ai",
            Self::VercelAiSdk => "vercel_ai_sdk",
            Self::OpenLlmetry => "open_llmetry",
            Self::PhoenixImport => "phoenix_import",
            Self::LangSmithImport => "langsmith_import",
            Self::LangfuseImport => "langfuse_import",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentSpanKind {
    AgentRun,
    AgentTurn,
    AgentPlan,
    AgentStep,
    LlmCall,
    ToolCall,
    McpRequest,
    RetrievalQuery,
    MemoryRead,
    MemoryWrite,
    GuardrailCheck,
    HumanReview,
    EvaluatorRun,
    ReplayRun,
}

impl AgentSpanKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AgentRun => "agent.run",
            Self::AgentTurn => "agent.turn",
            Self::AgentPlan => "agent.plan",
            Self::AgentStep => "agent.step",
            Self::LlmCall => "llm.call",
            Self::ToolCall => "tool.call",
            Self::McpRequest => "mcp.request",
            Self::RetrievalQuery => "retrieval.query",
            Self::MemoryRead => "memory.read",
            Self::MemoryWrite => "memory.write",
            Self::GuardrailCheck => "guardrail.check",
            Self::HumanReview => "human.review",
            Self::EvaluatorRun => "evaluator.run",
            Self::ReplayRun => "replay.run",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "agent.run" => Some(Self::AgentRun),
            "agent.turn" => Some(Self::AgentTurn),
            "agent.plan" => Some(Self::AgentPlan),
            "agent.step" => Some(Self::AgentStep),
            "llm.call" => Some(Self::LlmCall),
            "tool.call" => Some(Self::ToolCall),
            "mcp.request" => Some(Self::McpRequest),
            "retrieval.query" => Some(Self::RetrievalQuery),
            "memory.read" => Some(Self::MemoryRead),
            "memory.write" => Some(Self::MemoryWrite),
            "guardrail.check" => Some(Self::GuardrailCheck),
            "human.review" => Some(Self::HumanReview),
            "evaluator.run" => Some(Self::EvaluatorRun),
            "replay.run" => Some(Self::ReplayRun),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpanStatus {
    Ok,
    Error,
    Unset,
}

impl SpanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Error => "error",
            Self::Unset => "unset",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "ok" => Some(Self::Ok),
            "error" => Some(Self::Error),
            "unset" => Some(Self::Unset),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub artifact_id: ArtifactId,
    pub uri: String,
    pub sha256: Sha256Hash,
    pub size_bytes: u64,
    pub mime_type: String,
    pub redaction_class: RedactionClass,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    Public,
    Internal,
    Sensitive,
    Secret,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthContext {
    pub api_key_id: Option<ApiKeyId>,
    pub scopes: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawEnvelope {
    pub schema_version: u32,
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub environment_id: EnvironmentId,
    pub source: SourceDialect,
    pub source_schema_url: Option<String>,
    pub source_schema_version: Option<String>,
    pub received_at: Timestamp,
    pub idempotency_key: IdempotencyKey,
    pub payload_hash: Sha256Hash,
    pub body_ref: ArtifactRef,
    pub auth_context: AuthContext,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelRef {
    pub provider: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanonicalSpan {
    pub schema_version: u32,
    pub normalizer_version: String,
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub environment_id: EnvironmentId,
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub seq: u64,
    pub kind: AgentSpanKind,
    pub name: String,
    pub status: SpanStatus,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub model: Option<ModelRef>,
    pub cost: Option<Money>,
    pub tokens: Option<TokenCounts>,
    pub input_ref: Option<ArtifactRef>,
    pub output_ref: Option<ArtifactRef>,
    pub attributes: CanonicalAttrs,
    pub unmapped_attrs: Value,
    pub raw_ref: ArtifactRef,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CanonicalTraceBatch {
    pub raw_envelopes: Vec<RawEnvelope>,
    pub spans: Vec<CanonicalSpan>,
}

impl CanonicalTraceBatch {
    pub fn one(raw_envelope: RawEnvelope, span: CanonicalSpan) -> Self {
        Self {
            raw_envelopes: vec![raw_envelope],
            spans: vec![span],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteAck {
    pub accepted_raw: usize,
    pub accepted_spans: usize,
    pub duplicate_raw: usize,
    pub duplicate_spans: usize,
}

impl WriteAck {
    pub fn total_accepted(&self) -> usize {
        self.accepted_raw + self.accepted_spans
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TraceView {
    pub tenant_id: TenantId,
    pub trace_id: TraceId,
    pub spans: Vec<CanonicalSpan>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunSummary {
    pub tenant_id: TenantId,
    pub trace_id: TraceId,
    pub first_span_name: String,
    pub span_count: usize,
    pub status: SpanStatus,
    pub started_at: Timestamp,
    pub ended_at: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpanSummary {
    pub tenant_id: TenantId,
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub kind: AgentSpanKind,
    pub name: String,
    pub status: SpanStatus,
    pub started_at: Timestamp,
    pub ended_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunFilter {
    pub trace_id: Option<TraceId>,
    pub status: Option<SpanStatus>,
    pub kind: Option<AgentSpanKind>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpanFilter {
    pub trace_id: Option<TraceId>,
    pub span_id: Option<SpanId>,
    pub kind: Option<AgentSpanKind>,
    pub status: Option<SpanStatus>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluatorLane {
    DeterministicWasi,
    JudgeBroker,
    Human,
    Hybrid,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluatorVersion {
    pub evaluator_id: EvaluatorId,
    pub version_id: EvaluatorVersionId,
    pub lane: EvaluatorLane,
    pub name: String,
    pub code_hash: Option<Sha256Hash>,
    pub wasm_hash: Option<Sha256Hash>,
    pub rubric_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalReproducibility {
    pub dataset_version_id: DatasetVersionId,
    pub dataset_case_id: DatasetCaseId,
    pub agent_release_id: AgentReleaseId,
    pub prompt_version_id: Option<PromptVersionId>,
    pub evaluator_version_id: EvaluatorVersionId,
    pub code_hash: Option<Sha256Hash>,
    pub wasm_hash: Option<Sha256Hash>,
    pub wasi_abi_version: Option<String>,
    pub judge_model_id: Option<String>,
    pub judge_provider: Option<String>,
    pub judge_parameters: Value,
    pub judge_seed: Option<u64>,
    pub judge_rubric_version: Option<String>,
    pub normalizer_version: String,
    pub trace_schema_version: u32,
    pub input_artifact_hashes: Vec<Sha256Hash>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalResult {
    pub eval_result_id: EvalResultId,
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub trace_id: TraceId,
    pub span_id: Option<SpanId>,
    pub score: f64,
    pub label: Option<String>,
    pub evidence: Value,
    pub reproducibility: EvalReproducibility,
    pub cost: Option<Money>,
    pub tokens: Option<TokenCounts>,
    pub created_at: Timestamp,
    pub non_reproducible_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceCompletionState {
    Open,
    RootEnded,
    IdleComplete,
    LateWindowClosed,
    Complete,
    Incomplete,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCassette {
    pub tenant_id: TenantId,
    pub trace_id: TraceId,
    pub provider_events: usize,
    pub tool_events: usize,
    pub memory_events: usize,
    pub retrieval_events: usize,
    pub clock_events: usize,
    pub random_events: usize,
    pub missing_required_kinds: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetVersion {
    pub dataset_id: DatasetId,
    pub version_id: DatasetVersionId,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Experiment {
    pub experiment_id: ExperimentId,
    pub dataset_version_id: DatasetVersionId,
    pub baseline_release_id: AgentReleaseId,
    pub candidate_release_id: AgentReleaseId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gate {
    pub gate_id: GateId,
    pub min_sample_size: usize,
    pub max_regression: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptVersion {
    pub prompt_id: PromptId,
    pub version_id: PromptVersionId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    pub webhook_endpoint_id: WebhookEndpointId,
    pub signing_key_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Run {
    pub run_id: RunId,
    pub trace_id: TraceId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_ref: ArtifactRef,
}

pub type RunPage = Page<RunSummary>;
pub type SpanPage = Page<SpanSummary>;
pub type QueryPageRequest = PageRequest;

pub fn make_idempotency_key(
    scope: &TenantScope,
    trace_id: &TraceId,
    span_id: &SpanId,
    seq: u64,
    payload_hash: &Sha256Hash,
) -> Result<IdempotencyKey, beater_core::IdError> {
    IdempotencyKey::new(format!(
        "{}:{}:{}:{}:{}:{}",
        scope.tenant_id.as_str(),
        scope.project_id.as_str(),
        trace_id.as_str(),
        span_id.as_str(),
        seq,
        payload_hash.as_str()
    ))
}

pub fn now(clock: &(impl Clock + ?Sized)) -> Timestamp {
    clock.now()
}

pub fn span_matches(span: &CanonicalSpan, filter: &SpanFilter) -> bool {
    if let Some(trace_id) = &filter.trace_id {
        if span.trace_id != *trace_id {
            return false;
        }
    }
    if let Some(span_id) = &filter.span_id {
        if span.span_id != *span_id {
            return false;
        }
    }
    if let Some(kind) = &filter.kind {
        if span.kind != *kind {
            return false;
        }
    }
    if let Some(status) = &filter.status {
        if span.status != *status {
            return false;
        }
    }
    true
}

pub fn span_summary(span: CanonicalSpan) -> SpanSummary {
    SpanSummary {
        tenant_id: span.tenant_id,
        trace_id: span.trace_id,
        span_id: span.span_id,
        kind: span.kind,
        name: span.name,
        status: span.status,
        started_at: span.start_time,
        ended_at: span.end_time,
    }
}

pub fn roll_up_runs(tenant: TenantId, spans: Vec<SpanSummary>) -> Vec<RunSummary> {
    let mut runs = Vec::<RunSummary>::new();
    for span in spans {
        if let Some(run) = runs.iter_mut().find(|run| run.trace_id == span.trace_id) {
            run.span_count += 1;
            if run.status != SpanStatus::Error && span.status == SpanStatus::Error {
                run.status = SpanStatus::Error;
            }
            run.ended_at = match (run.ended_at, span.ended_at) {
                (Some(left), Some(right)) => Some(left.max(right)),
                (None, Some(right)) => Some(right),
                (left, None) => left,
            };
        } else {
            runs.push(RunSummary {
                tenant_id: tenant.clone(),
                trace_id: span.trace_id,
                first_span_name: span.name,
                span_count: 1,
                status: span.status,
                started_at: span.started_at,
                ended_at: span.ended_at,
            });
        }
    }

    runs.sort_by(|left, right| right.started_at.cmp(&left.started_at));
    runs
}

#[cfg(test)]
mod tests {
    use super::*;
    use beater_core::{EnvironmentId, ProjectId, TenantId};

    #[test]
    fn span_taxonomy_is_agent_native() {
        assert_eq!(AgentSpanKind::AgentRun.as_str(), "agent.run");
        assert_eq!(AgentSpanKind::McpRequest.as_str(), "mcp.request");
        assert_eq!(AgentSpanKind::ReplayRun.as_str(), "replay.run");
        assert_eq!(
            AgentSpanKind::parse("agent.step"),
            Some(AgentSpanKind::AgentStep)
        );
        assert_eq!(AgentSpanKind::parse("bogus"), None);
        assert_eq!(SpanStatus::Error.as_str(), "error");
        assert_eq!(SpanStatus::parse("unset"), Some(SpanStatus::Unset));
    }

    #[test]
    fn idempotency_key_includes_tenant_and_hash() {
        let scope = TenantScope::new(
            TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}")),
            ProjectId::new("project").unwrap_or_else(|err| panic!("{err}")),
            EnvironmentId::new("prod").unwrap_or_else(|err| panic!("{err}")),
        );
        let trace_id = TraceId::new("trace").unwrap_or_else(|err| panic!("{err}"));
        let span_id = SpanId::new("span").unwrap_or_else(|err| panic!("{err}"));
        let hash = Sha256Hash::new("abc").unwrap_or_else(|err| panic!("{err}"));

        let key = make_idempotency_key(&scope, &trace_id, &span_id, 7, &hash)
            .unwrap_or_else(|err| panic!("{err}"));

        assert_eq!(key.as_str(), "tenant:project:trace:span:7:abc");
    }
}
