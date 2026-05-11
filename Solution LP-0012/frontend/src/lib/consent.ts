import type { ConsentScope, ConsentValue, PrivacyConsentMatrix } from "../types/privacy";

export const DEFAULT_CONSENT: PrivacyConsentMatrix = {
  identity: "ask",
  balances: "ask",
  transactions: "ask",
  messages: "deny",
  analytics: "deny",
  support: "allow",
};

export function hasConsent(matrix: PrivacyConsentMatrix, scope: ConsentScope): boolean {
  return matrix[scope] === "allow";
}

export function canAsk(matrix: PrivacyConsentMatrix, scope: ConsentScope): boolean {
  return matrix[scope] === "ask";
}

export function setConsent(
  matrix: PrivacyConsentMatrix,
  scope: ConsentScope,
  value: ConsentValue
): PrivacyConsentMatrix {
  return { ...matrix, [scope]: value };
}

export function consentLabel(value: ConsentValue): string {
  switch (value) {
    case "allow":
      return "Allow";
    case "deny":
      return "Deny";
    case "ask":
      return "Ask each time";
  }
}

export function consentSummary(matrix: PrivacyConsentMatrix): string {
  return (Object.keys(matrix) as ConsentScope[])
    .map((scope) => `${scope}:${matrix[scope]}`)
    .join(", ");
}

export function consentFromRecord(raw: unknown): PrivacyConsentMatrix {
  const fallback = DEFAULT_CONSENT;
  if (typeof raw !== "object" || raw === null) return fallback;

  const obj = raw as Partial<PrivacyConsentMatrix>;
  return {
    identity: obj.identity ?? fallback.identity,
    balances: obj.balances ?? fallback.balances,
    transactions: obj.transactions ?? fallback.transactions,
    messages: obj.messages ?? fallback.messages,
    analytics: obj.analytics ?? fallback.analytics,
    support: obj.support ?? fallback.support,
  };
}
