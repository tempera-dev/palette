use anyhow::{anyhow, Context};
use async_trait::async_trait;
use beater_core::{
    sha256_hex, sha256_json_hash, AgentReleaseId, DatasetCaseId, DatasetId, DatasetVersionId,
    EnvironmentId, EvalResultId, EvaluatorVersionId, ProjectId, PromptVersionId, ProviderSecretId,
    Sha256Hash, SpanId, TenantId, Timestamp, TraceId,
};
use beater_eval::{evaluate_deterministic, EvaluationCase, EvaluatorSpec, ScoreResult};
use beater_judge::{JudgeBroker, JudgeBrokerOutcome, JudgeBrokerRequest};
use beater_schema::{CanonicalSpan, EvalReproducibility, EvalResult, TraceView};
use beater_store::{IntoStoreResult, StoreError, StoreResult};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[async_trait]
pub trait DatasetStore: Send + Sync {
    async fn create_dataset(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        name: String,
    ) -> StoreResult<Dataset>;

    async fn put_case(&self, case: DatasetCase) -> StoreResult<DatasetCase>;

    async fn list_cases(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
    ) -> StoreResult<Vec<DatasetCase>>;

    async fn create_version(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
        case_ids: Option<Vec<DatasetCaseId>>,
    ) -> StoreResult<DatasetVersionSnapshot>;

    async fn get_version(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
        version_id: DatasetVersionId,
    ) -> StoreResult<DatasetVersionSnapshot>;

    async fn write_eval_report(&self, report: DatasetEvalReport) -> StoreResult<DatasetEvalReport>;

    async fn get_eval_report(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        report_id: String,
    ) -> StoreResult<DatasetEvalReport>;

