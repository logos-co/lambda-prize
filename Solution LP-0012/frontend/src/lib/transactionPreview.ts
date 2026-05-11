import type { PrivacySettings, TransactionIntent, TransactionPreview } from "../types/privacy";
import { describeTransactionIntent, redactTxHash } from "./redaction";
import { PrivacyUiError, toAppErrorInfo } from "./errors";
import { assertPositiveDecimalString } from "./validation";

export function createTransactionIntent(params: {
  chainId: number;
  kind: TransactionIntent["kind"];
  sender?: string;
  recipient?: string;
  assetId?: string;
  amount?: string;
  memo?: string;
  privacyLevel: TransactionIntent["privacyLevel"];
}): TransactionIntent {
  if (params.amount) assertPositiveDecimalString(params.amount, "amount");

  return {
    id: crypto.randomUUID(),
    kind: params.kind,
    chainId: params.chainId,
    sender: params.sender as TransactionIntent["sender"],
    recipient: params.recipient as TransactionIntent["recipient"],
    assetId: params.assetId,
    amount: params.amount,
    memo: params.memo,
    privacyLevel: params.privacyLevel,
    createdAt: new Date().toISOString(),
    nonce: crypto.getRandomValues(new Uint32Array(4)).join("-"),
  };
}

export function buildTransactionPreview(
  intent: TransactionIntent,
  settings: PrivacySettings
): TransactionPreview {
  const warnings: string[] = [];
  const rawSummary = describeTransactionIntent(intent, settings);

  if (intent.privacyLevel === "public" && settings.localEncryptionEnabled) {
    warnings.push("This is a public transaction while local encryption is enabled.");
  }

  if (intent.kind === "message" && !settings.redactMessageBodies) {
    warnings.push("Message bodies are visible in the UI.");
  }

  if (intent.amount && Number(intent.amount) > 10_000) {
    warnings.push("High-value transfer detected.");
  }

  const gasLimit =
    intent.kind === "message" ? 80_000 : intent.kind === "transfer" ? 120_000 : 150_000;

  const estimatedFee = `${(gasLimit * 0.00000002).toFixed(6)} ETH`;
  const canSubmit =
    warnings.every((w) => !w.includes("High-value")) ||
    intent.privacyLevel !== "public";

  return {
    intent,
    estimatedFee,
    gasLimit,
    warnings,
    redactedSummary: `${intent.kind} • ${redactTxHash(intent.id, false)} • ${intent.privacyLevel}`,
    rawSummary,
    canSubmit,
  };
}

export function assertPreviewReady(preview: TransactionPreview): void {
  if (!preview.canSubmit) {
    throw new PrivacyUiError(
      toAppErrorInfo("PREVIEW_BLOCKED", "Transaction preview failed policy checks.", {
        hint: "Review privacy warnings before submitting.",
      })
    );
  }
}
