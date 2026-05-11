import type {
  Address,
  PrivacySettings,
  ShieldedBalance,
  TransactionIntent,
  MessageEnvelope,
} from "../types/privacy";
import { sanitizeLabel } from "./validation";

function keepStart(input: string, visible = 8): string {
  if (input.length <= visible) return input;
  return `${input.slice(0, visible)}…`;
}

export function redactAddress(address?: Address, showRaw = false): string {
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

export function redactMemo(memo?: string, showRaw = false): string {
  if (!memo) return "—";
  const clean = sanitizeLabel(memo, 160);
  if (showRaw) return clean;
  return clean.length > 24 ? `${clean.slice(0, 24)}…` : clean;
}

export function redactMessageBody(body?: string, showRaw = false): string {
  if (!body) return "—";
  if (showRaw) return body;
  return `Encrypted message (${body.length} chars hidden)`;
}

export function formatShieldedBalance(
  balance: ShieldedBalance,
  settings: PrivacySettings
): string {
  const available = settings.showRawAmounts
    ? balance.available
    : redactAmount(balance.available);
  const pending = settings.showRawAmounts
    ? balance.pending
    : redactAmount(balance.pending);
  const asset = settings.showCounterpartyNames
    ? balance.assetId
    : keepStart(balance.assetId, 10);
  return `${asset} • available ${available} • pending ${pending}`;
}

export function describeTransactionIntent(
  intent: TransactionIntent,
  settings: PrivacySettings
): string {
  const sender = settings.showRawAddresses
    ? intent.sender ?? "—"
    : redactAddress(intent.sender);
  const recipient = settings.showRawAddresses
    ? intent.recipient ?? "—"
    : redactAddress(intent.recipient);
  const amount = settings.showRawAmounts
    ? intent.amount ?? "—"
    : redactAmount(intent.amount);
  const memo = settings.showCounterpartyNames
    ? intent.memo ?? "—"
    : redactMemo(intent.memo);

  switch (intent.kind) {
    case "transfer":
      return `Transfer ${amount} from ${sender} to ${recipient} ${
        memo !== "—" ? `(${memo})` : ""
      }`.trim();
    case "message":
      return `Send private message to ${recipient}`;
    case "stake":
      return `Stake ${amount} from ${sender}`;
    case "vote":
      return `Cast vote from ${sender}`;
    default:
      return `Custom transaction from ${sender}`;
  }
}

export function redactMessageEnvelope(
  message: MessageEnvelope,
  settings: PrivacySettings
): MessageEnvelope {
  return {
    ...message,
    from: settings.showRawAddresses
      ? message.from
      : message.from
      ? (redactAddress(message.from) as Address)
      : undefined,
    to: settings.showRawAddresses
      ? message.to
      : message.to
      ? (redactAddress(message.to) as Address)
      : undefined,
    subject: settings.showCounterpartyNames
      ? message.subject
      : message.subject
      ? sanitizeLabel(message.subject, 20)
      : undefined,
    plaintextPreview: settings.redactMessageBodies
      ? message.ciphertext
        ? `Encrypted payload (${message.ciphertext.length} chars)`
        : undefined
      : message.plaintextPreview,
  };
}

export function privacySummary(settings: PrivacySettings): string {
  return [
    settings.localEncryptionEnabled
      ? "local encryption enabled"
      : "local encryption disabled",
    settings.showRawAddresses ? "raw addresses visible" : "addresses redacted",
    settings.showRawAmounts ? "raw amounts visible" : "amounts redacted",
    settings.redactMessageBodies
      ? "message bodies redacted"
      : "message bodies may preview",
  ].join(" • ");
}
