//! Orphaned-artifact sweeper.
//!
//! Reconciles a set of candidate artifact references against the artifacts still
//! referenced by live spans in a [`TraceStore`], deleting the bytes for any
//! candidate that is no longer reachable. This reclaims object-store space after
//! traces age out / are deleted from the hot store while their artifacts (raw
//! payloads, span inputs/outputs) linger.
//!
//! The sweep is conservative (delete only what is provably unreferenced) and
//! idempotent (delete of a missing object is a no-op), so it is safe to re-run
//! and safe to race with concurrent deletes.

use beater_core::{PageRequest, ProjectId, TenantId};
use beater_schema::{ArtifactRef, CanonicalSpan, SpanFilter};
use beater_store::{ArtifactStore, StoreResult, TraceStore};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Default number of spans materialized per `query_spans` page and number of
/// candidate artifacts processed per delete batch. A maintenance job must bound
/// its working set by a batch budget, not by total store size, so the sweep
/// pages the live-span scan and processes candidates in fixed-size batches
/// instead of materializing everything in one pass.
pub const DEFAULT_SWEEP_BATCH_SIZE: u32 = 500;

/// Tuning + checkpoint inputs for a sweep pass.
///
/// `batch_size` bounds both the live-span scan page (`query_spans` `limit`) and
/// the candidate delete batch, so per-page memory and wall-time scale with the
/// batch, not the store. `resume_from` lets a pass continue where a previous one
/// stopped without restarting the candidate stream from the beginning (see
/// [`SweepReport::checkpoint`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SweepConfig {
    /// Page/batch size used for both span paging and candidate batching. Never
    /// `u32::MAX`; callers should leave this at a bounded value. A value of `0`
    /// is treated as `1`.
    pub batch_size: u32,
    /// Optional checkpoint: skip every candidate whose URI is `<=` this marker.
    /// Pass the [`SweepReport::checkpoint`] returned by a previous pass to
    /// resume. Resume is only correct if candidates are supplied in a stable
    /// order (e.g. sorted by URI, as object-store listings are).
    pub resume_from: Option<String>,
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self {
            batch_size: DEFAULT_SWEEP_BATCH_SIZE,
            resume_from: None,
        }
    }
}

impl SweepConfig {
    /// Batch size as a `query_spans` page limit, clamped to a real bounded value
    /// (`>= 1`, never `u32::MAX` for the default).
    fn page_limit(&self) -> u32 {
        self.batch_size.max(1)
    }

    /// Batch size as a candidate-batch length.
    fn batch_len(&self) -> usize {
        self.batch_size.max(1) as usize
    }
}

/// Metrics + outcome of a sweep pass.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SweepReport {
    /// Span summaries scanned across all `query_spans` pages.
    pub scanned_spans: usize,
    /// Distinct live traces loaded to collect artifact references.
    pub scanned_traces: usize,
    /// Candidate artifacts considered this pass (in scope, after any resume skip).
    pub candidates: usize,
    /// Candidate artifacts whose URI was still referenced by a live span and so
    /// were left in place.
    pub retained: usize,
    /// Candidate artifacts that were unreferenced and deleted from the store.
    pub deleted: usize,
    /// Candidate deletes that failed (counted and skipped; the sweep is
    /// idempotent and safe to re-run, so a failed delete does not abort the pass).
    pub errors: usize,
    /// URIs of the artifacts that were deleted, in sorted order.
    pub deleted_uris: Vec<String>,
    /// URI of the last candidate processed this pass. Feed back as
    /// [`SweepConfig::resume_from`] to resume a long sweep without restarting.
    pub checkpoint: Option<String>,
    /// Wall-clock duration of the pass.
    pub duration: Duration,
}

/// Deletes object-store artifacts that are no longer referenced by any live span.
pub struct OrphanedArtifactSweeper {
    artifacts: Arc<dyn ArtifactStore>,
}

impl OrphanedArtifactSweeper {
    pub fn new(artifacts: Arc<dyn ArtifactStore>) -> Self {
        Self { artifacts }
    }

