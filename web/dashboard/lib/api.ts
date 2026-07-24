import type { components, operations } from "./generated/api-types";
import { apiSpanIoLabels } from "./span-kinds";
import { applyFilterParams } from "./dashboard-query";

type TraceListOperation = operations["traces.list"];
type TraceOperation = operations["traces.get"];
type SpanOperation = operations["spans.get"];
type SpanIoOperation = operations["spans.getIo"];
type SearchOperation = operations["search.spans"];
type TraceListQuery = NonNullable<TraceListOperation["parameters"]["query"]>;
type TraceListPathParams = TraceListOperation["parameters"]["path"];
type TracePathParams = TraceOperation["parameters"]["path"];
type TraceReadQuery = NonNullable<TraceOperation["parameters"]["query"]>;
type SpanPathParams = SpanOperation["parameters"]["path"];
type SpanIoPathParams = SpanIoOperation["parameters"]["path"];
type SearchQueryParams = NonNullable<SearchOperation["parameters"]["query"]>;
type SearchPathParams = SearchOperation["parameters"]["path"];

export type RunSummary = components["schemas"]["RunSummary"];
export type RunSummaryPage = components["schemas"]["TraceListResponse"];
export type Money = components["schemas"]["Money"];
export type CanonicalSpan = components["schemas"]["CanonicalSpan"];
export type TraceView = components["schemas"]["TraceView"];
export type SpanIoResponse = components["schemas"]["SpanIoResponse"];
export type SpanIoValue = SpanIoResponse["input"];
export type SearchHit = components["schemas"]["SearchHit"];
export type SearchResponse = components["schemas"]["SearchSpanListResponse"];

export type DashboardQuery = {
  tenantId: string;
  projectId?: string;
  environmentId?: string;
  traceId?: string;
  selectedSpanId?: string;
  status?: TraceListQuery["status"];
  kind?: TraceListQuery["kind"];
  startedAfter?: TraceListQuery["startedAfter"];
  startedBefore?: TraceListQuery["startedBefore"];
  model?: TraceListQuery["model"];
  release?: TraceListQuery["release"];
  minCostMicros?: TraceListQuery["minCostMicros"];
  maxCostMicros?: TraceListQuery["maxCostMicros"];
  minLatencyMs?: TraceListQuery["minLatencyMs"];
  maxLatencyMs?: TraceListQuery["maxLatencyMs"];
  unmask?: TraceReadQuery["unmask"];
  unmaskReason?: TraceReadQuery["reason"];
};

export type DashboardData = {
  apiBaseUrl: string;
  query: DashboardQuery;
  runs: RunSummaryPage;
  trace: TraceView | null;
  selectedSpan: CanonicalSpan | null;
  selectedIo: SpanIoResponse | null;
  error: string | null;
};

export type SearchQuery = {
  tenantId: string;
  projectId?: SearchQueryParams["projectId"];
  environmentId?: SearchQueryParams["environmentId"];
  q?: SearchQueryParams["q"];
  traceId?: SearchQueryParams["traceId"];
  spanId?: SearchQueryParams["spanId"];
  kind?: SearchQueryParams["kind"];
  status?: SearchQueryParams["status"];
  model?: SearchQueryParams["model"];
  tool?: SearchQueryParams["tool"];
  pageSize?: SearchQueryParams["pageSize"];
};

export type SearchData = {
  apiBaseUrl: string;
  query: SearchQuery;
  response: SearchResponse;
  error: string | null;
};

export function dashboardApiBaseUrl(): string {
  return (
    process.env.PALETTE_API_BASE_URL ??
    process.env.NEXT_PUBLIC_PALETTE_API_BASE_URL ??
    "http://127.0.0.1:8080"
  ).replace(/\/$/, "");
}

export function dashboardApiHeaders(
  query: Pick<DashboardQuery, "projectId" | "environmentId">
): HeadersInit {
  const headers: Record<string, string> = {};
  const bearerToken = process.env.PALETTE_API_TOKEN ?? process.env.PALETTE_API_BEARER_TOKEN;
  const apiKey = process.env.PALETTE_API_KEY;
  if (bearerToken) {
    headers.authorization = bearerToken.startsWith("Bearer ")
      ? bearerToken
      : `Bearer ${bearerToken}`;
  } else if (apiKey) {
    headers["x-palette-api-key"] = apiKey;
  }
  if (query.projectId) headers["x-palette-project-id"] = query.projectId;
  if (query.environmentId) headers["x-palette-environment-id"] = query.environmentId;
  return headers;
}

export function searchParamsForTraceList(query: DashboardQuery): URLSearchParams {
  const params = new URLSearchParams();
  if (query.projectId) params.set("projectId", query.projectId);
  if (query.environmentId) params.set("environmentId", query.environmentId);
  if (query.traceId) params.set("traceId", query.traceId);
  applyFilterParams(query, params);
  params.set("pageSize", "50");
  return params;
}

