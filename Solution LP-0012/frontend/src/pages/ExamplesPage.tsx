import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { Code2, PackageSearch, Blocks, Rocket } from "lucide-react";

const examples = [
  {
    icon: <Code2 className="h-5 w-5" />,
    title: "VRF prove & verify",
    body: "A snippet showing how to prove leadership for a slot and verify the output using the Cryptarchia-LLL VRF API.",
  },
  {
    icon: <Blocks className="h-5 w-5" />,
    title: "PoL v2 claim cycle",
    body: "A simple example that creates a witness commitment, builds public inputs, and verifies the claim commitment.",
  },
  {
    icon: <PackageSearch className="h-5 w-5" />,
    title: "Nullifier tracking",
    body: "A local nullifier set example for preventing double-leadership across slots in a single epoch.",
  },
  {
    icon: <Rocket className="h-5 w-5" />,
    title: "End-to-end epoch flow",
    body: "A guided path from genesis seed, through slot lottery evaluation, to epoch advance and nonce rotation.",
  },
];

export function ExamplesPage() {
  return (
    <AppShell>
      <PageHero
        badge="Examples"
        title="Practical, story-driven code examples"
        description="Use this page to explore real usage patterns, walkthroughs, and small code snippets that help builders get started with the crate."
        primary={{ to: "/roadmap", label: "See roadmap" }}
        secondary={{ to: "/docs", label: "Read docs" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {examples.map((item) => (
            <GlassCard key={item.title}>
              <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-cyan-400/10 text-cyan-300 ring-1 ring-cyan-300/20">
                {item.icon}
              </div>
              <h3 className="mt-5 text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </GlassCard>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
