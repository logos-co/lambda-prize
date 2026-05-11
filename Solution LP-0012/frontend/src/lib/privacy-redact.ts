import type { PrivacySettings } from "./privacy-types";

export function redactAddress(address?: string, showRaw = false): string {
  if (!address) return "—";
  return showRaw ? address : `${address.slice(0, 6)}…${address.slice(-4)}`;
}

export function redactTxHash(txHash?: string, showRaw = false): string {
  if (!txHash) return "—";
  return showRaw ? txHash : `${txHash.slice(0, 8)}…${txHash.slice(-6)}`;
}

export function redactAmount(amount?: string, showRaw = false): string {
  if (!amount) return "—";
  if (showRaw) return amount;
  const [whole] = amount.split(".");
  return `${whole.slice(0, 2)}…`;
}

export function redactMessage(message?: string, showRaw = false): string {
  if (!message) return "—";
  if (showRaw) return message;
  return `Encrypted message (${message.length} chars hidden)`;
}

export function redactWalletProvider(provider?: string, showRaw = false): string {
  if (!provider) return "—";
  if (showRaw) return provider;
  return provider.length > 8 ? `${provider.slice(0, 8)}…` : provider;
}
