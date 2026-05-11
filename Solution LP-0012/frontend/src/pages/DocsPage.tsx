import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { ScrollText, Blocks, TerminalSquare, BookOpenText } from "lucide-react";

const docs = [
  {
    icon: <ScrollText className="h-5 w-5" />,
    title: "Event system",
    body: "A section for the structured event model, wire format, and receipt flow from the Cryptarchia-LLL crate.",
  },
  {
    icon: <Blocks className="h-5 w-5" />,
    title: "SDK usage",
    body: "How programs and apps can emit, store, and inspect leadership proofs and nullifier data safely.",
  },
  {
    icon: <TerminalSquare className="h-5 w-5" />,
    title: "CLI guide",
    body: "Commands for decoding VRF outputs, running the slot lottery, and troubleshooting epoch transitions.",
  },
  {
    icon: <BookOpenText className="h-5 w-5" />,
    title: "Spec notes",
    body: "Space for formal spec pages, PoL v2 diagrams, and future Groth16/Plonk protocol details.",
  },
];

export function DocsPage() {
  return (
    <AppShell>
      <PageHero
        badge="Docs"
        title="A clear entry point for technical readers"
        description="This page becomes a structured hub for specifications, guides, API notes, and future documentation as the crate evolves."
        primary={{ to: "/examples", label: "See examples" }}
        secondary={{ to: "/", label: "Back home" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {docs.map((item) => (
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