export function traceListPath(query: DashboardQuery): string {
  const path: TraceListPathParams = { tenantId: query.tenantId };
  const params = searchParamsForTraceList(query);
  const suffix = params.toString();
  return `/v1/traces/${encodeURIComponent(path.tenantId)}${suffix ? `?${suffix}` : ""}`;
}

export function searchParamsForSpanSearch(query: SearchQuery): URLSearchParams {
  const params = new URLSearchParams();
  if (query.q) params.set("q", query.q);
  if (query.projectId) params.set("projectId", query.projectId);
  if (query.environmentId) params.set("environmentId", query.environmentId);
  if (query.traceId) params.set("traceId", query.traceId);
  if (query.spanId) params.set("spanId", query.spanId);
  if (query.kind) params.set("kind", query.kind);
  if (query.status) params.set("status", query.status);
  if (query.model) params.set("model", query.model);
  if (query.tool) params.set("tool", query.tool);
  if (query.pageSize !== undefined) params.set("pageSize", String(query.pageSize));
  return params;
}

export function searchSpansPath(query: SearchQuery): string {
  const path: SearchPathParams = { tenantId: query.tenantId };
  const params = searchParamsForSpanSearch(query);
  const suffix = params.toString();
  return `/v1/search/${encodeURIComponent(path.tenantId)}/spans${suffix ? `?${suffix}` : ""}`;
}

export function tracePath(query: DashboardQuery, traceId: string): string {
  const path: TracePathParams = { tenantId: query.tenantId, traceId: traceId };
  const params = traceReadParams(query);
  const suffix = params.toString();
  return `/v1/traces/${encodeURIComponent(path.tenantId)}/${encodeURIComponent(path.traceId)}${
    suffix ? `?${suffix}` : ""
  }`;
}

export function spanPath(query: DashboardQuery, traceId: string, spanId: string): string {
  const path: SpanPathParams = {
    tenantId: query.tenantId,
    traceId: traceId,
    spanId: spanId
  };
  const params = traceReadParams(query);
  const suffix = params.toString();
  return `/v1/spans/${encodeURIComponent(path.tenantId)}/${encodeURIComponent(
    path.traceId
  )}/${encodeURIComponent(path.spanId)}${suffix ? `?${suffix}` : ""}`;
}

export function spanIoPath(query: DashboardQuery, traceId: string, spanId: string): string {
  const path: SpanIoPathParams = {
    tenantId: query.tenantId,
    traceId: traceId,
    spanId: spanId
  };
  const params = traceReadParams(query);
  const suffix = params.toString();
  return `/v1/spans/${encodeURIComponent(path.tenantId)}/${encodeURIComponent(
    path.traceId
  )}/${encodeURIComponent(path.spanId)}/io${suffix ? `?${suffix}` : ""}`;
}

function traceReadParams(query: DashboardQuery): URLSearchParams {
  const params = new URLSearchParams();
  if (!query.unmask) return params;
  params.set("unmask", "true");
  if (query.unmaskReason) params.set("reason", query.unmaskReason);
  return params;
}

export async function loadSearchData(query: SearchQuery): Promise<SearchData> {
  const apiBaseUrl = dashboardApiBaseUrl();
  try {
    const response = await fetchJson<SearchResponse>(
      `${apiBaseUrl}${searchSpansPath(query)}`,
      dashboardApiHeaders(query)
    );
    return { apiBaseUrl, query, response, error: null };
  } catch (error) {
    return {
      apiBaseUrl,
      query,
      response: { hits: [] },
      error: errorMessage(error)
    };
  }
}

