import { PrivacyUiError, toAppErrorInfo } from "./errors";
import type {
  BlockchainApiResponse,
  NetworkHealth,
  ShieldedBalance,
  ShieldedTransferRequest,
  ShieldedTransferResponse,
  TxHash,
  WalletSession,
} from "../types/privacy";
import { assertAddress, assertHexString, assertTxHash } from "./validation";

export interface BlockchainClientOptions {
  baseUrl: string;
  chainId: number;
  timeoutMs?: number;
  fetchImpl?: typeof fetch;
}

export class PrivacyBlockchainClient {
  private readonly baseUrl: string;
  private readonly chainId: number;
  private readonly timeoutMs: number;
  private readonly fetchImpl: typeof fetch;

  constructor(options: BlockchainClientOptions) {
    this.baseUrl = options.baseUrl.replace(/\/+$/, "");
    this.chainId = options.chainId;
    this.timeoutMs = options.timeoutMs ?? 15_000;
    this.fetchImpl = options.fetchImpl ?? fetch;
  }

  private async request<T>(path: string, init: RequestInit = {}): Promise<T> {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), this.timeoutMs);

    try {
      const res = await this.fetchImpl(`${this.baseUrl}${path}`, {
        ...init,
        signal: controller.signal,
        headers: {
          "content-type": "application/json",
          ...(init.headers ?? {}),
        },
      });

      const json = (await res.json().catch(() => null)) as BlockchainApiResponse<T> | null;

      if (!res.ok) {
        const details = json?.error?.details;
        const msg = json?.error?.message ?? `HTTP ${res.status}`;
        throw new PrivacyUiError(
          toAppErrorInfo("BLOCKCHAIN_REQUEST_FAILED", msg, {
            retriable: res.status >= 500,
            hint: typeof details === "string" ? details : undefined,
          })
        );
      }

      if (json && json.ok && json.data !== undefined) return json.data;
      if (json && !json.ok && json.error) {
        throw new PrivacyUiError(
          toAppErrorInfo(json.error.code, json.error.message, { retriable: false })
        );
      }

      throw new PrivacyUiError(
        toAppErrorInfo("INVALID_RESPONSE", "Blockchain API returned an invalid response.")
      );
    } catch (cause) {
      if (cause instanceof PrivacyUiError) throw cause;
      throw new PrivacyUiError(
        toAppErrorInfo("NETWORK_ERROR", "Unable to reach blockchain backend.", {
          retriable: true,
          hint: "Check RPC connectivity and retry.",
        }),
        cause
      );
    } finally {
      clearTimeout(timeout);
    }
  }

  async getWalletSession(): Promise<WalletSession> {
    return this.request<WalletSession>(`/wallet/session?chainId=${this.chainId}`);
  }

  async getShieldedBalances(account: string): Promise<ShieldedBalance[]> {
    assertAddress(account);
    return this.request<ShieldedBalance[]>(
      `/shielded/balances?chainId=${this.chainId}&account=${encodeURIComponent(account)}`
    );
  }

  async getNetworkHealth(): Promise<NetworkHealth> {
    return this.request<NetworkHealth>(`/health?chainId=${this.chainId}`);
  }

  async submitPrivateTransfer(
    req: ShieldedTransferRequest
  ): Promise<ShieldedTransferResponse> {
    if (req.chainId !== this.chainId) {
      throw new PrivacyUiError(
        toAppErrorInfo("CHAIN_MISMATCH", "The selected chain does not match the backend chain.")
      );
    }
    assertHexString(req.fromCommitment, "fromCommitment");
    assertHexString(req.toCommitment, "toCommitment");

    return this.request<ShieldedTransferResponse>(`/shielded/transfer`, {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  async getAuditEvents(
    limit = 100
  ): Promise<Array<{ id: string; timestamp: string; title: string }>> {
    return this.request(`/audit/events?chainId=${this.chainId}&limit=${limit}`);
  }

  async getTxReceipt(txHash: TxHash): Promise<unknown> {
    assertTxHash(txHash);
    return this.request(`/tx/${txHash}/receipt?chainId=${this.chainId}`);
  }
}
