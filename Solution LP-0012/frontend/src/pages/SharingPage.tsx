import React, { useState } from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { ShieldCheck, Copy, AlertTriangle, Share2, XCircle } from "lucide-react";
import { useToasts } from "../hooks/useToasts";

export function SharingPage() {
  const { push } = useToasts();
  const [scope, setScope] = useState("summary-only");
  const [recipient, setRecipient] = useState("support-team");
  const [note, setNote] = useState("Share only the event ids and approximate times.");
  const [copied, setCopied] = useState(false);
  const [clipError, setClipError] = useState("");

  async function copyLink() {
    const payload = `share://${recipient}?scope=${scope}&note=${encodeURIComponent(note)}`;
    try {
      await navigator.clipboard.writeText(payload);
      setCopied(true);
      setClipError("");
      push({ kind: "success", title: "Share link copied", duration: 2000 });
      setTimeout(() => setCopied(false), 2000);
    } catch {
      const msg = "Clipboard access denied — copy the link manually.";
      setClipError(msg);
      push({ kind: "warn", title: "Clipboard unavailable", body: msg, duration: 4000 });
    }
  }

  return (
    <AppShell>
      <PageHero
        badge="Sharing"
        title="Export data with a smaller blast radius."
        description="Safe sharing means choosing the minimal scope, narrowing the recipient, and adding an explicit note about intent before sending anything."
        primary={{ to: "/consent", label: "Consent controls" }}
        secondary={{ to: "/privacy-center", label: "Privacy center" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <GlassCard>
          <div className="grid gap-4 md:grid-cols-2">
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                Export scope
              </span>
              <select
                value={scope}
                onChange={(e) => { setScope(e.target.value); setClipError(""); }}
                className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
              >
                <option value="summary-only">Summary only</option>
                <option value="event-ids">Event IDs only</option>
                <option value="redacted-full">Redacted full export</option>
                <option value="raw">Raw (unsafe)</option>
              </select>
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                Recipient
              </span>
              <select
                value={recipient}
                onChange={(e) => { setRecipient(e.target.value); setClipError(""); }}
                className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
              >
                <option value="support-team">Support team</option>
                <option value="self">Self (backup)</option>
                <option value="auditor">External auditor</option>
              </select>
            </label>
          </div>

          <div className="mt-4">
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4 block">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                Intent note
              </span>
              <textarea
                value={note}
                onChange={(e) => setNote(e.target.value)}
                rows={3}
                className="mt-2 w-full bg-transparent text-sm text-white outline-none resize-none"
              />
            </label>
          </div>

          <div className="mt-5 flex flex-wrap items-start gap-3">
            <button
              onClick={copyLink}
              className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100"
            >
              {copied ? (
                <>
                  <ShieldCheck className="h-4 w-4 text-emerald-600" /> Copied!
                </>
              ) : (
                <>
                  <Copy className="h-4 w-4" /> Copy share link
                </>
              )}
            </button>
            <div className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300">
              <ShieldCheck className="h-4 w-4 text-emerald-300" /> Redacted by default
            </div>
            {scope === "raw" ? (
              <div className="inline-flex items-center gap-2 rounded-full border border-rose-500/30 bg-rose-500/10 px-4 py-2 text-sm text-rose-200">
                <AlertTriangle className="h-4 w-4" /> Raw export exposes sensitive values
              </div>
            ) : null}
            {clipError ? (
              <div className="inline-flex items-center gap-2 rounded-full border border-amber-500/30 bg-amber-500/10 px-4 py-2 text-sm text-amber-200">
                <XCircle className="h-4 w-4" /> {clipError}
              </div>
            ) : null}
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-6 pb-24 lg:px-8">
        <div className="grid gap-5 md:grid-cols-3">
          {[
            {
              icon: <Share2 className="h-5 w-5" />,
              title: "Progressive disclosure",
              body: "Share only what the recipient actually needs. Start with event IDs and expand only if required.",
            },
            {
              icon: <ShieldCheck className="h-5 w-5" />,
              title: "Signed bundles",
              body: "Bundles include a SHA-256 checksum. The receiving side can verify integrity before importing.",
            },
            {
              icon: <AlertTriangle className="h-5 w-5" />,
              title: "Raw export warning",
              body: "Raw exports bypass redaction. Only use them in controlled environments with trusted recipients.",
            },
          ].map((item) => (
            <div
              key={item.title}
              className="rounded-3xl border border-white/10 bg-white/5 p-6"
            >
              <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                {item.icon}
              </div>
              <h3 className="mt-5 text-xl font-semibold text-white">
                {item.title}
              </h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </div>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
