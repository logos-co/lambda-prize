import type { AppErrorInfo, BlockchainRpcErrorShape } from "../types/privacy";

export class PrivacyUiError extends Error {
  public readonly code: string;
  public readonly retriable: boolean;
  public readonly hint?: string;

  constructor(info: AppErrorInfo, cause?: unknown) {
    super(info.message);
    this.name = "PrivacyUiError";
    this.code = info.code;
    this.retriable = info.retriable;
    this.hint = info.hint;
    if (cause !== undefined) {
      (this as { cause?: unknown }).cause = cause;
    }
  }
}

export function isBlockchainRpcErrorShape(value: unknown): value is BlockchainRpcErrorShape {
  return (
    typeof value === "object" &&
    value !== null &&
    "code" in value &&
    "message" in value &&
    typeof (value as { code: unknown }).code === "string" &&
    typeof (value as { message: unknown }).message === "string"
  );
}

export function toAppErrorInfo(
  code: string,
  message: string,
  options?: { hint?: string; retriable?: boolean }
): AppErrorInfo {
  return {
    code,
    message,
    hint: options?.hint,
    retriable: options?.retriable ?? false,
  };
}

export function normalizeUnknownError(err: unknown): AppErrorInfo {
  if (err instanceof PrivacyUiError) {
    return {
      code: err.code,
      message: err.message,
      hint: err.hint,
      retriable: err.retriable,
    };
  }

  if (err instanceof Error) {
    return toAppErrorInfo("UNKNOWN_ERROR", err.message, { retriable: false });
  }

  return toAppErrorInfo("UNKNOWN_ERROR", "An unknown error occurred.", { retriable: false });
}
