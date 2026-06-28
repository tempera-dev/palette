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
//!
//! ## Bounded working set
//!
//! Live references are collected by **paging** the trace store's keyset cursor
//! (`query_spans` returns a `next_cursor`; see [`SweepConfig::page_size`]). The
//! sweeper never asks the store for `u32::MAX` rows — each request is bounded by
//! `page_size`, so the per-request working set scales with the batch size rather
//! than the total number of spans in the tenant/project.
//!
//! Candidate artifacts are likewise consumed as an iterator and processed in
//! `page_size`-sized batches, so callers can stream an object-store listing
//! through the sweeper without materializing it all at once.
//!
//! ## Resume
//!
//! A sweep over a large candidate set can be checkpointed: every batch a callback
//! observes the [`SweepProgress`] (the number of candidates consumed so far). To
//! resume after an interruption, re-run [`OrphanedArtifactSweeper::sweep_iter`]
//! with the candidate iterator advanced past the already-processed prefix (e.g.
//! `candidates.skip(report.candidates_seen)` for an order-stable listing). Because
//! delete is idempotent, re-processing a prefix is harmless; the resume cursor
//! only avoids redundant work. Full server-side checkpoint persistence is out of
//! scope for this change — the returned [`SweepReport::candidates_seen`] is the
//! resume cursor.

use beater_core::{PageRequest, ProjectId, TenantId};
use beater_schema::{ArtifactRef, CanonicalSpan, SpanFilter};
use beater_store::{ArtifactStore, StoreResult, TraceStore};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Default number of span summaries / candidate artifacts processed per batch.
///
/// Chosen as a safe bound on the per-request working set; the sweeper never asks
/// the trace store for more than this many rows at a time.
pub const DEFAULT_SWEEP_PAGE_SIZE: u32 = 1000;

/// Tuning for a sweep pass.
///
/// A sweep paginates both the live-reference scan and the candidate stream in
/// `page_size`-sized chunks so its working set is bounded by the batch size, not
/// the total store size. `max_pages` is a safety valve that caps the number of
/// live-reference pages scanned in a single pass (a value of `0` means
/// "unbounded", i.e. scan until the cursor is exhausted).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SweepConfig {
    /// Maximum rows requested per `query_spans` call and candidates processed per
    /// delete batch. Must be >= 1; `0` is clamped to 1.
    pub page_size: u32,
    /// Safety cap on the number of live-reference pages scanned per sweep. `0`
    /// disables the cap (scan to cursor exhaustion). When the cap is hit the
    /// scan stops early and [`SweepReport::scan_truncated`] is set.
    pub max_pages: u32,
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self {
            page_size: DEFAULT_SWEEP_PAGE_SIZE,
            max_pages: 0,
        }
    }
}

impl SweepConfig {
    /// Effective page size, clamped to at least 1 so we never request an empty
    /// page (which a backend could interpret as "no limit").
    fn effective_page_size(&self) -> u32 {
        self.page_size.max(1)
    }
}

/// Progress checkpoint observed by the per-batch callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SweepProgress {
    /// Total candidate artifacts consumed from the input so far. Use this as the
    /// `skip` count to resume an interrupted sweep over an order-stable listing.
    pub candidates_seen: usize,
    /// Candidates deleted so far.
    pub deleted: usize,
    /// Candidates retained (still referenced or out of scope) so far.
    pub retained: usize,
}

/// Outcome and metrics of a sweep pass.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SweepReport {
    /// Candidate artifacts whose URI was still referenced by a live span (or that
    /// fell outside the sweep scope) and so were left in place.
    pub retained: usize,
    /// Candidate artifacts that were unreferenced and deleted from the store.
    pub deleted: usize,
    /// URIs of the artifacts that were deleted, in sorted order.
    pub deleted_uris: Vec<String>,
    /// Total candidate artifacts consumed from the input. Doubles as the resume
    /// cursor: re-run the sweep with the candidate iterator advanced by this many
    /// elements to continue where an interrupted pass left off.
    pub candidates_seen: usize,
    /// Number of live span summaries scanned to build the referenced-URI set.
    pub spans_scanned: usize,
    /// Number of distinct live traces whose full spans were loaded.
    pub traces_scanned: usize,
    /// Number of `query_spans` pages fetched while building the live set.
    pub pages_scanned: usize,
    /// Number of distinct artifact URIs found to be live-referenced.
    pub live_referenced: usize,
    /// True if the live-reference scan stopped early because `max_pages` was hit.
    /// When set, the sweep is incomplete and MUST NOT be trusted to delete
    /// orphans (it could delete a still-referenced artifact whose span lived on an
    /// unscanned page); callers should treat a truncated scan as a no-delete dry
    /// run. The sweeper enforces this by skipping deletes when truncated.
    pub scan_truncated: bool,
    /// Wall-clock duration of the sweep.
    pub duration: Duration,
}

