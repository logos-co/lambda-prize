import React, { useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { SectionHeading } from "../components/SectionHeading";
import { demoEventRows } from "../lib/mock-data";
import { Search, Filter, ArrowUpDown } from "lucide-react";

const TONE_COLORS: Record<string, string> = {
  success: "bg-emerald-500/10 text-emerald-200",
  warning: "bg-amber-500/10 text-amber-200",
  info: "bg-cyan-500/10 text-cyan-200",
  error: "bg-rose-500/10 text-rose-200",
};

export function EventsPage() {
  const [query, setQuery] = useState("");
  const [filter, setFilter] = useState<"all" | "success" | "warning" | "info" | "error">("all");
  const [sort, setSort] = useState<"recent" | "alpha">("recent");

  const rows = useMemo(() => {
    const q = query.trim().toLowerCase();
    return [...demoEventRows]
      .filter((item) => (filter === "all" ? true : item.type === filter))
      .filter((item) => {
        if (!q) return true;
        return (
          item.title.toLowerCase().includes(q) ||
          item.summary.toLowerCase().includes(q) ||
          item.details.toLowerCase().includes(q) ||
          item.tags.some((tag) => tag.toLowerCase().includes(q))
        );
      })
      .sort((a, b) =>
        sort === "alpha" ? a.title.localeCompare(b.title) : b.id.localeCompare(a.id)
      );
  }, [query, filter, sort]);

  return (
    <AppShell>
      <PageHero
        badge="Events"
        title="A privacy-aware event browser."
        description="Filter, search, and sort the event feed. Sensitive field summaries replace raw values when redaction is active."
        primary={{ to: "/dashboard", label: "Dashboard" }}
        secondary={{ to: "/audit", label: "Audit" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Explorer"
          title="Filter, search, and sort the event feed"
          description="Find the event that matters without exposing fields that should stay hidden."
        />

        <div className="mt-10 grid gap-4 md:grid-cols-3">
          <div className="rounded-3xl border border-white/10 bg-white/5 p-4">
            <label className="mb-2 flex items-center gap-2 text-sm text-slate-300">
              <Search className="h-4 w-4" /> Search
            </label>
            <input
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="title, summary, tag…"
              className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none focus:border-cyan-400/40 transition"
            />
          </div>

          <div className="rounded-3xl border border-white/10 bg-white/5 p-4">
            <label className="mb-2 flex items-center gap-2 text-sm text-slate-300">
              <Filter className="h-4 w-4" /> Filter
            </label>
            <select
              value={filter}
              onChange={(e) => setFilter(e.target.value as typeof filter)}
              className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none"
            >
              <option value="all">All types</option>
              <option value="success">Success</option>
              <option value="warning">Warning</option>
              <option value="info">Info</option>
              <option value="error">Error</option>
            </select>
          </div>

          <div className="rounded-3xl border border-white/10 bg-white/5 p-4">
            <label className="mb-2 flex items-center gap-2 text-sm text-slate-300">
              <ArrowUpDown className="h-4 w-4" /> Sort
            </label>
            <select
              value={sort}
              onChange={(e) => setSort(e.target.value as typeof sort)}
              className="w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none"
            >
              <option value="recent">Most recent</option>
              <option value="alpha">Alphabetical</option>
            </select>
          </div>
        </div>
      </section>

      <section className="mx-auto max-w-7xl px-6 pb-24 lg:px-8">
        {rows.length === 0 ? (
          <div className="rounded-2xl border border-dashed border-white/15 p-8 text-center text-sm text-slate-400">
            No events match your filters.
          </div>
        ) : (
          <div className="grid gap-5 md:grid-cols-2 xl:grid-cols-3">
            {rows.map((item) => (
              <GlassCard key={item.id}>
                <div className="flex items-start justify-between gap-3">
                  <div>
                    <h3 className="text-lg font-semibold text-white">{item.title}</h3>
                    <p className="mt-1 text-xs text-slate-500">{item.createdAt}</p>
                  </div>
                  <span
                    className={`rounded-full border border-white/10 px-3 py-1 text-xs uppercase tracking-[0.2em] ${TONE_COLORS[item.type] ?? "text-slate-300"}`}
                  >
                    {item.type}
                  </span>
                </div>
                <p className="mt-3 text-sm leading-7 text-slate-300">{item.summary}</p>
                <p className="mt-2 text-sm leading-7 text-slate-400">{item.details}</p>
                <div className="mt-4 flex flex-wrap gap-2">
                  {item.tags.map((tag) => (
                    <span
                      key={tag}
                      className="rounded-full border border-white/10 bg-white/5 px-2.5 py-0.5 text-xs text-slate-300"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              </GlassCard>
            ))}
          </div>
        )}
      </section>
    </AppShell>
  );
}
