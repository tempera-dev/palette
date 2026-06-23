import { createHash } from "node:crypto";

export const GATE2_CONFIRMATION_HASH_PREFIX = "gate2";
export const GATE2_CONFIRMATION_TEST_VECTOR = {
  salt: "gate2-contract-test-salt",
  traceId: "0123456789abcdef0123456789abcdef",
  spanId: "0123456789abcdef",
  code: "AB743641"
};

export function gate2ConfirmationCode({ salt = "", traceId, spanId }) {
  return createHash("sha256")
    .update(`${GATE2_CONFIRMATION_HASH_PREFIX}:${salt}:${traceId}:${spanId}`)
    .digest("hex")
    .slice(0, 8)
    .toUpperCase();
}
