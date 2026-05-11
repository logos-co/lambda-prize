import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { SectionHeading } from "../components/SectionHeading";
import { GlassCard } from "../components/GlassCard";
import { Lock, Shield, ScanEye, Fingerprint } from "lucide-react";

const privacySections = [
  {
    icon: <Lock className="h-5 w-5" />,
    title: "Redaction first",
    body: "Sensitive values are hidden by default. Identity and total stake live only in witness commitments — they never reach the public API.",
  },
  {
    icon: <Shield className="h-5 w-5" />,
    title: "Private-by-default UX",
    body: "The interface guides users toward safer defaults. Validators prove leadership without leaking their identity or stake weight.",
  },
  {
    icon: <ScanEye className="h-5 w-5" />,
    title: "Selective visibility",
    body: "Different viewers see different levels of detail. Proof verifiers only receive the minimal public inputs required for verification.",
  },
  {
    icon: <Fingerprint className="h-5 w-5" />,
    title: "Traceable without oversharing",
    body: "The frontend exposes provenance and verification cues — claim commitments, nullifier hashes, VRF outputs — without showing raw private values.",
  },
];

export function PrivacyPage() {
  return (
    <AppShell>
      <PageHero
        badge="Privacy"
        title="How the UI protects sensitive information while staying usable"
        description="This page turns privacy into a visible design principle. It explains why redaction, selective disclosure, and secure defaults belong in the front end."
        primary={{ to: "/leadership", label: "See leadership" }}
        secondary={{ to: "/overview", label: "Read overview" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Principles"
          title="The UI should reveal only what it needs to"
          description="A privacy-aware interface can still be elegant, friendly, and informative."
        />
        <div className="mt-14 grid gap-5 md:grid-cols-2">
          {privacySections.map((item) => (
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
          <h3 className="text-2xl font-semibold text-white">UI privacy checklist</h3>
          <ul className="mt-6 space-y-4 text-sm leading-7 text-slate-300">
            <li>• Show commitments and summaries before raw records.</li>
            <li>• Keep private fields visually distinct from public commitment outputs.</li>
            <li>• Explain why information is hidden, not just that it is hidden.</li>
            <li>• Avoid putting secrets into screenshots, logs, or default previews.</li>
            <li>• Derive nullifiers from secrets — never expose the raw secret itself.</li>
          </ul>
        </div>
      </section>
    </AppShell>
  );
}
