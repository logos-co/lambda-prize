import React, { useState } from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { PrivacyProfileCard } from "../components/PrivacyProfileCard";
import { SectionHeading } from "../components/SectionHeading";
import { usePrivacySettingsNew } from "../hooks/usePrivacySettingsNew";
import { privacySummary } from "../lib/privacy-defaults";

const settingsOptions = [
  {
    title: "Density",
    body: "Comfortable spacing is the default. Compact mode reduces padding for power users who want more content visible at once.",
  },
  {
    title: "Motion",
    body: "Subtle motion is on by default. Set to minimal for reduced-motion environments or accessibility preferences.",
  },
  {
    title: "Auto-lock",
    body: "The vault and sensitive views auto-lock after the configured idle period. 10 minutes is the safe default.",
  },
  {
    title: "Redaction by default",
    body: "All sensitive fields are redacted unless the user actively reveals them. This applies globally across all pages.",
  },
];

export function SettingsPage() {
  const { settings, patch, reset } = usePrivacySettingsNew();
  const [density, setDensity] = useState("comfortable");
  const [motion, setMotion] = useState("subtle");

  return (
    <AppShell>
      <PageHero
        badge="Settings"
        title="Density, motion, and privacy preferences."
        description="A single global control surface for how the interface looks, moves, and handles sensitive information."
        primary={{ to: "/privacy-center", label: "Privacy center" }}
        secondary={{ to: "/consent", label: "Consent controls" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <GlassCard>
          <div className="flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
            <div>
              <h3 className="text-xl font-semibold text-white">
                Current posture
              </h3>
              <p className="mt-2 text-sm text-slate-300">
                {privacySummary(settings)}
              </p>
            </div>
            <button
              onClick={reset}
              className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-200 transition hover:bg-white/10 self-start md:self-auto"
            >
              Reset to defaults
            </button>
          </div>

          <div className="mt-6 grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                Density
              </span>
              <select
                value={density}
                onChange={(e) => setDensity(e.target.value)}
                className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
              >
                <option value="comfortable">Comfortable</option>
                <option value="compact">Compact</option>
                <option value="spacious">Spacious</option>
              </select>
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                Motion
              </span>
              <select
                value={motion}
                onChange={(e) => setMotion(e.target.value)}
                className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
              >
                <option value="subtle">Subtle</option>
                <option value="minimal">Minimal</option>
                <option value="standard">Standard</option>
              </select>
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                Auto-lock (min)
              </span>
              <select
                value={settings.autoLockMinutes}
                onChange={(e) =>
                  patch({ autoLockMinutes: Number(e.target.value) })
                }
                className="mt-2 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
              >
                {[5, 10, 15, 30, 60].map((m) => (
                  <option key={m} value={m}>
                    {m} min
                  </option>
                ))}
              </select>
            </label>
            <label className="rounded-2xl border border-white/10 bg-slate-950/50 p-4">
              <span className="text-xs uppercase tracking-[0.25em] text-slate-500">
                Local encryption
              </span>
              <div className="mt-3">
                <input
                  type="checkbox"
                  checked={settings.localEncryptionEnabled}
                  onChange={(e) =>
                    patch({ localEncryptionEnabled: e.target.checked })
                  }
                  className="h-4 w-4 accent-cyan-300"
                />{" "}
                <span className="text-sm text-white">
                  {settings.localEncryptionEnabled ? "Enabled" : "Disabled"}
                </span>
              </div>
            </label>
          </div>

          <div className="mt-4 text-sm text-slate-400">
            Density: {density} • Motion: {motion} • Privacy:{" "}
            {settings.preferredLevel}
          </div>
        </GlassCard>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <SectionHeading
          eyebrow="Privacy level"
          title="Switch between public, private, and confidential modes"
          description="A single global level adjusts the whole interface toward more or less revealing defaults."
        />
        <div className="mt-10">
          <PrivacyProfileCard
            level={settings.preferredLevel}
            onSelect={(level) => patch({ preferredLevel: level })}
          />
        </div>
      </section>

      <section className="mx-auto max-w-5xl px-6 pb-24 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {settingsOptions.map((item) => (
            <GlassCard key={item.title}>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
