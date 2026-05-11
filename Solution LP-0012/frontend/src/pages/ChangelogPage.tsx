import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { Timeline } from "../components/Timeline";

const changes = [
  {
    phase: "v0.1",
    title: "Landing page",
    body: "Introduced the cosmic home screen, animated hero, feature grid, manifesto, and navigation shell.",
  },
  {
    phase: "v0.2",
    title: "Proof-of-Leadership v2",
    body: "Added pol module with witness-only commitments, Ed25519 VRF, nullifier set, and 48-test suite.",
  },
  {
    phase: "v0.3",
    title: "Multi-page expansion",
    body: "Added overview, privacy, leadership, proofs, simulator, docs, examples, roadmap, and changelog routes with shared components.",
  },
  {
    phase: "v1.0",
    title: "ZK-ready portal",
    body: "Prepared the frontend to host live metrics, Groth16/Plonk backend status, and community content.",
  },
];

export function ChangelogPage() {
  return (
    <AppShell>
      <PageHero
        badge="Changelog"
        title="A visible history of improvements"
        description="This page gives the site a release narrative and makes future changes easier to understand at a glance."
        primary={{ to: "/", label: "Home" }}
        secondary={{ to: "/roadmap", label: "Roadmap" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <Timeline items={changes} />
      </section>
    </AppShell>
  );
}
