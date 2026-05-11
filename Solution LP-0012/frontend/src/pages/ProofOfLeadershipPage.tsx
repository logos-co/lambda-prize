import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { SectionHeading } from "../components/SectionHeading";
import { BadgeCheck, Fingerprint, Layers, LockKeyhole } from "lucide-react";

const proofBlocks = [
  {
    icon: <Fingerprint className="h-5 w-5" />,
    title: "Claim commitment",
    body: "Bind the leadership claim to a commitment before anything is revealed publicly. The commitment is deterministic for the same slot inputs.",
  },
  {
    icon: <LockKeyhole className="h-5 w-5" />,
    title: "Witness secrecy",
    body: "Identity and total stake live only in the witness. The prover holds them locally — they never touch the public proof interface.",
  },
  {
    icon: <Layers className="h-5 w-5" />,
    title: "Public inputs",
    body: "Publish only the minimal inputs required for verification: chain ID, slot, threshold commitment, and the claim commitment itself.",
  },
  {
    icon: <BadgeCheck className="h-5 w-5" />,
    title: "Verification",
    body: "Observers check the claim against the commitment without learning the identity or the raw stake total. Tampered commitments fail immediately.",
  },
];

export function ProofOfLeadershipPage() {
  return (
    <AppShell>
      <PageHero
        badge="Proof of Leadership"
        title="Verify leadership claims without exposing identity or total stake"
        description="This page explains the proof boundary: what is public, what is committed, and what remains private in the protocol narrative."
        primary={{ to: "/simulator", label: "Try simulator" }}
        secondary={{ to: "/privacy", label: "Privacy notes" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Proof shape"
          title="A proof should prove the claim, not publish the secret"
          description="The UI mirrors the protocol: a claim gets committed, proven, and verified while the sensitive values stay sealed in the witness."
        />

        <div className="mt-14 grid gap-5 md:grid-cols-2">
          {proofBlocks.map((item) => (
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

      <section className="mx-auto max-w-5xl px-6 py-20 lg:px-8">
        <div className="rounded-[2rem] border border-white/10 bg-white/5 p-8">
          <h3 className="text-2xl font-semibold text-white">Verification flow</h3>
          <ol className="mt-6 space-y-4 text-sm leading-7 text-slate-300">
            <li>1. A node computes leadership eligibility locally using VRF output and stake.</li>
            <li>2. The selected slot is bound to a claim commitment (deterministic hash).</li>
            <li>3. The proof exposes only the public verification inputs — no raw witness data.</li>
            <li>4. The verifier checks the claim commitment against the expected hash.</li>
            <li>5. Private identity and total stake remain hidden in the witness layer.</li>
          </ol>
        </div>
      </section>
    </AppShell>
  );
}
