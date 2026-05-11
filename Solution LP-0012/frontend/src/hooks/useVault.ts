import { useMemo, useRef, useState } from "react";
import {
  deriveKey,
  encryptJson,
  decryptJson,
  randomId,
} from "../lib/privacy-crypto";
import { demoVaultNotes } from "../lib/mock-data";
import type { VaultNote, SecureBlob } from "../lib/privacy-types";

const STORAGE_KEY = "privacy2::vault-notes";

function isSecureCtx(): boolean {
  return (
    typeof window !== "undefined" &&
    window.isSecureContext &&
    typeof crypto !== "undefined" &&
    typeof crypto.subtle !== "undefined"
  );
}

function loadNotes(): VaultNote[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw);
      if (Array.isArray(parsed) && parsed.length > 0) return parsed as VaultNote[];
    }
  } catch {
    // fall through
  }
  return demoVaultNotes;
}

function saveNotes(notes: VaultNote[]) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(notes));
  } catch {
    // storage quota or unavailable — silently ignore
  }
}

export function useVault() {
  const [notes, setNotes] = useState<VaultNote[]>(loadNotes);
  const [busy, setBusy] = useState(false);
  const [passphrase, setPassphrase] = useState("");
  const [key, setKey] = useState<CryptoKey | null>(null);
  const [status, setStatus] = useState("Locked");
  const [error, setError] = useState<string | null>(null);

  // Stable refs so async callbacks always see the latest values
  const notesRef = useRef(notes);
  notesRef.current = notes;
  const keyRef = useRef(key);
  keyRef.current = key;

  return useMemo(
    () => ({
      notes,
      busy,
      passphrase,
      key,
      status,
      error,
      isSecureCtx: isSecureCtx(),
      setPassphrase(v: string) {
        setPassphrase(v);
        setError(null);
      },
      async unlock() {
        if (!isSecureCtx()) {
          setError(
            "Web Crypto requires a secure context (HTTPS or localhost). Encryption is unavailable here."
          );
          return false;
        }
        if (!passphrase.trim()) {
          setError("Enter a passphrase first.");
          return false;
        }
        setBusy(true);
        setStatus("Deriving key…");
        setError(null);
        try {
          const k = await deriveKey(passphrase);
          setKey(k);
          setStatus("Unlocked");
          return true;
        } catch (e) {
          const msg = e instanceof Error ? e.message : "Unlock failed";
          setError(msg);
          setStatus("Locked");
          return false;
        } finally {
          setBusy(false);
        }
      },
      lock() {
        setKey(null);
        setStatus("Locked");
        setError(null);
      },
      async saveNote(title: string, body: string) {
        const t = title.trim();
        const b = body.trim();
        if (!t) {
          setError("Note title cannot be empty.");
          return null;
        }
        const note: VaultNote = {
          id: randomId("note"),
          title: t,
          body: b,
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          encrypted: !!keyRef.current,
        };
        const next = [note, ...notesRef.current];
        setNotes(next);
        saveNotes(next);
        setError(null);
        return note;
      },
      async sealNote(
        note: VaultNote
      ): Promise<SecureBlob<{ title: string; body: string }>> {
        const k = keyRef.current;
        if (!k) {
          const msg = "Unlock the vault first before sealing notes.";
          setError(msg);
          throw new Error(msg);
        }
        const blob = await encryptJson(
          { title: note.title, body: note.body },
          k,
          note.id
        );
        const updated: VaultNote = {
          ...note,
          encrypted: true,
          checksum: blob.checksum,
          updatedAt: new Date().toISOString(),
        };
        const next = [updated, ...notesRef.current.filter((n) => n.id !== note.id)];
        setNotes(next);
        saveNotes(next);
        return blob;
      },
      async openBlob(blob: SecureBlob<{ title: string; body: string }>) {
        const k = keyRef.current;
        if (!k) {
          const msg = "Unlock the vault first to read sealed notes.";
          setError(msg);
          throw new Error(msg);
        }
        return decryptJson(blob, k);
      },
      clear() {
        setNotes([]);
        saveNotes([]);
        setStatus("Cleared");
        setError(null);
      },
      clearError() {
        setError(null);
      },
    }),
    [notes, busy, passphrase, key, status, error]
  );
}
