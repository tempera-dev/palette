/** Tracer setup: configure an OpenTelemetry pipeline that exports to Palette. */

import { trace, Tracer } from "@opentelemetry/api";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-proto";
import { Resource } from "@opentelemetry/resources";
import { BatchSpanProcessor, NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import type { SpanExporter } from "@opentelemetry/sdk-trace-base";

import { PaletteConfig, PaletteOptions, otlpHttpTracesUrl, resolveConfig } from "./config";

const TRACER_NAME = "palette.sdk";

let currentConfig: PaletteConfig | undefined;
let currentProvider: NodeTracerProvider | undefined;

/**
 * Initialize the Palette tracer. Call once at process start. All options fall
 * back to `PALETTE_*` env vars, so `init()` works with no args when configured.
 *
 * `options.exporter` overrides the default OTLP exporter (used for tests/advanced
 * pipelines).
 */
export function init(options: PaletteOptions & { exporter?: SpanExporter } = {}): PaletteConfig {
  const config = resolveConfig(options);
  const headers: Record<string, string> = {};
  if (config.apiKey) {
    headers["authorization"] = `Bearer ${config.apiKey}`;
  }

  const exporter: SpanExporter =
    options.exporter ??
    // compression: "none" — paletted's OTLP endpoint expects uncompressed protobuf.
    new OTLPTraceExporter({ url: otlpHttpTracesUrl(config), headers, compression: "none" as never });
  const provider = new NodeTracerProvider({
    resource: new Resource({ "service.name": config.serviceName }),
    spanProcessors: [new BatchSpanProcessor(exporter)],
  });
  provider.register();

  currentProvider = provider;
  currentConfig = config;
  return config;
}

export function getConfig(): PaletteConfig | undefined {
  return currentConfig;
}

export function getTracer(): Tracer {
  if (!currentConfig) {
    init();
  }
  return trace.getTracer(TRACER_NAME);
}

export async function flush(): Promise<void> {
  if (currentProvider) {
    await currentProvider.forceFlush();
  }
}

export async function shutdown(): Promise<void> {
  if (currentProvider) {
    await currentProvider.shutdown();
  }
}
