import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { ConsentMatrixEditor } from "../components/ConsentMatrixEditor";
import { useConsentMatrix } from "../hooks/useConsentMatrix";
import { CheckCircle2, ShieldOff, RefreshCcw } from "lucide-react";

export function ConsentPage() {
  const { consent, set, reset } = useConsentMatrix();

  return (
    <AppShell>
      <PageHero
        badge="Consent"
        title="Give every data scope its own rule."
        description="A safe privacy UI does not assume permission. It asks clearly, records the result, and makes every scope independently controllable."
        primary={{ to: "/privacy-center", label: "Privacy center" }}
        secondary={{ to: "/audit", label: "Audit log" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <div className="flex flex-wrap gap-3">
          <button
            onClick={reset}
            className="inline-flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-medium text-slate-950 transition hover:bg-cyan-100"
          >
            <RefreshCcw className="h-4 w-4" /> Reset defaults
          </button>
          <div className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300">
            <CheckCircle2 className="h-4 w-4 text-emerald-300" /> Ask-first baseline
          </div>
          <div className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300">
            <ShieldOff className="h-4 w-4 text-rose-300" /> Deny sensitive scopes
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-6 pb-24 lg:px-8">
        <GlassCard>
          <p className="text-sm text-slate-400 mb-6">
            Changes are saved automatically to local storage. They never leave your browser.
          </p>
          <ConsentMatrixEditor consent={consent} onChange={set} />
        </GlassCard>
      </section>
    </AppShell>
  );
}
