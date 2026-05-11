import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { communityNotes } from "../lib/mock-data";

export function CommunityPage() {
  return (
    <AppShell>
      <PageHero
        badge="Community"
        title="A place for contributors, maintainers, and reviewers."
        description="The site gets easier to grow when contribution norms, review expectations, and governance paths are visible to everyone."
        primary={{ to: "/roadmap", label: "Roadmap" }}
        secondary={{ to: "/support", label: "Support" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 pb-24 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {communityNotes.map((item) => (
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