export async function loadDashboardData(query: DashboardQuery): Promise<DashboardData> {
  const apiBaseUrl = dashboardApiBaseUrl();
  const headers = dashboardApiHeaders(query);
  let runs: RunSummaryPage;
  try {
    runs = await fetchJson<RunSummaryPage>(`${apiBaseUrl}${traceListPath(query)}`, headers);
  } catch (error) {
    return {
      apiBaseUrl,
      query,
      runs: { runs: [] },
      trace: null,
      selectedSpan: null,
      selectedIo: null,
      error: errorMessage(error)
    };
  }

  const activeRun = query.traceId
    ? runs.runs.find((run) => run.traceId === query.traceId) ?? runs.runs[0]
    : runs.runs[0];
  const activeTraceId = query.traceId || activeRun?.traceId;
  const activeRunMatchesTrace = activeRun !== undefined && activeRun.traceId === activeTraceId;
  const traceQuery =
    activeRunMatchesTrace && activeRun.projectId && !query.projectId
      ? { ...query, projectId: activeRun.projectId }
      : query;
  let trace: TraceView | null = null;
  let selectedSpan: CanonicalSpan | null = null;
  let selectedIo: SpanIoResponse | null = null;
  let error: string | null = null;

  if (activeTraceId) {
    try {
      trace = await fetchJson<TraceView>(
        `${apiBaseUrl}${tracePath(traceQuery, activeTraceId)}`,
        dashboardApiHeaders(traceQuery)
      );
    } catch (traceError) {
      error = errorMessage(traceError);
    }
  }

  const waterfallSpans = trace ? orderSpansForWaterfall(trace.spans) : [];
  const requestedSpanFromTrace =
    trace && query.selectedSpanId
      ? waterfallSpans.find((span) => span.spanId === query.selectedSpanId) ?? null
      : null;
  const selectedSpanFromTrace = query.selectedSpanId
    ? requestedSpanFromTrace
    : waterfallSpans[0] ?? null;
  if (trace && query.selectedSpanId && !requestedSpanFromTrace) {
    error = `Span ${query.selectedSpanId} was not found in trace ${trace.traceId}.`;
  }
  const activeSpanId = selectedSpanFromTrace?.spanId;

  if (trace && activeSpanId) {
    try {
      selectedSpan = await fetchJson<CanonicalSpan>(
        `${apiBaseUrl}${spanPath(traceQuery, trace.traceId, activeSpanId)}`,
        dashboardApiHeaders(traceQuery)
      );
    } catch (spanError) {
      selectedSpan = selectedSpanFromTrace;
      error = errorMessage(spanError);
    }
  }

  if (trace && selectedSpan) {
    try {
      selectedIo = await fetchJson<SpanIoResponse>(
        `${apiBaseUrl}${spanIoPath(traceQuery, trace.traceId, selectedSpan.spanId)}`,
        dashboardApiHeaders(traceQuery)
      );
    } catch (ioError) {
      error = errorMessage(ioError);
    }
  }

  return {
    apiBaseUrl,
    query,
    runs,
    trace,
    selectedSpan,
    selectedIo,
    error
  };
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

async function fetchJson<T>(url: string, headers: HeadersInit): Promise<T> {
  const response = await fetch(url, { cache: "no-store", headers });
  if (!response.ok) {
    throw new Error(formatApiError(response.status, response.statusText, await response.text()));
  }
  return (await response.json()) as T;
}

export function formatApiError(status: number, statusText: string, body: string): string {
  const statusLabel = statusText ? `${status} ${statusText}` : String(status);
  return `API ${statusLabel}: ${apiErrorDetail(body)}`;
}

function apiErrorDetail(body: string): string {
  const trimmed = body.trim();
  if (!trimmed) return "empty response";
  try {
    const parsed = JSON.parse(trimmed) as unknown;
    if (typeof parsed === "string") return truncateApiError(parsed);
    if (parsed && typeof parsed === "object") {
      const record = parsed as Record<string, unknown>;
      for (const key of ["message", "error", "detail", "title"]) {
        const value = record[key];
        if (typeof value === "string" && value.trim()) {
          return truncateApiError(value);
        }
      }
    }
  } catch {
    // Fall through to the raw body preview.
  }
  return truncateApiError(trimmed.replace(/\s+/g, " "));
}

function truncateApiError(value: string): string {
  const trimmed = value.trim();
  return trimmed.length > 240 ? `${trimmed.slice(0, 237)}...` : trimmed;
}

export function durationMs(start: string, end: string | null | undefined): number | null {
  if (!end) return null;
  const startMicros = timestampMicros(start);
  const endMicros = timestampMicros(end);
  if (startMicros === null || endMicros === null) return null;
  return Math.max(0, endMicros - startMicros) / 1000;
}

export function formatDuration(start: string, end: string | null | undefined): string {
  const ms = durationMs(start, end);
  if (ms === null) return "open";
  if (ms < 1000) return formatMilliseconds(ms);
  return `${(ms / 1000).toFixed(2)} s`;
}

export function timestampMicros(value: string): number | null {
  const match = value.match(
    /^(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})(?:\.(\d{1,9}))?(Z|[+-]\d{2}:\d{2})$/
  );
  if (match) {
    const [, wholeSecond, fraction = "", zone] = match;
    const secondMs = Date.parse(`${wholeSecond}${zone}`);
    if (!Number.isFinite(secondMs)) return null;
    const micros = Number(fraction.padEnd(6, "0").slice(0, 6));
    return secondMs * 1000 + micros;
  }
  const parsedMs = Date.parse(value);
  return Number.isFinite(parsedMs) ? parsedMs * 1000 : null;
}

export function formatCost(cost: Money | null | undefined): string {
  if (!cost) return "none";
  return `${cost.currency} ${(cost.amountMicros / 1_000_000).toFixed(6)}`;
}

export function formatModels(models: RunSummary["models"] | undefined): string {
  if (!models?.length) return "no model";
  return models.map((model) => `${model.provider}/${model.name}`).join(", ");
}

