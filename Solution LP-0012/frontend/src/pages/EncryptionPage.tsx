import React, { useState } from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { CodeBlock } from "../components/CodeBlock";
import { deriveKey, encryptJson, decryptJson } from "../lib/privacy-crypto";
import { AlertTriangle, CheckCircle2, Lock, ShieldCheck, Copy, XCircle } from "lucide-react";
import { useToasts } from "../hooks/useToasts";

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
          or localhost. Encryption operations are unavailable here.
        </span>
      </div>
    </div>
  );
}

type StatusKind = "idle" | "busy" | "ok" | "error";

function StatusBadge({ text, kind }: { text: string; kind: StatusKind }) {
  const styles: Record<StatusKind, string> = {
    idle: "border-white/10 bg-white/5 text-slate-300",
    busy: "border-cyan-500/30 bg-cyan-500/10 text-cyan-200",
    ok: "border-emerald-500/30 bg-emerald-500/10 text-emerald-200",
    error: "border-rose-500/30 bg-rose-500/10 text-rose-200",
  };
  const icons: Record<StatusKind, React.ReactNode> = {
    idle: null,
    busy: <span className="live-dot bg-cyan-400" />,
    ok: <CheckCircle2 className="h-3.5 w-3.5" />,
    error: <XCircle className="h-3.5 w-3.5" />,
  };
  return (
    <span
      className={`inline-flex items-center gap-1.5 rounded-full border px-4 py-2 text-sm ${styles[kind]}`}
    >
      {icons[kind]}
      {text}
    </span>
  );
}

export function EncryptionPage() {
  const { push } = useToasts();
  const [passphrase, setPassphrase] = useState("");
  const [secret, setSecret] = useState("Sensitive payload kept local.");
  const [ciphertext, setCiphertext] = useState("");
  const [plain, setPlain] = useState("");
  const [status, setStatus] = useState("Idle");
  const [statusKind, setStatusKind] = useState<StatusKind>("idle");

  function setMsg(text: string, kind: StatusKind) {
    setStatus(text);
    setStatusKind(kind);
  }

  async function seal(e: React.FormEvent) {
    e.preventDefault();
    if (!passphrase.trim()) {
      setMsg("Enter a passphrase first", "error");
      return;
    }
    try {
      setMsg("Deriving key…", "busy");
      const key = await deriveKey(passphrase);
      setMsg("Encrypting…", "busy");
      const blob = await encryptJson({ secret }, key, "demo-secret");
      setCiphertext(JSON.stringify(blob, null, 2));
      setPlain("");
      setMsg("Encrypted locally ✓", "ok");
    } catch (e) {
      const msg = e instanceof Error ? e.message : "Encryption failed";
      setMsg(msg, "error");
      push({ kind: "error", title: "Encryption error", body: msg, duration: 4000 });
    }
  }

  async function openCipher() {
    if (!ciphertext.trim()) {
      setMsg("Encrypt something first", "error");
      return;
    }
    try {
      setMsg("Deriving key…", "busy");
      const key = await deriveKey(passphrase);
      const blob = JSON.parse(ciphertext);
      const data = await decryptJson(blob, key);
      setPlain(JSON.stringify(data, null, 2));
      setMsg("Decrypted locally ✓", "ok");
    } catch (e) {
      const msg =
        e instanceof SyntaxError
          ? "Invalid ciphertext — not valid JSON."
          : e instanceof Error
            ? e.message
            : "Decryption failed — wrong passphrase?";
      setMsg(msg, "error");
      push({ kind: "error", title: "Decryption error", body: msg, duration: 4000 });
    }
  }

  async function copyCipher() {
    if (!ciphertext) {
      setMsg("Nothing to copy — encrypt first", "error");
      return;
    }
    try {
      await navigator.clipboard.writeText(ciphertext);
      setMsg("Blob copied to clipboard ✓", "ok");
      push({ kind: "success", title: "Copied to clipboard", duration: 2000 });
    } catch {
      const msg = "Clipboard access denied — copy the text manually.";
      setMsg(msg, "error");
      push({ kind: "warn", title: "Clipboard unavailable", body: msg, duration: 4000 });
    }
  }

  const isSecure =
    typeof window === "undefined" ||
    (window.isSecureContext && typeof crypto?.subtle !== "undefined");

  return (
    <AppShell>
      <PageHero
        badge="Encryption"
        title="Local encryption that stays in the browser."
        description="Passphrase-derived AES-GCM keys seal and re-open data without sending any plaintext elsewhere. Try it live — everything runs in Web Crypto."
        primary={{ to: "/vault", label: "Open vault" }}
        secondary={{ to: "/security", label: "Security notes" }}
      />

      <InsecureContextBanner />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <GlassCard>
          <form onSubmit={seal}>
            <input type="hidden" name="username" autoComplete="username" value="encryption-demo" readOnly />
            <div className="grid gap-4 md:grid-cols-3">
              <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
                <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                  Passphrase
                </span>
                <input
                  value={passphrase}
                  onChange={(e) => {
                    setPassphrase(e.target.value);
                    if (statusKind === "error") setMsg("Idle", "idle");
                  }}
                  type="password"
                  name="enc-passphrase"
                  autoComplete="current-password"
                  placeholder="Enter passphrase… (min 8 chars)"
                  className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none focus:border-cyan-400/40 transition"
                />
              </label>
              <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4 md:col-span-2">
                <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                  Secret payload
                </span>
                <input
                  value={secret}
                  onChange={(e) => {
                    setSecret(e.target.value);
                    if (statusKind === "error") setMsg("Idle", "idle");
                  }}
                  className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none focus:border-cyan-400/40 transition"
                />
              </label>
            </div>

            <div className="mt-5 flex flex-wrap items-center gap-3">
              <button
                type="submit"
                disabled={!isSecure}
                title={isSecure ? undefined : "Requires HTTPS or localhost"}
                className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <Lock className="h-4 w-4" /> Encrypt
              </button>
              <button
                type="button"
                onClick={openCipher}
                disabled={!isSecure}
                title={isSecure ? undefined : "Requires HTTPS or localhost"}
                className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                <ShieldCheck className="h-4 w-4" /> Decrypt
              </button>
              <button
                type="button"
                onClick={copyCipher}
                className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10"
              >
                <Copy className="h-4 w-4" /> Copy blob
              </button>
              <StatusBadge text={status} kind={statusKind} />
            </div>
          </form>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-6 pb-24 lg:px-8">
        <div className="grid gap-5 lg:grid-cols-2">
          <CodeBlock
            title="Encrypted blob (AES-GCM)"
            code={ciphertext || "Run Encrypt to generate a secure blob."}
          />
          <CodeBlock
            title="Decrypted payload"
            code={
              plain ||
              "Run Decrypt (with the same passphrase) to view the JSON payload."
            }
          />
        </div>
      </section>
    </AppShell>
  );
}
