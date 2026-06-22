import { expect, type Locator } from "@playwright/test";

export type ExpectedTokenBreakdown = {
  label: string;
  value: string;
};

export async function expectTokenBreakdown(
  breakdown: Locator,
  expected: ExpectedTokenBreakdown[]
) {
  await expect(breakdown.locator(".token-chip")).toHaveCount(expected.length);
  const actual = await tokenBreakdownPairs(breakdown);
  expect(actual).toEqual(expected);
}

async function tokenBreakdownPairs(breakdown: Locator): Promise<ExpectedTokenBreakdown[]> {
  return breakdown.locator(".token-chip").evaluateAll((chips) =>
    chips.map((chip) => ({
      label: chip.querySelector("b")?.textContent?.trim() ?? "",
      value: chip.querySelector("span")?.textContent?.trim() ?? ""
    }))
  );
}
