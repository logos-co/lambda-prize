import { describe, expect, it } from "vitest";
import { assertAddress, assertTxHash, assertPositiveDecimalString } from "../validation";

describe("validation", () => {
  it("accepts valid address", () => {
    expect(() =>
      assertAddress("0x1234567890abcdef1234567890abcdef12345678")
    ).not.toThrow();
  });

  it("rejects invalid tx hash", () => {
    expect(() => assertTxHash("0xabc")).toThrow();
  });

  it("rejects invalid amount", () => {
    expect(() => assertPositiveDecimalString("-1")).toThrow();
  });
});
