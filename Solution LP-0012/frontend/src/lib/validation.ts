import { PrivacyUiError, toAppErrorInfo } from "./errors";
import type { Address, HexString, TxHash } from "../types/privacy";

const HEX_RE = /^0x[0-9a-fA-F]+$/;
const ADDRESS_RE = /^0x[0-9a-fA-F]{40}$/;
const TX_HASH_RE = /^0x[0-9a-fA-F]{64}$/;

export function assertHexString(value: string, name = "value"): asserts value is HexString {
  if (!HEX_RE.test(value)) {
    throw new PrivacyUiError(
      toAppErrorInfo("INVALID_HEX", `${name} must be a 0x-prefixed hex string.`)
    );
  }
}

export function assertAddress(value: string): asserts value is Address {
  if (!ADDRESS_RE.test(value)) {
    throw new PrivacyUiError(
      toAppErrorInfo("INVALID_ADDRESS", "Address must be a 20-byte 0x-prefixed hex string.")
    );
  }
}

export function assertTxHash(value: string): asserts value is TxHash {
  if (!TX_HASH_RE.test(value)) {
    throw new PrivacyUiError(
      toAppErrorInfo("INVALID_TX_HASH", "Transaction hash must be a 32-byte 0x-prefixed hex string.")
    );
  }
}

export function assertPositiveDecimalString(value: string, field = "amount"): void {
  if (!/^\d+(\.\d+)?$/.test(value) || Number(value) <= 0) {
    throw new PrivacyUiError(
      toAppErrorInfo("INVALID_AMOUNT", `${field} must be a positive decimal string.`)
    );
  }
}

export function sanitizeLabel(value: string, maxLen = 80): string {
  const trimmed = value.trim();
  if (!trimmed) return "";
  return trimmed.slice(0, maxLen);
}

export function normalizeHex(value: string): HexString {
  const lower = value.trim().toLowerCase();
  assertHexString(lower);
  return lower as HexString;
}

export function safeJsonParse<T>(raw: string, fallback: T): T {
  try {
    return JSON.parse(raw) as T;
  } catch {
    return fallback;
  }
}