/// Deletes object-store artifacts that are no longer referenced by any live span.
pub struct OrphanedArtifactSweeper {
    artifacts: Arc<dyn ArtifactStore>,
    config: SweepConfig,
}

impl OrphanedArtifactSweeper {
    /// Construct a sweeper with the default [`SweepConfig`].
    pub fn new(artifacts: Arc<dyn ArtifactStore>) -> Self {
        Self::with_config(artifacts, SweepConfig::default())
    }

    /// Construct a sweeper with an explicit batch/page configuration.
    pub fn with_config(artifacts: Arc<dyn ArtifactStore>, config: SweepConfig) -> Self {
        Self { artifacts, config }
    }

    /// The configuration this sweeper runs with.
    pub fn config(&self) -> SweepConfig {
        self.config
    }

    /// Reconciles `candidates` against the artifacts referenced by the live spans
    /// of `tenant` (optionally scoped to `project`) in `trace_store`, deleting any
    /// candidate whose URI is not referenced.
    ///
    /// Convenience wrapper over [`Self::sweep_iter`] for callers that already hold
    /// the candidates as a slice. Candidates are still processed in `page_size`
    /// batches internally, so passing a large slice does not change the bounded
    /// per-request working set of the live-reference scan.
    pub async fn sweep(
        &self,
        trace_store: &dyn TraceStore,
        tenant: TenantId,
        project: Option<ProjectId>,
        candidates: &[ArtifactRef],
    ) -> StoreResult<SweepReport> {
        self.sweep_iter(trace_store, tenant, project, candidates.iter().cloned())
            .await
    }

    /// Streaming variant of [`Self::sweep`]: consumes candidates from any
    /// iterator (e.g. a lazily-paged object-store listing) instead of requiring a
    /// fully-materialized slice. Candidates are pulled and deleted in `page_size`
    /// batches so the candidate side never has to be held in memory all at once.
    pub async fn sweep_iter<I>(
        &self,
        trace_store: &dyn TraceStore,
        tenant: TenantId,
        project: Option<ProjectId>,
        candidates: I,
    ) -> StoreResult<SweepReport>
    where
        I: IntoIterator<Item = ArtifactRef>,
    {
        self.sweep_with_progress(trace_store, tenant, project, candidates, |_| {})
            .await
    }

    /// Like [`Self::sweep_iter`] but invokes `on_progress` after each candidate
    /// batch with the running [`SweepProgress`]. Callers can persist
    /// `progress.candidates_seen` as a resume checkpoint.
    pub async fn sweep_with_progress<I, F>(
        &self,
        trace_store: &dyn TraceStore,
        tenant: TenantId,
        project: Option<ProjectId>,
        candidates: I,
        mut on_progress: F,
    ) -> StoreResult<SweepReport>
    where
        I: IntoIterator<Item = ArtifactRef>,
        F: FnMut(SweepProgress),
    {
        let started = Instant::now();
        let sweep_tenant = tenant.clone();
        let sweep_project = project.clone();

        let mut report = SweepReport::default();
        let live = self
            .collect_live_references(trace_store, tenant, project, &mut report)
            .await?;

        let batch = self.config.effective_page_size() as usize;
        let mut buf: Vec<ArtifactRef> = Vec::with_capacity(batch);
        for candidate in candidates {
            buf.push(candidate);
            if buf.len() >= batch {
                self.process_batch(
                    &buf,
                    &live,
                    &sweep_tenant,
                    sweep_project.as_ref(),
                    &mut report,
                )
                .await?;
                buf.clear();
                on_progress(SweepProgress {
                    candidates_seen: report.candidates_seen,
                    deleted: report.deleted,
                    retained: report.retained,
                });
            }
        }
        if !buf.is_empty() {
            self.process_batch(
                &buf,
                &live,
                &sweep_tenant,
                sweep_project.as_ref(),
                &mut report,
            )
            .await?;
            on_progress(SweepProgress {
                candidates_seen: report.candidates_seen,
                deleted: report.deleted,
                retained: report.retained,
            });
        }

        report.deleted_uris.sort();
        report.live_referenced = live.len();
        report.duration = started.elapsed();
        Ok(report)
    }

