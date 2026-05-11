import React, { useEffect, useState } from "react";
import type { AuditEvent } from "../types/privacy";

type Props = {
  events: AuditEvent[];
};

/* ── Helpers ─────────────────────────────────────────────────────────── */

function relativeTime(iso: string): string {
  const diff = Date.now() - new Date(iso).getTime();
  const s = Math.floor(diff / 1000);
  if (s < 5)  return "just now";
  if (s < 60) return `${s}s ago`;
  const m = Math.floor(s / 60);
  if (m < 60) return `${m}m ago`;
  const h = Math.floor(m / 60);
  if (h < 24) return `${h}h ago`;
  return `${Math.floor(h / 24)}d ago`;
}

function absTime(iso: string): string {
  return new Date(iso).toLocaleString(undefined, {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

/* ── Styling maps ────────────────────────────────────────────────────── */

const CATEGORY_META: Record<
  AuditEvent["category"],
  { icon: string; dot: string; label: string; badge: string }
> = {
  consent:  { icon: "⊗", dot: "bg-violet-400",  label: "Consent",    badge: "badge-violet" },
  wallet:   { icon: "◈", dot: "bg-blue-400",     label: "Wallet",     badge: "badge-blue"   },
  tx:       { icon: "⬡", dot: "bg-emerald-400",  label: "Tx",         badge: "badge-emerald"},
  message:  { icon: "◻", dot: "bg-amber-400",    label: "Message",    badge: "badge-amber"  },
  storage:  { icon: "⊞", dot: "bg-slate-400",    label: "Storage",    badge: "badge-slate"  },
  security: { icon: "⊛", dot: "bg-rose-400",     label: "Security",   badge: "badge-rose"   },
};

const LEVEL_BADGE: Record<AuditEvent["level"], string> = {
  info:  "badge-violet",
  warn:  "badge-amber",
  error: "badge-rose",
};

const LEVEL_DOT: Record<AuditEvent["level"], string> = {
  info:  "bg-violet-500",
  warn:  "bg-amber-500",
  error: "bg-rose-500",
};

type FilterCat = AuditEvent["category"] | "all";

const CATEGORIES: FilterCat[] = [
  "all", "consent", "wallet", "tx", "message", "storage", "security",
];

/* ── Component ───────────────────────────────────────────────────────── */

export function AuditTrailPanel({ events }: Props) {
  const [filter, setFilter] = useState<FilterCat>("all");
  const [, setTick] = useState(0);

  // Re-render every 15 s so relative times stay fresh
  useEffect(() => {
    const id = setInterval(() => setTick((n) => n + 1), 15_000);
    return () => clearInterval(id);
  }, []);

  const visible = events
    .slice()
    .reverse()
    .filter((e) => filter === "all" || e.category === filter);

  // Count per category for badges
  const counts: Record<FilterCat, number> = { all: events.length } as Record<FilterCat, number>;
  for (const cat of CATEGORIES.slice(1)) {
    counts[cat] = events.filter((e) => e.category === cat).length;
  }

  return (
    <div className="card p-5">
      {/* Header */}
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h2 className="text-lg font-semibold text-slate-100">Audit trail</h2>
          <p className="mt-0.5 text-xs text-slate-500">
            {events.length} event{events.length !== 1 ? "s" : ""} recorded
          </p>
        </div>
        {events.length === 0 ? null : (
          <div className="flex items-center gap-1.5 text-xs text-slate-500">
            <span className="live-dot" aria-hidden="true" />
            live
          </div>
        )}
      </div>

      {/* Filter pills with count badges */}
      <div className="mt-3 flex flex-wrap gap-1.5">
        {CATEGORIES.map((cat) => {
          const count = counts[cat] ?? 0;
          const isActive = filter === cat;
          const meta = cat !== "all" ? CATEGORY_META[cat as AuditEvent["category"]] : null;
          return (
            <button
              key={cat}
              onClick={() => setFilter(cat)}
              className={[
                "flex items-center gap-1.5 rounded-lg px-2.5 py-1 text-xs font-medium transition-colors capitalize",
                isActive
                  ? "bg-violet-600/20 text-violet-300 ring-1 ring-violet-500/30"
                  : "text-slate-500 hover:bg-slate-800 hover:text-slate-300",
              ].join(" ")}
            >
              {meta ? <span aria-hidden="true">{meta.icon}</span> : null}
              {cat === "all" ? "All" : (meta?.label ?? cat)}
              {count > 0 ? (
                <span
                  className={[
                    "rounded-full px-1.5 py-0 text-[10px] font-bold",
                    isActive ? "bg-violet-500/30 text-violet-300" : "bg-slate-800 text-slate-500",
                  ].join(" ")}
                >
                  {count}
                </span>
              ) : null}
            </button>
          );
        })}
      </div>

      {/* Timeline */}
      {visible.length === 0 ? (
        <div className="mt-4 rounded-xl border border-dashed border-slate-700 p-8 text-center">
          <div className="text-2xl mb-2 opacity-30">⊞</div>
          <p className="text-sm text-slate-500">
            No events{filter !== "all" ? ` in "${filter}"` : ""}.
          </p>
        </div>
      ) : (
        <div className="relative mt-4 max-h-[440px] overflow-y-auto">
          {/* Vertical timeline rail */}
          <div className="absolute left-[7px] top-3 bottom-3 w-px bg-slate-800 pointer-events-none" />

          <div className="space-y-2 pl-6">
            {visible.map((event, i) => {
              const meta = CATEGORY_META[event.category];
              return (
                <article
                  key={event.id}
                  className={[
                    "relative rounded-xl border border-slate-800 bg-slate-800/25 p-3",
                    "hover:bg-slate-800/50 transition-colors duration-150",
                    i === 0 ? "slot-new" : "",
                  ].join(" ")}
                >
                  {/* Timeline dot */}
                  <div
                    className={`absolute -left-[19px] top-4 h-2.5 w-2.5 rounded-full ring-2 ring-slate-900 ${LEVEL_DOT[event.level]}`}
                  />

                  <div className="flex items-start justify-between gap-3 flex-wrap">
                    <div className="flex items-center gap-2 flex-wrap">
                      {/* Category chip */}
                      <span className={`flex items-center gap-1 ${meta.badge} text-[11px]`}>
                        <span aria-hidden="true">{meta.icon}</span>
                        {meta.label}
                      </span>
                      <span className="font-medium text-slate-200 text-sm">
                        {event.title}
                      </span>
                    </div>
                    <span className={`shrink-0 ${LEVEL_BADGE[event.level]}`}>
                      {event.level}
                    </span>
                  </div>

                  {event.description ? (
                    <p className="mt-1 text-xs text-slate-400 leading-relaxed">
                      {event.description}
                    </p>
                  ) : null}

                  <div className="mt-1.5 flex items-center gap-2">
                    <time
                      dateTime={event.timestamp}
                      className="text-xs text-slate-600 tabular-nums"
                      title={absTime(event.timestamp)}
                    >
                      {relativeTime(event.timestamp)}
                    </time>
                    <span className="text-slate-800">·</span>
                    <span className="text-xs text-slate-700 tabular-nums">
                      {absTime(event.timestamp)}
                    </span>
                  </div>
                </article>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
