// Vercel AI SDK + OTLP -> Palette example app (Phase 6 §20.8 #6.3).
//
// Demonstrates wrapping an AI SDK agent-style generateText call and tool
// execution with stock OpenTelemetry spans, then exporting to paletted over
// OTLP/HTTP. This stays standards-first: no Palette SDK and no provider wrapper
// import are required.
//
// By default this uses a local stub with the same generateText/tool shape, so
// the example can run without a model key or live network. To run against a
// real AI SDK model, install `ai`, set PALETTE_EXAMPLE_LIVE_AI=1, and provide a
// model/credential supported by your AI SDK setup.
//
// Run a local paletted (`docker compose up`) and then:
//
//   npm install @opentelemetry/api @opentelemetry/sdk-trace-node \
//     @opentelemetry/exporter-trace-otlp-proto
//   node examples/typescript/frameworks/vercel-ai-sdk-otlp.mjs
//
// Live AI SDK run:
//
//   npm install ai @opentelemetry/api @opentelemetry/sdk-trace-node \
//     @opentelemetry/exporter-trace-otlp-proto
//   PALETTE_EXAMPLE_LIVE_AI=1 AI_MODEL=anthropic/claude-sonnet-4.5 \
//     node examples/typescript/frameworks/vercel-ai-sdk-otlp.mjs

import { SpanStatusCode, trace } from "@opentelemetry/api";
import { NodeTracerProvider } from "@opentelemetry/sdk-trace-node";
import { BatchSpanProcessor } from "@opentelemetry/sdk-trace-base";
import { OTLPTraceExporter } from "@opentelemetry/exporter-trace-otlp-proto";

const apiBase =
  process.env.OTEL_EXPORTER_OTLP_ENDPOINT ?? "http://127.0.0.1:8080";
const tenant = process.env.PALETTE_TENANT_ID ?? "demo";
const project = process.env.PALETTE_PROJECT_ID ?? "demo";
const environment = process.env.PALETTE_ENVIRONMENT_ID ?? "local";
const url = `${apiBase}/v1/otlp/${tenant}/${project}/${environment}/v1/traces`;

const provider = new NodeTracerProvider();
provider.addSpanProcessor(
  new BatchSpanProcessor(new OTLPTraceExporter({ url })),
);
provider.register();

const tracer = trace.getTracer("palette.example.vercel-ai-sdk");
const release = process.env.PALETTE_RELEASE_ID ?? "vercel-ai-sdk-example";
const model = process.env.AI_MODEL ?? "anthropic/claude-sonnet-4.5";

async function loadAiSdk() {
  if (process.env.PALETTE_EXAMPLE_LIVE_AI === "1") {
    return await import("ai");
  }
  return {
    generateText: stubGenerateText,
    jsonSchema: (schema) => schema,
    tool: (definition) => definition,
  };
}

async function stubGenerateText({ prompt, tools }) {
  const toolResult = await tools.lookupOrder.execute({ orderId: "demo-42" }, {});
  return {
    text: `Stub response for "${prompt}" with ${toolResult.status} order data.`,
    totalUsage: {
      inputTokens: 41,
      outputTokens: 16,
      totalTokens: 57,
      reasoningTokens: 0,
      cachedInputTokens: 0,
    },
  };
}

function recordTokenUsage(span, result) {
  const usage = result.totalUsage ?? result.usage ?? {};
  if (usage.inputTokens != null) {
    span.setAttribute("llm.token_count.prompt", Number(usage.inputTokens));
  }
  if (usage.outputTokens != null) {
    span.setAttribute("llm.token_count.completion", Number(usage.outputTokens));
  }
  if (usage.reasoningTokens != null) {
    span.setAttribute("llm.token_count.reasoning", Number(usage.reasoningTokens));
  }
  if (usage.cachedInputTokens != null) {
    span.setAttribute("llm.token_count.cache_read", Number(usage.cachedInputTokens));
  }
}

function orderLookupTool(tool, jsonSchema) {
  return tool({
    description: "Look up order status for customer support triage.",
    inputSchema: jsonSchema({
      type: "object",
      additionalProperties: false,
      required: ["orderId"],
      properties: {
        orderId: {
          type: "string",
          description: "The customer order id to inspect.",
        },
      },
    }),
    execute: async ({ orderId }) =>
      tracer.startActiveSpan(
        "lookup_order",
        {
          attributes: {
            "palette.span.kind": "tool.call",
            "palette.release_id": release,
            "tool.name": "lookupOrder",
            "input.value": orderId,
          },
        },
        async (span) => {
          const result = { orderId, status: "delayed", etaDays: 2 };
          span.setAttribute("output.value", JSON.stringify(result));
          span.setStatus({ code: SpanStatusCode.OK });
          span.end();
          return result;
        },
      ),
  });
}

async function main() {
  const { generateText, jsonSchema, tool } = await loadAiSdk();
  const prompt =
    process.env.PROMPT ?? "Customer asks whether order demo-42 needs escalation.";

  await tracer.startActiveSpan(
    "support_triage",
    {
      attributes: {
        "palette.span.kind": "agent.run",
        "palette.release_id": release,
        "agent.framework": "vercel-ai-sdk",
        "input.value": prompt,
      },
    },
    async (root) => {
      const result = await tracer.startActiveSpan(
        "ai_sdk_generate_text",
        {
          attributes: {
            "palette.span.kind": "llm.call",
            "palette.release_id": release,
            "llm.provider": "ai-gateway",
            "llm.model_name": model,
            "input.value": prompt,
          },
        },
        async (llm) => {
          try {
            const response = await generateText({
              model,
              prompt,
              tools: {
                lookupOrder: orderLookupTool(tool, jsonSchema),
              },
              toolChoice: { type: "tool", toolName: "lookupOrder" },
              maxRetries: 0,
              experimental_telemetry: {
                isEnabled: true,
                functionId: "palette.examples.vercel_ai_sdk",
                metadata: { palette_example: "vercel-ai-sdk-otlp" },
              },
            });
            recordTokenUsage(llm, response);
            llm.setAttribute("output.value", response.text ?? "");
            llm.setStatus({ code: SpanStatusCode.OK });
            return response;
          } catch (error) {
            llm.recordException(error);
            llm.setStatus({
              code: SpanStatusCode.ERROR,
              message: String(error?.message ?? error),
            });
            throw error;
          } finally {
            llm.end();
          }
        },
      );
      root.setAttribute("output.value", result.text ?? "");
      root.setStatus({ code: SpanStatusCode.OK });
      root.end();
      console.log("vercel ai sdk trace flushed:", result.text ?? "(no text)");
    },
  );

  await provider.shutdown();
}

main().catch(async (error) => {
  console.error(error);
  await provider.shutdown().catch(() => {});
  process.exit(1);
});
