import { describe, expect, it } from "vitest";
import { DEFAULT_CONSENT, consentLabel, hasConsent, setConsent } from "../consent";

describe("consent helpers", () => {
  it("returns false when consent is ask", () => {
    expect(hasConsent(DEFAULT_CONSENT, "messages")).toBe(false);
  });

  it("updates consent immutably", () => {
    const next = setConsent(DEFAULT_CONSENT, "support", "allow");
    expect(next.support).toBe("allow");
  });

  it("labels consent values", () => {
    expect(consentLabel("ask")).toMatch(/Ask/);
  });
});
