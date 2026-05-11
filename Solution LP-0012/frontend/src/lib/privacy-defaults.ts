import type { ConsentMatrix, PrivacySettings } from "./privacy-types";

export const DEFAULT_CONSENT: ConsentMatrix = {
  identity: "ask",
  wallet: "ask",
  balances: "ask",
  transactions: "ask",
  messages: "deny",
  analytics: "deny",
  support: "allow",
  sharing: "ask",
};

export const DEFAULT_SETTINGS: PrivacySettings = {
  showRawAddresses: false,
  showRawAmounts: false,
  showTxHashes: false,
  showMessageBodies: false,
  showWalletProvider: false,
  redactInSearch: true,
  localEncryptionEnabled: true,
  autoLockMinutes: 10,
  preferredLevel: "private",
};

export function consentLabel(value: "allow" | "deny" | "ask"): string {
  if (value === "allow") return "Always allow";
  if (value === "deny") return "Always deny";
  return "Ask each time";
}

export function privacySummary(s: PrivacySettings): string {
  return [
    s.localEncryptionEnabled ? "local encryption on" : "local encryption off",
    s.showRawAddresses ? "raw addresses visible" : "addresses redacted",
    s.showRawAmounts ? "raw amounts visible" : "amounts redacted",
    s.showMessageBodies ? "messages visible" : "messages hidden",
  ].join(" • ");
}
