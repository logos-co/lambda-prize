import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { RedactionPreview } from "../components/RedactionPreview";
import { usePrivacySettingsNew } from "../hooks/usePrivacySettingsNew";

export function RedactionPage() {
  const { settings, patch } = usePrivacySettingsNew();

  return (
    <AppShell>
      <PageHero
        badge="Redaction"
        title="Show the difference between raw data and safe previews."
        description="This page makes privacy visible by comparing what the app holds versus what the user is actually allowed to see. Toggle each field to see the effect live."
        primary={{ to: "/privacy-center", label: "Privacy center" }}
        secondary={{ to: "/sharing", label: "Sharing" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <GlassCard>
          <p className="text-sm font-semibold text-white mb-4">
            Reveal toggles
          </p>
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            {(
              [
                { key: "showRawAddresses", label: "Raw addresses" },
                { key: "showRawAmounts", label: "Raw amounts" },
                { key: "showTxHashes", label: "Tx hashes" },
                { key: "showMessageBodies", label: "Message bodies" },
              ] as const
            ).map(({ key, label }) => (
              <label
                key={key}
                className="flex items-center justify-between rounded-2xl border border-white/10 bg-slate-950/50 p-4 cursor-pointer"
              >
                <span className="text-sm text-slate-300">{label}</span>
                <input
                  type="checkbox"
                  checked={settings[key]}
                  onChange={(e) => patch({ [key]: e.target.checked })}
                  className="h-4 w-4 accent-cyan-300"
                />
              </label>
            ))}
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-6 pb-24 lg:px-8">
        <RedactionPreview settings={settings} />
      </section>
    </AppShell>
  );
}
