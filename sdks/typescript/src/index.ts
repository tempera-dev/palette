/**
 * Palette TypeScript SDK — ergonomic agent observability (OpenTelemetry-native).
 *
 *   import * as palette from "@palette/sdk";
 *   palette.init({ tenantId: "acme", projectId: "support-bot", environmentId: "prod" });
 *   const handle = palette.observe(rawHandle, { kind: palette.SpanKind.AGENT_RUN });
 *
 * This is the hand-written ergonomic (Layer 2) SDK. The generated control-plane
 * client (Layer 1) ships separately as `@palette/client`, generated from the
 * OpenAPI contract so it never drifts from the API.
 */

export { init, getConfig, flush, shutdown } from "./tracing";
export { observe, span, setInput, setOutput } from "./observe";
export { auto, instrument } from "./auto";
export type { InstrumentOptions, InstrumentProvider, InstrumentResult, SkippedProvider } from "./auto";
export { wrapOpenAI } from "./providers/openai";
export { wrapAnthropic } from "./providers/anthropic";
export { SpanKind, Attr, SPAN_KINDS } from "./semconv";
export type { SpanKindValue } from "./semconv";
export type { PaletteConfig, PaletteOptions } from "./config";
export { PaletteCallbackHandler } from "./integrations/langchain";
export { instrumentLlamaIndex } from "./integrations/llamaindex";
export type {
  LlamaIndexCallbackManager,
  LlamaIndexEventHandler,
  LlamaIndexEventName,
  LlamaIndexInstrumentation,
} from "./integrations/llamaindex";
export {
  INTEGRATION_AVAILABLE,
  INTEGRATION_PLANNED,
  availableIntegrations,
  findIntegration,
  integrationCatalog,
  plannedIntegrations,
} from "./integrations/registry";
export type { IntegrationKind, IntegrationSpec, IntegrationStatus } from "./integrations/registry";
export { vercelAiTelemetry, withVercelAiTelemetry } from "./integrations/vercel-ai";
export type {
  PaletteVercelAiTelemetryOptions,
  VercelAiTelemetryConfig,
  VercelAiTelemetryOptionName,
  WithVercelAiTelemetryOptions,
} from "./integrations/vercel-ai";

export const VERSION = "0.1.0";
