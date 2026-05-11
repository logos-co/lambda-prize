import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { Timeline } from "../components/Timeline";
import { GlassCard } from "../components/GlassCard";
import { Crown, GitBranch, ShieldAlert, Sparkles } from "lucide-react";

const steps = [
  {
    phase: "01",
    title: "Local eligibility",
    body: "A node checks whether it is eligible to lead for the current slot using only local state — no network broadcast needed.",
  },
  {
    phase: "02",
    title: "Private selection",
    body: "The winner is determined through a VRF-based local lottery. The result is deterministic and verifiable but hides identity.",
  },
  {
    phase: "03",
    title: "Commitment creation",
    body: "A claim commitment is created before revealing any sensitive details. Identity and stake move into witness-only storage.",
  },
  {
    phase: "04",
    title: "Verification",
    body: "Observers validate the proof against the commitment without needing the leader's raw identity or total stake in the public view.",
  },
];

export function LeadershipPage() {
  return (
    <AppShell>
      <PageHero
        badge="Leadership"
        title="A conceptual view of proposer selection"
        description="This page helps visitors understand how leadership is selected, announced, and verified without turning the UI into a wall of cryptographic jargon."
        primary={{ to: "/proof-of-leadership", label: "Read PoL" }}
        secondary={{ to: "/simulator", label: "Open simulator" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <Timeline items={steps} />
      </section>

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-4">
          <GlassCard>
            <Crown className="h-5 w-5 text-amber-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Leadership</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">
              The selected proposer is a cryptographic role, not a public identity leak.
            </p>
          </GlassCard>
          <GlassCard>
            <GitBranch className="h-5 w-5 text-fuchsia-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Branching paths</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">
              Separate flows for validators, observers, and operators keep the UI clean.
            </p>
          </GlassCard>
          <GlassCard>
            <ShieldAlert className="h-5 w-5 text-cyan-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Safety</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">
              Public information is enough to verify, but not enough to expose private identity.
            </p>
          </GlassCard>
          <GlassCard>
            <Sparkles className="h-5 w-5 text-emerald-300" />
            <h3 className="mt-5 text-xl font-semibold text-white">Clarity</h3>
            <p className="mt-3 text-sm leading-7 text-slate-300">
              A complex protocol idea expressed as a visual story in four steps.
            </p>
          </GlassCard>
        </div>
      </section>
    </AppShell>
  );
}
