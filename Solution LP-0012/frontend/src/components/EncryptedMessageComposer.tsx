import React, { useState } from "react";
import type { MessageEnvelope, PrivacySettings } from "../types/privacy";
import { encryptJson } from "../lib/crypto";
import { sanitizeLabel } from "../lib/validation";

type Props = {
  settings: PrivacySettings;
  encryptionKey?: CryptoKey;
  onCreate: (message: MessageEnvelope) => void;
};

const BODY_MAX = 500;

export function EncryptedMessageComposer({ settings, encryptionKey, onCreate }: Props) {
  const [subject, setSubject] = useState("");
  const [body, setBody]       = useState("");
  const [recipient, setRecipient] = useState("");
  const [saving, setSaving]   = useState(false);
  const [error, setError]     = useState<string | null>(null);

  const isEncrypted = settings.localEncryptionEnabled && !!encryptionKey;
  const bodyCharsLeft = BODY_MAX - body.length;
  const bodyNearLimit = bodyCharsLeft < 80;

  async function submit() {
    setSaving(true);
    setError(null);
    try {
      const envelope: MessageEnvelope = {
        id: crypto.randomUUID(),
        createdAt: new Date().toISOString(),
        to: recipient ? (recipient as `0x${string}`) : undefined,
        subject: sanitizeLabel(subject),
        privacyLevel: settings.preferredPrivacyLevel,
        tags: ["private-message"],
      };

      if (isEncrypted && encryptionKey) {
        const sealed = await encryptJson(
          { body, subject: envelope.subject, recipient: envelope.to },
          encryptionKey,
          "private-message"
        );
        envelope.ciphertext = sealed.ciphertext;
        envelope.plaintextPreview = undefined;
      } else {
        envelope.plaintextPreview = body.slice(0, 60);
      }

      onCreate(envelope);
      setSubject("");
      setBody("");
      setRecipient("");
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : "Unable to create message");
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="card p-5">
      {/* Header */}
      <div className="flex items-start justify-between gap-3 mb-4">
        <div>
          <h3 className="text-base font-semibold text-slate-100">
            Encrypted message composer
          </h3>
          <p className="mt-0.5 text-xs text-slate-500">
            {isEncrypted
              ? "Messages are AES-GCM encrypted locally before leaving the browser."
              : "Unlock a local key to enable end-to-end encryption."}
          </p>
        </div>
        <div className="flex flex-col items-end gap-1 shrink-0">
          <span
            className={
              isEncrypted ? "badge-emerald" : "badge-amber"
            }
          >
            {isEncrypted ? "🔒 encrypted" : "plaintext"}
          </span>
          {isEncrypted && (
            <span className="text-[10px] text-slate-600 font-mono">
              AES-256-GCM
            </span>
          )}
        </div>
      </div>

      {/* Fields */}
      <div className="space-y-2.5">
        <input
          value={recipient}
          onChange={(e) => setRecipient(e.target.value)}
          placeholder="Recipient address (optional)"
          className="input-dark w-full"
          aria-label="Recipient address"
        />
        <input
          value={subject}
          onChange={(e) => setSubject(e.target.value)}
          placeholder="Subject"
          className="input-dark w-full"
          aria-label="Subject"
        />
        <div className="relative">
          <textarea
            value={body}
            onChange={(e) => setBody(e.target.value.slice(0, BODY_MAX))}
            placeholder="Write a private message…"
            rows={4}
            className="input-dark w-full resize-none pb-6"
            aria-label="Message body"
            aria-describedby="body-counter"
          />
          <div
            id="body-counter"
            className={`absolute bottom-2 right-3 text-[10px] tabular-nums transition-colors ${
              bodyNearLimit
                ? bodyCharsLeft < 20
                  ? "text-rose-400"
                  : "text-amber-400"
                : "text-slate-600"
            }`}
          >
            {bodyCharsLeft} / {BODY_MAX}
          </div>
        </div>
      </div>

      {/* Error */}
      {error ? (
        <p role="alert" className="mt-3 text-sm text-rose-400 flex items-center gap-1.5">
          <span aria-hidden="true">✕</span> {error}
        </p>
      ) : null}

      {/* Footer */}
      <div className="mt-4 flex items-center justify-between gap-3">
        {/* Clear */}
        {(subject || body || recipient) ? (
          <button
            onClick={() => { setSubject(""); setBody(""); setRecipient(""); setError(null); }}
            className="btn-ghost text-xs px-3 py-1.5"
          >
            Clear
          </button>
        ) : (
          <span className="text-xs text-slate-700">
            {isEncrypted ? "Key active" : "No key loaded"}
          </span>
        )}

        <button
          onClick={submit}
          disabled={saving || (!subject.trim() && !body.trim())}
          className="btn-primary"
        >
          {saving ? (
            <span className="flex items-center gap-2">
              <span className="inline-block h-3 w-3 animate-spin rounded-full border-2 border-white border-t-transparent" />
              Saving…
            </span>
          ) : (
            <span className="flex items-center gap-1.5">
              {isEncrypted ? "🔒" : ""}
              Create message
            </span>
          )}
        </button>
      </div>
    </div>
  );
}
