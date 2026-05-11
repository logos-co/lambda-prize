import React, { useState } from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { SecureField } from "../components/SecureField";
import { useVault } from "../hooks/useVault";
import { AlertTriangle, Lock, Save, Trash2, Eye } from "lucide-react";
import { useToasts } from "../hooks/useToasts";
import type { VaultNote } from "../lib/privacy-types";

function InsecureContextBanner() {
  if (
    typeof window === "undefined" ||
    (window.isSecureContext && typeof crypto?.subtle !== "undefined")
  )
    return null;
  return (
    <div className="mx-auto max-w-7xl px-6 pb-2 lg:px-8">
      <div className="flex items-start gap-3 rounded-2xl border border-amber-500/30 bg-amber-500/10 px-5 py-4 text-sm text-amber-200">
        <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0 text-amber-300" />
        <span>
          <strong>Insecure context:</strong> Web Crypto (AES-GCM) requires HTTPS
          or localhost. Encryption and decryption are unavailable in this
          environment.
        </span>
      </div>
    </div>
  );
}

export function VaultPage() {
  const vault = useVault();
  const { push } = useToasts();
  const [title, setTitle] = useState("");
  const [body, setBody] = useState("");
  const [titleError, setTitleError] = useState("");
  const [lastJson, setLastJson] = useState("");

  async function handleUnlock(e: React.FormEvent) {
    e.preventDefault();
    const ok = await vault.unlock();
    if (ok) push({ kind: "success", title: "Vault unlocked", duration: 2500 });
  }

  async function save() {
    if (!title.trim()) {
      setTitleError("Note title cannot be empty.");
      return;
    }
    setTitleError("");
    const note = await vault.saveNote(title.trim(), body.trim());
    if (note) {
      setTitle("");
      setBody("");
      push({ kind: "success", title: "Note saved", duration: 2000 });
    }
  }

  async function handleSeal(note: VaultNote) {
    try {
      await vault.sealNote(note);
      push({ kind: "success", title: "Note sealed with AES-GCM", duration: 2500 });
    } catch (e) {
      push({
        kind: "error",
        title: "Seal failed",
        body: e instanceof Error ? e.message : "Unknown error",
        duration: 4000,
      });
    }
  }

  return (
    <AppShell>
      <PageHero
        badge="Vault"
        title="Keep private notes sealed locally in the browser."
        description="Local-only AES-GCM encryption. Notes never leave your browser in plaintext — unlock with a passphrase to seal them cryptographically."
        primary={{ to: "/encryption", label: "Encryption demo" }}
        secondary={{ to: "/security", label: "Security notes" }}
      />

      <InsecureContextBanner />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <GlassCard>
          <div className="flex flex-col gap-4 md:flex-row md:items-start md:justify-between">
            <div>
              <h3 className="text-xl font-semibold text-white">Vault status</h3>
              <p className="mt-2 text-sm text-slate-300">{vault.status}</p>
              {vault.error ? (
                <p className="mt-2 flex items-center gap-1.5 text-sm text-rose-300">
                  <AlertTriangle className="h-3.5 w-3.5 shrink-0" />
                  {vault.error}
                </p>
              ) : null}
            </div>

            <form
              onSubmit={handleUnlock}
              className="flex flex-wrap gap-2"
            >
              <input type="hidden" name="username" autoComplete="username" value="vault" readOnly />
              <div className="rounded-2xl border border-white/10 bg-white/5 px-4 py-2">
                <input
                  type="password"
                  name="vault-passphrase"
                  autoComplete="current-password"
                  value={vault.passphrase}
                  onChange={(e) => vault.setPassphrase(e.target.value)}
                  placeholder="Passphrase…"
                  className="bg-transparent text-sm text-white outline-none placeholder:text-slate-500 w-40"
                />
              </div>
              <button
                type="submit"
                disabled={vault.busy || !vault.isSecureCtx}
                title={vault.isSecureCtx ? undefined : "Requires HTTPS or localhost"}
                className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <Lock className="h-4 w-4" />
                {vault.busy ? "Unlocking…" : "Unlock"}
              </button>
              <button
                type="button"
                onClick={() => vault.lock()}
                className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10"
              >
                Lock
              </button>
              <button
                type="button"
                onClick={() => {
                  vault.clear();
                  push({ kind: "info", title: "Vault cleared", duration: 2000 });
                }}
                className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10"
              >
                <Trash2 className="h-4 w-4" /> Clear
              </button>
            </form>
          </div>

          <div className="mt-5 grid gap-4 sm:grid-cols-2">
            <SecureField
              label="Passphrase"
              value={vault.passphrase || undefined}
              visible={false}
              copyable={false}
            />
            <SecureField
              label="Encryption key"
              value={vault.key ? "Active — AES-GCM 256-bit" : "Not loaded"}
              visible={true}
              copyable={false}
            />
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-6 pb-24 lg:px-8">
        <div className="grid gap-5 lg:grid-cols-[0.9fr_1.1fr]">
          <GlassCard>
            <h3 className="text-lg font-semibold text-white">Create note</h3>
            <div className="mt-4 space-y-3">
              <div>
                <input
                  value={title}
                  onChange={(e) => {
                    setTitle(e.target.value);
                    if (e.target.value.trim()) setTitleError("");
                  }}
                  placeholder="Title"
                  aria-invalid={!!titleError}
                  className={`w-full rounded-2xl border px-4 py-3 text-sm text-white outline-none transition ${
                    titleError
                      ? "border-rose-500/50 bg-rose-500/[0.06] focus:border-rose-400/60"
                      : "border-white/10 bg-slate-950 focus:border-cyan-400/40"
                  }`}
                />
                {titleError ? (
                  <p className="mt-1.5 flex items-center gap-1 text-xs text-rose-300">
                    <AlertTriangle className="h-3 w-3 shrink-0" />
                    {titleError}
                  </p>
                ) : null}
              </div>
              <textarea
                value={body}
                onChange={(e) => setBody(e.target.value)}
                placeholder="Private note — stays in the browser…"
                rows={7}
                className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none focus:border-cyan-400/40 transition resize-none"
              />
              <button
                onClick={save}
                className="inline-flex items-center gap-2 rounded-full bg-cyan-300 px-4 py-2 text-sm font-semibold text-slate-950 transition hover:bg-cyan-200"
              >
                <Save className="h-4 w-4" /> Save note
              </button>
            </div>
          </GlassCard>

          <GlassCard>
            <div className="flex items-center justify-between gap-3">
              <h3 className="text-lg font-semibold text-white">
                Saved notes ({vault.notes.length})
              </h3>
              <button
                onClick={() =>
                  setLastJson(JSON.stringify(vault.notes, null, 2))
                }
                className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1.5 text-xs text-slate-200 transition hover:bg-white/10"
              >
                <Eye className="h-3.5 w-3.5" /> Preview JSON
              </button>
            </div>

            {vault.notes.length === 0 ? (
              <div className="mt-6 rounded-2xl border border-white/8 bg-white/[0.03] py-10 text-center">
                <p className="text-sm text-slate-500">No notes yet — create one on the left.</p>
              </div>
            ) : (
              <div className="mt-4 space-y-3 max-h-64 overflow-y-auto">
                {vault.notes.map((note) => (
                  <div
                    key={note.id}
                    className="rounded-2xl border border-white/10 bg-slate-950/60 p-4"
                  >
                    <div className="flex items-center justify-between gap-3">
                      <h4 className="font-medium text-white text-sm">
                        {note.title}
                      </h4>
                      <div className="flex items-center gap-2">
                        <span className="text-xs text-slate-500">
                          {note.encrypted ? "🔒 Encrypted" : "Plaintext"}
                        </span>
                        {vault.key && !note.encrypted ? (
                          <button
                            onClick={() => handleSeal(note)}
                            className="rounded-full border border-cyan-500/30 bg-cyan-500/10 px-2.5 py-1 text-xs text-cyan-300 transition hover:bg-cyan-500/20"
                          >
                            Seal
                          </button>
                        ) : null}
                      </div>
                    </div>
                    <p className="mt-2 text-sm leading-7 text-slate-300 line-clamp-2">
                      {note.body}
                    </p>
                    {note.checksum ? (
                      <p className="mt-1 font-mono text-xs text-slate-600">
                        cksum: {note.checksum.slice(0, 16)}…
                      </p>
                    ) : null}
                  </div>
                ))}
              </div>
            )}

            {lastJson ? (
              <pre className="mt-4 overflow-auto rounded-2xl bg-black/40 p-3 text-xs text-slate-300 max-h-40">
                {lastJson}
              </pre>
            ) : null}
          </GlassCard>
        </div>
      </section>
    </AppShell>
  );
}