    /// Builds the set of artifact URIs referenced by live spans by paging the
    /// trace store's keyset cursor in `page_size` chunks. Never requests more than
    /// `page_size` rows in a single call, so the working set is bounded by the
    /// batch size rather than the tenant/project's total span count.
    async fn collect_live_references(
        &self,
        trace_store: &dyn TraceStore,
        tenant: TenantId,
        project: Option<ProjectId>,
        report: &mut SweepReport,
    ) -> StoreResult<BTreeSet<String>> {
        // `query_spans` returns span summaries, which intentionally drop the
        // artifact refs, so reconcile against full traces instead. We discover the
        // live trace ids by paging the span summaries (bounded `limit`, following
        // the keyset `next_cursor`), then load each distinct trace's full spans to
        // collect every referenced artifact uri.
        let limit = self.config.effective_page_size();
        let mut cursor: Option<String> = None;
        let mut trace_ids: BTreeSet<_> = BTreeSet::new();

        loop {
            if self.config.max_pages != 0 && report.pages_scanned as u32 >= self.config.max_pages {
                report.scan_truncated = true;
                break;
            }

            let page = trace_store
                .query_spans(
                    tenant.clone(),
                    SpanFilter {
                        project_id: project.clone(),
                        ..SpanFilter::default()
                    },
                    PageRequest {
                        limit,
                        cursor: cursor.take(),
                    },
                )
                .await?;
            report.pages_scanned += 1;
            report.spans_scanned += page.items.len();

            for summary in page.items {
                trace_ids.insert(summary.trace_id);
            }

            match page.next_cursor {
                Some(next) => cursor = Some(next),
                None => break,
            }
        }

        report.traces_scanned = trace_ids.len();

        let mut referenced = BTreeSet::new();
        for trace_id in trace_ids {
            let view = match &project {
                Some(project) => {
                    trace_store
                        .get_project_trace(tenant.clone(), project.clone(), trace_id)
                        .await?
                }
                None => trace_store.get_trace(tenant.clone(), trace_id).await?,
            };
            referenced.extend(referenced_artifact_uris(&view.spans));
        }
        Ok(referenced)
    }

    /// Reconcile one batch of candidates against the live set, deleting orphans.
    ///
    /// If the live-reference scan was truncated (`max_pages` hit) the live set is
    /// incomplete, so deleting would be unsafe — we count every candidate as
    /// retained instead, turning a truncated sweep into a no-delete dry run.
    async fn process_batch(
        &self,
        candidates: &[ArtifactRef],
        live: &BTreeSet<String>,
        tenant: &TenantId,
        project: Option<&ProjectId>,
        report: &mut SweepReport,
    ) -> StoreResult<()> {
        for candidate in candidates {
            report.candidates_seen += 1;
            if !artifact_uri_in_scope(&candidate.uri, tenant, project) {
                continue;
            }
            if report.scan_truncated || live.contains(&candidate.uri) {
                report.retained += 1;
                continue;
            }
            self.artifacts.delete_bytes(candidate).await?;
            report.deleted += 1;
            report.deleted_uris.push(candidate.uri.clone());
        }
        Ok(())
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
    use beater_core::{
        ArtifactId, EnvironmentId, IdempotencyKey, Page, Sha256Hash, SpanId, TraceId,
    };
    use beater_schema::{
        AgentSpanKind, CanonicalTraceBatch, RawEnvelope, RedactionClass, SpanStatus, SpanSummary,
        TraceView, WriteAck, CANONICAL_SCHEMA_VERSION,
    };
    use beater_store::TraceStore;
    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use std::collections::BTreeMap;
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
    async fn process_batch_skips_candidates_outside_sweep_scope() {
        let artifacts = Arc::new(RecordingArtifactStore::default());
        let sweeper = OrphanedArtifactSweeper::new(artifacts.clone());
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));
        let live = BTreeSet::from([String::from("artifact://tenant/project/live")]);

