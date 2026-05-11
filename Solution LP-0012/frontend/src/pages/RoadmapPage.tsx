import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { Timeline } from "../components/Timeline";
import { roadmapItems } from "../lib/site";

export function RoadmapPage() {
  return (
    <AppShell>
      <PageHero
        badge="Roadmap"
        title="A path for expanding the system with more capabilities"
        description="The site is structured so it can keep growing without needing a redesign every time a new idea appears."
        primary={{ to: "/changelog", label: "View changelog" }}
        secondary={{ to: "/examples", label: "See examples" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <Timeline items={roadmapItems} />
      </section>
    </AppShell>
  );
}