    /// Reconciles `candidates` against the artifacts referenced by the live spans
    /// of `tenant` (optionally scoped to `project`) in `trace_store`, deleting any
    /// candidate whose URI is not referenced.
    ///
    /// `candidates` is the set of artifacts known to exist in the object store
    /// (e.g. an object-store listing). It is consumed as an iterator and
    /// processed in batches of `config.batch_size`, so the caller may stream a
    /// lazy listing instead of materializing the full set. Anything in
    /// `candidates` not referenced by a live span is treated as orphaned and
    /// deleted.
    ///
    /// Bounded vs. unbounded memory: the span scan is paged (one
    /// `config.batch_size` page in flight at a time) and candidates are batched,
    /// so both scale with the batch budget. The *set of live artifact URIs* must
    /// still be complete before any delete is safe, so it is held in memory and
    /// scales with the live data of the tenant/project. Pushing that membership
    /// test into the store (so the sweeper never materializes the full live set)
    /// is tracked by the store-side query push-down in #201.
    pub async fn sweep(
        &self,
        trace_store: &dyn TraceStore,
        tenant: TenantId,
        project: Option<ProjectId>,
        candidates: impl IntoIterator<Item = ArtifactRef>,
        config: &SweepConfig,
    ) -> StoreResult<SweepReport> {
        let started = Instant::now();
        let mut report = SweepReport::default();
        let live = self
            .live_referenced_uris(trace_store, &tenant, project.as_ref(), config, &mut report)
            .await?;
        self.delete_orphans(
            candidates,
            &live,
            &tenant,
            project.as_ref(),
            config,
            &mut report,
        )
        .await?;
        report.deleted_uris.sort();
        report.duration = started.elapsed();
        Ok(report)
    }

    async fn live_referenced_uris(
        &self,
        trace_store: &dyn TraceStore,
        tenant: &TenantId,
        project: Option<&ProjectId>,
        config: &SweepConfig,
        report: &mut SweepReport,
    ) -> StoreResult<BTreeSet<String>> {
        // `query_spans` returns span summaries, which intentionally drop the
        // artifact refs, so reconcile against full traces instead. We page the
        // span summaries with a bounded `limit` (never u32::MAX), discovering live
        // trace ids one page at a time, then load each *new* trace's full spans to
        // collect every referenced artifact uri. Only one page of summaries is in
        // flight at a time; the set of seen trace ids + referenced uris is the
        // part that must be complete (see `sweep` docs / #201).
        let mut referenced = BTreeSet::new();
        let mut seen_traces = BTreeSet::new();
        let mut cursor = None;
        loop {
            let page = trace_store
                .query_spans(
                    tenant.clone(),
                    SpanFilter {
                        project_id: project.cloned(),
                        ..SpanFilter::default()
                    },
                    PageRequest {
                        limit: config.page_limit(),
                        cursor: cursor.take(),
                    },
                )
                .await?;
            report.scanned_spans += page.items.len();
            for summary in page.items {
                if !seen_traces.insert(summary.trace_id.clone()) {
                    continue;
                }
                report.scanned_traces += 1;
                let view = match project {
                    Some(project) => {
                        trace_store
                            .get_project_trace(tenant.clone(), project.clone(), summary.trace_id)
                            .await?
                    }
                    None => {
                        trace_store
                            .get_trace(tenant.clone(), summary.trace_id)
                            .await?
                    }
                };
                referenced.extend(referenced_artifact_uris(&view.spans));
            }
            match page.next_cursor {
                Some(next) => cursor = Some(next),
                None => break,
            }
        }
        Ok(referenced)
    }

    async fn delete_orphans(
        &self,
        candidates: impl IntoIterator<Item = ArtifactRef>,
        live: &BTreeSet<String>,
        tenant: &TenantId,
        project: Option<&ProjectId>,
        config: &SweepConfig,
        report: &mut SweepReport,
    ) -> StoreResult<()> {
        let batch_len = config.batch_len();
        let resume_from = config.resume_from.as_deref();
        let mut batch = Vec::with_capacity(batch_len);
        for candidate in candidates {
            if !artifact_uri_in_scope(&candidate.uri, tenant, project) {
                continue;
            }
            if let Some(marker) = resume_from {
                if candidate.uri.as_str() <= marker {
                    continue;
                }
            }
            batch.push(candidate);
            if batch.len() >= batch_len {
                self.process_candidate_batch(&batch, live, report).await;
                batch.clear();
            }
        }
        if !batch.is_empty() {
            self.process_candidate_batch(&batch, live, report).await;
        }
        Ok(())
    }

    async fn process_candidate_batch(
        &self,
        batch: &[ArtifactRef],
        live: &BTreeSet<String>,
        report: &mut SweepReport,
    ) {
        for candidate in batch {
            report.candidates += 1;
            report.checkpoint = Some(candidate.uri.clone());
            if live.contains(&candidate.uri) {
                report.retained += 1;
                continue;
            }
            match self.artifacts.delete_bytes(candidate).await {
                Ok(()) => {
                    report.deleted += 1;
                    report.deleted_uris.push(candidate.uri.clone());
                }
                Err(_) => {
                    report.errors += 1;
                }
            }
        }
    }
}

