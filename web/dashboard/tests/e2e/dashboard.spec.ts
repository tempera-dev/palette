import { expect, test } from "@playwright/test";

test("renders a stock OTLP llm span through table, waterfall, detail, and I/O", async ({
  page
}) => {
  await page.goto("/?tenant=demo&project=demo&environment=local&kind=llm.call");

  await expect(page.getByRole("heading", { name: "Agent Trace Debugger" })).toBeVisible();
  await expect(page.getByLabel("Traces")).toContainText("refund-agent-run");
  await expect(page.getByLabel("Traces")).toContainText("openai/gpt-demo");

  const waterfall = page.getByLabel("Agent span waterfall");
  await expect(waterfall).toContainText("call-policy-model");
  await expect(waterfall).toContainText("llm.call");

  await waterfall.getByText("call-policy-model").click();

  const detail = page.getByLabel("Span detail");
  await expect(detail).toContainText("openai/gpt-demo");
  await expect(detail).toContainText("USD 0.002500");
  await expect(detail).toContainText("Can this order be refunded after 31 days?");
  await expect(detail).toContainText("Escalate because the order is outside the standard window.");
});
