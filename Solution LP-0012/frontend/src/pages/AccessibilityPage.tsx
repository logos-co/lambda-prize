import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { accessibilityNotes, uiChecklist } from "../lib/mock-data";

export function AccessibilityPage() {
  return (
    <AppShell>
      <PageHero
        badge="Accessibility"
        title="Responsive UI that stays readable and keyboard-friendly."
        description="Privacy controls should be usable by everyone, including keyboard users and people on small screens or high-contrast displays."
        primary={{ to: "/support", label: "Support" }}
        secondary={{ to: "/overview", label: "Overview" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {accessibilityNotes.map((item) => (
            <GlassCard key={item.title}>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>

      <section className="mx-auto max-w-5xl px-6 pb-24 lg:px-8">
        <div className="rounded-[2rem] border border-white/10 bg-white/5 p-8">
          <h3 className="text-2xl font-semibold text-white">
            Implementation checklist
          </h3>
          <ul className="mt-6 space-y-4 text-sm leading-7 text-slate-300">
            {uiChecklist.map((item) => (
              <li key={item}>• {item}</li>
            ))}
          </ul>
        </div>
      </section>
    </AppShell>
  );
}