fn artifact_uri_in_scope(uri: &str, tenant: &TenantId, project: Option<&ProjectId>) -> bool {
    let Some(path) = uri.strip_prefix("artifact://") else {
        return false;
    };
    let segments = path.split('/').collect::<Vec<_>>();
    if segments.len() != 3
        || segments
            .iter()
            .any(|segment| segment.is_empty() || *segment == "." || *segment == "..")
    {
        return false;
    }
    if segments[0] != tenant.as_str() {
        return false;
    }
    match project {
        Some(project) => segments[1] == project.as_str(),
        None => true,
    }
}

/// Collects every artifact uri referenced by the given spans (raw payloads plus
/// any span input/output artifacts).
fn referenced_artifact_uris(spans: &[CanonicalSpan]) -> BTreeSet<String> {
    let mut uris = BTreeSet::new();
    for span in spans {
        uris.insert(span.raw_ref.uri.clone());
        if let Some(input) = &span.input_ref {
            uris.insert(input.uri.clone());
        }
        if let Some(output) = &span.output_ref {
            uris.insert(output.uri.clone());
        }
    }
    uris
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use beater_core::{ArtifactId, Sha256Hash, SpanId, TraceId};
    use beater_schema::RedactionClass;
    use std::sync::Mutex;

    #[test]
    fn artifact_uri_scope_uses_exact_path_segments() {
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let other_tenant = TenantId::new("tenant-other").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));

        assert!(artifact_uri_in_scope(
            "artifact://tenant/project/artifact",
            &tenant,
            Some(&project)
        ));
        assert!(artifact_uri_in_scope(
            "artifact://tenant/other-project/artifact",
            &tenant,
            None
        ));
        assert!(!artifact_uri_in_scope(
            "artifact://tenant/projectile/artifact",
            &tenant,
            Some(&project)
        ));
        assert!(!artifact_uri_in_scope(
            "artifact://tenant-other/project/artifact",
            &tenant,
            Some(&project)
        ));
        assert!(!artifact_uri_in_scope(
            "artifact://tenant/project/artifact/extra",
            &tenant,
            Some(&project)
        ));
        assert!(!artifact_uri_in_scope(
            "artifact://tenant/project",
            &tenant,
            Some(&project)
        ));
        assert!(!artifact_uri_in_scope(
            "artifact://tenant/../project",
            &tenant,
            None
        ));
        assert!(!artifact_uri_in_scope(
            "https://tenant/project/artifact",
            &tenant,
            Some(&project)
        ));

        let slash_tenant = TenantId::new("tenant/slash").unwrap_or_else(|err| panic!("{err}"));
        assert!(!artifact_uri_in_scope(
            "artifact://tenant/slash/project/artifact",
            &slash_tenant,
            Some(&project)
        ));
        assert!(!artifact_uri_in_scope(
            "artifact://tenant/project/artifact",
            &other_tenant,
            None
        ));
    }

    #[tokio::test]
    async fn delete_orphans_skips_candidates_outside_sweep_scope() {
        let artifacts = Arc::new(RecordingArtifactStore::default());
        let sweeper = OrphanedArtifactSweeper::new(artifacts.clone());
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));
        let live = BTreeSet::from([String::from("artifact://tenant/project/live")]);

        let mut report = SweepReport::default();
        sweeper
            .delete_orphans(
                [
                    artifact("artifact://tenant/project/live"),
                    artifact("artifact://tenant/project/orphan"),
                    artifact("artifact://tenant/projectile/orphan"),
                    artifact("artifact://tenant/other-project/orphan"),
                    artifact("artifact://tenant-other/project/orphan"),
                    artifact("artifact://tenant/project/orphan/extra"),
                    artifact("artifact://tenant/project"),
                    artifact("https://tenant/project/orphan"),
                ],
                &live,
                &tenant,
                Some(&project),
                &SweepConfig::default(),
                &mut report,
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        report.deleted_uris.sort();

        assert_eq!(report.retained, 1);
        assert_eq!(report.deleted, 1);
        assert_eq!(report.candidates, 2, "only the two in-scope candidates");
        assert_eq!(
            report.deleted_uris,
            vec![String::from("artifact://tenant/project/orphan")]
        );
        assert_eq!(
            artifacts.deleted(),
            vec![String::from("artifact://tenant/project/orphan")]
        );
    }

    #[tokio::test]
    async fn sweep_pages_scan_in_bounded_batches_and_reports_metrics() {
        // 12 live traces (one referenced raw artifact each) and 20 candidates
        // (the 12 referenced + 8 orphans). With batch_size 4 the sweep must page
        // the span scan in bounded pages — never one u32::MAX query — so memory
        // and scan work scale with the batch, not the 12-trace store size.
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));

        let mut store = RecordingTraceStore::default();
        let mut candidates = Vec::new();
        for index in 0..12u32 {
            let uri = format!("artifact://tenant/project/raw-{index:02}");
            store.push_trace(&tenant, &project, index, &uri);
            candidates.push(artifact(&uri)); // referenced -> retained
        }
        for index in 0..8u32 {
            candidates.push(artifact(&format!(
                "artifact://tenant/project/orphan-{index:02}"
            )));
        }
        // Stable (sorted) order so the returned checkpoint resumes correctly.
        candidates.sort_by(|left, right| left.uri.cmp(&right.uri));

        let store = Arc::new(store);
        let artifacts = Arc::new(RecordingArtifactStore::default());
        let sweeper = OrphanedArtifactSweeper::new(artifacts.clone());
        let config = SweepConfig {
            batch_size: 4,
            resume_from: None,
        };

        let report = sweeper
            .sweep(
                store.as_ref(),
                tenant.clone(),
                Some(project.clone()),
                candidates.clone(),
                &config,
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));

        // Scan was paged in bounded batches: every page asked for exactly the
        // batch size, none used u32::MAX, and 12 spans at limit 4 took >1 page.
        let pages = store.page_requests();
        assert!(pages.len() > 1, "scan must take multiple bounded pages");
        assert!(
            pages.iter().all(|page| page.limit == config.batch_size),
            "every page uses the batch size, got {pages:?}"
        );
        assert!(
            pages.iter().all(|page| page.limit != u32::MAX),
            "reconciliation must never scan u32::MAX"
        );
        assert_eq!(
            pages[0].cursor, None,
            "first page starts from the beginning"
        );
        assert!(
            pages[1..].iter().all(|page| page.cursor.is_some()),
            "later pages follow the cursor"
        );

        // Metrics: scanned traces/spans, candidate count, delete/retain/errors.
        assert_eq!(report.scanned_spans, 12);
        assert_eq!(report.scanned_traces, 12);
        assert_eq!(report.candidates, 20);
        assert_eq!(report.retained, 12);
        assert_eq!(report.deleted, 8);
        assert_eq!(report.errors, 0);
        assert_eq!(report.deleted_uris.len(), 8);
        assert_eq!(
            report.checkpoint.as_deref(),
            Some("artifact://tenant/project/raw-11"),
            "checkpoint is the last candidate processed"
        );
        assert_eq!(artifacts.deleted().len(), 8);

        // Resume: feeding the checkpoint back skips every already-processed
        // candidate, so a re-run does no work instead of restarting the stream.
        let resumed = sweeper
            .sweep(
                store.as_ref(),
                tenant,
                Some(project),
                candidates,
                &SweepConfig {
                    batch_size: 4,
                    resume_from: report.checkpoint.clone(),
                },
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(
            resumed.candidates, 0,
            "all candidates are past the checkpoint"
        );
        assert_eq!(resumed.deleted, 0);
        assert_eq!(resumed.retained, 0);
    }

    /// `TraceStore` test double that records every `query_spans` page request and
    /// serves bounded, cursor-paged results (offset cursors, like the in-memory
    /// store) so a test can assert the sweep pages rather than scanning u32::MAX.
    #[derive(Default)]
    struct RecordingTraceStore {
        summaries: Vec<beater_schema::SpanSummary>,
        traces: std::collections::BTreeMap<TraceId, Vec<CanonicalSpan>>,
        page_requests: Mutex<Vec<PageRequest>>,
    }

    impl RecordingTraceStore {
        fn push_trace(
            &mut self,
            tenant: &TenantId,
            project: &ProjectId,
            index: u32,
            raw_uri: &str,
        ) {
            let trace_id =
                TraceId::new(format!("trace-{index:02}")).unwrap_or_else(|err| panic!("{err}"));
            let span_id =
                SpanId::new(format!("span-{index:02}")).unwrap_or_else(|err| panic!("{err}"));
            self.summaries.push(beater_schema::SpanSummary {
                tenant_id: tenant.clone(),
                project_id: project.clone(),
                trace_id: trace_id.clone(),
                span_id: span_id.clone(),
                kind: beater_schema::AgentSpanKind::AgentRun,
                name: "run".to_string(),
                status: beater_schema::SpanStatus::Ok,
                started_at: chrono::Utc::now(),
                ended_at: None,
                model: None,
                cost: None,
                release_id: None,
            });
            let span = CanonicalSpan {
                schema_version: beater_schema::CANONICAL_SCHEMA_VERSION,
                normalizer_version: "test".to_string(),
                tenant_id: tenant.clone(),
                project_id: project.clone(),
                environment_id: beater_core::EnvironmentId::new("prod")
                    .unwrap_or_else(|err| panic!("{err}")),
                trace_id: trace_id.clone(),
                span_id,
                parent_span_id: None,
                seq: 1,
                kind: beater_schema::AgentSpanKind::AgentRun,
                name: "run".to_string(),
                status: beater_schema::SpanStatus::Ok,
                start_time: chrono::Utc::now(),
                end_time: None,
                model: None,
                cost: None,
                tokens: None,
                input_ref: None,
                output_ref: None,
                attributes: std::collections::BTreeMap::new(),
                unmapped_attrs: serde_json::json!({}),
                raw_ref: artifact(raw_uri),
            };
            self.traces.insert(trace_id, vec![span]);
        }

        fn page_requests(&self) -> Vec<PageRequest> {
            self.page_requests
                .lock()
                .unwrap_or_else(|err| panic!("{err}"))
                .clone()
        }
    }

    #[async_trait]
    impl TraceStore for RecordingTraceStore {
        async fn write_batch(
            &self,
            _batch: beater_schema::CanonicalTraceBatch,
        ) -> StoreResult<beater_schema::WriteAck> {
            panic!("not used")
        }

        async fn get_trace(
            &self,
            tenant: TenantId,
            trace: TraceId,
        ) -> StoreResult<beater_schema::TraceView> {
            let spans = self
                .traces
                .get(&trace)
                .cloned()
                .unwrap_or_else(|| panic!("missing trace"));
            Ok(beater_schema::TraceView {
                tenant_id: tenant,
                trace_id: trace,
                spans,
            })
        }

        async fn get_project_trace(
            &self,
            tenant: TenantId,
            _project: ProjectId,
            trace: TraceId,
        ) -> StoreResult<beater_schema::TraceView> {
            self.get_trace(tenant, trace).await
        }

        async fn get_raw_envelope(
            &self,
            _tenant: TenantId,
            _project: ProjectId,
            _idempotency_key: beater_core::IdempotencyKey,
        ) -> StoreResult<Option<beater_schema::RawEnvelope>> {
            panic!("not used")
        }

        async fn query_runs(
            &self,
            _tenant: TenantId,
            _filter: beater_schema::RunFilter,
            _page: PageRequest,
        ) -> StoreResult<beater_core::Page<beater_schema::RunSummary>> {
            panic!("not used")
        }

        async fn query_spans(
            &self,
            _tenant: TenantId,
            _filter: SpanFilter,
            page: PageRequest,
        ) -> StoreResult<beater_core::Page<beater_schema::SpanSummary>> {
            self.page_requests
                .lock()
                .unwrap_or_else(|err| panic!("{err}"))
                .push(page.clone());
            Ok(beater_store::page_vec(self.summaries.clone(), page))
        }
    }

    #[derive(Default)]
    struct RecordingArtifactStore {
        deleted: Mutex<Vec<String>>,
    }

    impl RecordingArtifactStore {
        fn deleted(&self) -> Vec<String> {
            self.deleted
                .lock()
                .unwrap_or_else(|err| panic!("{err}"))
                .clone()
        }
    }

    #[async_trait]
    impl ArtifactStore for RecordingArtifactStore {
        async fn put_bytes(
            &self,
            _tenant_id: &TenantId,
            _project_id: &ProjectId,
            _mime_type: &str,
            _redaction_class: RedactionClass,
            _bytes: &[u8],
        ) -> StoreResult<ArtifactRef> {
            panic!("not used")
        }

        async fn get_bytes(&self, _artifact_ref: &ArtifactRef) -> StoreResult<Vec<u8>> {
            panic!("not used")
        }

        async fn delete_bytes(&self, artifact_ref: &ArtifactRef) -> StoreResult<()> {
            self.deleted
                .lock()
                .unwrap_or_else(|err| panic!("{err}"))
                .push(artifact_ref.uri.clone());
            Ok(())
        }
    }

    fn artifact(uri: &str) -> ArtifactRef {
        ArtifactRef {
            artifact_id: ArtifactId::new(uri.replace("://", "-").replace('/', "-"))
                .unwrap_or_else(|err| panic!("{err}")),
            uri: uri.to_string(),
            sha256: Sha256Hash::new("test-sha").unwrap_or_else(|err| panic!("{err}")),
            size_bytes: 1,
            mime_type: "application/octet-stream".to_string(),
            redaction_class: RedactionClass::Internal,
        }
    }
}
