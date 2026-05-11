import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { securityNotes } from "../lib/mock-data";

export function SecurityPage() {
  return (
    <AppShell>
      <PageHero
        badge="Security"
        title="Make trust boundaries visible in the frontend."
        description="A security page helps explain what the UI does not show, as much as what it does — and why those limits matter."
        primary={{ to: "/privacy-center", label: "Privacy center" }}
        secondary={{ to: "/support", label: "Support" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 pb-24 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {securityNotes.map((item) => (
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
