import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { SectionHeading } from "../components/SectionHeading";
import { Timeline } from "../components/Timeline";
import { GlassCard } from "../components/GlassCard";
import { featureGrid } from "../lib/site";

const architecture = [
  {
    phase: "01",
    title: "Presentation layer",
    body: "The landing page and overview routes frame the project as a readable system with clear entry points.",
  },
  {
    phase: "02",
    title: "Docs layer",
    body: "Dedicated pages hold specs, concepts, walkthroughs, and architecture notes for builders.",
  },
  {
    phase: "03",
    title: "Demo layer",
    body: "The simulator and examples pages show how the system behaves in motion.",
  },
  {
    phase: "04",
    title: "Privacy layer",
    body: "Witness commitments, nullifier tracking, and VRF outputs kept separate from the public API surface.",
  },
];

export function OverviewPage() {
  return (
    <AppShell>
      <PageHero
        badge="Project overview"
        title="A compact map of the system and its direction"
        description="This page explains how the frontend can evolve from a single landing screen into a full product narrative with clearer entry points and deeper docs."
        primary={{ to: "/privacy", label: "Explore privacy" }}
        secondary={{ to: "/", label: "Back home" }}
      />

      <section className="mx-auto max-w-7xl px-6 pb-20 lg:px-8">
        <SectionHeading
          eyebrow="Architecture"
          title="Built in layers"
          description="Each part of the site has a role: introduce, explain, demonstrate, support, and evolve."
        />
        <div className="mt-14">
          <Timeline items={architecture} />
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-3">
          {featureGrid.map((feature) => (
            <GlassCard key={feature.title}>
              <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                {feature.icon}
              </div>
              <h3 className="mt-5 text-xl font-semibold text-white">{feature.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{feature.description}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
