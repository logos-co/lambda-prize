import { describe, expect, it } from "vitest";
import { redactAddress, redactAmount, privacySummary } from "../redaction";

describe("redaction helpers", () => {
  it("redacts addresses by default", () => {
    expect(
      redactAddress("0x1234567890abcdef1234567890abcdef12345678" as `0x${string}`)
    ).toContain("…");
  });

  it("keeps amounts visible when requested", () => {
    expect(redactAmount("12345.678", true)).toBe("12345.678");
  });

  it("summarizes privacy settings", () => {
    expect(
      privacySummary({
        showRawAddresses: false,
        showRawAmounts: false,
        showTxHashes: false,
        showCounterpartyNames: false,
        redactMessageBodies: true,
        localEncryptionEnabled: true,
        autoLockMinutes: 10,
        preferredPrivacyLevel: "private",
      })
    ).toContain("local encryption enabled");
  });
});
