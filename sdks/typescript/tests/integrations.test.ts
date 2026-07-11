import assert from "node:assert";
import { test } from "node:test";

import * as palette from "../src/index";

test("TypeScript integration registry exposes current SDK support", () => {
  const available = Object.fromEntries(palette.availableIntegrations().map((spec) => [spec.slug, spec]));

  assert.equal(available.openai.module, "@palette/sdk");
  assert.equal(available.anthropic.module, "@palette/sdk");
  assert.equal(available.langchain.status, palette.INTEGRATION_AVAILABLE);
  assert.match(available["vercel-ai-sdk"].notes, /OpenTelemetry/);
  assert.ok(palette.availableIntegrations().every((spec) => spec.status === palette.INTEGRATION_AVAILABLE));
});

test("TypeScript integration registry tracks architecture backlog candidates", () => {
  const planned = new Set(palette.plannedIntegrations().map((spec) => spec.slug));

  for (const slug of [
    "llamaindex",
    "langgraph",
    "openai-agents-sdk",
    "mastra",
    "livekit-agents",
    "agentscope",
    "google-adk",
    "litellm",
    "bedrock",
    "mistral",
    "groq",
    "gemini",
  ]) {
    assert.ok(planned.has(slug), `expected ${slug} to be tracked`);
  }
});

test("TypeScript integration registry is stable and searchable", () => {
  const catalog = palette.integrationCatalog();
  const slugs = catalog.map((spec) => spec.slug);

  assert.equal(slugs.length, new Set(slugs).size);
  assert.deepEqual(slugs, [...slugs].sort());
  assert.equal(palette.findIntegration("  Vercel-AI-SDK  ")?.status, palette.INTEGRATION_AVAILABLE);
  assert.equal(palette.findIntegration("missing"), undefined);
});

test("Vercel AI SDK telemetry helper builds supported telemetry option shapes", () => {
  const telemetry = palette.vercelAiTelemetry({
    functionId: "support-reply",
    recordInputs: false,
    recordOutputs: false,
    metadata: { tenant: "acme" },
  });

  assert.deepEqual(telemetry, {
    isEnabled: true,
    functionId: "support-reply",
    recordInputs: false,
    recordOutputs: false,
    metadata: { tenant: "acme" },
  });

  const request = { prompt: "Write a short reply" };
  const current = palette.withVercelAiTelemetry(request, { functionId: "current" });
  const stable = palette.withVercelAiTelemetry(request, {
    optionName: "telemetry",
    enabled: false,
  });

  assert.deepEqual(request, { prompt: "Write a short reply" });
  assert.equal(current.experimental_telemetry?.functionId, "current");
  assert.equal(stable.telemetry?.isEnabled, false);
});
