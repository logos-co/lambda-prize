export type PrivacyScope =
  | "identity"
  | "wallet"
  | "balances"
  | "transactions"
  | "messages"
  | "analytics"
  | "support"
  | "sharing";

export type ConsentValue = "allow" | "deny" | "ask";

export interface ConsentMatrix {
  identity: ConsentValue;
  wallet: ConsentValue;
  balances: ConsentValue;
  transactions: ConsentValue;
  messages: ConsentValue;
  analytics: ConsentValue;
  support: ConsentValue;
  sharing: ConsentValue;
}

export interface PrivacySettings {
  showRawAddresses: boolean;
  showRawAmounts: boolean;
  showTxHashes: boolean;
  showMessageBodies: boolean;
  showWalletProvider: boolean;
  redactInSearch: boolean;
  localEncryptionEnabled: boolean;
  autoLockMinutes: number;
  preferredLevel: "public" | "private" | "confidential";
}

export interface AuditEvent {
  id: string;
  time: string;
  category:
    | "consent"
    | "security"
    | "storage"
    | "share"
    | "message"
    | "wallet"
    | "settings";
  level: "info" | "warn" | "error";
  title: string;
  summary: string;
  details?: string;
  tags: string[];
}

export interface VaultNote {
  id: string;
  title: string;
  body: string;
  createdAt: string;
  updatedAt: string;
  encrypted?: boolean;
  checksum?: string;
}

export interface SecureBlob<T = unknown> {
  version: number;
  label: string;
  createdAt: string;
  nonce: string;
  ciphertext: string;
  checksum: string;
  payloadType: string;
  data?: T;
}

export interface DataCategory {
  name: string;
  purpose: string;
  storedLocally: boolean;
  sharedExternally: boolean;
  retention: string;
  sensitive: boolean;
}

export interface EventRow {
  id: string;
  type: "success" | "warning" | "info" | "error";
  title: string;
  summary: string;
  details: string;
  privacyLevel: "public" | "private" | "confidential";
  createdAt: string;
  tags: string[];
}

export interface SectionBlock {
  title: string;
  body: string;
}