        let mut report = SweepReport::default();
        sweeper
            .process_batch(
                &[
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
                &mut report,
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        report.deleted_uris.sort();

        assert_eq!(report.retained, 1);
        assert_eq!(report.deleted, 1);
        assert_eq!(
            report.deleted_uris,
            vec![String::from("artifact://tenant/project/orphan")]
        );
        assert_eq!(
            artifacts.deleted(),
            vec![String::from("artifact://tenant/project/orphan")]
        );
    }

    /// The headline acceptance test: a store holding many spans is swept with a
    /// small page size, and we assert the sweeper never requested more than
    /// `page_size` rows in any single `query_spans` call — i.e. the working set
    /// scales with the batch size, not the total store size (and is never
    /// `u32::MAX`).
    #[tokio::test]
    async fn live_reference_scan_requests_at_most_page_size_per_call() {
        const TOTAL_SPANS: usize = 25_000;
        const PAGE_SIZE: u32 = 1000;

        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));

        // Each span is its own trace and references a unique live artifact, plus
        // one orphan candidate that is referenced by nothing.
        let store = PagingTraceStore::new(&tenant, &project, TOTAL_SPANS, PAGE_SIZE);
        let artifacts = Arc::new(RecordingArtifactStore::default());
        let sweeper = OrphanedArtifactSweeper::with_config(
            artifacts.clone(),
            SweepConfig {
                page_size: PAGE_SIZE,
                max_pages: 0,
            },
        );

        let orphan = artifact("artifact://tenant/project/orphan");
        let live_sample = artifact(&format!(
            "artifact://tenant/project/live-{}",
            TOTAL_SPANS - 1
        ));
        let candidates = vec![orphan.clone(), live_sample.clone()];

        let report = sweeper
            .sweep(&store, tenant.clone(), Some(project.clone()), &candidates)
            .await
            .unwrap_or_else(|err| panic!("{err}"));

        // The crucial invariant: the largest limit ever requested equals the
        // configured page size, NOT u32::MAX and NOT TOTAL_SPANS.
        assert_eq!(
            store.max_limit_requested(),
            PAGE_SIZE,
            "sweeper must never request more than page_size rows per call"
        );
        assert!(store.max_limit_requested() < u32::MAX);

        // And it did fully scan via pagination.
        assert_eq!(report.spans_scanned, TOTAL_SPANS);
        assert_eq!(report.traces_scanned, TOTAL_SPANS);
        assert_eq!(
            report.pages_scanned,
            TOTAL_SPANS.div_ceil(PAGE_SIZE as usize)
        );
        assert!(!report.scan_truncated);