    async fn latest_eval_report(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
        version_id: DatasetVersionId,
        evaluator_version_id: Option<EvaluatorVersionId>,
    ) -> StoreResult<Option<DatasetEvalReport>>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Dataset {
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub dataset_id: DatasetId,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DatasetCase {
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub dataset_id: DatasetId,
    pub case_id: DatasetCaseId,
    pub source_trace_id: TraceId,
    pub source_span_id: SpanId,
    pub source_environment_id: EnvironmentId,
    #[schema(value_type = serde_json::Value)]
    pub input: Value,
    #[schema(value_type = serde_json::Value)]
    pub output: Value,
    #[schema(value_type = Option<serde_json::Value>)]
    pub reference: Option<Value>,
    #[schema(value_type = serde_json::Value)]
    pub trace: Value,
    pub normalizer_version: String,
    pub trace_schema_version: u32,
    pub input_artifact_hashes: Vec<Sha256Hash>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DatasetVersionSnapshot {
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub dataset_id: DatasetId,
    pub version_id: DatasetVersionId,
    pub cases: Vec<DatasetCase>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: Timestamp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetSplit {
    Train,
    Dev,
    Test,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SplitProportions {
    pub train_bps: u16,
    pub dev_bps: u16,
    pub test_bps: u16,
}

impl SplitProportions {
    pub const DEFAULT: Self = Self {
        train_bps: 7_000,
        dev_bps: 1_500,
        test_bps: 1_500,
    };

    pub const BASIS_POINTS: u16 = 10_000;

    pub const fn new(train_bps: u16, dev_bps: u16, test_bps: u16) -> Self {
        Self {
            train_bps,
            dev_bps,
            test_bps,
        }
    }

    pub fn validate(self) -> Result<Self, SplitAssignmentError> {
        if self.train_bps == 0 || self.dev_bps == 0 || self.test_bps == 0 {
            return Err(SplitAssignmentError::ZeroBucket);
        }
        if u32::from(self.train_bps) + u32::from(self.dev_bps) + u32::from(self.test_bps)
            != u32::from(Self::BASIS_POINTS)
        {
            return Err(SplitAssignmentError::InvalidTotal);
        }
        Ok(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SplitAssignmentError {
    ZeroBucket,
    InvalidTotal,
}

impl fmt::Display for SplitAssignmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroBucket => write!(formatter, "dataset split proportions must be non-zero"),
            Self::InvalidTotal => write!(
                formatter,
                "dataset split proportions must sum to 10000 basis points"
            ),
        }
    }
}

impl std::error::Error for SplitAssignmentError {}

pub fn assign_dataset_split(dataset_version_seed: &str, case_id: &DatasetCaseId) -> DatasetSplit {
    match assign_dataset_split_with_proportions(
        dataset_version_seed,
        case_id,
        SplitProportions::DEFAULT,
    ) {
        Ok(split) => split,
        Err(_) => unreachable!("default split proportions are valid"),
    }
}

pub fn assign_dataset_split_with_proportions(
    dataset_version_seed: &str,
    case_id: &DatasetCaseId,
    proportions: SplitProportions,
) -> Result<DatasetSplit, SplitAssignmentError> {
    let proportions = proportions.validate()?;
    let bucket = split_bucket(dataset_version_seed, case_id);
    if bucket < u64::from(proportions.train_bps) {
        Ok(DatasetSplit::Train)
    } else if bucket < u64::from(proportions.train_bps + proportions.dev_bps) {
        Ok(DatasetSplit::Dev)
    } else {
        Ok(DatasetSplit::Test)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NearDuplicatePolicy {
    pub ngram_width: usize,
    pub jaccard_threshold: f64,
}

impl Default for NearDuplicatePolicy {
    fn default() -> Self {
        Self {
            ngram_width: 5,
            jaccard_threshold: 0.8,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContaminationCase {
    pub case_id: DatasetCaseId,
    pub split: DatasetSplit,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContaminationOverlap {
    pub train_case_id: DatasetCaseId,
    pub test_case_id: DatasetCaseId,
    pub similarity: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContaminationError {
    InvalidPolicy(String),
    TrainTestOverlap(ContaminationOverlap),
}

impl fmt::Display for ContaminationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy(reason) => write!(formatter, "invalid contamination policy: {reason}"),
            Self::TrainTestOverlap(overlap) => write!(
                formatter,
                "near-duplicate contamination between train case {} and test case {} (similarity {:.3})",
                overlap.train_case_id.as_str(),
                overlap.test_case_id.as_str(),
                overlap.similarity
            ),
        }
    }
}

impl std::error::Error for ContaminationError {}

pub fn contamination_text_for_case(case: &DatasetCase) -> String {
    let mut parts = Vec::new();
    parts.push(case.input.to_string());
    if let Some(reference) = &case.reference {
        parts.push(reference.to_string());
    }
    parts.push(case.output.to_string());
    parts.join("\n")
}

pub fn reject_train_test_contamination(
    cases: &[ContaminationCase],
    policy: &NearDuplicatePolicy,
) -> Result<(), ContaminationError> {
    validate_near_duplicate_policy(policy)?;
    let train = cases
        .iter()
        .filter(|case| case.split == DatasetSplit::Train)
        .collect::<Vec<_>>();
    let test = cases
        .iter()
        .filter(|case| case.split == DatasetSplit::Test)
        .collect::<Vec<_>>();

    for train_case in train {
        for test_case in &test {
            let similarity =
                near_duplicate_similarity(&train_case.text, &test_case.text, policy.ngram_width);
            if similarity >= policy.jaccard_threshold {
                return Err(ContaminationError::TrainTestOverlap(ContaminationOverlap {
                    train_case_id: train_case.case_id.clone(),
                    test_case_id: test_case.case_id.clone(),
                    similarity,
                }));
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetEvalSpec {
    pub evaluator: EvaluatorSpec,
    pub evaluator_version_id: EvaluatorVersionId,
    pub agent_release_id: AgentReleaseId,
    pub prompt_version_id: Option<PromptVersionId>,
    pub code_hash: Option<Sha256Hash>,
    pub wasm_hash: Option<Sha256Hash>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetJudgeEvalSpec {
    pub eval: DatasetEvalSpec,
    pub provider_secret_id: ProviderSecretId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct DatasetEvalReport {
    pub report_id: String,
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub dataset_id: DatasetId,
    pub dataset_version_id: DatasetVersionId,
    pub evaluator_version_id: EvaluatorVersionId,
    pub result_count: usize,
    pub aggregate_score: f64,
    pub results: Vec<EvalResult>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: Timestamp,
}

#[derive(Clone)]
pub struct SqliteDatasetStore {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteDatasetStore {
    pub fn in_memory() -> anyhow::Result<Self> {
        let connection = Connection::open_in_memory().context("open in-memory dataset sqlite")?;
        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
        };
        store.init()?;
        Ok(store)
    }

    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create dataset sqlite dir {}", parent.display()))?;
        }
        let connection = Connection::open(path)
            .with_context(|| format!("open sqlite dataset store {}", path.display()))?;
        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
        };
        store.init()?;
        Ok(store)
    }

    fn init(&self) -> anyhow::Result<()> {
        let connection = self.lock()?;
        connection
            .execute_batch(
                r#"
                PRAGMA journal_mode = WAL;
                PRAGMA foreign_keys = ON;

                CREATE TABLE IF NOT EXISTS datasets (
                    tenant_id TEXT NOT NULL,
                    project_id TEXT NOT NULL,
                    dataset_id TEXT NOT NULL,
                    name TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    dataset_json TEXT NOT NULL,
                    PRIMARY KEY (tenant_id, project_id, dataset_id)
                );

                CREATE TABLE IF NOT EXISTS dataset_cases (
                    tenant_id TEXT NOT NULL,
                    project_id TEXT NOT NULL,
                    dataset_id TEXT NOT NULL,
                    case_id TEXT NOT NULL,
                    source_trace_id TEXT NOT NULL,
                    source_span_id TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    case_json TEXT NOT NULL,
                    PRIMARY KEY (tenant_id, project_id, dataset_id, case_id)
                );

                CREATE TABLE IF NOT EXISTS dataset_versions (
                    tenant_id TEXT NOT NULL,
                    project_id TEXT NOT NULL,
                    dataset_id TEXT NOT NULL,
                    version_id TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    PRIMARY KEY (tenant_id, project_id, dataset_id, version_id)
                );

                CREATE TABLE IF NOT EXISTS dataset_version_cases (
                    tenant_id TEXT NOT NULL,
                    project_id TEXT NOT NULL,
                    dataset_id TEXT NOT NULL,
                    version_id TEXT NOT NULL,
                    case_id TEXT NOT NULL,
                    position INTEGER NOT NULL,
                    PRIMARY KEY (tenant_id, project_id, dataset_id, version_id, case_id)
                );

                CREATE TABLE IF NOT EXISTS dataset_eval_reports (
                    report_id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    project_id TEXT NOT NULL,
                    dataset_id TEXT NOT NULL,
                    version_id TEXT NOT NULL,
                    evaluator_version_id TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    report_json TEXT NOT NULL
                );
                "#,
            )
            .context("initialize sqlite dataset store")?;
        Ok(())
    }

    fn lock(&self) -> anyhow::Result<std::sync::MutexGuard<'_, Connection>> {
        self.connection
            .lock()
            .map_err(|err| anyhow!("sqlite dataset connection mutex poisoned: {err}"))
    }
}

#[async_trait]
impl DatasetStore for SqliteDatasetStore {
    async fn create_dataset(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        name: String,
    ) -> StoreResult<Dataset> {
        let dataset = Dataset {
            tenant_id,
            project_id,
            dataset_id: DatasetId::new(Uuid::new_v4().to_string()).map_err(StoreError::backend)?,
            name,
            created_at: Utc::now(),
        };
        let dataset_json = serde_json::to_string(&dataset)
            .context("serialize dataset")
            .into_store()?;
        let connection = self.lock().into_store()?;
        connection
            .execute(
                r#"
                INSERT INTO datasets
                  (tenant_id, project_id, dataset_id, name, created_at, dataset_json)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    dataset.tenant_id.as_str(),
                    dataset.project_id.as_str(),
                    dataset.dataset_id.as_str(),
                    dataset.name,
                    dataset.created_at.to_rfc3339(),
                    dataset_json
                ],
            )
            .context("insert dataset")
            .into_store()?;
        Ok(dataset)
    }

    async fn put_case(&self, case: DatasetCase) -> StoreResult<DatasetCase> {
        let case_json = serde_json::to_string(&case)
            .context("serialize dataset case")
            .into_store()?;
        let connection = self.lock().into_store()?;
        connection
            .execute(
                r#"
                INSERT INTO dataset_cases
                  (tenant_id, project_id, dataset_id, case_id, source_trace_id, source_span_id, created_at, case_json)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
                params![
                    case.tenant_id.as_str(),
                    case.project_id.as_str(),
                    case.dataset_id.as_str(),
                    case.case_id.as_str(),
                    case.source_trace_id.as_str(),
                    case.source_span_id.as_str(),
                    case.created_at.to_rfc3339(),
                    case_json
                ],
            )
            .context("insert dataset case")
            .into_store()?;
        Ok(case)
    }

    async fn list_cases(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
    ) -> StoreResult<Vec<DatasetCase>> {
        let connection = self.lock().into_store()?;
        let mut statement = connection
            .prepare(
                r#"
                SELECT case_json
                FROM dataset_cases
                WHERE tenant_id = ?1 AND project_id = ?2 AND dataset_id = ?3
                ORDER BY created_at ASC, case_id ASC
                "#,
            )
            .context("prepare list dataset cases")
            .into_store()?;
        let rows = statement
            .query_map(
                params![tenant_id.as_str(), project_id.as_str(), dataset_id.as_str()],
                |row| row.get::<_, String>(0),
            )
            .context("query dataset cases")
            .into_store()?;
        decode_json_rows(rows, "dataset case").into_store()
    }

    async fn create_version(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
        case_ids: Option<Vec<DatasetCaseId>>,
    ) -> StoreResult<DatasetVersionSnapshot> {
        let all_cases = self
            .list_cases(tenant_id.clone(), project_id.clone(), dataset_id.clone())
            .await?;
        let cases = select_cases(all_cases, case_ids).into_store()?;
        if cases.is_empty() {
            return Err(StoreError::Conflict(
                "cannot create a dataset version with no cases".to_string(),
            ));
        }
        let snapshot = DatasetVersionSnapshot {
            tenant_id,
            project_id,
            dataset_id,
            version_id: DatasetVersionId::new(Uuid::new_v4().to_string())
                .map_err(StoreError::backend)?,
            cases,
            created_at: Utc::now(),
        };
        let mut connection = self.lock().into_store()?;
        let tx = connection
            .transaction()
            .context("begin dataset version transaction")
            .into_store()?;
        tx.execute(
            r#"
            INSERT INTO dataset_versions
              (tenant_id, project_id, dataset_id, version_id, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                snapshot.tenant_id.as_str(),
                snapshot.project_id.as_str(),
                snapshot.dataset_id.as_str(),
                snapshot.version_id.as_str(),
                snapshot.created_at.to_rfc3339()
            ],
        )
        .context("insert dataset version")
        .into_store()?;
        for (position, case) in snapshot.cases.iter().enumerate() {
            tx.execute(
                r#"
                INSERT INTO dataset_version_cases
                  (tenant_id, project_id, dataset_id, version_id, case_id, position)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    snapshot.tenant_id.as_str(),
                    snapshot.project_id.as_str(),
                    snapshot.dataset_id.as_str(),
                    snapshot.version_id.as_str(),
                    case.case_id.as_str(),
                    position as i64
                ],
            )
            .context("insert dataset version case")
            .into_store()?;
        }
        tx.commit()
            .context("commit dataset version transaction")
            .into_store()?;
        Ok(snapshot)
    }

    async fn get_version(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
        version_id: DatasetVersionId,
    ) -> StoreResult<DatasetVersionSnapshot> {
        let connection = self.lock().into_store()?;
        let created_at = connection
            .query_row(
                r#"
                SELECT created_at
                FROM dataset_versions
                WHERE tenant_id = ?1 AND project_id = ?2 AND dataset_id = ?3 AND version_id = ?4
                "#,
                params![
                    tenant_id.as_str(),
                    project_id.as_str(),
                    dataset_id.as_str(),
                    version_id.as_str()
                ],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .context("query dataset version")
            .into_store()?
            .ok_or_else(|| {
                StoreError::NotFound(format!("dataset version {} not found", version_id.as_str()))
            })?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at)
            .context("parse dataset version created_at")
            .into_store()?
            .with_timezone(&Utc);
        let mut statement = connection
            .prepare(
                r#"
                SELECT c.case_json
                FROM dataset_version_cases vc
                JOIN dataset_cases c
                  ON c.tenant_id = vc.tenant_id
                 AND c.project_id = vc.project_id
                 AND c.dataset_id = vc.dataset_id
                 AND c.case_id = vc.case_id
                WHERE vc.tenant_id = ?1
                  AND vc.project_id = ?2
                  AND vc.dataset_id = ?3
                  AND vc.version_id = ?4
                ORDER BY vc.position ASC
                "#,
            )
            .context("prepare get dataset version cases")
            .into_store()?;
        let rows = statement
            .query_map(
                params![
                    tenant_id.as_str(),
                    project_id.as_str(),
                    dataset_id.as_str(),
                    version_id.as_str()
                ],
                |row| row.get::<_, String>(0),
            )
            .context("query dataset version cases")
            .into_store()?;
        let cases = decode_json_rows(rows, "dataset version case").into_store()?;
        Ok(DatasetVersionSnapshot {
            tenant_id,
            project_id,
            dataset_id,
            version_id,
            cases,
            created_at,
        })
    }

    async fn write_eval_report(&self, report: DatasetEvalReport) -> StoreResult<DatasetEvalReport> {
        let report_json = serde_json::to_string(&report)
            .context("serialize dataset eval report")
            .into_store()?;
        let connection = self.lock().into_store()?;
        connection
            .execute(
                r#"
                INSERT INTO dataset_eval_reports
                  (report_id, tenant_id, project_id, dataset_id, version_id, evaluator_version_id, created_at, report_json)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                "#,
                params![
                    report.report_id,
                    report.tenant_id.as_str(),
                    report.project_id.as_str(),
                    report.dataset_id.as_str(),
                    report.dataset_version_id.as_str(),
                    report.evaluator_version_id.as_str(),
                    report.created_at.to_rfc3339(),
                    report_json
                ],
            )
            .context("insert dataset eval report")
            .into_store()?;
        Ok(report)
    }

    async fn get_eval_report(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        report_id: String,
    ) -> StoreResult<DatasetEvalReport> {
        let connection = self.lock().into_store()?;
        let report_json = connection
            .query_row(
                r#"
                SELECT report_json
                FROM dataset_eval_reports
                WHERE tenant_id = ?1 AND project_id = ?2 AND report_id = ?3
                "#,
                params![tenant_id.as_str(), project_id.as_str(), report_id],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .context("query dataset eval report")
            .into_store()?
            .ok_or_else(|| StoreError::NotFound("dataset eval report not found".to_string()))?;
        serde_json::from_str(&report_json)
            .context("decode dataset eval report")
            .into_store()
    }

    async fn latest_eval_report(
        &self,
        tenant_id: TenantId,
        project_id: ProjectId,
        dataset_id: DatasetId,
        version_id: DatasetVersionId,
        evaluator_version_id: Option<EvaluatorVersionId>,
    ) -> StoreResult<Option<DatasetEvalReport>> {
        let evaluator_version_id = evaluator_version_id.as_ref().map(|id| id.as_str());
        let connection = self.lock().into_store()?;
        let report_json = connection
            .query_row(
                r#"
                SELECT report_json
                FROM dataset_eval_reports
                WHERE tenant_id = ?1
                  AND project_id = ?2
                  AND dataset_id = ?3
                  AND version_id = ?4
                  AND (?5 IS NULL OR evaluator_version_id = ?5)
                ORDER BY created_at DESC, report_id DESC
                LIMIT 1
                "#,
                params![
                    tenant_id.as_str(),
                    project_id.as_str(),
                    dataset_id.as_str(),
                    version_id.as_str(),
                    evaluator_version_id
                ],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .context("query latest dataset eval report")
            .into_store()?;
        report_json
            .map(|report_json| {
                serde_json::from_str(&report_json).context("decode dataset eval report")
            })
            .transpose()
            .into_store()
    }
}

pub fn promote_trace_span_to_case(
    tenant_id: TenantId,
    project_id: ProjectId,
    dataset_id: DatasetId,
    trace: &TraceView,
    span_id: Option<SpanId>,
    reference: Option<Value>,
) -> anyhow::Result<DatasetCase> {
    if trace.tenant_id.as_str() != tenant_id.as_str() {
        return Err(anyhow!("trace promotion crosses tenant boundary"));
    }
    let span = select_span(&trace.spans, span_id.as_ref())?;
    if span.project_id.as_str() != project_id.as_str() {
        return Err(anyhow!("trace promotion crosses project boundary"));
    }
    let trace_json = serde_json::to_value(span).context("serialize promoted span trace")?;
    Ok(DatasetCase {
        tenant_id,
        project_id,
        dataset_id,
        case_id: DatasetCaseId::new(Uuid::new_v4().to_string())?,
        source_trace_id: span.trace_id.clone(),
        source_span_id: span.span_id.clone(),
        source_environment_id: span.environment_id.clone(),
        input: span_value(
            span,
            "input.value",
            span.input_ref.as_ref().map(|artifact| &artifact.uri),
        ),
        output: span_value(
            span,
            "output.value",
            span.output_ref.as_ref().map(|artifact| &artifact.uri),
        ),
        reference,
        trace: trace_json,
        normalizer_version: span.normalizer_version.clone(),
        trace_schema_version: span.schema_version,
        input_artifact_hashes: artifact_hashes(span),
        created_at: Utc::now(),
    })
}

pub fn evaluate_dataset_version(
    snapshot: &DatasetVersionSnapshot,
    spec: DatasetEvalSpec,
) -> anyhow::Result<DatasetEvalReport> {
    if snapshot.cases.is_empty() {
        return Err(anyhow!("cannot evaluate an empty dataset version"));
    }
    let code_hash = spec
        .code_hash
        .clone()
        .map(Ok)
        .unwrap_or_else(|| evaluator_spec_hash(&spec.evaluator))?;
    let mut results = Vec::with_capacity(snapshot.cases.len());
    for case in &snapshot.cases {
        let eval_case = EvaluationCase {
            input: case.input.clone(),
            output: case.output.clone(),
            reference: case.reference.clone(),
            trace: Some(case.trace.clone()),
        };
        let score = evaluate_deterministic(&spec.evaluator, &eval_case)
            .with_context(|| format!("evaluate dataset case {}", case.case_id.as_str()))?;
        results.push(eval_result_from_score(
            snapshot, case, &spec, &code_hash, score,
        )?);
    }
    let aggregate_score =
        results.iter().map(|result| result.score).sum::<f64>() / results.len() as f64;
    Ok(DatasetEvalReport {
        report_id: Uuid::new_v4().to_string(),
        tenant_id: snapshot.tenant_id.clone(),
        project_id: snapshot.project_id.clone(),
        dataset_id: snapshot.dataset_id.clone(),
        dataset_version_id: snapshot.version_id.clone(),
        evaluator_version_id: spec.evaluator_version_id,
        result_count: results.len(),
        aggregate_score,
        results,
        created_at: Utc::now(),
    })
}

pub async fn evaluate_dataset_version_with_judge<B>(
    snapshot: &DatasetVersionSnapshot,
    spec: DatasetJudgeEvalSpec,
    judge_broker: &B,
) -> anyhow::Result<DatasetEvalReport>
where
    B: JudgeBroker + ?Sized,
{
    if snapshot.cases.is_empty() {
        return Err(anyhow!("cannot evaluate an empty dataset version"));
    }
    let code_hash = spec
        .eval
        .code_hash
        .clone()
        .map(Ok)
        .unwrap_or_else(|| evaluator_spec_hash(&spec.eval.evaluator))?;
    let mut results = Vec::with_capacity(snapshot.cases.len());
    for case in &snapshot.cases {
        let eval_case = EvaluationCase {
            input: case.input.clone(),
            output: case.output.clone(),
            reference: case.reference.clone(),
            trace: Some(case.trace.clone()),
        };
        let outcome = judge_broker
            .evaluate(JudgeBrokerRequest {
                tenant_id: snapshot.tenant_id.clone(),
                project_id: snapshot.project_id.clone(),
                evaluator: spec.eval.evaluator.clone(),
                case: eval_case,
                provider_secret_id: spec.provider_secret_id.clone(),
            })
            .await
            .with_context(|| format!("judge dataset case {}", case.case_id.as_str()))?;
        results.push(eval_result_from_judge_outcome(
            snapshot, case, &spec, &code_hash, outcome,
        )?);
    }
    let aggregate_score =
        results.iter().map(|result| result.score).sum::<f64>() / results.len() as f64;
    Ok(DatasetEvalReport {
        report_id: Uuid::new_v4().to_string(),
        tenant_id: snapshot.tenant_id.clone(),
        project_id: snapshot.project_id.clone(),
        dataset_id: snapshot.dataset_id.clone(),
        dataset_version_id: snapshot.version_id.clone(),
        evaluator_version_id: spec.eval.evaluator_version_id,
        result_count: results.len(),
        aggregate_score,
        results,
        created_at: Utc::now(),
    })
}

fn eval_result_from_score(
    snapshot: &DatasetVersionSnapshot,
    case: &DatasetCase,
    spec: &DatasetEvalSpec,
    code_hash: &Sha256Hash,
    score: ScoreResult,
) -> anyhow::Result<EvalResult> {
    Ok(EvalResult {
        eval_result_id: EvalResultId::new(Uuid::new_v4().to_string())?,
        tenant_id: snapshot.tenant_id.clone(),
        project_id: snapshot.project_id.clone(),
        trace_id: case.source_trace_id.clone(),
        span_id: Some(case.source_span_id.clone()),
        score: score.score,
        label: score.label,
        evidence: score.evidence,
        reproducibility: EvalReproducibility {
            dataset_version_id: snapshot.version_id.clone(),
            dataset_case_id: case.case_id.clone(),
            agent_release_id: spec.agent_release_id.clone(),
            prompt_version_id: spec.prompt_version_id.clone(),
            evaluator_version_id: spec.evaluator_version_id.clone(),
            code_hash: Some(code_hash.clone()),
            wasm_hash: spec.wasm_hash.clone(),
            wasi_abi_version: Some("beater-deterministic-v1".to_string()),
            judge_model_id: None,
            judge_provider: None,
            judge_parameters: serde_json::json!({}),
            judge_seed: None,
            judge_rubric_version: None,
            normalizer_version: case.normalizer_version.clone(),
            trace_schema_version: case.trace_schema_version,
            input_artifact_hashes: case.input_artifact_hashes.clone(),
        },
        cost: None,
        tokens: None,
        created_at: Utc::now(),
        non_reproducible_reason: None,
    })
}

fn eval_result_from_judge_outcome(
    snapshot: &DatasetVersionSnapshot,
    case: &DatasetCase,
    spec: &DatasetJudgeEvalSpec,
    code_hash: &Sha256Hash,
    outcome: JudgeBrokerOutcome,
) -> anyhow::Result<EvalResult> {
    let audit = outcome.audit;
    Ok(EvalResult {
        eval_result_id: EvalResultId::new(Uuid::new_v4().to_string())?,
        tenant_id: snapshot.tenant_id.clone(),
        project_id: snapshot.project_id.clone(),
        trace_id: case.source_trace_id.clone(),
        span_id: Some(case.source_span_id.clone()),
        score: outcome.result.score,
        label: outcome.result.label,
        evidence: outcome.result.evidence,
        reproducibility: EvalReproducibility {
            dataset_version_id: snapshot.version_id.clone(),
            dataset_case_id: case.case_id.clone(),
            agent_release_id: spec.eval.agent_release_id.clone(),
            prompt_version_id: spec.eval.prompt_version_id.clone(),
            evaluator_version_id: spec.eval.evaluator_version_id.clone(),
            code_hash: Some(code_hash.clone()),
            wasm_hash: None,
            wasi_abi_version: None,
            judge_model_id: Some(audit.model.clone()),
            judge_provider: Some(audit.provider.clone()),
            judge_parameters: serde_json::json!({
                "judge_call_id": audit.judge_call_id,
                "provider_secret_id": audit.provider_secret_id,
                "request_hash": audit.request_hash,
                "response_hash": audit.response_hash,
                "cached": audit.cached,
                "provider_cost_micros": audit.provider_cost.amount_micros,
                "charged_cost_micros": audit.charged_cost.amount_micros,
                "currency": audit.provider_cost.currency.as_str()
            }),
            judge_seed: None,
            judge_rubric_version: Some(spec.eval.evaluator_version_id.as_str().to_string()),
            normalizer_version: case.normalizer_version.clone(),
            trace_schema_version: case.trace_schema_version,
            input_artifact_hashes: case.input_artifact_hashes.clone(),
        },
        cost: Some(audit.charged_cost),
        tokens: None,
        created_at: Utc::now(),
        non_reproducible_reason: None,
    })
}

fn select_span<'a>(
    spans: &'a [CanonicalSpan],
    span_id: Option<&SpanId>,
) -> anyhow::Result<&'a CanonicalSpan> {
    if let Some(span_id) = span_id {
        return spans
            .iter()
            .find(|span| span.span_id.as_str() == span_id.as_str())
            .ok_or_else(|| anyhow!("span {} not found in trace", span_id.as_str()));
    }
    spans
        .iter()
        .find(|span| matches!(span.status, beater_schema::SpanStatus::Error))
        .or_else(|| spans.first())
        .ok_or_else(|| anyhow!("cannot promote an empty trace"))
}

fn span_value(span: &CanonicalSpan, attr_key: &str, artifact_uri: Option<&String>) -> Value {
    span.attributes
        .get(attr_key)
        .cloned()
        .or_else(|| artifact_uri.map(|uri| serde_json::json!({ "artifact_uri": uri })))
        .unwrap_or(Value::Null)
}

fn artifact_hashes(span: &CanonicalSpan) -> Vec<Sha256Hash> {
    let mut hashes = Vec::new();
    if let Some(artifact) = &span.input_ref {
        hashes.push(artifact.sha256.clone());
    }
    if let Some(artifact) = &span.output_ref {
        hashes.push(artifact.sha256.clone());
    }
    hashes.push(span.raw_ref.sha256.clone());
    hashes
}

fn evaluator_spec_hash(spec: &EvaluatorSpec) -> anyhow::Result<Sha256Hash> {
    sha256_json_hash(spec).context("serialize evaluator spec for hash")
}

fn split_bucket(dataset_version_seed: &str, case_id: &DatasetCaseId) -> u64 {
    let mut hash_input =
        Vec::with_capacity(dataset_version_seed.len() + case_id.as_str().len() + 1);
    hash_input.extend_from_slice(dataset_version_seed.as_bytes());
    hash_input.push(0);
    hash_input.extend_from_slice(case_id.as_str().as_bytes());
    let digest = sha256_hex(&hash_input);
    let value = digest
        .bytes()
        .take(16)
        .fold(0u64, |value, byte| (value << 4) | hex_nibble(byte));
    value % u64::from(SplitProportions::BASIS_POINTS)
}

fn hex_nibble(byte: u8) -> u64 {
    match byte {
        b'0'..=b'9' => u64::from(byte - b'0'),
        b'a'..=b'f' => u64::from(byte - b'a' + 10),
        b'A'..=b'F' => u64::from(byte - b'A' + 10),
        _ => unreachable!("sha256 digest contains only hex digits"),
    }
}

fn validate_near_duplicate_policy(policy: &NearDuplicatePolicy) -> Result<(), ContaminationError> {
    if policy.ngram_width == 0 {
        return Err(ContaminationError::InvalidPolicy(
            "ngram_width must be greater than zero".to_string(),
        ));
    }
    if !(0.0..=1.0).contains(&policy.jaccard_threshold) || policy.jaccard_threshold.is_nan() {
        return Err(ContaminationError::InvalidPolicy(
            "jaccard_threshold must be between 0.0 and 1.0".to_string(),
        ));
    }
    Ok(())
}

fn near_duplicate_similarity(left: &str, right: &str, ngram_width: usize) -> f64 {
    let left_normalized = normalize_contamination_text(left);
    let right_normalized = normalize_contamination_text(right);
    if left_normalized.is_empty() || right_normalized.is_empty() {
        return 0.0;
    }
    if left_normalized == right_normalized {
        return 1.0;
    }
    let left_ngrams = word_ngrams(&left_normalized, ngram_width);
    let right_ngrams = word_ngrams(&right_normalized, ngram_width);
    jaccard_similarity(&left_ngrams, &right_ngrams)
}

fn normalize_contamination_text(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut pending_space = false;
    for character in text.chars() {
        if character.is_alphanumeric() {
            if pending_space && !normalized.is_empty() {
                normalized.push(' ');
            }
            for lower in character.to_lowercase() {
                normalized.push(lower);
            }
            pending_space = false;
        } else {
            pending_space = true;
        }
    }
    normalized
}

fn word_ngrams(normalized: &str, ngram_width: usize) -> BTreeSet<String> {
    let words = normalized.split_whitespace().collect::<Vec<_>>();
    if words.is_empty() {
        return BTreeSet::new();
    }
    if words.len() <= ngram_width {
        return BTreeSet::from([words.join(" ")]);
    }
    words
        .windows(ngram_width)
        .map(|window| window.join(" "))
        .collect()
}

fn jaccard_similarity(left: &BTreeSet<String>, right: &BTreeSet<String>) -> f64 {
    if left.is_empty() || right.is_empty() {
        return 0.0;
    }
    let intersection = left.intersection(right).count();
    let union = left.union(right).count();
    intersection as f64 / union as f64
}

fn select_cases(
    all_cases: Vec<DatasetCase>,
    case_ids: Option<Vec<DatasetCaseId>>,
) -> anyhow::Result<Vec<DatasetCase>> {
    let Some(case_ids) = case_ids else {
        return Ok(all_cases);
    };
    // Index cases by id once (first occurrence wins, matching the prior
    // `.find` semantics) so each requested id is resolved in O(1) instead of a
    // linear scan over all_cases — turns selection from O(requested × total)
    // into O(requested + total).
    let mut by_id = std::collections::HashMap::<&str, &DatasetCase>::with_capacity(all_cases.len());
    for case in &all_cases {
        by_id.entry(case.case_id.as_str()).or_insert(case);
    }
    let mut selected = Vec::with_capacity(case_ids.len());
    for case_id in case_ids {
        let case = by_id
            .get(case_id.as_str())
            .map(|case| (*case).clone())
            .ok_or_else(|| anyhow!("dataset case {} not found", case_id.as_str()))?;
        selected.push(case);
    }
    Ok(selected)
}

fn decode_json_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<String>>,
    label: &str,
) -> anyhow::Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let mut decoded = Vec::new();
    for row in rows {
        let json = row.with_context(|| format!("read {label} row"))?;
        decoded.push(serde_json::from_str::<T>(&json).with_context(|| format!("decode {label}"))?);
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use beater_core::{ArtifactId, JudgeCallId, Money, TokenCounts};
    use beater_eval::EvaluatorKind;
    use beater_judge::{JudgeAuditRecord, JudgeBrokerError};
    use beater_schema::{
        AgentSpanKind, ArtifactRef, ModelRef, RedactionClass, SpanStatus, CANONICAL_SCHEMA_VERSION,
    };
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn split_assignment_is_seeded_and_stable() {
        let case_id = DatasetCaseId::new("case-42").unwrap_or_else(|err| panic!("{err}"));

        let first = assign_dataset_split("dataset-version-seed-a", &case_id);
        let second = assign_dataset_split("dataset-version-seed-a", &case_id);
        assert_eq!(first, second);

        let custom = assign_dataset_split_with_proportions(
            "dataset-version-seed-a",
            &case_id,
            SplitProportions::new(9_999, 0, 1),
        );
        assert_eq!(custom, Err(SplitAssignmentError::ZeroBucket));

        let mostly_train = assign_dataset_split_with_proportions(
            "dataset-version-seed-a",
            &case_id,
            SplitProportions::new(9_998, 1, 1),
        )
        .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(mostly_train, DatasetSplit::Train);
    }

    #[test]
    fn split_assignment_tracks_default_proportions_without_reshuffling() {
        let mut train = 0;
        let mut dev = 0;
        let mut test = 0;
        for index in 0..1_000 {
            let case_id =
                DatasetCaseId::new(format!("case-{index}")).unwrap_or_else(|err| panic!("{err}"));
            match assign_dataset_split("frozen-version-seed", &case_id) {
                DatasetSplit::Train => train += 1,
                DatasetSplit::Dev => dev += 1,
                DatasetSplit::Test => test += 1,
            }
        }

        assert!((650..=750).contains(&train), "train={train}");
        assert!((100..=200).contains(&dev), "dev={dev}");
        assert!((100..=200).contains(&test), "test={test}");
    }

    #[test]
    fn contamination_guard_rejects_train_test_near_duplicate() {
        let policy = NearDuplicatePolicy {
            ngram_width: 2,
            jaccard_threshold: 0.35,
        };
        let result = reject_train_test_contamination(
            &[
                contamination_case(
                    "train-1",
                    DatasetSplit::Train,
                    "Reset the customer password after verifying email ownership",
                ),
                contamination_case(
                    "test-1",
                    DatasetSplit::Test,
                    "reset customer password after verifying the email ownership",
                ),
            ],
            &policy,
        );

        let Err(ContaminationError::TrainTestOverlap(overlap)) = result else {
            panic!("expected train/test contamination rejection");
        };
        assert_eq!(overlap.train_case_id.as_str(), "train-1");
        assert_eq!(overlap.test_case_id.as_str(), "test-1");
        assert!(overlap.similarity >= 0.35, "{}", overlap.similarity);
    }

    #[test]
    fn contamination_guard_allows_dev_overlap_and_distinct_test() {
        let cases = [
            contamination_case(
                "train-1",
                DatasetSplit::Train,
                "Summarize the failed payment dispute for an account manager",
            ),
            contamination_case(
                "dev-1",
                DatasetSplit::Dev,
                "Summarize failed payment dispute for the account manager",
            ),
            contamination_case(
                "test-1",
                DatasetSplit::Test,
                "Classify whether a support answer contains a citation",
            ),
        ];

        reject_train_test_contamination(&cases, &NearDuplicatePolicy::default())
            .unwrap_or_else(|err| panic!("{err}"));
    }

    #[test]
    fn contamination_text_uses_case_io_without_trace_payload() {
        let case = fixture_dataset_case(
            "case-with-trace",
            json!({ "prompt": "question" }),
            json!({ "answer": "response" }),
            Some(json!("reference")),
            json!({ "large_trace": "not part of fingerprint text" }),
        );

        let text = contamination_text_for_case(&case);
        assert!(text.contains("question"));
        assert!(text.contains("reference"));
        assert!(text.contains("response"));
        assert!(!text.contains("large_trace"));
    }

    #[tokio::test]
    async fn promotes_trace_case_versions_and_runs_deterministic_eval() {
        let store = SqliteDatasetStore::in_memory().unwrap_or_else(|err| panic!("{err}"));
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));
        let dataset = store
            .create_dataset(tenant.clone(), project.clone(), "failures".to_string())
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        let trace = fixture_trace(&tenant, &project);
        let span_id = SpanId::new("span").unwrap_or_else(|err| panic!("{err}"));
        let case = promote_trace_span_to_case(
            tenant.clone(),
            project.clone(),
            dataset.dataset_id.clone(),
            &trace,
            Some(span_id),
            Some(json!("answer")),
        )
        .unwrap_or_else(|err| panic!("{err}"));
        let case = store
            .put_case(case)
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(case.input, json!("question"));
        assert_eq!(case.output, json!("answer"));

        let version = store
            .create_version(
                tenant.clone(),
                project.clone(),
                dataset.dataset_id.clone(),
                None,
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(version.cases.len(), 1);

        let report = evaluate_dataset_version(
            &version,
            DatasetEvalSpec {
                evaluator: EvaluatorSpec {
                    id: "exact".to_string(),
                    lane: beater_schema::EvaluatorLane::DeterministicWasi,
                    kind: EvaluatorKind::ExactMatch,
                },
                evaluator_version_id: EvaluatorVersionId::new("exact-v1")
                    .unwrap_or_else(|err| panic!("{err}")),
                agent_release_id: AgentReleaseId::new("release-a")
                    .unwrap_or_else(|err| panic!("{err}")),
                prompt_version_id: None,
                code_hash: None,
                wasm_hash: None,
            },
        )
        .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(report.aggregate_score, 1.0);
        assert_eq!(report.result_count, 1);
        let result = &report.results[0];
        assert_eq!(
            result.reproducibility.dataset_version_id,
            version.version_id
        );
        assert_eq!(result.reproducibility.dataset_case_id, case.case_id);
        assert!(result.reproducibility.code_hash.is_some());
        assert_eq!(result.reproducibility.normalizer_version, "beater-test-v1");

        let stored = store
            .write_eval_report(report)
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(stored.result_count, 1);

        let provider_secret_id =
            ProviderSecretId::new("judge-secret").unwrap_or_else(|err| panic!("{err}"));
        let judge_report = evaluate_dataset_version_with_judge(
            &version,
            DatasetJudgeEvalSpec {
                eval: DatasetEvalSpec {
                    evaluator: EvaluatorSpec {
                        id: "llm-judge".to_string(),
                        lane: beater_schema::EvaluatorLane::JudgeBroker,
                        kind: EvaluatorKind::LlmJudge {
                            rubric: "correctness".to_string(),
                            model: "judge-model".to_string(),
                        },
                    },
                    evaluator_version_id: EvaluatorVersionId::new("judge-v1")
                        .unwrap_or_else(|err| panic!("{err}")),
                    agent_release_id: AgentReleaseId::new("release-a")
                        .unwrap_or_else(|err| panic!("{err}")),
                    prompt_version_id: None,
                    code_hash: None,
                    wasm_hash: None,
                },
                provider_secret_id,
            },
            &FixedDatasetJudgeBroker,
        )
        .await
        .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(judge_report.result_count, 1);
        assert_eq!(judge_report.aggregate_score, 0.8);
        let judge_result = &judge_report.results[0];
        assert_eq!(judge_result.cost, Some(Money::usd_micros(17)));
        assert_eq!(
            judge_result.reproducibility.judge_model_id.as_deref(),
            Some("judge-model")
        );
        assert_eq!(
            judge_result.reproducibility.judge_provider.as_deref(),
            Some("openai")
        );
        assert!(judge_result.reproducibility.wasi_abi_version.is_none());
        assert!(judge_result.reproducibility.wasm_hash.is_none());
        assert_eq!(
            judge_result.reproducibility.judge_rubric_version.as_deref(),
            Some("judge-v1")
        );
        assert_eq!(
            judge_result.reproducibility.judge_parameters["cached"],
            json!(false)
        );
        let serialized = serde_json::to_string(judge_result).unwrap_or_else(|err| panic!("{err}"));
        assert!(!serialized.contains("sk-"));

        let stored = store
            .write_eval_report(judge_report)
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(stored.result_count, 1);
    }

    #[tokio::test]
    async fn dataset_store_is_tenant_scoped() {
        let store = SqliteDatasetStore::in_memory().unwrap_or_else(|err| panic!("{err}"));
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let other = TenantId::new("other").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));
        let dataset = store
            .create_dataset(tenant.clone(), project.clone(), "failures".to_string())
            .await
            .unwrap_or_else(|err| panic!("{err}"));

        let cases = store
            .list_cases(other, project, dataset.dataset_id)
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert!(cases.is_empty());
    }

    fn contamination_case(case_id: &str, split: DatasetSplit, text: &str) -> ContaminationCase {
        ContaminationCase {
            case_id: DatasetCaseId::new(case_id).unwrap_or_else(|err| panic!("{err}")),
            split,
            text: text.to_string(),
        }
    }

    fn fixture_dataset_case(
        case_id: &str,
        input: Value,
        output: Value,
        reference: Option<Value>,
        trace: Value,
    ) -> DatasetCase {
        DatasetCase {
            tenant_id: TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}")),
            project_id: ProjectId::new("project").unwrap_or_else(|err| panic!("{err}")),
            dataset_id: DatasetId::new("dataset").unwrap_or_else(|err| panic!("{err}")),
            case_id: DatasetCaseId::new(case_id).unwrap_or_else(|err| panic!("{err}")),
            source_trace_id: TraceId::new("trace").unwrap_or_else(|err| panic!("{err}")),
            source_span_id: SpanId::new("span").unwrap_or_else(|err| panic!("{err}")),
            source_environment_id: EnvironmentId::new("prod").unwrap_or_else(|err| panic!("{err}")),
            input,
            output,
            reference,
            trace,
            normalizer_version: "beater-test-v1".to_string(),
            trace_schema_version: CANONICAL_SCHEMA_VERSION,
            input_artifact_hashes: Vec::new(),
            created_at: Utc::now(),
        }
    }

    fn fixture_trace(tenant: &TenantId, project: &ProjectId) -> TraceView {
        TraceView {
            tenant_id: tenant.clone(),
            trace_id: TraceId::new("trace").unwrap_or_else(|err| panic!("{err}")),
            spans: vec![fixture_span(tenant, project)],
        }
    }

    fn fixture_span(tenant: &TenantId, project: &ProjectId) -> CanonicalSpan {
        let mut attributes = BTreeMap::new();
        attributes.insert("input.value".to_string(), json!("question"));
        attributes.insert("output.value".to_string(), json!("answer"));
        CanonicalSpan {
            schema_version: CANONICAL_SCHEMA_VERSION,
            normalizer_version: "beater-test-v1".to_string(),
            tenant_id: tenant.clone(),
            project_id: project.clone(),
            environment_id: EnvironmentId::new("prod").unwrap_or_else(|err| panic!("{err}")),
            trace_id: TraceId::new("trace").unwrap_or_else(|err| panic!("{err}")),
            span_id: SpanId::new("span").unwrap_or_else(|err| panic!("{err}")),
            parent_span_id: None,
            seq: 1,
            kind: AgentSpanKind::AgentRun,
            name: "run".to_string(),
            status: SpanStatus::Error,
            start_time: Utc::now(),
            end_time: Some(Utc::now()),
            model: Some(ModelRef {
                provider: "openai".to_string(),
                name: "model".to_string(),
            }),
            cost: Some(Money::usd_micros(12)),
            tokens: Some(TokenCounts {
                input: 1,
                output: 2,
                reasoning: 0,
                cache_read: 0,
            }),
            input_ref: None,
            output_ref: None,
            attributes,
            unmapped_attrs: json!({}),
            raw_ref: ArtifactRef {
                artifact_id: ArtifactId::new("raw").unwrap_or_else(|err| panic!("{err}")),
                uri: "artifact://tenant/project/raw".to_string(),
                sha256: Sha256Hash::new("ab".repeat(32)).unwrap_or_else(|err| panic!("{err}")),
                size_bytes: 2,
                mime_type: "application/json".to_string(),
                redaction_class: RedactionClass::Internal,
            },
        }
    }

    struct FixedDatasetJudgeBroker;

    #[async_trait]
    impl JudgeBroker for FixedDatasetJudgeBroker {
        async fn evaluate(
            &self,
            request: JudgeBrokerRequest,
        ) -> Result<JudgeBrokerOutcome, JudgeBrokerError> {
            Ok(JudgeBrokerOutcome {
                result: ScoreResult {
                    score: 0.8,
                    label: Some("pass".to_string()),
                    evidence: json!({ "rationale": "fixed dataset judge" }),
                },
                audit: JudgeAuditRecord {
                    judge_call_id: JudgeCallId::new("judge-call")
                        .unwrap_or_else(|err| panic!("{err}")),
                    tenant_id: request.tenant_id,
                    project_id: request.project_id,
                    evaluator_id: request.evaluator.id,
                    provider: "openai".to_string(),
                    provider_secret_id: request.provider_secret_id,
                    model: "judge-model".to_string(),
                    request_hash: Sha256Hash::new("11".repeat(32))
                        .unwrap_or_else(|err| panic!("{err}")),
                    response_hash: Sha256Hash::new("22".repeat(32))
                        .unwrap_or_else(|err| panic!("{err}")),
                    score: 0.8,
                    provider_cost: Money::usd_micros(17),
                    charged_cost: Money::usd_micros(17),
                    cached: false,
                    created_at: Utc::now(),
                },
                remaining_budget: Money::usd_micros(83),
            })
        }

        fn remaining_budget(&self) -> Result<Money, JudgeBrokerError> {
            Ok(Money::usd_micros(83))
        }
    }
}
