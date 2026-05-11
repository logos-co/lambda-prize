export type HexString = `0x${string}`;
export type Address = HexString;
export type TxHash = HexString;
export type ChainId = number;

export type PrivacyLevel = "public" | "private" | "confidential";

export type ConsentScope =
  | "identity"
  | "balances"
  | "transactions"
  | "messages"
  | "analytics"
  | "support";

export type ConsentValue = "allow" | "deny" | "ask";

export interface PrivacyConsentMatrix {
  identity: ConsentValue;
  balances: ConsentValue;
  transactions: ConsentValue;
  messages: ConsentValue;
  analytics: ConsentValue;
  support: ConsentValue;
}

export interface PrivacySettings {
  showRawAddresses: boolean;
  showRawAmounts: boolean;
  showTxHashes: boolean;
  showCounterpartyNames: boolean;
  redactMessageBodies: boolean;
  localEncryptionEnabled: boolean;
  autoLockMinutes: number;
  preferredPrivacyLevel: PrivacyLevel;
}

export interface WalletSession {
  connected: boolean;
  account?: Address;
  chainId?: ChainId;
  provider?: "injected" | "walletconnect" | "custodial" | "demo";
  lastConnectedAt?: string;
}

export interface ShieldedBalance {
  assetId: string;
  ownerCommitment: string;
  balanceCommitment: string;
  available: string;
  pending: string;
  shielded: boolean;
}

export interface MessageEnvelope {
  id: string;
  createdAt: string;
  from?: Address;
  to?: Address;
  subject?: string;
  ciphertext?: string;
  plaintextPreview?: string;
  privacyLevel: PrivacyLevel;
  tags: string[];
}

export interface TransactionIntent {
  id: string;
  kind: "transfer" | "message" | "stake" | "vote" | "custom";
  chainId: ChainId;
  sender?: Address;
  recipient?: Address;
  assetId?: string;
  amount?: string;
  memo?: string;
  privacyLevel: PrivacyLevel;
  createdAt: string;
  nonce: string;
}

export interface TransactionPreview {
  intent: TransactionIntent;
  estimatedFee: string;
  gasLimit: number;
  warnings: string[];
  redactedSummary: string;
  rawSummary: string;
  canSubmit: boolean;
}

export interface AuditEvent {
  id: string;
  timestamp: string;
  category: "consent" | "wallet" | "tx" | "message" | "storage" | "security";
  level: "info" | "warn" | "error";
  title: string;
  description: string;
  metadata?: Record<string, unknown>;
}

export interface SupportBundle {
  generatedAt: string;
  appVersion: string;
  chainId?: ChainId;
  wallet?: WalletSession;
  settings?: PrivacySettings;
  consent?: PrivacyConsentMatrix;
  recentEvents: AuditEvent[];
  notes: string[];
}

export interface SecureBlob<T = unknown> {
  version: number;
  createdAt: string;
  label: string;
  nonce: string;
  ciphertext: string;
  checksum: string;
  payloadType: string;
  data?: T;
}

export interface BlockchainRpcErrorShape {
  code: string;
  message: string;
  details?: unknown;
}

export interface BlockchainApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: BlockchainRpcErrorShape;
}

export interface ShieldedTransferRequest {
  chainId: ChainId;
  assetId: string;
  fromCommitment: string;
  toCommitment: string;
  amount: string;
  memo?: string;
  privacyLevel: PrivacyLevel;
}

export interface ShieldedTransferResponse {
  txHash: TxHash;
  nullifier: string;
  commitment: string;
  status: "submitted" | "confirmed" | "failed";
  explorerUrl?: string;
}

export interface NetworkHealth {
  chainId: ChainId;
  rpcHealthy: boolean;
  websocketHealthy: boolean;
  lastCheckedAt: string;
  latencyMs: number;
  note?: string;
}

export interface AppErrorInfo {
  code: string;
  message: string;
  hint?: string;
  retriable: boolean;
}

export interface NodeStatus {
  blockHeight: number;
  slotNumber: number;
  epochNumber: number;
  validatorCount: number;
  activeValidators: number;
  networkParticipation: number;
  pendingTxCount: number;
  finalisedTxCount: number;
  coverTrafficRate: number;
  rpcLatencyMs: number;
  isSynced: boolean;
}

export interface BlendPacketEvent {
  id: string;
  timestamp: string;
  hopCount: number;
  isReal: boolean;
  latencyMs: number;
}

export interface LotterySlot {
  slot: number;
  epoch: number;
  vrfOutput: string;
  difficulty: number;
  won: boolean;
  proposalId?: string;
}

export interface StakePosition {
  id: string;
  commitment: string;
  nullifier: string;
  amount: string;
  assetId: string;
  status: "active" | "pending" | "withdrawn";
  createdAt: string;
  privacyLevel: PrivacyLevel;
}
