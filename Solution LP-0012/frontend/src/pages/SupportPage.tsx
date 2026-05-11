import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { FAQ } from "../components/FAQ";
import { faqItems } from "../lib/site";
import { LifeBuoy, Bug, FileWarning, MessageSquareText } from "lucide-react";

const supportCards = [
  {
    icon: <Bug className="h-5 w-5" />,
    title: "Bug reports",
    body: "Include the page route, steps to reproduce, and expected vs. actual behavior. Never include raw private keys or seed phrases.",
  },
  {
    icon: <FileWarning className="h-5 w-5" />,
    title: "Safety notes",
    body: "Flag anything that could expose sensitive data, break the privacy story, or bypass the witness commitment boundary.",
  },
  {
    icon: <MessageSquareText className="h-5 w-5" />,
    title: "Feedback",
    body: "Use this route to collect product feedback and visual suggestions. Specific, actionable feedback is most useful.",
  },
  {
    icon: <LifeBuoy className="h-5 w-5" />,
    title: "Help resources",
    body: "Link docs, examples, issue templates, or community channels here when the site is ready for a broader audience.",
  },
];

export function SupportPage() {
  return (
    <AppShell>
      <PageHero
        badge="Support"
        title="Make the project easier to use and easier to maintain."
        description="Support content keeps the frontend honest: it shows visitors how to report problems, escalate safely, and find the next step."
        primary={{ to: "/changelog", label: "Open changelog" }}
        secondary={{ to: "/docs", label: "Read docs" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 lg:px-8">
        <div className="grid gap-5 md:grid-cols-2">
          {supportCards.map((item) => (
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

      <section className="mx-auto max-w-5xl px-6 pb-24 lg:px-8">
        <FAQ items={faqItems} />
      </section>
    </AppShell>
  );
}
