The repo’s README describes a LEZ event system with failure-path persistence, deterministic encoding, strict limits, and a privacy-first design direction, so the code below tur(GitHub)ed privacy-oriented frontend layer with local encryption, consent controls, redaction, audit trails, and 15+ responsive pages. (GitHub)json id="frontend/package.json"
{
"name": "logos-privacy-frontend",
"private": true,
"version": "1.0.0",
"type": "module",
"scripts": {
"dev": "next dev",
"build": "next build",
"start": "next start",
"lint": "next lint"
},
"dependencies": {
"clsx": "^2.1.1",
"framer-motion": "^11.11.9",
"lucide-react": "^0.452.0",
"next": "^15.0.3",
"react": "^18.3.1",
"react-dom": "^18.3.1"
},
"devDependencies": {
"@types/node": "^22.9.0",
"@types/react": "^18.3.12",
"@types/react-dom": "^18.3.1",
"autoprefixer": "^10.4.20",
"postcss": "^8.4.49",
"tailwindcss": "^3.4.15",
"typescript": "^5.6.3"
}
}

```json id="frontend/tsconfig.json"
{
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["dom", "dom.iterable", "es2022"],
    "allowJs": false,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "baseUrl": ".",
    "paths": {
      "@/components/*": ["components/*"],
      "@/hooks/*": ["hooks/*"],
      "@/lib/*": ["lib/*"],
      "@/lib/privacy/*": ["lib/privacy/*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx"],
  "exclude": ["node_modules"]
}

/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    typedRoutes: true
  }
};

export default nextConfig;

export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {}
  }
};

import type { Config } from "tailwindcss";

export default {
  content: ["./app/**/*.{ts,tsx}", "./components/**/*.{ts,tsx}", "./hooks/**/*.{ts,tsx}", "./lib/**/*.{ts,tsx}"],
  theme: {
    extend: {
      boxShadow: {
        glow: "0 0 80px rgba(56, 189, 248, 0.18)"
      },
      backgroundImage: {
        "radial-glow":
          "radial-gradient(circle at top, rgba(56,189,248,0.16), transparent 35%), radial-gradient(circle at 80% 20%, rgba(168,85,247,0.16), transparent 30%), radial-gradient(circle at 20% 20%, rgba(251,191,36,0.08), transparent 28%)"
      }
    }
  },
  plugins: []
} satisfies Config;

@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  color-scheme: dark;
}

html {
  scroll-behavior: smooth;
}

body {
  margin: 0;
  min-height: 100vh;
  background: #020617;
  color: white;
  font-family:
    Inter,
    ui-sans-serif,
    system-ui,
    -apple-system,
    BlinkMacSystemFont,
    "Segoe UI",
    sans-serif;
}

* {
  box-sizing: border-box;
}

::selection {
  background: rgba(103, 232, 249, 0.34);
  color: white;
}

a {
  color: inherit;
  text-decoration: none;
}

button,
input,
textarea,
select {
  font: inherit;
}

code,
pre {
  font-family:
    ui-monospace,
    SFMono-Regular,
    Menlo,
    Monaco,
    Consolas,
    "Liberation Mono",
    monospace;
}

import type React from "react";

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
  category: "consent" | "security" | "storage" | "share" | "message" | "wallet" | "settings";
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

export interface SearchItem {
  href: string;
  label: string;
  description: string;
}

export interface AppToast {
  id: string;
  tone: "success" | "info" | "warning" | "error";
  title: string;
  message: string;
}

export interface PrivacyMetric {
  label: string;
  value: string;
}

export interface PageAction {
  href: string;
  label: string;
}

export interface SectionBlock {
  title: string;
  body: string;
  icon?: React.ReactNode;
}

import type { ConsentMatrix, DataCategory, EventRow, PrivacySettings, PrivacyMetric, SearchItem } from "./types";

export const DEFAULT_CONSENT: ConsentMatrix = {
  identity: "ask",
  wallet: "ask",
  balances: "ask",
  transactions: "ask",
  messages: "deny",
  analytics: "deny",
  support: "allow",
  sharing: "ask"
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
  preferredLevel: "private"
};

export const privacyMetrics: PrivacyMetric[] = [
  { label: "Redaction mode", value: "Default on" },
  { label: "Local encryption", value: "Enabled" },
  { label: "Consent default", value: "Ask first" },
  { label: "Sharing mode", value: "Progressive" }
];

export const dataCategories: DataCategory[] = [
  {
    name: "Identity hints",
    purpose: "Show the minimum needed to orient users in the UI.",
    storedLocally: true,
    sharedExternally: false,
    retention: "Until cleared by the user",
    sensitive: true
  },
  {
    name: "Wallet session",
    purpose: "Keep the browser session responsive without re-authentication loops.",
    storedLocally: true,
    sharedExternally: false,
    retention: "Session only",
    sensitive: true
  },
  {
    name: "Transaction summaries",
    purpose: "Let users review activity without exposing full payloads.",
    storedLocally: true,
    sharedExternally: false,
    retention: "User-controlled",
    sensitive: true
  },
  {
    name: "Support bundles",
    purpose: "Create compact, redacted reports for troubleshooting.",
    storedLocally: true,
    sharedExternally: true,
    retention: "Exported on demand",
    sensitive: false
  }
];

export const eventFeed: EventRow[] = [
  {
    id: "evt_001",
    type: "success",
    title: "Encrypted note saved",
    summary: "A note was written to local secure storage.",
    details: "The note was sealed using the current browser key and stored with a checksum.",
    privacyLevel: "private",
    createdAt: "02:14 UTC",
    tags: ["vault", "local", "encrypted"]
  },
  {
    id: "evt_002",
    type: "info",
    title: "Consent preference updated",
    summary: "Sharing policy for messages was tightened.",
    details: "Messages remain hidden unless the user explicitly reveals them.",
    privacyLevel: "confidential",
    createdAt: "02:18 UTC",
    tags: ["consent", "messages", "policy"]
  },
  {
    id: "evt_003",
    type: "warning",
    title: "High-sensitivity action blocked",
    summary: "An unsafe share was prevented by the UI.",
    details: "The policy layer required a stronger disclosure review before export.",
    privacyLevel: "private",
    createdAt: "02:20 UTC",
    tags: ["share", "review", "blocked"]
  },
  {
    id: "evt_004",
    type: "info",
    title: "Redaction preview shown",
    summary: "A summary view replaced raw details in the event browser.",
    details: "The user can expand specific fields only when needed.",
    privacyLevel: "public",
    createdAt: "02:25 UTC",
    tags: ["redaction", "preview", "ui"]
  }
];

export const searchItems: SearchItem[] = [
  { href: "/", label: "Home", description: "Overview of privacy controls and trust cues." },
  { href: "/privacy", label: "Privacy center", description: "Manage privacy defaults and disclosures." },
  { href: "/consent", label: "Consent", description: "Scope-based permissions and defaults." },
  { href: "/vault", label: "Vault", description: "Encrypted notes and local secrets." },
  { href: "/redaction", label: "Redaction", description: "How the UI hides sensitive values." },
  { href: "/sharing", label: "Sharing", description: "Safe export and progressive disclosure." },
  { href: "/audit", label: "Audit", description: "A readable privacy trail." },
  { href: "/events", label: "Events", description: "Filterable privacy-sensitive event feed." },
  { href: "/dashboard", label: "Dashboard", description: "Status and shortcuts." },
  { href: "/settings", label: "Settings", description: "Density, motion, and privacy preferences." },
  { href: "/security", label: "Security", description: "Trust boundaries and safe handling." },
  { href: "/accessibility", label: "Accessibility", description: "Keyboard and contrast guidance." },
  { href: "/research", label: "Research", description: "Open privacy questions and future work." },
  { href: "/support", label: "Support", description: "Troubleshooting and safe reports." },
  { href: "/data-map", label: "Data map", description: "What data is collected and why." },
  { href: "/encryption", label: "Encryption", description: "Local key derivation and sealing." }
];

import type { SecureBlob } from "./types";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

function toBase64(bytes: Uint8Array): string {
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary);
}

function fromBase64(input: string): Uint8Array {
  const binary = atob(input);
  const out = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) out[i] = binary.charCodeAt(i);
  return out;
}

export async function sha256Hex(data: string | ArrayBuffer | Uint8Array): Promise<string> {
  const bytes = typeof data === "string" ? encoder.encode(data) : data instanceof ArrayBuffer ? new Uint8Array(data) : data;
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return Array.from(new Uint8Array(digest), (b) => b.toString(16).padStart(2, "0")).join("");
}

export function nonce(): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(12));
}

export async function deriveKeyFromPassphrase(passphrase: string, salt: string, iterations = 210_000): Promise<CryptoKey> {
  const pass = passphrase.trim();
  if (!pass) throw new Error("Passphrase is required");

  const baseKey = await crypto.subtle.importKey("raw", encoder.encode(pass), { name: "PBKDF2" }, false, ["deriveKey"]);
  return crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt: encoder.encode(salt),
      iterations,
      hash: "SHA-256"
    },
    baseKey,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt", "decrypt"]
  );
}

export async function encryptJson<T>(payload: T, key: CryptoKey, label: string): Promise<SecureBlob<T>> {
  const plaintext = encoder.encode(JSON.stringify(payload));
  const iv = nonce();
  const ciphertext = await crypto.subtle.encrypt({ name: "AES-GCM", iv }, key, plaintext);
  const bytes = new Uint8Array(ciphertext);

  return {
    version: 1,
    createdAt: new Date().toISOString(),
    label,
    nonce: toBase64(iv),
    ciphertext: toBase64(bytes),
    checksum: await sha256Hex(bytes),
    payloadType: typeof payload
  };
}

export async function decryptJson<T>(blob: SecureBlob<T>, key: CryptoKey): Promise<T> {
  const iv = fromBase64(blob.nonce);
  const ciphertext = fromBase64(blob.ciphertext);
  const plaintext = await crypto.subtle.decrypt({ name: "AES-GCM", iv }, key, ciphertext);
  return JSON.parse(decoder.decode(plaintext)) as T;
}

export async function sealText(text: string, key: CryptoKey, label: string): Promise<SecureBlob<{ text: string }>> {
  return encryptJson({ text }, key, label);
}

export async function openText(blob: SecureBlob<{ text: string }>, key: CryptoKey): Promise<string> {
  const payload = await decryptJson(blob, key);
  return payload.text;
}

export function randomId(prefix = "id"): string {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  return `${prefix}_${toBase64(bytes).replace(/[^a-zA-Z0-9]/g, "").slice(0, 22)}`;
}

import type { SecureBlob, VaultNote } from "./types";

export interface StorageAdapter {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
  keys(): string[];
}

class MemoryStorageAdapter implements StorageAdapter {
  private readonly store = new Map<string, string>();

  getItem(key: string): string | null {
    return this.store.get(key) ?? null;
  }

  setItem(key: string, value: string): void {
    this.store.set(key, value);
  }

  removeItem(key: string): void {
    this.store.delete(key);
  }

  keys(): string[] {
    return [...this.store.keys()];
  }
}

class BrowserStorageAdapter implements StorageAdapter {
  constructor(private readonly storage: Storage) {}

  getItem(key: string): string | null {
    return this.storage.getItem(key);
  }

  setItem(key: string, value: string): void {
    this.storage.setItem(key, value);
  }

  removeItem(key: string): void {
    this.storage.removeItem(key);
  }

  keys(): string[] {
    return Object.keys(this.storage);
  }
}

export function createStorageAdapter(): StorageAdapter {
  if (typeof window === "undefined" || typeof window.localStorage === "undefined") return new MemoryStorageAdapter();
  try {
    const probe = "__privacy_probe__";
    window.localStorage.setItem(probe, "1");
    window.localStorage.removeItem(probe);
    return new BrowserStorageAdapter(window.localStorage);
  } catch {
    return new MemoryStorageAdapter();
  }
}

export function createSessionStorageAdapter(): StorageAdapter {
  if (typeof window === "undefined" || typeof window.sessionStorage === "undefined") return new MemoryStorageAdapter();
  try {
    const probe = "__privacy_probe__";
    window.sessionStorage.setItem(probe, "1");
    window.sessionStorage.removeItem(probe);
    return new BrowserStorageAdapter(window.sessionStorage);
  } catch {
    return new MemoryStorageAdapter();
  }
}

export function createJsonStore(adapter: StorageAdapter, namespace: string) {
  const prefix = `${namespace}::`;
  return {
    get<T>(key: string, fallback: T): T {
      const raw = adapter.getItem(prefix + key);
      if (!raw) return fallback;
      try {
        return JSON.parse(raw) as T;
      } catch {
        return fallback;
      }
    },
    set<T>(key: string, value: T): void {
      adapter.setItem(prefix + key, JSON.stringify(value));
    },
    remove(key: string): void {
      adapter.removeItem(prefix + key);
    },
    listKeys(): string[] {
      return adapter.keys().filter((k) => k.startsWith(prefix)).map((k) => k.slice(prefix.length));
    }
  };
}

export function loadVaultNotes(store: ReturnType<typeof createJsonStore>): VaultNote[] {
  return store.get<VaultNote[]>("vault-notes", []);
}

export function saveVaultNotes(store: ReturnType<typeof createJsonStore>, notes: VaultNote[]): void {
  store.set("vault-notes", notes);
}

export function loadSecureBlobMap(store: ReturnType<typeof createJsonStore>): Record<string, SecureBlob> {
  return store.get<Record<string, SecureBlob>>("blobs", {});
}

export function saveSecureBlobMap(store: ReturnType<typeof createJsonStore>, blobs: Record<string, SecureBlob>): void {
  store.set("blobs", blobs);
}

import type { EventRow, PrivacySettings, VaultNote } from "./types";

function keepEnd(input: string, visible = 6): string {
  if (input.length <= visible) return input;
  return `…${input.slice(-visible)}`;
}

function keepStart(input: string, visible = 8): string {
  if (input.length <= visible) return input;
  return `${input.slice(0, visible)}…`;
}

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
  return showRaw ? provider : keepStart(provider, 8);
}

export function redactVaultNote(note: VaultNote, showRaw = false): VaultNote {
  return {
    ...note,
    title: showRaw ? note.title : keepStart(note.title, 18),
    body: showRaw ? note.body : `Encrypted note (${note.body.length} chars hidden)`
  };
}

export function redactEvent(event: EventRow, settings: PrivacySettings): EventRow {
  return {
    ...event,
    title: settings.redactInSearch ? keepStart(event.title, 20) : event.title,
    summary: settings.redactInSearch ? keepStart(event.summary, 48) : event.summary,
    details: settings.redactInSearch ? `Hidden detail (${event.details.length} chars)` : event.details
  };
}

export function privacySummary(settings: PrivacySettings): string {
  return [
    settings.localEncryptionEnabled ? "local encryption enabled" : "local encryption disabled",
    settings.showRawAddresses ? "raw addresses visible" : "addresses redacted",
    settings.showRawAmounts ? "raw amounts visible" : "amounts redacted",
    settings.showMessageBodies ? "message bodies visible" : "message bodies redacted"
  ].join(" • ");
}

import { randomId } from "./crypto";
import type { AuditEvent } from "./types";

export function createAuditEvent(
  category: AuditEvent["category"],
  title: string,
  summary: string,
  details?: string,
  level: AuditEvent["level"] = "info",
  tags: string[] = []
): AuditEvent {
  return {
    id: randomId("audit"),
    time: new Date().toISOString(),
    category,
    level,
    title,
    summary,
    details,
    tags
  };
}

export function formatAuditTime(value: string): string {
  try {
    return new Date(value).toLocaleString();
  } catch {
    return value;
  }
}

import type { EventRow, VaultNote } from "./types";

export const demoVaultNotes: VaultNote[] = [
  {
    id: "note_001",
    title: "Recovery phrase checklist",
    body: "Do not paste seed phrases into the UI. Keep them offline and never export them.",
    createdAt: "2026-05-01T09:00:00.000Z",
    updatedAt: "2026-05-01T09:10:00.000Z",
    encrypted: true,
    checksum: "8f1a2b"
  },
  {
    id: "note_002",
    title: "Support escalation",
    body: "When filing support, share the event id, approximate time, and the page route only.",
    createdAt: "2026-05-01T10:30:00.000Z",
    updatedAt: "2026-05-01T10:35:00.000Z",
    encrypted: true,
    checksum: "f17c90"
  }
];

export const demoEventRows: EventRow[] = [
  {
    id: "evt_101",
    type: "success",
    title: "Private note sealed",
    summary: "A note was saved using the browser key.",
    details: "The note content never had to leave the browser in plaintext.",
    privacyLevel: "private",
    createdAt: "03:12 UTC",
    tags: ["vault", "sealed", "local"]
  },
  {
    id: "evt_102",
    type: "warning",
    title: "Unsafe share blocked",
    summary: "A broad export was stopped before leaving the page.",
    details: "The policy demanded a narrower export scope and a manual review.",
    privacyLevel: "confidential",
    createdAt: "03:15 UTC",
    tags: ["share", "policy", "blocked"]
  },
  {
    id: "evt_103",
    type: "info",
    title: "Redaction preview active",
    summary: "Sensitive fields are summarized in this view.",
    details: "Use the reveal action only when you really need the raw value.",
    privacyLevel: "public",
    createdAt: "03:18 UTC",
    tags: ["redaction", "preview", "safe-default"]
  },
  {
    id: "evt_104",
    type: "error",
    title: "Export fingerprint mismatch",
    summary: "The exported bundle checksum did not match the expected fingerprint.",
    details: "The UI recommended re-exporting the bundle from a trusted session.",
    privacyLevel: "private",
    createdAt: "03:24 UTC",
    tags: ["export", "fingerprint", "integrity"]
  }
];

import { useEffect, useMemo, useState } from "react";
import { createJsonStore, createStorageAdapter } from "@/lib/privacy/storage";

export function usePersistentState<T>(namespace: string, key: string, fallback: T): [T, React.Dispatch<React.SetStateAction<T>>] {
  const store = useMemo(() => createJsonStore(createStorageAdapter(), namespace), [namespace]);
  const [value, setValue] = useState<T>(() => store.get(key, fallback));

  useEffect(() => {
    store.set(key, value);
  }, [key, store, value]);

  return [value, setValue];
}

import { useMemo } from "react";
import { usePersistentState } from "./usePersistentState";
import { DEFAULT_SETTINGS } from "@/lib/privacy/constants";
import type { PrivacySettings } from "@/lib/privacy/types";

export function usePrivacySettings() {
  const [settings, setSettings] = usePersistentState<PrivacySettings>("privacy", "settings", DEFAULT_SETTINGS);

  return useMemo(() => {
    return {
      settings,
      setSettings,
      patch(partial: Partial<PrivacySettings>) {
        setSettings((prev) => ({ ...prev, ...partial }));
      },
      reset() {
        setSettings(DEFAULT_SETTINGS);
      }
    };
  }, [settings, setSettings]);
}

import { useMemo } from "react";
import { usePersistentState } from "./usePersistentState";
import { DEFAULT_CONSENT } from "@/lib/privacy/constants";
import type { ConsentMatrix, ConsentValue, PrivacyScope } from "@/lib/privacy/types";

export function useConsentMatrix() {
  const [consent, setConsent] = usePersistentState<ConsentMatrix>("privacy", "consent", DEFAULT_CONSENT);

  return useMemo(() => {
    return {
      consent,
      setConsent,
      set(scope: PrivacyScope, value: ConsentValue) {
        setConsent((prev) => ({ ...prev, [scope]: value }));
      },
      reset() {
        setConsent(DEFAULT_CONSENT);
      }
    };
  }, [consent, setConsent]);
}

"use client";

import { useMemo, useState } from "react";
import { createJsonStore, createStorageAdapter, loadVaultNotes, saveVaultNotes } from "@/lib/privacy/storage";
import { demoVaultNotes } from "@/lib/privacy/mock";
import { randomId, sha256Hex, encryptJson, decryptJson } from "@/lib/privacy/crypto";
import type { VaultNote, SecureBlob } from "@/lib/privacy/types";

const store = createJsonStore(createStorageAdapter(), "privacy");

export function useVault() {
  const [notes, setNotes] = useState<VaultNote[]>(() => {
    const saved = loadVaultNotes(store);
    return saved.length ? saved : demoVaultNotes;
  });
  const [busy, setBusy] = useState(false);
  const [passphrase, setPassphrase] = useState("");
  const [key, setKey] = useState<CryptoKey | null>(null);

  const api = useMemo(() => {
    return {
      notes,
      busy,
      passphrase,
      key,
      setPassphrase,
      async unlock() {
        setBusy(true);
        try {
          const k = await import("@/lib/privacy/crypto").then((m) => m.deriveKeyFromPassphrase(passphrase, "logos-privacy-salt"));
          setKey(k);
          return true;
        } finally {
          setBusy(false);
        }
      },
      lock() {
        setKey(null);
      },
      async saveNote(title: string, body: string) {
        const note: VaultNote = {
          id: randomId("note"),
          title,
          body,
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          encrypted: !!key
        };

        const next = [note, ...notes];
        setNotes(next);
        saveVaultNotes(store, next);
        return note;
      },
      async sealNote(note: VaultNote): Promise<SecureBlob<{ title: string; body: string }>> {
        if (!key) throw new Error("Unlock the vault first");
        const blob = await encryptJson({ title: note.title, body: note.body }, key, note.id);
        const updated = {
          ...note,
          encrypted: true,
          checksum: blob.checksum,
          updatedAt: new Date().toISOString()
        };
        const next = [updated, ...notes.filter((n) => n.id !== note.id)];
        setNotes(next);
        saveVaultNotes(store, next);
        return blob;
      },
      async openBlob(blob: SecureBlob<{ title: string; body: string }>) {
        if (!key) throw new Error("Unlock the vault first");
        return decryptJson(blob, key);
      },
      clear() {
        setNotes([]);
        saveVaultNotes(store, []);
      }
    };
  }, [busy, key, notes, passphrase]);

  return api;
}

import clsx from "clsx";

export function GlassCard({
  children,
  className = ""
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <div
      className={clsx(
        "rounded-3xl border border-white/10 bg-white/5 p-6 shadow-lg shadow-black/20 backdrop-blur-sm transition hover:bg-white/[0.07]",
        className
      )}
    >
      {children}
    </div>
  );
}

export function SectionHeading({
  eyebrow,
  title,
  description
}: {
  eyebrow: string;
  title: string;
  description: string;
}) {
  return (
    <div className="mx-auto max-w-3xl text-center">
      <p className="text-sm font-semibold uppercase tracking-[0.35em] text-slate-400">{eyebrow}</p>
      <h2 className="mt-4 text-3xl font-semibold tracking-tight text-white sm:text-4xl">{title}</h2>
      <p className="mt-4 text-base leading-7 text-slate-300 sm:text-lg">{description}</p>
    </div>
  );
}

import { ArrowRight } from "lucide-react";

export function PageHero({
  badge,
  title,
  description,
  primary,
  secondary
}: {
  badge: string;
  title: string;
  description: string;
  primary: { href: string; label: string };
  secondary?: { href: string; label: string };
}) {
  return (
    <section className="mx-auto max-w-7xl px-4 pb-14 pt-12 sm:px-6 lg:px-8 lg:pt-20">
      <div className="max-w-3xl">
        <div className="inline-flex items-center gap-2 rounded-full border border-cyan-300/20 bg-cyan-300/10 px-4 py-2 text-sm text-cyan-200">
          {badge}
        </div>

        <h1 className="mt-7 text-4xl font-semibold tracking-tight text-white sm:text-5xl lg:text-6xl">
          {title}
        </h1>

        <p className="mt-6 max-w-2xl text-lg leading-8 text-slate-300">{description}</p>

        <div className="mt-8 flex flex-col gap-3 sm:flex-row">
          <a
            href={primary.href}
            className="inline-flex items-center justify-center gap-2 rounded-full bg-cyan-300 px-6 py-3.5 font-semibold text-slate-950 transition hover:bg-cyan-200"
          >
            {primary.label}
            <ArrowRight className="h-4 w-4" />
          </a>
          {secondary ? (
            <a
              href={secondary.href}
              className="inline-flex items-center justify-center rounded-full border border-white/15 bg-white/5 px-6 py-3.5 font-semibold text-white transition hover:bg-white/10"
            >
              {secondary.label}
            </a>
          ) : null}
        </div>
      </div>
    </section>
  );
}

export function MetricGrid({
  items
}: {
  items: { label: string; value: string }[];
}) {
  return (
    <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {items.map((item) => (
        <div key={item.label} className="rounded-3xl border border-white/10 bg-white/5 p-5 backdrop-blur-sm">
          <p className="text-sm text-slate-400">{item.label}</p>
          <p className="mt-3 text-3xl font-semibold tracking-tight text-white">{item.value}</p>
        </div>
      ))}
    </div>
  );
}

"use client";

import { useMemo, useState } from "react";
import clsx from "clsx";

export function TabGroup({
  tabs
}: {
  tabs: { id: string; label: string; content: React.ReactNode }[];
}) {
  const [active, setActive] = useState(tabs[0]?.id ?? "");
  const activeTab = useMemo(() => tabs.find((t) => t.id === active) ?? tabs[0], [tabs, active]);

  return (
    <div className="rounded-[2rem] border border-white/10 bg-white/5 p-4">
      <div className="flex flex-wrap gap-2">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActive(tab.id)}
            className={clsx(
              "rounded-full px-4 py-2 text-sm transition",
              tab.id === active ? "bg-white text-slate-950" : "text-slate-300 hover:bg-white/10 hover:text-white"
            )}
          >
            {tab.label}
          </button>
        ))}
      </div>
      <div className="mt-4 rounded-3xl border border-white/10 bg-slate-950/60 p-5">{activeTab?.content}</div>
    </div>
  );
}

export function CodeBlock({
  title,
  code
}: {
  title: string;
  code: string;
}) {
  return (
    <section className="rounded-[2rem] border border-white/10 bg-slate-950/80 p-6">
      <div className="mb-4 flex items-center justify-between gap-3">
        <h3 className="text-lg font-semibold text-white">{title}</h3>
        <span className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-slate-300">
          Copy-ready
        </span>
      </div>
      <pre className="overflow-auto rounded-2xl bg-black/40 p-4 text-sm leading-7 text-slate-200">
        <code>{code}</code>
      </pre>
    </section>
  );
}

import Link from "next/link";
import { ChevronRight } from "lucide-react";

export function Breadcrumbs({
  items
}: {
  items: { label: string; href?: string }[];
}) {
  return (
    <nav aria-label="Breadcrumb" className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
      <ol className="flex flex-wrap items-center gap-2 text-sm text-slate-400">
        {items.map((item, index) => {
          const isLast = index === items.length - 1;
          return (
            <li key={`${item.label}-${index}`} className="flex items-center gap-2">
              {item.href && !isLast ? (
                <Link href={item.href} className="hover:text-white">
                  {item.label}
                </Link>
              ) : (
                <span className={isLast ? "text-white" : ""}>{item.label}</span>
              )}
              {!isLast ? <ChevronRight className="h-3.5 w-3.5" /> : null}
            </li>
          );
        })}
      </ol>
    </nav>
  );
}

export function StatPill({
  label,
  value
}: {
  label: string;
  value: string;
}) {
  return (
    <div className="inline-flex items-center gap-3 rounded-full border border-white/10 bg-white/5 px-4 py-2">
      <span className="text-xs uppercase tracking-[0.25em] text-slate-500">{label}</span>
      <span className="text-sm font-semibold text-white">{value}</span>
    </div>
  );
}

"use client";

import { AnimatePresence, motion } from "framer-motion";
import { CheckCircle2, Info, XCircle, AlertTriangle } from "lucide-react";
import clsx from "clsx";

export type ToastTone = "success" | "info" | "warning" | "error";

export type Toast = {
  id: string;
  title: string;
  message: string;
  tone: ToastTone;
};

export function ToastHost({
  items,
  onDismiss
}: {
  items: Toast[];
  onDismiss: (id: string) => void;
}) {
  const iconMap = {
    success: <CheckCircle2 className="h-4 w-4" />,
    info: <Info className="h-4 w-4" />,
    warning: <AlertTriangle className="h-4 w-4" />,
    error: <XCircle className="h-4 w-4" />
  };

  return (
    <div className="pointer-events-none fixed right-4 top-20 z-50 flex w-[min(92vw,360px)] flex-col gap-3">
      <AnimatePresence>
        {items.map((toast) => (
          <motion.div
            key={toast.id}
            initial={{ opacity: 0, y: 12, scale: 0.98 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: 12, scale: 0.98 }}
            className={clsx(
              "pointer-events-auto rounded-2xl border p-4 shadow-xl backdrop-blur",
              toast.tone === "success" && "border-emerald-400/20 bg-emerald-500/10",
              toast.tone === "info" && "border-cyan-400/20 bg-cyan-500/10",
              toast.tone === "warning" && "border-amber-400/20 bg-amber-500/10",
              toast.tone === "error" && "border-rose-400/20 bg-rose-500/10"
            )}
          >
            <div className="flex gap-3">
              <div className="mt-0.5 text-white">{iconMap[toast.tone]}</div>
              <div className="min-w-0 flex-1">
                <div className="text-sm font-semibold text-white">{toast.title}</div>
                <div className="mt-1 text-sm text-slate-200">{toast.message}</div>
              </div>
              <button onClick={() => onDismiss(toast.id)} className="text-slate-200 hover:text-white">
                ×
              </button>
            </div>
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}

"use client";

import { useMemo, useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { Search, X } from "lucide-react";
import Link from "next/link";
import { navItems, pages } from "@/lib/site";

export function SearchOverlay({
  open,
  onClose
}: {
  open: boolean;
  onClose: () => void;
}) {
  const [query, setQuery] = useState("");

  const results = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) {
      return [
        ...pages.slice(0, 8).map((page) => ({ href: page.href, label: page.title, description: page.description })),
        ...navItems.slice(0, 8).map((item) => ({ href: item.href, label: item.label, description: "Navigation page" }))
      ];
    }

    return [...pages, ...navItems]
      .filter((item: any) => {
        const label = (item.title ?? item.label ?? "").toLowerCase();
        const description = (item.description ?? "").toLowerCase();
        return label.includes(q) || description.includes(q) || item.href.includes(q);
      })
      .slice(0, 8)
      .map((item: any) => ({
        href: item.href,
        label: item.title ?? item.label,
        description: item.description ?? "Navigation page"
      }));
  }, [query]);

  return (
    <AnimatePresence>
      {open ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm"
          onClick={onClose}
        >
          <motion.div
            initial={{ y: 20, opacity: 0, scale: 0.98 }}
            animate={{ y: 0, opacity: 1, scale: 1 }}
            exit={{ y: 20, opacity: 0, scale: 0.98 }}
            className="mx-auto mt-20 w-[min(92vw,760px)] rounded-[2rem] border border-white/10 bg-slate-950 p-4 shadow-2xl shadow-black/40"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="flex items-center gap-3 rounded-2xl border border-white/10 bg-white/5 px-4 py-3">
              <Search className="h-4 w-4 text-slate-400" />
              <input
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search pages, docs, and privacy tools..."
                className="w-full bg-transparent text-sm text-white outline-none placeholder:text-slate-500"
                autoFocus
              />
              <button
                onClick={onClose}
                className="inline-flex h-9 w-9 items-center justify-center rounded-full text-slate-300 transition hover:bg-white/10 hover:text-white"
                aria-label="Close search"
              >
                <X className="h-4 w-4" />
              </button>
            </div>

            <div className="mt-4 grid gap-2">
              {results.map((item) => (
                <Link
                  key={item.href}
                  href={item.href}
                  onClick={onClose}
                  className="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 transition hover:bg-white/10"
                >
                  <div className="text-sm font-medium text-white">{item.label}</div>
                  <div className="mt-1 text-xs text-slate-400">{item.description}</div>
                </Link>
              ))}
            </div>
          </motion.div>
        </motion.div>
      ) : null}
    </AnimatePresence>
  );
}

"use client";

import { consentLabel } from "@/lib/privacy/consent";
import type { ConsentMatrix, PrivacyScope } from "@/lib/privacy/types";

const scopes: PrivacyScope[] = ["identity", "wallet", "balances", "transactions", "messages", "analytics", "support", "sharing"];

export function ConsentMatrix({
  consent,
  onChange
}: {
  consent: ConsentMatrix;
  onChange: (scope: PrivacyScope, value: "allow" | "deny" | "ask") => void;
}) {
  return (
    <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
      {scopes.map((scope) => (
        <label key={scope} className="rounded-2xl border border-white/10 bg-white/5 p-4">
          <div className="flex items-center justify-between gap-3">
            <span className="font-medium capitalize text-white">{scope}</span>
            <span className="text-xs text-slate-400">{consentLabel(consent[scope])}</span>
          </div>
          <select
            value={consent[scope]}
            onChange={(e) => onChange(scope, e.target.value as "allow" | "deny" | "ask")}
            className="mt-3 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
          >
            <option value="allow">Allow</option>
            <option value="deny">Deny</option>
            <option value="ask">Ask each time</option>
          </select>
        </label>
      ))}
    </div>
  );
}

"use client";

import { useState } from "react";
import { Eye, EyeOff, Copy } from "lucide-react";

export function SecureField({
  label,
  value,
  visible = false,
  copyable = true,
  fallback = "—"
}: {
  label: string;
  value?: string;
  visible?: boolean;
  copyable?: boolean;
  fallback?: string;
}) {
  const [revealed, setRevealed] = useState(visible);

  const rendered = value
    ? revealed
      ? value
      : value.length > 24
        ? `${value.slice(0, 8)}…${value.slice(-6)}`
        : value
    : fallback;

  async function copy() {
    if (!value || !copyable || typeof navigator === "undefined") return;
    try {
      await navigator.clipboard.writeText(value);
    } catch {
      // ignore clipboard failures
    }
  }

  return (
    <div className="rounded-2xl border border-white/10 bg-white/5 p-4">
      <div className="flex items-center justify-between gap-2">
        <span className="text-xs font-medium uppercase tracking-[0.25em] text-slate-500">{label}</span>
        <div className="flex gap-2">
          <button onClick={() => setRevealed((v) => !v)} className="text-slate-300 hover:text-white" aria-label="Reveal or hide">
            {revealed ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
          </button>
          {copyable ? (
            <button onClick={copy} className="text-slate-300 hover:text-white" aria-label="Copy">
              <Copy className="h-4 w-4" />
            </button>
          ) : null}
        </div>
      </div>
      <div className="mt-2 break-all font-mono text-sm text-white">{rendered}</div>
    </div>
  );
}

import { Shield, Lock, Eye, MessageSquareText } from "lucide-react";
import Link from "next/link";

export function PrivacyBanner() {
  return (
    <section className="mx-auto max-w-7xl px-4 pt-6 sm:px-6 lg:px-8">
      <div className="grid gap-4 rounded-[2rem] border border-cyan-300/20 bg-cyan-300/10 p-5 md:grid-cols-[1.2fr_0.8fr]">
        <div>
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-white/10">
              <Shield className="h-5 w-5 text-cyan-200" />
            </div>
            <div>
              <h3 className="text-lg font-semibold text-white">Privacy-first by default</h3>
              <p className="text-sm text-cyan-100/90">Keep raw values hidden unless the user opts in.</p>
            </div>
          </div>
          <p className="mt-4 max-w-2xl text-sm leading-7 text-cyan-50/90">
            The interface uses redaction, local encryption, and selective disclosure to make sensitive data easier to manage on mobile and desktop.
          </p>
        </div>

        <div className="grid gap-2 sm:grid-cols-2">
          <Link href="/privacy" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><Lock className="h-4 w-4" /> Privacy center</div>
            <div className="mt-1 text-xs text-slate-300">Review settings and defaults.</div>
          </Link>
          <Link href="/sharing" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><Eye className="h-4 w-4" /> Safe sharing</div>
            <div className="mt-1 text-xs text-slate-300">Export only what is needed.</div>
          </Link>
          <Link href="/consent" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><MessageSquareText className="h-4 w-4" /> Consent matrix</div>
            <div className="mt-1 text-xs text-slate-300">Set scope-level permissions.</div>
          </Link>
          <Link href="/vault" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><Lock className="h-4 w-4" /> Vault</div>
            <div className="mt-1 text-xs text-slate-300">Keep notes sealed locally.</div>
          </Link>
        </div>
      </div>
    </section>
  );
}

import { formatAuditTime } from "@/lib/privacy/audit";
import type { AuditEvent } from "@/lib/privacy/types";

export function AuditLog({ items }: { items: AuditEvent[] }) {
  return (
    <div className="space-y-3">
      {items.length === 0 ? (
        <div className="rounded-2xl border border-dashed border-white/15 p-5 text-sm text-slate-400">
          No privacy events yet.
        </div>
      ) : (
        items
          .slice()
          .reverse()
          .map((item) => (
            <article key={item.id} className="rounded-2xl border border-white/10 bg-white/5 p-4">
              <div className="flex items-center justify-between gap-3">
                <h4 className="text-sm font-semibold text-white">{item.title}</h4>
                <span
                  className={`rounded-full px-3 py-1 text-xs uppercase tracking-[0.2em] ${
                    item.level === "error"
                      ? "bg-rose-500/10 text-rose-200"
                      : item.level === "warn"
                        ? "bg-amber-500/10 text-amber-200"
                        : "bg-cyan-500/10 text-cyan-200"
                  }`}
                >
                  {item.level}
                </span>
              </div>
              <p className="mt-2 text-sm text-slate-300">{item.summary}</p>
              {item.details ? <p className="mt-2 text-sm text-slate-400">{item.details}</p> : null}
              <div className="mt-3 flex flex-wrap items-center gap-2 text-xs text-slate-500">
                <span>{item.category}</span>
                <span>•</span>
                <span>{formatAuditTime(item.time)}</span>
                {item.tags.map((tag) => (
                  <span key={tag} className="rounded-full border border-white/10 bg-white/5 px-2 py-1">
                    {tag}
                  </span>
                ))}
              </div>
            </article>
          ))
      )}
    </div>
  );
}

import type { DataCategory } from "@/lib/privacy/types";

export function DataMap({ items }: { items: DataCategory[] }) {
  return (
    <div className="grid gap-5 md:grid-cols-2">
      {items.map((item) => (
        <div key={item.name} className="rounded-3xl border border-white/10 bg-white/5 p-6">
          <div className="flex items-center justify-between gap-3">
            <h3 className="text-xl font-semibold text-white">{item.name}</h3>
            <span
              className={`rounded-full px-3 py-1 text-xs uppercase tracking-[0.2em] ${
                item.sensitive ? "bg-rose-500/10 text-rose-200" : "bg-emerald-500/10 text-emerald-200"
              }`}
            >
              {item.sensitive ? "Sensitive" : "Low risk"}
            </span>
          </div>
          <p className="mt-3 text-sm leading-7 text-slate-300">{item.purpose}</p>
          <dl className="mt-5 grid gap-3 sm:grid-cols-2">
            <div className="rounded-2xl bg-slate-950/50 p-3">
              <dt className="text-xs uppercase tracking-[0.2em] text-slate-500">Stored locally</dt>
              <dd className="mt-1 text-sm text-white">{item.storedLocally ? "Yes" : "No"}</dd>
            </div>
            <div className="rounded-2xl bg-slate-950/50 p-3">
              <dt className="text-xs uppercase tracking-[0.2em] text-slate-500">Shared externally</dt>
              <dd className="mt-1 text-sm text-white">{item.sharedExternally ? "Sometimes" : "No"}</dd>
            </div>
            <div className="rounded-2xl bg-slate-950/50 p-3 sm:col-span-2">
              <dt className="text-xs uppercase tracking-[0.2em] text-slate-500">Retention</dt>
              <dd className="mt-1 text-sm text-white">{item.retention}</dd>
            </div>
          </dl>
        </div>
      ))}
    </div>
  );
}

import { Download, ShieldCheck, FileOutput } from "lucide-react";

export function SupportBundleCard({
  title,
  summary,
  onDownload
}: {
  title: string;
  summary: string;
  onDownload: () => void;
}) {
  return (
    <div className="rounded-3xl border border-white/10 bg-white/5 p-6">
      <div className="flex items-center gap-3">
        <div className="flex h-11 w-11 items-center justify-center rounded-2xl bg-emerald-400/10 text-emerald-300 ring-1 ring-emerald-300/20">
          <ShieldCheck className="h-5 w-5" />
        </div>
        <div>
          <h3 className="text-lg font-semibold text-white">{title}</h3>
          <p className="text-sm text-slate-400">{summary}</p>
        </div>
      </div>
      <div className="mt-4 flex gap-2">
        <button
          onClick={onDownload}
          className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100"
        >
          <Download className="h-4 w-4" /> Export bundle
        </button>
        <div className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300">
          <FileOutput className="h-4 w-4" /> Redacted by default
        </div>
      </div>
    </div>
  );
}

"use client";

import { Lock, EyeOff, Zap } from "lucide-react";

export function PrivacyProfileCard({
  level,
  onSelect
}: {
  level: "public" | "private" | "confidential";
  onSelect: (level: "public" | "private" | "confidential") => void;
}) {
  const profiles = [
    {
      id: "public" as const,
      title: "Public",
      icon: <EyeOff className="h-4 w-4" />,
      body: "Show more detail, useful for demos and public dashboards."
    },
    {
      id: "private" as const,
      title: "Private",
      icon: <Lock className="h-4 w-4" />,
      body: "Prefer summaries and redaction with local-only details."
    },
    {
      id: "confidential" as const,
      title: "Confidential",
      icon: <Zap className="h-4 w-4" />,
      body: "Use the strictest settings for sensitive workflows."
    }
  ];

  return (
    <div className="grid gap-4 md:grid-cols-3">
      {profiles.map((profile) => {
        const active = profile.id === level;
        return (
          <button
            key={profile.id}
            onClick={() => onSelect(profile.id)}
            className={`rounded-3xl border p-5 text-left transition ${
              active
                ? "border-cyan-300/30 bg-cyan-300/10"
                : "border-white/10 bg-white/5 hover:bg-white/[0.07]"
            }`}
          >
            <div className="flex items-center gap-2 text-sm font-semibold text-white">
              {profile.icon} {profile.title}
            </div>
            <p className="mt-3 text-sm leading-7 text-slate-300">{profile.body}</p>
          </button>
        );
      })}
    </div>
  );
}

import { redactAddress, redactAmount, redactMessage, redactTxHash, redactWalletProvider } from "@/lib/privacy/redaction";
import type { PrivacySettings } from "@/lib/privacy/types";

export function RedactionPreview({
  settings
}: {
  settings: PrivacySettings;
}) {
  const raw = {
    address: "0x1234567890abcdef1234567890abcdef12345678",
    amount: "12345.6789",
    txHash: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd",
    message: "This is a sensitive note that should stay hidden until explicitly revealed.",
    provider: "walletconnect"
  };

  return (
    <div className="grid gap-4 md:grid-cols-2">
      <div className="rounded-3xl border border-white/10 bg-white/5 p-5">
        <h3 className="text-lg font-semibold text-white">Raw values</h3>
        <div className="mt-4 space-y-3 font-mono text-sm text-slate-300">
          <div>Address: {raw.address}</div>
          <div>Amount: {raw.amount}</div>
          <div>Tx hash: {raw.txHash}</div>
          <div>Message: {raw.message}</div>
          <div>Provider: {raw.provider}</div>
        </div>
      </div>

      <div className="rounded-3xl border border-white/10 bg-white/5 p-5">
        <h3 className="text-lg font-semibold text-white">Redacted view</h3>
        <div className="mt-4 space-y-3 font-mono text-sm text-slate-300">
          <div>Address: {redactAddress(raw.address, settings.showRawAddresses)}</div>
          <div>Amount: {redactAmount(raw.amount, settings.showRawAmounts)}</div>
          <div>Tx hash: {redactTxHash(raw.txHash, settings.showTxHashes)}</div>
          <div>Message: {redactMessage(raw.message, settings.showMessageBodies)}</div>
          <div>Provider: {redactWalletProvider(raw.provider, settings.showWalletProvider)}</div>
        </div>
      </div>
    </div>
  );
}

import Link from "next/link";
import { Shield, Lock, Eye, MessageSquareText } from "lucide-react";

export function PrivacyBanner() {
  return (
    <section className="mx-auto max-w-7xl px-4 pt-6 sm:px-6 lg:px-8">
      <div className="grid gap-4 rounded-[2rem] border border-cyan-300/20 bg-cyan-300/10 p-5 md:grid-cols-[1.2fr_0.8fr]">
        <div>
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-white/10">
              <Shield className="h-5 w-5 text-cyan-200" />
            </div>
            <div>
              <h3 className="text-lg font-semibold text-white">Privacy-first by default</h3>
              <p className="text-sm text-cyan-100/90">Keep raw values hidden unless the user opts in.</p>
            </div>
          </div>
          <p className="mt-4 max-w-2xl text-sm leading-7 text-cyan-50/90">
            The interface uses redaction, local encryption, and selective disclosure to make sensitive data easier to manage on mobile and desktop.
          </p>
        </div>

        <div className="grid gap-2 sm:grid-cols-2">
          <Link href="/privacy" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><Lock className="h-4 w-4" /> Privacy center</div>
            <div className="mt-1 text-xs text-slate-300">Review settings and defaults.</div>
          </Link>
          <Link href="/sharing" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><Eye className="h-4 w-4" /> Safe sharing</div>
            <div className="mt-1 text-xs text-slate-300">Export only what is needed.</div>
          </Link>
          <Link href="/consent" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><MessageSquareText className="h-4 w-4" /> Consent matrix</div>
            <div className="mt-1 text-xs text-slate-300">Set scope-level permissions.</div>
          </Link>
          <Link href="/vault" className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10">
            <div className="flex items-center gap-2 font-medium text-white"><Lock className="h-4 w-4" /> Vault</div>
            <div className="mt-1 text-xs text-slate-300">Keep notes sealed locally.</div>
          </Link>
        </div>
      </div>
    </section>
  );
}

"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useMemo, useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { Github, Menu, X, Stars, Search } from "lucide-react";
import clsx from "clsx";
import { navItems, site } from "@/lib/site";
import { SearchOverlay } from "@/components/SearchOverlay";

export function SiteShell({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const [open, setOpen] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);
  const activePath = useMemo(() => pathname ?? "/", [pathname]);

  return (
    <div className="relative min-h-screen overflow-hidden bg-slate-950 text-white">
      <div className="pointer-events-none absolute inset-0 -z-20 bg-radial-glow" />
      <div className="pointer-events-none absolute inset-0 -z-30 bg-[linear-gradient(to_bottom,rgba(2,6,23,0.92),rgba(2,6,23,1))]" />
      <div className="pointer-events-none absolute inset-0 -z-40 opacity-40 bg-[radial-gradient(circle_at_10%_20%,rgba(56,189,248,0.08),transparent_18%),radial-gradient(circle_at_80%_15%,rgba(168,85,247,0.09),transparent_20%),radial-gradient(circle_at_50%_60%,rgba(251,191,36,0.05),transparent_20%)]" />

      <header className="sticky top-0 z-40 border-b border-white/8 bg-slate-950/75 backdrop-blur-xl">
        <div className="mx-auto flex max-w-7xl items-center justify-between gap-4 px-4 py-4 sm:px-6 lg:px-8">
          <Link href="/" className="flex min-w-0 items-center gap-3">
            <div className="flex h-11 w-11 items-center justify-center rounded-2xl border border-white/10 bg-white/5">
              <Stars className="h-5 w-5 text-cyan-300" />
            </div>
            <div className="min-w-0">
              <p className="truncate text-sm font-semibold text-white">{site.name}</p>
              <p className="truncate text-xs text-slate-400">{site.subtitle}</p>
            </div>
          </Link>

          <nav className="hidden flex-wrap items-center gap-1 xl:flex">
            {navItems.slice(0, 10).map((item) => {
              const active = item.href === activePath;
              return (
                <Link
                  key={item.href}
                  href={item.href}
                  className={clsx(
                    "rounded-full px-3 py-2 text-sm transition",
                    active ? "bg-white text-slate-950" : "text-slate-300 hover:bg-white/6 hover:text-white"
                  )}
                >
                  {item.label}
                </Link>
              );
            })}
          </nav>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setSearchOpen(true)}
              className="inline-flex h-11 w-11 items-center justify-center rounded-full border border-white/10 bg-white/5 text-white transition hover:bg-white/10"
              aria-label="Open search"
            >
              <Search className="h-5 w-5" />
            </button>

            <a
              href={site.repo}
              target="_blank"
              rel="noreferrer"
              className="hidden items-center gap-2 rounded-full border border-white/10 bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100 sm:inline-flex"
            >
              <Github className="h-4 w-4" />
              Repo
            </a>

            <button
              className="inline-flex h-11 w-11 items-center justify-center rounded-full border border-white/10 bg-white/5 text-white xl:hidden"
              onClick={() => setOpen((v) => !v)}
              aria-label="Toggle navigation"
            >
              {open ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
            </button>
          </div>
        </div>

        <AnimatePresence>
          {open ? (
            <motion.div
              initial={{ height: 0, opacity: 0 }}
              animate={{ height: "auto", opacity: 1 }}
              exit={{ height: 0, opacity: 0 }}
              className="border-t border-white/8 bg-slate-950/90 xl:hidden"
            >
              <div className="mx-auto grid max-w-7xl gap-1 px-4 py-4 sm:px-6">
                {navItems.map((item) => {
                  const active = item.href === activePath;
                  return (
                    <Link
                      key={item.href}
                      href={item.href}
                      onClick={() => setOpen(false)}
                      className={clsx(
                        "rounded-2xl px-4 py-3 text-sm transition",
                        active ? "bg-white text-slate-950" : "text-slate-300 hover:bg-white/6 hover:text-white"
                      )}
                    >
                      {item.label}
                    </Link>
                  );
                })}
              </div>
            </motion.div>
          ) : null}
        </AnimatePresence>
      </header>

      <main>{children}</main>

      <footer className="border-t border-white/8 bg-slate-950/80">
        <div className="mx-auto flex max-w-7xl flex-col gap-6 px-4 py-10 sm:px-6 lg:flex-row lg:items-center lg:justify-between lg:px-8">
          <div>
            <p className="text-sm font-semibold text-white">{site.name}</p>
            <p className="mt-2 max-w-xl text-sm text-slate-400">
              A responsive multi-page frontend that makes the project easier to explain and easier to extend.
            </p>
          </div>

          <div className="flex flex-wrap gap-3">
            {navItems.slice(0, 8).map((item) => (
              <Link
                key={item.href}
                href={item.href}
                className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300 transition hover:bg-white/10 hover:text-white"
              >
                {item.label}
              </Link>
            ))}
          </div>
        </div>
      </footer>

      <SearchOverlay open={searchOpen} onClose={() => setSearchOpen(false)} />
    </div>
  );
}

import "./globals.css";
import type { Metadata } from "next";
import { SiteShell } from "@/components/SiteShell";

export const metadata: Metadata = {
  title: "Logos Heraclitus Cosmic Principle",
  description: "A privacy-first responsive frontend for the LEZ event system."
};

export default function RootLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <SiteShell>{children}</SiteShell>
      </body>
    </html>
  );
}

import Link from "next/link";
import { MetricGrid } from "@/components/MetricGrid";
import { SectionHeading } from "@/components/SectionHeading";
import { FeatureCard } from "@/components/FeatureCard";
import { GlassCard } from "@/components/GlassCard";
import { PageHero } from "@/components/PageHero";
import { FAQ } from "@/components/FAQ";
import { PrivacyBanner } from "@/components/PrivacyBanner";
import { featureGrid, faqItems, heroMetrics, pages, principles } from "@/lib/privacy/constants";
import { Shield, Lock, Eye, MessageSquareText } from "lucide-react";

export default function HomePage() {
  return (
    <>
      <PageHero
        badge="Privacy-first UX"
        title="A calm, secure front door for private-by-default workflows."
        description="The interface below is built to reduce accidental disclosure, keep sensitive information local, and make security settings understandable on small screens."
        primary={{ href: "/privacy", label: "Open privacy center" }}
        secondary={{ href: "/dashboard", label: "View dashboard" }}
      />

      <section className="mx-auto max-w-7xl px-4 pb-10 sm:px-6 lg:px-8">
        <PrivacyBanner />
      </section>

      <section className="mx-auto max-w-7xl px-4 pb-20 sm:px-6 lg:px-8">
        <MetricGrid items={heroMetrics} />
      </section>

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <SectionHeading
          eyebrow="Experience"
          title="Responsive pages with a shared privacy language"
          description="The layout keeps the experience consistent across mobile, tablet, and desktop while still giving each page its own purpose."
        />
        <div className="mt-14 grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {featureGrid.map((feature) => (
            <FeatureCard
              key={feature.title}
              icon={feature.icon}
              title={feature.title}
              description={feature.description}
            />
          ))}
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-10 lg:grid-cols-[1.05fr_0.95fr]">
          <div className="rounded-[2rem] border border-white/10 bg-white/5 p-8 shadow-xl shadow-black/20">
            <p className="text-sm font-semibold uppercase tracking-[0.35em] text-slate-400">Privacy manifesto</p>
            <h2 className="mt-4 text-3xl font-semibold tracking-tight text-white sm:text-4xl">
              Progressive disclosure, local encryption, and clear consent.
            </h2>
            <p className="mt-6 text-base leading-8 text-slate-300">
              The app should explain what is being shared, why it is being shared, and how to keep the
              raw values sealed until they are actually needed.
            </p>
            <p className="mt-4 text-base leading-8 text-slate-300">
              That means visible defaults, readable audit history, and a user journey that never assumes
              permission to disclose private data.
            </p>
          </div>

          <div className="grid gap-5">
            {principles.map((item) => (
              <GlassCard key={item.title}>
                <div className="flex items-center gap-3">
                  <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-fuchsia-400/10 text-fuchsia-300 ring-1 ring-fuchsia-300/20">
                    {item.icon}
                  </div>
                  <h3 className="text-lg font-semibold text-white">{item.title}</h3>
                </div>
                <p className="mt-4 text-sm leading-7 text-slate-300">{item.body}</p>
              </GlassCard>
            ))}
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <SectionHeading
          eyebrow="Routes"
          title="A structured set of privacy pages"
          description="These pages make the site feel complete and give sensitive content room to breathe on smaller screens."
        />
        <div className="mt-14 grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {pages.slice(0, 9).map((page) => (
            <Link
              key={page.href}
              href={page.href}
              className="rounded-3xl border border-white/10 bg-white/5 p-6 transition hover:-translate-y-1 hover:bg-white/8"
            >
              <div className="flex items-center gap-3">
                <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                  {page.icon}
                </div>
                <h3 className="text-lg font-semibold text-white">{page.title}</h3>
              </div>
              <p className="mt-4 text-sm leading-7 text-slate-300">{page.description}</p>
            </Link>
          ))}
        </div>
      </section>

      <section className="mx-auto max-w-5xl px-4 py-20 sm:px-6 lg:px-8">
        <SectionHeading
          eyebrow="FAQ"
          title="Useful answers for visitors"
          description="These quick answers reduce uncertainty and help the site feel complete on the first visit."
        />
        <div className="mt-14">
          <FAQ items={faqItems} />
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { SectionHeading } from "@/components/SectionHeading";
import { GlassCard } from "@/components/GlassCard";
import { ConsentMatrix } from "@/components/ConsentMatrix";
import { useConsentMatrix } from "@/hooks/useConsentMatrix";
import { usePrivacySettings } from "@/hooks/usePrivacySettings";
import { privacySummary } from "@/lib/privacy/redaction";
import { Shield, Lock, Eye, MessageSquareText } from "lucide-react";
import { PrivacyProfileCard } from "@/components/PrivacyProfileCard";

export default function PrivacyPage() {
  const { consent, set, reset } = useConsentMatrix();
  const { settings, patch } = usePrivacySettings();

  return (
    <>
      <PageHero
        badge="Privacy center"
        title="Control what the UI reveals, stores, and shares."
        description="This page turns privacy into a first-class control surface: defaults, consent scopes, privacy level, and local-only behavior."
        primary={{ href: "/vault", label: "Open vault" }}
        secondary={{ href: "/sharing", label: "Review sharing" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
            <div>
              <h3 className="text-xl font-semibold text-white">Current privacy posture</h3>
              <p className="mt-2 text-sm text-slate-300">{privacySummary(settings)}</p>
            </div>
            <button
              onClick={() => reset()}
              className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10"
            >
              Reset consent
            </button>
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <SectionHeading
          eyebrow="Consent"
          title="Scope-level controls"
          description="Each data type gets its own rule so the user can permit what is needed and refuse the rest."
        />
        <div className="mt-10">
          <ConsentMatrix consent={consent} onChange={set} />
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-14 sm:px-6 lg:px-8">
        <SectionHeading
          eyebrow="Privacy profiles"
          title="Switch the app between public, private, and confidential modes"
          description="A single global level helps users make the whole interface less or more revealing at once."
        />
        <div className="mt-10">
          <PrivacyProfileCard
            level={settings.preferredLevel}
            onSelect={(level) => patch({ preferredLevel: level })}
          />
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-14 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-4">
          <GlassCard>
            <Shield className="h-5 w-5 text-cyan-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Redaction first</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">Sensitive fields are hidden by default.</p>
          </GlassCard>
          <GlassCard>
            <Lock className="h-5 w-5 text-emerald-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Local encryption</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">The browser can keep encrypted material sealed locally.</p>
          </GlassCard>
          <GlassCard>
            <Eye className="h-5 w-5 text-fuchsia-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Progressive disclosure</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">Reveal only what the task needs.</p>
          </GlassCard>
          <GlassCard>
            <MessageSquareText className="h-5 w-5 text-amber-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Consent clarity</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">Make every permission understandable and reversible.</p>
          </GlassCard>
        </div>
      </section>
    </>
  );
}

"use client";

import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { ConsentMatrix } from "@/components/ConsentMatrix";
import { useConsentMatrix } from "@/hooks/useConsentMatrix";
import { CheckCircle2, ShieldOff, RefreshCcw } from "lucide-react";

export default function ConsentPage() {
  const { consent, set, reset } = useConsentMatrix();

  return (
    <>
      <PageHero
        badge="Consent"
        title="Give every data scope its own rule."
        description="A safe privacy UI does not assume permission. It asks clearly and records the result."
        primary={{ href: "/privacy", label: "Privacy center" }}
        secondary={{ href: "/audit", label: "Audit log" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <div className="flex flex-wrap gap-3">
          <button
            onClick={() => reset()}
            className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100"
          >
            <RefreshCcw className="h-4 w-4" /> Reset defaults
          </button>
          <div className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300">
            <CheckCircle2 className="h-4 w-4" /> Ask-first baseline
          </div>
          <div className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300">
            <ShieldOff className="h-4 w-4" /> Deny sensitive scopes
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <ConsentMatrix consent={consent} onChange={set} />
        </GlassCard>
      </section>
    </>
  );
}

"use client";

import { useState } from "react";
import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { SecureField } from "@/components/SecureField";
import { useVault } from "@/hooks/useVault";
import { Lock, Upload, Save, Trash2 } from "lucide-react";

export default function VaultPage() {
  const vault = useVault();
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [status, setStatus] = useState("Locked");
  const [lastBlob, setLastBlob] = useState<string>("");

  async function save() {
    setStatus("Saving...");
    try {
      await vault.saveNote(title, body);
      setTitle("");
      setBody("");
      setStatus("Saved locally");
    } catch (e) {
      setStatus(e instanceof Error ? e.message : "Unable to save");
    }
  }

  async function unlock() {
    setStatus("Unlocking...");
    try {
      await vault.unlock();
      setStatus("Unlocked");
    } catch (e) {
      setStatus(e instanceof Error ? e.message : "Unlock failed");
    }
  }

  return (
    <>
      <PageHero
        badge="Vault"
        title="Keep private notes sealed locally in the browser."
        description="This page demonstrates local-only encryption, note creation, and safe export of encrypted blobs."
        primary={{ href: "/encryption", label: "Encryption" }}
        secondary={{ href: "/support", label: "Support" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <div className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
            <div>
              <h3 className="text-xl font-semibold text-white">Vault status</h3>
              <p className="mt-2 text-sm text-slate-300">{status}</p>
            </div>
            <div className="flex flex-wrap gap-2">
              <button onClick={unlock} className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100">
                <Lock className="h-4 w-4" /> Unlock
              </button>
              <button onClick={() => vault.lock()} className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10">
                Lock
              </button>
              <button onClick={() => vault.clear()} className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10">
                <Trash2 className="h-4 w-4" /> Clear notes
              </button>
            </div>
          </div>

          <div className="mt-5 grid gap-4 md:grid-cols-2">
            <SecureField label="Passphrase" value={vault.passphrase || "not set"} visible={false} copyable={false} />
            <SecureField label="Key active" value={vault.key ? "yes" : "no"} visible={true} copyable={false} />
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
        <div className="grid gap-5 lg:grid-cols-[0.95fr_1.05fr]">
          <GlassCard>
            <h3 className="text-lg font-semibold text-white">Create note</h3>
            <div className="mt-4 space-y-3">
              <input value={title} onChange={(e) => setTitle(e.target.value)} placeholder="Title" className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none" />
              <textarea value={body} onChange={(e) => setBody(e.target.value)} placeholder="Private note..." rows={8} className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none" />
              <button onClick={save} className="inline-flex items-center gap-2 rounded-full bg-cyan-300 px-4 py-2 text-sm font-semibold text-slate-950 transition hover:bg-cyan-200">
                <Save className="h-4 w-4" /> Save
              </button>
            </div>
          </GlassCard>

          <GlassCard>
            <div className="flex items-center justify-between gap-3">
              <h3 className="text-lg font-semibold text-white">Saved notes</h3>
              <button
                onClick={() => {
                  const json = JSON.stringify(vault.notes, null, 2);
                  setLastBlob(json);
                }}
                className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10"
              >
                <Upload className="h-4 w-4" /> Preview
              </button>
            </div>

            <div className="mt-4 space-y-3">
              {vault.notes.map((note) => (
                <div key={note.id} className="rounded-2xl border border-white/10 bg-slate-950/60 p-4">
                  <div className="flex items-center justify-between gap-3">
                    <h4 className="font-medium text-white">{note.title}</h4>
                    <span className="text-xs text-slate-400">{note.encrypted ? "Encrypted" : "Plaintext"}</span>
                  </div>
                  <p className="mt-2 text-sm leading-7 text-slate-300">{note.body}</p>
                  {note.checksum ? <p className="mt-2 text-xs text-slate-500">Checksum: {note.checksum}</p> : null}
                </div>
              ))}
            </div>

            {lastBlob ? (
              <pre className="mt-4 overflow-auto rounded-2xl bg-black/40 p-4 text-xs text-slate-200">{lastBlob}</pre>
            ) : null}
          </GlassCard>
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { RedactionPreview } from "@/components/RedactionPreview";
import { usePrivacySettings } from "@/hooks/usePrivacySettings";

export default function RedactionPage() {
  const { settings, patch } = usePrivacySettings();

  return (
    <>
      <PageHero
        badge="Redaction"
        title="Show the difference between raw data and safe previews."
        description="This page makes privacy visible by comparing what the app has, and what the user is allowed to see."
        primary={{ href: "/privacy", label: "Privacy center" }}
        secondary={{ href: "/sharing", label: "Sharing" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Show raw addresses</span>
              <input type="checkbox" checked={settings.showRawAddresses} onChange={(e) => patch({ showRawAddresses: e.target.checked })} className="mt-2" />
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Show raw amounts</span>
              <input type="checkbox" checked={settings.showRawAmounts} onChange={(e) => patch({ showRawAmounts: e.target.checked })} className="mt-2" />
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Show tx hashes</span>
              <input type="checkbox" checked={settings.showTxHashes} onChange={(e) => patch({ showTxHashes: e.target.checked })} className="mt-2" />
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Show messages</span>
              <input type="checkbox" checked={settings.showMessageBodies} onChange={(e) => patch({ showMessageBodies: e.target.checked })} className="mt-2" />
            </label>
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <RedactionPreview settings={settings} />
      </section>
    </>
  );
}

"use client";

import { useState } from "react";
import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { ShieldCheck, Copy, AlertTriangle, Share2 } from "lucide-react";

export default function SharingPage() {
  const [scope, setScope] = useState("summary-only");
  const [recipient, setRecipient] = useState("support-team");
  const [note, setNote] = useState("Share only the event ids and approximate times.");

  async function copyLink() {
    const payload = `share://${recipient}?scope=${scope}&note=${encodeURIComponent(note)}`;
    try {
      await navigator.clipboard.writeText(payload);
    } catch {
      // ignore clipboard failures
    }
  }

  return (
    <>
      <PageHero
        badge="Sharing"
        title="Export data with smaller blast radius."
        description="Safe sharing keeps the user in control by defaulting to summaries, fingerprints, and scoped exports."
        primary={{ href: "/audit", label: "Audit trail" }}
        secondary={{ href: "/security", label: "Security" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <div className="grid gap-4 md:grid-cols-3">
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Scope</span>
              <select value={scope} onChange={(e) => setScope(e.target.value)} className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none">
                <option value="summary-only">Summary only</option>
                <option value="redacted-bundle">Redacted bundle</option>
                <option value="diagnostic">Diagnostic</option>
              </select>
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Recipient</span>
              <input value={recipient} onChange={(e) => setRecipient(e.target.value)} className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none" />
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Message</span>
              <input value={note} onChange={(e) => setNote(e.target.value)} className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none" />
            </label>
          </div>

          <div className="mt-5 flex flex-wrap gap-3">
            <button onClick={copyLink} className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100">
              <Copy className="h-4 w-4" /> Copy share link
            </button>
            <div className="inline-flex items-center gap-2 rounded-full border border-emerald-400/20 bg-emerald-500/10 px-4 py-2 text-sm text-emerald-200">
              <ShieldCheck className="h-4 w-4" /> Share the minimum needed
            </div>
            <div className="inline-flex items-center gap-2 rounded-full border border-amber-400/20 bg-amber-500/10 px-4 py-2 text-sm text-amber-200">
              <AlertTriangle className="h-4 w-4" /> Review before sending
            </div>
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-5xl px-4 py-10 sm:px-6 lg:px-8">
        <div className="rounded-[2rem] border border-white/10 bg-white/5 p-8">
          <div className="flex items-center gap-2 text-white">
            <Share2 className="h-5 w-5 text-cyan-300" /> Suggested export rules
          </div>
          <ul className="mt-5 space-y-3 text-sm leading-7 text-slate-300">
            <li>• Default to summary-only bundles.</li>
            <li>• Hide raw identifiers unless a recipient truly needs them.</li>
            <li>• Avoid attachments that contain secrets, seed phrases, or private keys.</li>
            <li>• Add a checksum so support can verify integrity after transfer.</li>
          </ul>
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { AuditLog } from "@/components/AuditLog";
import { GlassCard } from "@/components/GlassCard";
import { createAuditEvent } from "@/lib/privacy/audit";

const auditItems = [
  createAuditEvent("settings", "Privacy setting updated", "Redaction mode was tightened.", "The user opted into stricter previews.", "info", ["settings", "redaction"]),
  createAuditEvent("storage", "Encrypted record stored", "A vault note was sealed in local storage.", "The note stayed local to the browser.", "info", ["vault", "encrypted"]),
  createAuditEvent("share", "Export limited", "A wide export request was narrowed down.", "The UI asked for a safer scope.", "warn", ["share", "bundle"]),
  createAuditEvent("security", "Potential disclosure prevented", "A raw value stayed hidden in the preview.", "Progressive disclosure protected the user.", "info", ["safe-default", "preview"])
];

export default function AuditPage() {
  return (
    <>
      <PageHero
        badge="Audit"
        title="A readable privacy trail for support and trust."
        description="Users should be able to inspect what the UI did without exposing sensitive details in the log."
        primary={{ href: "/support", label: "Support" }}
        secondary={{ href: "/security", label: "Security" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <h3 className="text-xl font-semibold text-white">Recent privacy events</h3>
          <p className="mt-2 text-sm text-slate-300">Entries are summarized by default and can be expanded in support workflows.</p>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
        <AuditLog items={auditItems} />
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { MetricGrid } from "@/components/MetricGrid";
import { GlassCard } from "@/components/GlassCard";
import { dashboardMetrics, pages } from "@/lib/privacy/constants";
import Link from "next/link";

export default function DashboardPage() {
  return (
    <>
      <PageHero
        badge="Dashboard"
        title="A responsive control center for privacy and navigation."
        description="This page gives users a fast way to see the current posture, jump to key areas, and keep privacy tools within reach."
        primary={{ href: "/settings", label: "Open settings" }}
        secondary={{ href: "/privacy", label: "Privacy center" }}
      />

      <section className="mx-auto max-w-7xl px-4 pb-20 sm:px-6 lg:px-8">
        <MetricGrid items={dashboardMetrics} />
      </section>

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {pages.slice(0, 9).map((page) => (
            <GlassCard key={page.href}>
              <div className="flex items-center gap-3">
                <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                  {page.icon}
                </div>
                <h3 className="text-lg font-semibold text-white">{page.title}</h3>
              </div>
              <p className="mt-4 text-sm leading-7 text-slate-300">{page.description}</p>
              <Link href={page.href} className="mt-5 inline-flex text-sm font-medium text-cyan-300">
                Open section →
              </Link>
            </GlassCard>
          ))}
        </div>
      </section>
    </>
  );
}

"use client";

import { useState } from "react";
import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { settingsOptions } from "@/lib/privacy/constants";
import { usePrivacySettings } from "@/hooks/usePrivacySettings";

export default function SettingsPage() {
  const { settings, patch, reset } = usePrivacySettings();
  const [density, setDensity] = useState("comfortable");
  const [motion, setMotion] = useState("subtle");

  return (
    <>
      <PageHero
        badge="Settings"
        title="Tune the UI for comfort, density, and readability"
        description="This page gives the frontend a small preference center that feels useful on both large and small screens."
        primary={{ href: "/dashboard", label: "Dashboard" }}
        secondary={{ href: "/accessibility", label: "Accessibility" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-4">
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Density</span>
              <select value={density} onChange={(e) => setDensity(e.target.value)} className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none">
                <option value="comfortable">Comfortable</option>
                <option value="compact">Compact</option>
                <option value="spacious">Spacious</option>
              </select>
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Motion</span>
              <select value={motion} onChange={(e) => setMotion(e.target.value)} className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none">
                <option value="subtle">Subtle</option>
                <option value="minimal">Minimal</option>
                <option value="standard">Standard</option>
              </select>
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Privacy level</span>
              <select
                value={settings.preferredLevel}
                onChange={(e) => patch({ preferredLevel: e.target.value as "public" | "private" | "confidential" })}
                className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
              >
                <option value="public">Public</option>
                <option value="private">Private</option>
                <option value="confidential">Confidential</option>
              </select>
            </label>
            <button onClick={reset} className="rounded-2xl border border-white/10 bg-white/5 p-4 text-left transition hover:bg-white/10">
              <div className="text-sm font-semibold text-white">Reset privacy settings</div>
              <div className="mt-1 text-xs text-slate-400">Return to safe defaults</div>
            </button>
          </div>

          <div className="mt-5 text-sm text-slate-300">
            Density: {density} • Motion: {motion} • Privacy: {settings.preferredLevel}
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-5xl px-4 py-10 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {settingsOptions.map((item) => (
            <GlassCard key={item.title}>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { DataMap } from "@/components/DataMap";
import { dataCategories } from "@/lib/privacy/constants";

export default function DataMapPage() {
  return (
    <>
      <PageHero
        badge="Data map"
        title="See what is stored, why it exists, and how sensitive it is."
        description="A good privacy UI makes data flows understandable, not hidden."
        primary={{ href: "/privacy", label: "Privacy center" }}
        secondary={{ href: "/security", label: "Security" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <DataMap items={dataCategories} />
      </section>
    </>
  );
}

"use client";

import { useState } from "react";
import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { CodeBlock } from "@/components/CodeBlock";
import { deriveKeyFromPassphrase, encryptJson, decryptJson } from "@/lib/privacy/crypto";
import { Lock, ShieldCheck, Copy } from "lucide-react";

export default function EncryptionPage() {
  const [passphrase, setPassphrase] = useState("");
  const [secret, setSecret] = useState("Sensitive payload kept local.");
  const [ciphertext, setCiphertext] = useState<string>("");
  const [plain, setPlain] = useState<string>("");
  const [status, setStatus] = useState("Idle");

  async function seal() {
    try {
      setStatus("Deriving key...");
      const key = await deriveKeyFromPassphrase(passphrase, "logos-privacy-salt");
      setStatus("Encrypting...");
      const blob = await encryptJson({ secret }, key, "demo-secret");
      setCiphertext(JSON.stringify(blob, null, 2));
      setStatus("Encrypted locally");
    } catch (e) {
      setStatus(e instanceof Error ? e.message : "Encryption failed");
    }
  }

  async function open() {
    try {
      setStatus("Deriving key...");
      const key = await deriveKeyFromPassphrase(passphrase, "logos-privacy-salt");
      const blob = JSON.parse(ciphertext);
      const data = await decryptJson(blob, key);
      setPlain(JSON.stringify(data, null, 2));
      setStatus("Decrypted locally");
    } catch (e) {
      setStatus(e instanceof Error ? e.message : "Decryption failed");
    }
  }

  async function copyCipher() {
    try {
      await navigator.clipboard.writeText(ciphertext);
    } catch {
      // ignore clipboard failures
    }
  }

  return (
    <>
      <PageHero
        badge="Encryption"
        title="Local encryption that stays in the browser."
        description="This page demonstrates how passphrase-derived keys can seal and re-open data without sending plaintext anywhere else."
        primary={{ href: "/vault", label: "Vault" }}
        secondary={{ href: "/security", label: "Security" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <GlassCard>
          <div className="grid gap-4 md:grid-cols-3">
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4 md:col-span-1">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Passphrase</span>
              <input value={passphrase} onChange={(e) => setPassphrase(e.target.value)} type="password" className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none" />
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4 md:col-span-2">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">Secret payload</span>
              <input value={secret} onChange={(e) => setSecret(e.target.value)} className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none" />
            </label>
          </div>

          <div className="mt-5 flex flex-wrap gap-3">
            <button onClick={seal} className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100">
              <Lock className="h-4 w-4" /> Encrypt
            </button>
            <button onClick={open} className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10">
              <ShieldCheck className="h-4 w-4" /> Decrypt
            </button>
            <button onClick={copyCipher} className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10">
              <Copy className="h-4 w-4" /> Copy blob
            </button>
            <div className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300">{status}</div>
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
        <div className="grid gap-5 lg:grid-cols-2">
          <CodeBlock title="Encrypted blob" code={ciphertext || "Run Encrypt to generate a secure blob."} />
          <CodeBlock title="Decrypted payload" code={plain || "Run Decrypt after encrypting to view the JSON payload."} />
        </div>
      </section>
    </>
  );
}

"use client";

import { useMemo, useState } from "react";
import { PageHero } from "@/components/PageHero";
import { SectionHeading } from "@/components/SectionHeading";
import { GlassCard } from "@/components/GlassCard";
import { eventFeed } from "@/lib/privacy/constants";
import { Search, Filter, ArrowUpDown } from "lucide-react";
import { redactEvent } from "@/lib/privacy/redaction";
import { usePrivacySettings } from "@/hooks/usePrivacySettings";

export default function EventsPage() {
  const [query, setQuery] = useState("");
  const [filter, setFilter] = useState<"all" | "success" | "warning" | "info" | "error">("all");
  const [sort, setSort] = useState<"recent" | "alpha">("recent");
  const { settings } = usePrivacySettings();

  const rows = useMemo(() => {
    const q = query.trim().toLowerCase();
    return [...eventFeed]
      .filter((item) => (filter === "all" ? true : item.type === filter))
      .filter((item) => {
        if (!q) return true;
        return (
          item.title.toLowerCase().includes(q) ||
          item.summary.toLowerCase().includes(q) ||
          item.details.toLowerCase().includes(q) ||
          item.tags.some((tag) => tag.toLowerCase().includes(q))
        );
      })
      .sort((a, b) => (sort === "alpha" ? a.title.localeCompare(b.title) : b.id.localeCompare(a.id)))
      .map((item) => redactEvent(item, settings));
  }, [query, filter, sort, settings]);

  return (
    <>
      <PageHero
        badge="Events"
        title="A privacy-aware event browser for better user experience"
        description="The browser keeps the UI simple while still making it easy to find the event that matters."
        primary={{ href: "/dashboard", label: "Dashboard" }}
        secondary={{ href: "/audit", label: "Audit" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <SectionHeading
          eyebrow="Explorer"
          title="Filter, search, and sort the event feed"
          description="The browser keeps the UI simple while still making it easy to find the event that matters."
        />

        <div className="mt-10 grid gap-4 md:grid-cols-3">
          <div className="rounded-3xl border border-white/10 bg-white/5 p-4">
            <label className="mb-2 flex items-center gap-2 text-sm text-slate-300">
              <Search className="h-4 w-4" /> Search
            </label>
            <input
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="title, summary, tag..."
              className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none"
            />
          </div>

          <div className="rounded-3xl border border-white/10 bg-white/5 p-4">
            <label className="mb-2 flex items-center gap-2 text-sm text-slate-300">
              <Filter className="h-4 w-4" /> Filter
            </label>
            <select
              value={filter}
              onChange={(e) => setFilter(e.target.value as typeof filter)}
              className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none"
            >
              <option value="all">All</option>
              <option value="success">Success</option>
              <option value="warning">Warning</option>
              <option value="info">Info</option>
              <option value="error">Error</option>
            </select>
          </div>

          <div className="rounded-3xl border border-white/10 bg-white/5 p-4">
            <label className="mb-2 flex items-center gap-2 text-sm text-slate-300">
              <ArrowUpDown className="h-4 w-4" /> Sort
            </label>
            <select
              value={sort}
              onChange={(e) => setSort(e.target.value as typeof sort)}
              className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none"
            >
              <option value="recent">Most recent</option>
              <option value="alpha">Alphabetical</option>
            </select>
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-4 pb-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {rows.map((item) => (
            <GlassCard key={item.id}>
              <div className="flex items-start justify-between gap-3">
                <div>
                  <h3 className="text-xl font-semibold text-white">{item.title}</h3>
                  <p className="mt-2 text-sm text-slate-400">{item.createdAt}</p>
                </div>
                <span className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs uppercase tracking-[0.2em] text-slate-300">
                  {item.type}
                </span>
              </div>
              <p className="mt-4 text-sm leading-7 text-slate-300">{item.summary}</p>
              <p className="mt-3 text-sm leading-7 text-slate-400">{item.details}</p>
              <div className="mt-4 flex flex-wrap gap-2">
                {item.tags.map((tag) => (
                  <span key={tag} className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-slate-300">
                    {tag}
                  </span>
                ))}
              </div>
            </GlassCard>
          ))}
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { securityNotes } from "@/lib/privacy/constants";

export default function SecurityPage() {
  return (
    <>
      <PageHero
        badge="Security"
        title="Make trust boundaries visible in the frontend"
        description="A security page helps explain what the UI does not show, as much as what it does."
        primary={{ href: "/privacy", label: "Privacy" }}
        secondary={{ href: "/support", label: "Support" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {securityNotes.map((item) => (
            <GlassCard key={item.title}>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { accessibilityNotes, uiChecklist } from "@/lib/privacy/constants";

export default function AccessibilityPage() {
  return (
    <>
      <PageHero
        badge="Accessibility"
        title="Responsive UI that stays readable and keyboard-friendly"
        description="Privacy controls should be usable by everyone, including keyboard users and people on small screens."
        primary={{ href: "/support", label: "Support" }}
        secondary={{ href: "/overview", label: "Overview" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {accessibilityNotes.map((item) => (
            <GlassCard key={item.title}>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>

      <section className="mx-auto max-w-5xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="rounded-[2rem] border border-white/10 bg-white/5 p-8">
          <h3 className="text-2xl font-semibold text-white">Implementation notes</h3>
          <ul className="mt-6 space-y-4 text-sm leading-7 text-slate-300">
            {uiChecklist.map((item) => (
              <li key={item}>• {item}</li>
            ))}
          </ul>
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { researchNotes } from "@/lib/privacy/constants";

export default function ResearchPage() {
  return (
    <>
      <PageHero
        badge="Research"
        title="Open questions for future UI and privacy work"
        description="A research page makes the project feel alive by showing what is still being explored."
        primary={{ href: "/privacy", label: "Privacy center" }}
        secondary={{ href: "/community", label: "Community" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {researchNotes.map((item) => (
            <GlassCard key={item.title}>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { GlassCard } from "@/components/GlassCard";
import { communityNotes } from "@/lib/privacy/constants";

export default function CommunityPage() {
  return (
    <>
      <PageHero
        badge="Community"
        title="A place for contributors, maintainers, and reviewers"
        description="The site gets easier to grow when contribution norms and review expectations are visible."
        primary={{ href: "/roadmap", label: "Roadmap" }}
        secondary={{ href: "/support", label: "Support" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {communityNotes.map((item) => (
            <GlassCard key={item.title}>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { FAQ } from "@/components/FAQ";
import { faqItems } from "@/lib/privacy/constants";
import { GlassCard } from "@/components/GlassCard";
import { LifeBuoy, Bug, FileWarning, MessageSquareText } from "lucide-react";

const supportCards = [
  { icon: <Bug className="h-5 w-5" />, title: "Bug reports", body: "Include the page, steps, and expected behavior." },
  { icon: <FileWarning className="h-5 w-5" />, title: "Safety notes", body: "Flag anything that could expose sensitive data or break the privacy story." },
  { icon: <MessageSquareText className="h-5 w-5" />, title: "Feedback", body: "Use this route to collect product feedback and visual suggestions." },
  { icon: <LifeBuoy className="h-5 w-5" />, title: "Help resources", body: "Link docs, examples, issue templates, or community channels here." }
];

export default function SupportPage() {
  return (
    <>
      <PageHero
        badge="Support"
        title="Make the project easier to use and easier to maintain"
        description="Support content keeps the frontend honest: it shows visitors how to report problems and where to go next."
        primary={{ href: "/changelog", label: "Open changelog" }}
        secondary={{ href: "/docs", label: "Read docs" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {supportCards.map((item) => (
            <GlassCard key={item.title}>
              <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                {item.icon}
              </div>
              <h3 className="mt-5 text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>

      <section className="mx-auto max-w-5xl px-4 py-16 sm:px-6 lg:px-8">
        <FAQ items={faqItems} />
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { Timeline } from "@/components/Timeline";
import { roadmap } from "@/lib/privacy/constants";

export default function RoadmapPage() {
  return (
    <>
      <PageHero
        badge="Roadmap"
        title="A path for expanding the site with more privacy pages"
        description="The site is structured so it can keep growing without needing a redesign every time a new idea appears."
        primary={{ href: "/support", label: "Support page" }}
        secondary={{ href: "/changelog", label: "Changelog" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <Timeline items={roadmap} />
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { Timeline } from "@/components/Timeline";

const changes = [
  { phase: "v0.1", title: "Privacy shell", body: "Introduced the responsive shell and navigation." },
  { phase: "v0.2", title: "Consent controls", body: "Added a scope-based consent matrix and safe defaults." },
  { phase: "v0.3", title: "Vault and redaction", body: "Added local encrypted notes and privacy previews." },
  { phase: "v1.0", title: "Public-ready portal", body: "Prepared the frontend to host docs, demos, and updates cleanly." }
];

export default function ChangelogPage() {
  return (
    <>
      <PageHero
        badge="Changelog"
        title="A visible history of frontend improvements"
        description="This page gives the site a release narrative and makes future changes easier to understand."
        primary={{ href: "/", label: "Home" }}
        secondary={{ href: "/roadmap", label: "Roadmap" }}
      />

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <Timeline items={changes} />
      </section>
    </>
  );
}

import { PageHero } from "@/components/PageHero";
import { MetricGrid } from "@/components/MetricGrid";
import { GlassCard } from "@/components/GlassCard";
import { dashboardMetrics, pages } from "@/lib/privacy/constants";
import Link from "next/link";

export default function DashboardPage() {
  return (
    <>
      <PageHero
        badge="Dashboard"
        title="A responsive control center for privacy and navigation."
        description="This page gives users a fast way to see the current posture, jump to key areas, and keep privacy tools within reach."
        primary={{ href: "/settings", label: "Open settings" }}
        secondary={{ href: "/privacy", label: "Privacy center" }}
      />

      <section className="mx-auto max-w-7xl px-4 pb-20 sm:px-6 lg:px-8">
        <MetricGrid items={dashboardMetrics} />
      </section>

      <section className="mx-auto max-w-7xl px-4 py-20 sm:px-6 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {pages.slice(0, 9).map((page) => (
            <GlassCard key={page.href}>
              <div className="flex items-center gap-3">
                <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                  {page.icon}
                </div>
                <h3 className="text-lg font-semibold text-white">{page.title}</h3>
              </div>
              <p className="mt-4 text-sm leading-7 text-slate-300">{page.description}</p>
              <Link href={page.href} className="mt-5 inline-flex text-sm font-medium text-cyan-300">
                Open section →
              </Link>
            </GlassCard>
          ))}
        </div>
      </section>
    </>
  );
}

import "./globals.css";
import type { Metadata } from "next";
import { SiteShell } from "@/components/SiteShell";

export const metadata: Metadata = {
  title: "Logos Heraclitus Cosmic Principle",
  description: "A privacy-first responsive frontend for the LEZ event system."
};

export default function RootLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <SiteShell>{children}</SiteShell>
      </body>
    </html>
  );
}

# Frontend UI/UX

This folder adds a responsive multi-page Next.js frontend focused on privacy.

## Run

```bash
cd frontend
npm install
npm run dev

Pages
Home, Privacy, Consent, Vault, Redaction, Sharing, Audit, Dashboard, Settings, Events, Security, Accessibility, Research, Community, Support, Roadmap, Changelog, Encryption, Data Map.
Notes
The shell is responsive and uses a mobile drawer menu.
Sensitive values are redacted by default.
Local encryption stays in the browser unless explicitly exported.

If you want the next step, I can :contentReference[oaicite:4]{index=4}.
::contentReference[oaicite:3]{index=3}


