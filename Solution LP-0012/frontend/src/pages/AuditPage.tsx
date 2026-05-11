import React, { useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { SectionHeading } from "../components/SectionHeading";
import { AuditLog } from "../components/AuditLog";
import type { AuditEvent } from "../lib/privacy-types";
import { demoEventRows } from "../lib/mock-data";

function rowToAudit(
  r: (typeof demoEventRows)[number],
  i: number
): AuditEvent {
  return {
    id: `audit_${i}_${r.title.toLowerCase().replace(/\W+/g, "_").slice(0, 18)}`,
    time: new Date(Date.now() - i * 3_600_000).toISOString(),
    category: r.tags.includes("consent")
      ? "consent"
      : r.tags.includes("vault") || r.tags.includes("sealed")
        ? "storage"
        : r.tags.includes("share") || r.tags.includes("export")
          ? "share"
          : "security",
    level:
      r.type === "error" ? "error" : r.type === "warning" ? "warn" : "info",
    title: r.title,
    summary: r.summary,
    details: r.details,
    tags: r.tags,
  };
}

export function AuditPage() {
  const [category, setCategory] = useState("all");
  const baseItems = useMemo(() => demoEventRows.map(rowToAudit), []);

  const items = useMemo(
    () =>
      category === "all"
        ? baseItems
        : baseItems.filter((e) => e.category === category),
    [baseItems, category]
  );

  return (
    <AppShell>
      <PageHero
        badge="Audit"
        title="A readable privacy trail."
        description="Every significant privacy action is logged locally. Filter by category to find the events that matter most."
        primary={{ to: "/events", label: "Event browser" }}
        secondary={{ to: "/privacy-center", label: "Privacy center" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Log"
          title="Privacy events — categorised and timestamped"
          description="Consent changes, vault seals, share attempts, and security flags all appear here."
        />

        <div className="mt-10 flex flex-wrap gap-2">
          {["all", "consent", "storage", "share", "security", "settings"].map(
            (cat) => (
              <button
                key={cat}
                onClick={() => setCategory(cat)}
                className={`rounded-full px-4 py-2 text-sm capitalize transition ${
                  category === cat
                    ? "bg-white text-slate-950 font-medium"
                    : "border border-white/10 bg-white/5 text-slate-300 hover:bg-white/10 hover:text-white"
                }`}
              >
                {cat}
              </button>
            )
          )}
        </div>

        <div className="mt-8">
          {items.length === 0 ? (
            <div className="rounded-3xl border border-white/8 bg-white/[0.03] py-20 text-center">
              <p className="text-sm text-slate-500">
                No events in the <span className="font-medium text-slate-300">{category}</span> category.
              </p>
            </div>
          ) : (
            <AuditLog items={items} />
          )}
        </div>
      </section>
    </AppShell>
  );
}