        // Reconciliation is still correct: the orphan is deleted, the live one
        // retained.
        assert_eq!(report.deleted, 1);
        assert_eq!(report.retained, 1);
        assert_eq!(report.deleted_uris, vec![orphan.uri.clone()]);
        assert_eq!(report.candidates_seen, 2);
    }

    /// Candidates streamed through an iterator are processed in `page_size`
    /// batches, and the per-batch progress callback reports a monotonic resume
    /// cursor.
    #[tokio::test]
    async fn streamed_candidates_processed_in_bounded_batches_with_progress() {
        const PAGE_SIZE: u32 = 4;
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));

        // Empty store => no live references => every in-scope candidate is orphan.
        let store = PagingTraceStore::new(&tenant, &project, 0, PAGE_SIZE);
        let artifacts = Arc::new(RecordingArtifactStore::default());
        let sweeper = OrphanedArtifactSweeper::with_config(
            artifacts.clone(),
            SweepConfig {
                page_size: PAGE_SIZE,
                max_pages: 0,
            },
        );

        let candidates: Vec<ArtifactRef> = (0..10)
            .map(|i| artifact(&format!("artifact://tenant/project/c{i}")))
            .collect();

        let progress = Arc::new(Mutex::new(Vec::<SweepProgress>::new()));
        let progress_for_cb = progress.clone();
        let report = sweeper
            .sweep_with_progress(
                &store,
                tenant.clone(),
                Some(project.clone()),
                candidates.iter().cloned(),
                move |p| {
                    progress_for_cb
                        .lock()
                        .unwrap_or_else(|err| panic!("{err}"))
                        .push(p);
                },
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));

        assert_eq!(report.deleted, 10);
        assert_eq!(report.candidates_seen, 10);

        // 10 candidates / batch of 4 => batches at 4, 8, 10.
        let seen: Vec<usize> = progress
            .lock()
            .unwrap_or_else(|err| panic!("{err}"))
            .iter()
            .map(|p| p.candidates_seen)
            .collect();
        assert_eq!(seen, vec![4, 8, 10]);
    }

    /// Resuming from a checkpoint: re-running with the candidate iterator advanced
    /// by `candidates_seen` does not re-delete the already-handled prefix.
    #[tokio::test]
    async fn resume_skips_already_processed_prefix() {
        const PAGE_SIZE: u32 = 3;
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));
        let store = PagingTraceStore::new(&tenant, &project, 0, PAGE_SIZE);
        let artifacts = Arc::new(RecordingArtifactStore::default());
        let sweeper = OrphanedArtifactSweeper::with_config(
            artifacts.clone(),
            SweepConfig {
                page_size: PAGE_SIZE,
                max_pages: 0,
            },
        );

        let candidates: Vec<ArtifactRef> = (0..6)
            .map(|i| artifact(&format!("artifact://tenant/project/c{i}")))
            .collect();

        // First pass: only consume the first 3 candidates (simulate interruption).
        let first = sweeper
            .sweep_iter(
                &store,
                tenant.clone(),
                Some(project.clone()),
                candidates.iter().take(3).cloned(),
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(first.candidates_seen, 3);
        assert_eq!(first.deleted, 3);

        // Resume from the cursor: skip the processed prefix.
        let second = sweeper
            .sweep_iter(
                &store,
                tenant.clone(),
                Some(project.clone()),
                candidates.iter().skip(first.candidates_seen).cloned(),
            )
            .await
            .unwrap_or_else(|err| panic!("{err}"));
        assert_eq!(second.candidates_seen, 3);
        assert_eq!(second.deleted, 3);

        // Across both passes every candidate was deleted exactly once.
        let mut deleted = artifacts.deleted();
        deleted.sort();
        let expected: Vec<String> = (0..6)
            .map(|i| format!("artifact://tenant/project/c{i}"))
            .collect();
        assert_eq!(deleted, expected);
    }

    /// A scan capped by `max_pages` is treated as a no-delete dry run: no orphan
    /// is deleted because the live set is known-incomplete.
    #[tokio::test]
    async fn truncated_scan_does_not_delete() {
        const PAGE_SIZE: u32 = 100;
        let tenant = TenantId::new("tenant").unwrap_or_else(|err| panic!("{err}"));
        let project = ProjectId::new("project").unwrap_or_else(|err| panic!("{err}"));
        let store = PagingTraceStore::new(&tenant, &project, 1000, PAGE_SIZE);
        let artifacts = Arc::new(RecordingArtifactStore::default());
        let sweeper = OrphanedArtifactSweeper::with_config(
            artifacts.clone(),
            SweepConfig {
                page_size: PAGE_SIZE,
                max_pages: 2,
            },
        );

        let orphan = artifact("artifact://tenant/project/orphan");
        let report = sweeper
            .sweep(&store, tenant, Some(project), &[orphan])
            .await
            .unwrap_or_else(|err| panic!("{err}"));

        assert!(report.scan_truncated);
        assert_eq!(report.pages_scanned, 2);
        assert_eq!(report.deleted, 0, "truncated scan must not delete");
        assert_eq!(report.retained, 1);
        assert!(artifacts.deleted().is_empty());
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

    /// In-memory trace store that holds `total` single-span traces and serves them
    /// through `query_spans` using a numeric offset cursor. It records the largest
    /// `limit` it was ever asked for so a test can assert the sweeper's bounded
    /// working set.
    struct PagingTraceStore {
        summaries: Vec<SpanSummary>,
        spans: BTreeMap<String, CanonicalSpan>,
        max_limit_requested: Mutex<u32>,
        max_page_size: u32,
    }

    impl PagingTraceStore {
        fn new(tenant: &TenantId, project: &ProjectId, total: usize, max_page_size: u32) -> Self {
            let mut summaries = Vec::with_capacity(total);
            let mut spans = BTreeMap::new();
            for i in 0..total {
                let trace_id =
                    TraceId::new(format!("trace-{i}")).unwrap_or_else(|err| panic!("{err}"));
                let span = single_span(tenant, project, i, &trace_id);
                summaries.push(span_summary(tenant, project, &trace_id));
                spans.insert(trace_id.as_str().to_string(), span);
            }
            Self {
                summaries,
                spans,
                max_limit_requested: Mutex::new(0),
                max_page_size,
            }
        }

        fn max_limit_requested(&self) -> u32 {
            *self
                .max_limit_requested
                .lock()
                .unwrap_or_else(|err| panic!("{err}"))
        }
    }

    #[async_trait]
    impl TraceStore for PagingTraceStore {
        async fn write_batch(&self, _batch: CanonicalTraceBatch) -> StoreResult<WriteAck> {
            panic!("not used")
        }

        async fn get_trace(&self, tenant: TenantId, trace: TraceId) -> StoreResult<TraceView> {
            let span = self
                .spans
                .get(trace.as_str())
                .cloned()
                .unwrap_or_else(|| panic!("missing trace {}", trace.as_str()));
            Ok(TraceView {
                tenant_id: tenant,
                trace_id: trace,
                spans: vec![span],
            })
        }

        async fn get_project_trace(
            &self,
            tenant: TenantId,
            _project: ProjectId,
            trace: TraceId,
        ) -> StoreResult<TraceView> {
            let span = self
                .spans
                .get(trace.as_str())
                .cloned()
                .unwrap_or_else(|| panic!("missing trace {}", trace.as_str()));
            Ok(TraceView {
                tenant_id: tenant,
                trace_id: trace,
                spans: vec![span],
            })
        }

        async fn get_raw_envelope(
            &self,
            _tenant: TenantId,
            _project: ProjectId,
            _idempotency_key: IdempotencyKey,
        ) -> StoreResult<Option<RawEnvelope>> {
            panic!("not used")
        }

        async fn query_runs(
            &self,
            _tenant: TenantId,
            _filter: beater_schema::RunFilter,
            _page: PageRequest,
        ) -> StoreResult<Page<beater_schema::RunSummary>> {
            panic!("not used")
        }

        async fn query_spans(
            &self,
            _tenant: TenantId,
            _filter: SpanFilter,
            page: PageRequest,
        ) -> StoreResult<Page<SpanSummary>> {
            // Record the largest limit ever requested, and reject any caller that
            // tries to exceed the configured page size — the whole point of the
            // test is that the sweeper stays bounded.
            {
                let mut max = self
                    .max_limit_requested
                    .lock()
                    .unwrap_or_else(|err| panic!("{err}"));
                if page.limit > *max {
                    *max = page.limit;
                }
            }
            assert!(
                page.limit <= self.max_page_size,
                "store asked for limit {} > page_size {}",
                page.limit,
                self.max_page_size
            );

            let offset = page
                .cursor
                .as_deref()
                .and_then(|c| c.parse::<usize>().ok())
                .unwrap_or(0);
            let limit = page.limit.max(1) as usize;
            let end = (offset + limit).min(self.summaries.len());
            let items = self.summaries[offset.min(self.summaries.len())..end].to_vec();
            let next_cursor = if end < self.summaries.len() {
                Some(end.to_string())
            } else {
                None
            };
            Ok(Page::new(items, next_cursor))
        }
    }

    fn span_summary(tenant: &TenantId, project: &ProjectId, trace: &TraceId) -> SpanSummary {
        SpanSummary {
            tenant_id: tenant.clone(),
            project_id: project.clone(),
            trace_id: trace.clone(),
            span_id: SpanId::new("root").unwrap_or_else(|err| panic!("{err}")),
            kind: AgentSpanKind::AgentRun,
            name: "run".to_string(),
            status: SpanStatus::Ok,
            started_at: Utc
                .with_ymd_and_hms(2026, 1, 1, 0, 0, 1)
                .single()
                .unwrap_or_else(|| panic!("valid timestamp")),
            ended_at: None,
            model: None,
            cost: None,
            release_id: None,
        }
    }

    fn single_span(
        tenant: &TenantId,
        project: &ProjectId,
        idx: usize,
        trace: &TraceId,
    ) -> CanonicalSpan {
        CanonicalSpan {
            schema_version: CANONICAL_SCHEMA_VERSION,
            normalizer_version: "beater-native-v1".to_string(),
            tenant_id: tenant.clone(),
            project_id: project.clone(),
            environment_id: EnvironmentId::new("prod").unwrap_or_else(|err| panic!("{err}")),
            trace_id: trace.clone(),
            span_id: SpanId::new("root").unwrap_or_else(|err| panic!("{err}")),
            parent_span_id: None,
            seq: 1,
            kind: AgentSpanKind::AgentRun,
            name: "run".to_string(),
            status: SpanStatus::Ok,
            start_time: Utc
                .with_ymd_and_hms(2026, 1, 1, 0, 0, 1)
                .single()
                .unwrap_or_else(|| panic!("valid timestamp")),
            end_time: None,
            model: None,
            cost: None,
            tokens: None,
            input_ref: None,
            output_ref: None,
            attributes: BTreeMap::new(),
            unmapped_attrs: json!({}),
            raw_ref: artifact(&format!("artifact://tenant/project/live-{idx}")),
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
