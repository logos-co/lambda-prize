import { PrivacyUiError, toAppErrorInfo } from "./errors";
import type { SecureBlob } from "../types/privacy";

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();

function toBase64(bytes: Uint8Array): string {
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary);
}

function fromBase64(input: string): Uint8Array<ArrayBuffer> {
  const binary = atob(input);
  const out = new Uint8Array(new ArrayBuffer(binary.length));
  for (let i = 0; i < binary.length; i += 1) out[i] = binary.charCodeAt(i);
  return out;
}

export function randomId(prefix = "id"): string {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  return `${prefix}_${toBase64(bytes).replace(/[^a-zA-Z0-9]/g, "").slice(0, 22)}`;
}

export async function sha256Hex(
  data: string | ArrayBuffer | Uint8Array
): Promise<string> {
  let buf: ArrayBuffer;
  if (typeof data === "string") {
    buf = textEncoder.encode(data).buffer as ArrayBuffer;
  } else if (data instanceof ArrayBuffer) {
    buf = data;
  } else {
    buf = data.buffer as ArrayBuffer;
  }
  // Fix: pass a plain ArrayBuffer — Web Crypto requires ArrayBuffer, not SharedArrayBuffer
  const digest = await crypto.subtle.digest("SHA-256", buf);
  return Array.from(new Uint8Array(digest), (b) =>
    b.toString(16).padStart(2, "0")
  ).join("");
}

export async function deriveKeyFromPassphrase(
  passphrase: string,
  salt: string,
  iterations = 210_000
): Promise<CryptoKey> {
  if (!passphrase.trim()) {
    throw new PrivacyUiError(
      toAppErrorInfo("MISSING_PASSPHRASE", "Passphrase is required.")
    );
  }

  const baseKey = await crypto.subtle.importKey(
    "raw",
    textEncoder.encode(passphrase),
    { name: "PBKDF2" },
    false,
    ["deriveKey"]
  );

  return crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt: textEncoder.encode(salt),
      iterations,
      hash: "SHA-256",
    },
    baseKey,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt", "decrypt"]
  );
}

export async function deriveKeyFingerprint(key: CryptoKey): Promise<string> {
  const jwk = await crypto.subtle.exportKey("jwk", key);
  return sha256Hex(JSON.stringify(jwk));
}

export function generateNonce(): Uint8Array<ArrayBuffer> {
  // Explicitly create an ArrayBuffer-backed Uint8Array so Web Crypto accepts it
  return crypto.getRandomValues(new Uint8Array(new ArrayBuffer(12)));
}

export async function encryptJson<T>(
  payload: T,
  key: CryptoKey,
  label: string
): Promise<SecureBlob<T>> {
  const plaintext = textEncoder.encode(JSON.stringify(payload));
  const nonce = generateNonce(); // Uint8Array<ArrayBuffer> — accepted by Web Crypto
  const ciphertext = await crypto.subtle.encrypt(
    { name: "AES-GCM", iv: nonce },
    key,
    plaintext
  );
  const cipherBytes = new Uint8Array(ciphertext);

  return {
    version: 1,
    createdAt: new Date().toISOString(),
    label,
    nonce: toBase64(nonce),
    ciphertext: toBase64(cipherBytes),
    checksum: await sha256Hex(cipherBytes),
    payloadType: typeof payload,
  };
}

export async function decryptJson<T>(blob: SecureBlob<T>, key: CryptoKey): Promise<T> {
  try {
    const nonce = fromBase64(blob.nonce); // Uint8Array<ArrayBuffer> — accepted by Web Crypto
    const cipherBytes = fromBase64(blob.ciphertext);
    const plaintext = await crypto.subtle.decrypt(
      { name: "AES-GCM", iv: nonce },
      key,
      cipherBytes
    );
    return JSON.parse(textDecoder.decode(plaintext)) as T;
  } catch (cause) {
    throw new PrivacyUiError(
      toAppErrorInfo("DECRYPT_FAILED", "Unable to decrypt local secure data.", {
        hint: "Check that the correct passphrase or device key is being used.",
      }),
      cause
    );
  }
}

// Fix: encrypt the string directly (not wrapped in {text}) so the generic
// type parameter T=string is consistent between sealText and openText.
export async function sealText(
  text: string,
  key: CryptoKey,
  label: string
): Promise<SecureBlob<string>> {
  return encryptJson(text, key, label);
}

export async function openText(
  blob: SecureBlob<string>,
  key: CryptoKey
): Promise<string> {
  return decryptJson<string>(blob, key);
}
