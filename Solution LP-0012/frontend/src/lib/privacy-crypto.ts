import type { SecureBlob } from "./privacy-types";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

function assertCryptoAvailable(): void {
  if (
    typeof crypto === "undefined" ||
    typeof crypto.subtle === "undefined"
  ) {
    throw new Error(
      "Web Crypto API (crypto.subtle) is unavailable. " +
        "This page requires a secure context — use HTTPS or localhost."
    );
  }
}

function toBase64(bytes: Uint8Array): string {
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary);
}

function fromBase64(input: string): Uint8Array<ArrayBuffer> {
  const binary = atob(input);
  const buf = new ArrayBuffer(binary.length);
  const out = new Uint8Array(buf);
  for (let i = 0; i < binary.length; i++) out[i] = binary.charCodeAt(i);
  return out;
}

export async function sha256Hex(data: string): Promise<string> {
  assertCryptoAvailable();
  const digest = await crypto.subtle.digest("SHA-256", encoder.encode(data));
  return Array.from(new Uint8Array(digest), (b) =>
    b.toString(16).padStart(2, "0")
  ).join("");
}

export function randomId(prefix = "id"): string {
  if (typeof crypto === "undefined" || typeof crypto.getRandomValues === "undefined") {
    const fallback = Math.random().toString(36).slice(2, 18).padEnd(16, "0");
    return `${prefix}_${fallback}`;
  }
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  return `${prefix}_${toBase64(bytes).replace(/[^a-zA-Z0-9]/g, "").slice(0, 22)}`;
}

export async function deriveKey(
  passphrase: string,
  salt = "cryptarchia-privacy-salt",
  iterations = 210_000
): Promise<CryptoKey> {
  assertCryptoAvailable();
  const pass = passphrase.trim();
  if (!pass) throw new Error("Passphrase is required");
  if (pass.length < 8) throw new Error("Passphrase must be at least 8 characters");
  const base = await crypto.subtle.importKey(
    "raw",
    encoder.encode(pass),
    { name: "PBKDF2" },
    false,
    ["deriveKey"]
  );
  return crypto.subtle.deriveKey(
    { name: "PBKDF2", salt: encoder.encode(salt), iterations, hash: "SHA-256" },
    base,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt", "decrypt"]
  );
}

export async function encryptJson<T>(
  payload: T,
  key: CryptoKey,
  label: string
): Promise<SecureBlob<T>> {
  assertCryptoAvailable();
  const iv = crypto.getRandomValues(new Uint8Array(12));
  const plaintext = encoder.encode(JSON.stringify(payload));
  const cipherBuffer = await crypto.subtle.encrypt(
    { name: "AES-GCM", iv },
    key,
    plaintext
  );
  const cipherBytes = new Uint8Array(cipherBuffer);
  return {
    version: 1,
    label,
    createdAt: new Date().toISOString(),
    nonce: toBase64(iv),
    ciphertext: toBase64(cipherBytes),
    checksum: await sha256Hex(toBase64(cipherBytes)),
    payloadType: typeof payload,
  };
}

export async function decryptJson<T>(
  blob: SecureBlob<T>,
  key: CryptoKey
): Promise<T> {
  assertCryptoAvailable();
  if (!blob.nonce || !blob.ciphertext) {
    throw new Error("Invalid encrypted blob — missing nonce or ciphertext.");
  }
  const iv = fromBase64(blob.nonce);
  const cipherBytes = fromBase64(blob.ciphertext);
  let plainBuffer: ArrayBuffer;
  try {
    plainBuffer = await crypto.subtle.decrypt(
      { name: "AES-GCM", iv },
      key,
      cipherBytes
    );
  } catch (e) {
    if (e instanceof DOMException && e.name === "OperationError") {
      throw new Error("Wrong passphrase or corrupted ciphertext — decryption failed.");
    }
    throw e;
  }
  try {
    return JSON.parse(decoder.decode(plainBuffer)) as T;
  } catch {
    throw new Error("Decrypted data is not valid JSON — the blob may be corrupted.");
  }
}