export function formatReleases(releaseIds: string[] | undefined): string {
  if (!releaseIds?.length) return "no release";
  return releaseIds.join(", ");
}

export function formatLatency(durationMs: number | null | undefined): string {
  if (durationMs === null || durationMs === undefined || !Number.isFinite(durationMs) || durationMs < 0) {
    return "open";
  }
  if (durationMs < 1000) return formatMilliseconds(durationMs);
  return `${(durationMs / 1000).toFixed(2)} s`;
}

export function spanTokenTotal(span: Pick<CanonicalSpan, "tokens">): number {
  if (!span.tokens) return 0;
  return span.tokens.input + span.tokens.output + span.tokens.cacheRead + span.tokens.reasoning;
}

export function spanTokenSummary(span: Pick<CanonicalSpan, "kind" | "tokens">): string {
  if (!span.tokens) return "none";
  const total = spanTokenTotal(span);
  const { input: inputLabel, output: outputLabel } = apiSpanIoLabels(span.kind);
  const parts = [
    `${total.toLocaleString("en-US")} total`,
    `${span.tokens.input.toLocaleString("en-US")} ${inputLabel}`,
    `${span.tokens.output.toLocaleString("en-US")} ${outputLabel}`
  ];
  if (span.tokens.reasoning > 0) {
    parts.push(`${span.tokens.reasoning.toLocaleString("en-US")} reasoning`);
  }
  if (span.tokens.cacheRead > 0) {
    parts.push(`${span.tokens.cacheRead.toLocaleString("en-US")} cached`);
  }
  return parts.join(", ");
}

export function isRedactedIoValue(value: SpanIoValue | undefined): boolean {
  if (!value) return false;
  if (value.kind === "redacted") return true;
  return value.kind === "inline" && value.value === "[redacted]";
}

export function ioVisibilityLabel(hasRedactedIo: boolean, unmask: boolean | undefined): string {
  if (hasRedactedIo) return unmask ? "still redacted" : "redacted";
  return unmask ? "unmask requested" : "captured";
}

type WaterfallSpan = Pick<
  CanonicalSpan,
  "spanId" | "parentSpanId" | "startTime" | "seq"
>;

export function orderSpansForWaterfall<T extends WaterfallSpan>(spans: T[]): T[] {
  const ids = new Set(spans.map((span) => span.spanId));
  const children = new Map<string | null, T[]>();
  for (const span of spans) {
    const parentId =
      span.parentSpanId && span.parentSpanId !== span.spanId && ids.has(span.parentSpanId)
        ? span.parentSpanId
        : null;
    const bucket = children.get(parentId) ?? [];
    bucket.push(span);
    children.set(parentId, bucket);
  }

  for (const bucket of children.values()) {
    bucket.sort(compareWaterfallSiblings);
  }

  const ordered: T[] = [];
  const seen = new Set<string>();
  const visit = (span: T) => {
    if (seen.has(span.spanId)) return;
    seen.add(span.spanId);
    ordered.push(span);
    for (const child of children.get(span.spanId) ?? []) {
      visit(child);
    }
  };

  for (const root of children.get(null) ?? []) {
    visit(root);
  }
  for (const span of [...spans].sort(compareWaterfallSiblings)) {
    visit(span);
  }
  return ordered;
}

function compareWaterfallSiblings(left: WaterfallSpan, right: WaterfallSpan): number {
  const leftStart = timestampMicros(left.startTime) ?? Number.MAX_SAFE_INTEGER;
  const rightStart = timestampMicros(right.startTime) ?? Number.MAX_SAFE_INTEGER;
  if (leftStart !== rightStart) return leftStart - rightStart;
  if (left.seq !== right.seq) return left.seq - right.seq;
  if (left.spanId < right.spanId) return -1;
  if (left.spanId > right.spanId) return 1;
  return 0;
}

function formatMilliseconds(ms: number): string {
  if (ms > 0 && ms < 1) return `${ms.toFixed(3)} ms`;
  if (ms > 0 && ms < 10 && !Number.isInteger(ms)) return `${ms.toFixed(1)} ms`;
  return `${Math.round(ms)} ms`;
}

export function spanDepth(span: CanonicalSpan, spans: CanonicalSpan[]): number {
  let depth = 0;
  let parent = span.parentSpanId;
  const byId = new Map(spans.map((candidate) => [candidate.spanId, candidate]));
  const seen = new Set([span.spanId]);
  while (parent && byId.has(parent) && !seen.has(parent) && depth < 12) {
    depth += 1;
    seen.add(parent);
    parent = byId.get(parent)?.parentSpanId ?? null;
  }
  return depth;
}

export function statusLabel(status: string): string {
  if (status === "ok") return "OK";
  if (status === "error") return "Error";
  return "Unset";
}
