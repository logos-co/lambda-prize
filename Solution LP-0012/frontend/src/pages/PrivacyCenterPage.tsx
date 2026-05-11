import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { SectionHeading } from "../components/SectionHeading";
import { ConsentMatrixEditor } from "../components/ConsentMatrixEditor";
import { PrivacyProfileCard } from "../components/PrivacyProfileCard";
import { useConsentMatrix } from "../hooks/useConsentMatrix";
import { usePrivacySettingsNew } from "../hooks/usePrivacySettingsNew";
import { privacySummary } from "../lib/privacy-defaults";
import { Shield, Lock, Eye, MessageSquareText } from "lucide-react";

export function PrivacyCenterPage() {
  const { consent, set, reset: resetConsent } = useConsentMatrix();
  const { settings, patch } = usePrivacySettingsNew();

  return (
    <AppShell>
      <PageHero
        badge="Privacy center"
        title="Control what the UI reveals, stores, and shares."
        description="Defaults, consent scopes, privacy level, and local-only behavior — all in one place. Changes save automatically to your browser."
        primary={{ to: "/vault", label: "Open vault" }}
        secondary={{ to: "/sharing", label: "Review sharing" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <GlassCard>
          <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
            <div>
              <h3 className="text-xl font-semibold text-white">
                Current privacy posture
              </h3>
              <p className="mt-2 text-sm text-slate-300">
                {privacySummary(settings)}
              </p>
            </div>
            <button
              onClick={resetConsent}
              className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10 self-start md:self-auto"
            >
              Reset consent
            </button>
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <SectionHeading
          eyebrow="Consent"
          title="Scope-level controls"
          description="Each data type gets its own rule so you can permit what is needed and refuse the rest."
        />
        <div className="mt-10">
          <ConsentMatrixEditor consent={consent} onChange={set} />
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-14 lg:px-8">
        <SectionHeading
          eyebrow="Privacy level"
          title="Switch the interface between public, private, and confidential"
          description="A single global level helps adjust the whole interface toward more or less revealing defaults."
        />
        <div className="mt-10">
          <PrivacyProfileCard
            level={settings.preferredLevel}
            onSelect={(level) => patch({ preferredLevel: level })}
          />
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-6 pb-24 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-4">
          {[
            { icon: <Shield className="h-5 w-5 text-cyan-300" />, title: "Redaction first", body: "Sensitive fields are hidden by default." },
            { icon: <Lock className="h-5 w-5 text-emerald-300" />, title: "Local encryption", body: "The browser keeps encrypted material sealed locally via AES-GCM." },
            { icon: <Eye className="h-5 w-5 text-fuchsia-300" />, title: "Progressive disclosure", body: "Reveal only what the current task needs." },
            { icon: <MessageSquareText className="h-5 w-5 text-amber-300" />, title: "Consent clarity", body: "Every permission is understandable and reversible." },
          ].map((item) => (
            <GlassCard key={item.title}>
              {item.icon}
              <h3 className="mt-5 text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
