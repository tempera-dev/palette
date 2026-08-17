//! Enterprise decision/evidence contract for Palette's validation plane.
//!
//! A [`DecisionRecordV1`] is an immutable observation of one important agent
//! decision. It is deliberately not an authorization token: source-owned
//! authority and execution evidence are referenced by digest and must be
//! verified by the appropriate consumer.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use thiserror::Error;
use utoipa::ToSchema;

pub const DECISION_RECORD_SCHEMA_V1: &str = "data-engine.agent-decision-trace.v1";
pub const DECISION_RECORD_PRODUCER: &str = "tempera-validation";
pub const DECISION_RECORD_MAX_IDENTIFIER_CHARS: usize = 256;
pub const DECISION_RECORD_MAX_GOAL_CHARS: usize = 16_384;
pub const DECISION_RECORD_MAX_TARGET_CHARS: usize = 2_048;
pub const DECISION_RECORD_MAX_REFS: usize = 64;
pub const DECISION_RECORD_MAX_CLASSIFICATIONS: usize = 32;
pub const DECISION_RECORD_MAX_LATENCY_MS: u64 = 86_400_000;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DecisionRecordV1 {
    pub schema: String,
    pub decision_id: String,
    pub trace_id: String,
    pub session_id: String,
    pub parent_decision_digests: Vec<String>,
    pub workspace: DecisionWorkspaceV1,
    pub initiator: DecisionInitiatorV1,
    pub objective: DecisionObjectiveV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_call: Option<ModelCallEvidenceV1>,
    pub proposed_action: ProposedActionV1,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_digests: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memory_digests: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retrieval_digests: Vec<String>,
    pub authority: AuthorityObservationV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionObservationV1>,
    pub governance: DecisionGovernanceV1,
    pub provenance: DecisionProvenanceV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DecisionWorkspaceV1 {
    pub organization_id: String,
    pub project_id: String,
    pub environment_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DecisionInitiatorV1 {
    pub subject_digest: String,
    pub agent_id: String,
    pub agent_product: AgentProduct,
    pub surface: InteractionSurface,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_version: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentProduct {
    Cursor,
    ClaudeCode,
    Codex,
    Litellm,
    TemperaMcp,
    TemperaCode,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InteractionSurface {
    Mcp,
    Browser,
    Cli,
    Api,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DecisionObjectiveV1 {
    pub goal_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_family: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ModelCallEvidenceV1 {
    pub provider: String,
    pub model: String,
    pub prompt_digest: String,
    pub context_digest: String,
    pub response_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ProposedActionV1 {
    pub kind: String,
    pub effect_class: EffectClass,
    pub target: String,
    pub arguments_digest: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_digests: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_classifications: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EffectClass {
    Read,
    Write,
    ExternalSideEffect,
    Privileged,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AuthorityObservationV1 {
    pub outcome: AuthorityOutcome,
    pub source: AuthoritySource,
    pub evidence_digests: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityOutcome {
    NotEvaluated,
    ShadowAllow,
    ShadowDeny,
    Allow,
    Deny,
    RequireApproval,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySource {
    NotEvaluated,
    ValidationPolicy,
    SourceReceipt,
    CallerReported,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ExecutionObservationV1 {
    pub status: ExecutionStatus,
    pub attempt_digest: String,
    pub latency_ms: u64,
    pub success_source: SuccessSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_request_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_receipt_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effect_digest: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    NotStarted,
    Succeeded,
    Failed,
    OutcomeUnknown,
    Cancelled,
    Denied,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SuccessSource {
    CallerAsserted,
    Deterministic,
    Human,
    ExternalReceipt,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DecisionGovernanceV1 {
    pub allowed_uses: Vec<AllowedUse>,
    pub retention_class: String,
    pub raw_payload_policy: RawPayloadPolicy,
    pub data_classifications: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights_basis_digest: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUse {
    Audit,
    Analytics,
    Eval,
    InternalTraining,
    CustomerTraining,
    ExternalPublish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RawPayloadPolicy {
    DigestOnly,
    EncryptedRetained,
    NotRetained,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct DecisionProvenanceV1 {
    pub producer: String,
    pub producer_version: String,
    pub normalization_version: String,
    pub observed_at: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DecisionRecordError {
    #[error("{0} is invalid")]
    Invalid(&'static str),
    #[error("{0} contains duplicate values")]
    Duplicate(&'static str),
    #[error("evaluated authority observations require source evidence")]
    AuthorityEvidenceRequired,
    #[error("not_evaluated authority outcome and source must be paired")]
    InvalidAuthorityPair,
    #[error("training/publication reuse requires a rights basis digest")]
    RightsBasisRequired,
    #[error("audit use is mandatory for an enterprise decision record")]
    AuditUseRequired,
    #[error("terminal execution requires a result digest")]
    TerminalResultRequired,
    #[error("outcome_unknown must keep success source unknown")]
    UnknownOutcomeMustRemainUnknown,
    #[error("external receipt success requires provider receipt evidence")]
    ExternalReceiptRequired,
    #[error("disclosed goal does not match goal_digest")]
    GoalDigestMismatch,
}

impl DecisionRecordV1 {
    /// Validate the bounded, digest-first observation contract.
    ///
    /// This validates the trace envelope only. It does not verify referenced
    /// Auth Hub, Risk, MCP, or provider receipts.
    pub fn validate(&self) -> Result<(), DecisionRecordError> {
        if self.schema != DECISION_RECORD_SCHEMA_V1 {
            return Err(DecisionRecordError::Invalid("schema"));
        }

        self.validate_identity()?;
        self.validate_objective()?;
        self.validate_model_call()?;
        self.validate_action_and_evidence()?;
        self.validate_authority()?;
        self.validate_execution()?;
        self.validate_governance()?;
        self.validate_provenance()?;
        Ok(())
    }

    fn validate_identity(&self) -> Result<(), DecisionRecordError> {
        validate_identifier(&self.decision_id, "decision_id")?;
        validate_identifier(&self.trace_id, "trace_id")?;
        validate_identifier(&self.session_id, "session_id")?;
        validate_digest_vec(&self.parent_decision_digests, "parent_decision_digests")?;
        for (field, value) in [
            ("workspace.organization_id", &self.workspace.organization_id),
            ("workspace.project_id", &self.workspace.project_id),
            ("workspace.environment_id", &self.workspace.environment_id),
            ("initiator.agent_id", &self.initiator.agent_id),
        ] {
            validate_identifier(value, field)?;
        }
        validate_digest(&self.initiator.subject_digest, "initiator.subject_digest")?;
        if let Some(version) = &self.initiator.harness_version {
            validate_bounded(version, 256, "initiator.harness_version")?;
        }
        Ok(())
    }

    fn validate_objective(&self) -> Result<(), DecisionRecordError> {
        validate_digest(&self.objective.goal_digest, "objective.goal_digest")?;
        if let Some(goal) = &self.objective.goal {
            validate_bounded(goal, DECISION_RECORD_MAX_GOAL_CHARS, "objective.goal")?;
            let expected = format!("sha256:{:x}", Sha256::digest(goal.as_bytes()));
            if self.objective.goal_digest != expected {
                return Err(DecisionRecordError::GoalDigestMismatch);
            }
        }
        if let Some(task_family) = &self.objective.task_family {
            validate_identifier(task_family, "objective.task_family")?;
        }
        Ok(())
    }

    fn validate_model_call(&self) -> Result<(), DecisionRecordError> {
        let Some(model) = &self.model_call else {
            return Ok(());
        };
        validate_identifier(&model.provider, "model_call.provider")?;
        validate_bounded(&model.model, 512, "model_call.model")?;
        validate_digest(&model.prompt_digest, "model_call.prompt_digest")?;
        validate_digest(&model.context_digest, "model_call.context_digest")?;
        validate_digest(&model.response_digest, "model_call.response_digest")?;
        if let Some(value) = &model.model_revision {
            validate_bounded(value, 512, "model_call.model_revision")?;
        }
        if let Some(value) = &model.request_digest {
            validate_digest(value, "model_call.request_digest")?;
        }
        if model
            .latency_ms
            .is_some_and(|value| value > DECISION_RECORD_MAX_LATENCY_MS)
        {
            return Err(DecisionRecordError::Invalid("model_call.latency_ms"));
        }
        Ok(())
    }

    fn validate_action_and_evidence(&self) -> Result<(), DecisionRecordError> {
        validate_identifier(&self.proposed_action.kind, "proposed_action.kind")?;
        validate_bounded(
            &self.proposed_action.target,
            DECISION_RECORD_MAX_TARGET_CHARS,
            "proposed_action.target",
        )?;
        validate_digest(
            &self.proposed_action.arguments_digest,
            "proposed_action.arguments_digest",
        )?;
        validate_digest_vec(
            &self.proposed_action.resource_digests,
            "proposed_action.resource_digests",
        )?;
        validate_classifications(
            &self.proposed_action.data_classifications,
            "proposed_action.data_classifications",
        )?;
        validate_digest_vec(&self.evidence_digests, "evidence_digests")?;
        validate_digest_vec(&self.memory_digests, "memory_digests")?;
        validate_digest_vec(&self.retrieval_digests, "retrieval_digests")?;
        Ok(())
    }

    fn validate_authority(&self) -> Result<(), DecisionRecordError> {
        validate_digest_vec(
            &self.authority.evidence_digests,
            "authority.evidence_digests",
        )?;
        let outcome_unset = self.authority.outcome == AuthorityOutcome::NotEvaluated;
        let source_unset = self.authority.source == AuthoritySource::NotEvaluated;
        if outcome_unset != source_unset {
            return Err(DecisionRecordError::InvalidAuthorityPair);
        }
        if !outcome_unset && self.authority.evidence_digests.is_empty() {
            return Err(DecisionRecordError::AuthorityEvidenceRequired);
        }
        Ok(())
    }

    fn validate_execution(&self) -> Result<(), DecisionRecordError> {
        let Some(execution) = &self.execution else {
            return Ok(());
        };
        validate_digest(&execution.attempt_digest, "execution.attempt_digest")?;
        if execution.latency_ms > DECISION_RECORD_MAX_LATENCY_MS {
            return Err(DecisionRecordError::Invalid("execution.latency_ms"));
        }
        for (field, value) in [
            (
                "execution.provider_request_digest",
                &execution.provider_request_digest,
            ),
            (
                "execution.provider_receipt_digest",
                &execution.provider_receipt_digest,
            ),
            ("execution.result_digest", &execution.result_digest),
            (
                "execution.side_effect_digest",
                &execution.side_effect_digest,
            ),
        ] {
            if let Some(value) = value {
                validate_digest(value, field)?;
            }
        }
        if matches!(
            execution.status,
            ExecutionStatus::Succeeded | ExecutionStatus::Failed | ExecutionStatus::Cancelled
        ) && execution.result_digest.is_none()
        {
            return Err(DecisionRecordError::TerminalResultRequired);
        }
        if execution.status == ExecutionStatus::OutcomeUnknown
            && execution.success_source != SuccessSource::Unknown
        {
            return Err(DecisionRecordError::UnknownOutcomeMustRemainUnknown);
        }
        if execution.success_source == SuccessSource::ExternalReceipt
            && execution.provider_receipt_digest.is_none()
        {
            return Err(DecisionRecordError::ExternalReceiptRequired);
        }
        Ok(())
    }

    fn validate_governance(&self) -> Result<(), DecisionRecordError> {
        validate_identifier(
            &self.governance.retention_class,
            "governance.retention_class",
        )?;
        validate_classifications(
            &self.governance.data_classifications,
            "governance.data_classifications",
        )?;
        validate_unique(&self.governance.allowed_uses, "governance.allowed_uses")?;
        if !self.governance.allowed_uses.contains(&AllowedUse::Audit) {
            return Err(DecisionRecordError::AuditUseRequired);
        }
        let needs_rights_basis = self.governance.allowed_uses.iter().any(|use_| {
            matches!(
                use_,
                AllowedUse::InternalTraining
                    | AllowedUse::CustomerTraining
                    | AllowedUse::ExternalPublish
            )
        });
        if needs_rights_basis && self.governance.rights_basis_digest.is_none() {
            return Err(DecisionRecordError::RightsBasisRequired);
        }
        if let Some(value) = &self.governance.rights_basis_digest {
            validate_digest(value, "governance.rights_basis_digest")?;
        }
        Ok(())
    }

    fn validate_provenance(&self) -> Result<(), DecisionRecordError> {
        if self.provenance.producer != DECISION_RECORD_PRODUCER {
            return Err(DecisionRecordError::Invalid("provenance.producer"));
        }
        validate_bounded(
            &self.provenance.producer_version,
            256,
            "provenance.producer_version",
        )?;
        validate_bounded(
            &self.provenance.normalization_version,
            256,
            "provenance.normalization_version",
        )?;
        validate_bounded(&self.provenance.observed_at, 64, "provenance.observed_at")?;
        Ok(())
    }
}

/// Return only the reuse capabilities permitted by every required source.
#[must_use]
pub fn intersect_allowed_uses<'a>(
    policies: impl IntoIterator<Item = &'a [AllowedUse]>,
) -> Vec<AllowedUse> {
    let mut policies = policies.into_iter();
    let Some(first) = policies.next() else {
        return Vec::new();
    };
    let mut intersection: BTreeSet<AllowedUse> = first.iter().copied().collect();
    for policy in policies {
        let current: BTreeSet<AllowedUse> = policy.iter().copied().collect();
        intersection = intersection.intersection(&current).copied().collect();
    }
    intersection.into_iter().collect()
}

fn validate_bounded(
    value: &str,
    max_chars: usize,
    field: &'static str,
) -> Result<(), DecisionRecordError> {
    if value.is_empty() || value.chars().count() > max_chars {
        return Err(DecisionRecordError::Invalid(field));
    }
    Ok(())
}

fn validate_identifier(value: &str, field: &'static str) -> Result<(), DecisionRecordError> {
    validate_bounded(value, DECISION_RECORD_MAX_IDENTIFIER_CHARS, field)?;
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return Err(DecisionRecordError::Invalid(field));
    };
    if !first.is_ascii_alphanumeric()
        || chars.any(|value| {
            !(value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | ':' | '/' | '-'))
        })
    {
        return Err(DecisionRecordError::Invalid(field));
    }
    Ok(())
}

fn validate_digest(value: &str, field: &'static str) -> Result<(), DecisionRecordError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(DecisionRecordError::Invalid(field));
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(DecisionRecordError::Invalid(field));
    }
    Ok(())
}

fn validate_digest_vec(values: &[String], field: &'static str) -> Result<(), DecisionRecordError> {
    if values.len() > DECISION_RECORD_MAX_REFS {
        return Err(DecisionRecordError::Invalid(field));
    }
    for value in values {
        validate_digest(value, field)?;
    }
    validate_unique(values, field)
}

fn validate_classifications(
    values: &[String],
    field: &'static str,
) -> Result<(), DecisionRecordError> {
    if values.len() > DECISION_RECORD_MAX_CLASSIFICATIONS {
        return Err(DecisionRecordError::Invalid(field));
    }
    for value in values {
        validate_identifier(value, field)?;
    }
    validate_unique(values, field)
}

fn validate_unique<T: Ord>(values: &[T], field: &'static str) -> Result<(), DecisionRecordError> {
    let unique: BTreeSet<&T> = values.iter().collect();
    if unique.len() != values.len() {
        return Err(DecisionRecordError::Duplicate(field));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(byte: char) -> String {
        format!("sha256:{}", byte.to_string().repeat(64))
    }

    fn goal_digest(goal: &str) -> String {
        format!("sha256:{:x}", Sha256::digest(goal.as_bytes()))
    }

    fn record() -> DecisionRecordV1 {
        let goal = "observe the refund safely";
        DecisionRecordV1 {
            schema: DECISION_RECORD_SCHEMA_V1.into(),
            decision_id: "dec_1".into(),
            trace_id: "trace_1".into(),
            session_id: "session_1".into(),
            parent_decision_digests: Vec::new(),
            workspace: DecisionWorkspaceV1 {
                organization_id: "org_acme".into(),
                project_id: "payments".into(),
                environment_id: "prod".into(),
            },
            initiator: DecisionInitiatorV1 {
                subject_digest: digest('a'),
                agent_id: "agent.codex.1".into(),
                agent_product: AgentProduct::Codex,
                surface: InteractionSurface::Mcp,
                harness_version: Some("app-server-v1".into()),
            },
            objective: DecisionObjectiveV1 {
                goal_digest: goal_digest(goal),
                goal: Some(goal.into()),
                task_family: Some("refund-reconciliation".into()),
            },
            model_call: Some(ModelCallEvidenceV1 {
                provider: "openai".into(),
                model: "gpt-test".into(),
                prompt_digest: digest('b'),
                context_digest: digest('c'),
                response_digest: digest('d'),
                model_revision: None,
                request_digest: Some(digest('e')),
                input_tokens: Some(100),
                output_tokens: Some(20),
                reasoning_tokens: None,
                cache_read_tokens: None,
                cache_write_tokens: None,
                latency_ms: Some(200),
            }),
            proposed_action: ProposedActionV1 {
                kind: "tools/call".into(),
                effect_class: EffectClass::Read,
                target: "payments.refund.observe".into(),
                arguments_digest: digest('1'),
                resource_digests: vec![digest('2')],
                data_classifications: vec!["financial".into()],
            },
            evidence_digests: vec![digest('3')],
            memory_digests: Vec::new(),
            retrieval_digests: Vec::new(),
            authority: AuthorityObservationV1 {
                outcome: AuthorityOutcome::Allow,
                source: AuthoritySource::SourceReceipt,
                evidence_digests: vec![digest('4')],
            },
            execution: Some(ExecutionObservationV1 {
                status: ExecutionStatus::Succeeded,
                attempt_digest: digest('5'),
                latency_ms: 50,
                success_source: SuccessSource::Deterministic,
                provider_request_digest: Some(digest('6')),
                provider_receipt_digest: None,
                result_digest: Some(digest('7')),
                side_effect_digest: None,
            }),
            governance: DecisionGovernanceV1 {
                allowed_uses: vec![
                    AllowedUse::Audit,
                    AllowedUse::Analytics,
                    AllowedUse::Eval,
                    AllowedUse::InternalTraining,
                ],
                retention_class: "enterprise-30d".into(),
                raw_payload_policy: RawPayloadPolicy::EncryptedRetained,
                data_classifications: vec!["financial".into()],
                rights_basis_digest: Some(digest('8')),
            },
            provenance: DecisionProvenanceV1 {
                producer: DECISION_RECORD_PRODUCER.into(),
                producer_version: "0.1.0".into(),
                normalization_version: "decision-trace-v1".into(),
                observed_at: "2026-08-15T14:00:00Z".into(),
            },
        }
    }

    #[test]
    fn valid_record_is_evidence_not_authority() {
        let record = record();
        assert_eq!(record.validate(), Ok(()));
        assert_eq!(record.authority.source, AuthoritySource::SourceReceipt);
    }

    #[test]
    fn disclosed_goal_is_content_bound() {
        let mut record = record();
        record.objective.goal = Some("different goal".into());
        assert_eq!(
            record.validate(),
            Err(DecisionRecordError::GoalDigestMismatch)
        );
    }

    #[test]
    fn evaluated_authority_requires_evidence() {
        let mut record = record();
        record.authority.evidence_digests.clear();
        assert_eq!(
            record.validate(),
            Err(DecisionRecordError::AuthorityEvidenceRequired)
        );
    }

    #[test]
    fn unknown_outcome_cannot_be_relabelled_as_success() {
        let mut record = record();
        let Some(execution) = record.execution.as_mut() else {
            panic!("fixture execution missing");
        };
        execution.status = ExecutionStatus::OutcomeUnknown;
        execution.success_source = SuccessSource::Deterministic;
        assert_eq!(
            record.validate(),
            Err(DecisionRecordError::UnknownOutcomeMustRemainUnknown)
        );
    }

    #[test]
    fn external_receipt_reward_requires_receipt_reference() {
        let mut record = record();
        {
            let Some(execution) = record.execution.as_mut() else {
                panic!("fixture execution missing");
            };
            execution.success_source = SuccessSource::ExternalReceipt;
        }
        assert_eq!(
            record.validate(),
            Err(DecisionRecordError::ExternalReceiptRequired)
        );
        {
            let Some(execution) = record.execution.as_mut() else {
                panic!("fixture execution missing");
            };
            execution.provider_receipt_digest = Some(digest('9'));
        }
        assert_eq!(record.validate(), Ok(()));
    }

    #[test]
    fn training_permission_requires_rights_basis() {
        let mut record = record();
        record.governance.rights_basis_digest = None;
        assert_eq!(
            record.validate(),
            Err(DecisionRecordError::RightsBasisRequired)
        );
    }

    #[test]
    fn rights_compose_by_intersection_not_ordinal_rank() {
        let a = [
            AllowedUse::Audit,
            AllowedUse::Eval,
            AllowedUse::InternalTraining,
        ];
        let b = [
            AllowedUse::Audit,
            AllowedUse::Eval,
            AllowedUse::CustomerTraining,
        ];
        assert_eq!(
            intersect_allowed_uses([a.as_slice(), b.as_slice()]),
            vec![AllowedUse::Audit, AllowedUse::Eval]
        );
    }

    #[test]
    fn audit_cannot_be_removed_from_enterprise_evidence() {
        let mut record = record();
        record
            .governance
            .allowed_uses
            .retain(|value| *value != AllowedUse::Audit);
        assert_eq!(
            record.validate(),
            Err(DecisionRecordError::AuditUseRequired)
        );
    }
}
