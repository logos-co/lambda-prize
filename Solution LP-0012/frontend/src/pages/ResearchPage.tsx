import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { researchNotes } from "../lib/mock-data";

export function ResearchPage() {
  return (
    <AppShell>
      <PageHero
        badge="Research"
        title="Open questions for future UI and privacy work."
        description="A research page makes the project feel alive by surfacing what is still being explored — questions without clean answers yet."
        primary={{ to: "/privacy-center", label: "Privacy center" }}
        secondary={{ to: "/community", label: "Community" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 pb-24 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {researchNotes.map((item) => (
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
