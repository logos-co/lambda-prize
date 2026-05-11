import React from "react";
import type { AuditEvent } from "../lib/privacy-types";

function formatTime(value: string): string {
  try {
    return new Date(value).toLocaleString();
  } catch {
    return value;
  }
}

export function AuditLog({ items }: { items: AuditEvent[] }) {
  if (items.length === 0) {
    return (
      <div className="rounded-2xl border border-dashed border-white/15 p-5 text-sm text-slate-400">
        No privacy events yet.
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {[...items].reverse().map((item) => (
        <article
          key={item.id}
          className="rounded-2xl border border-white/10 bg-white/5 p-4"
        >
          <div className="flex items-center justify-between gap-3">
            <h4 className="text-sm font-semibold text-white">{item.title}</h4>
            <span
              className={`rounded-full px-3 py-1 text-xs uppercase tracking-[0.2em] ${
                item.level === "error"
                  ? "bg-rose-500/10 text-rose-200"
                  : item.level === "warn"
                    ? "bg-amber-500/10 text-amber-200"
                    : "bg-cyan-500/10 text-cyan-200"
              }`}
            >
              {item.level}
            </span>
          </div>
          <p className="mt-2 text-sm text-slate-300">{item.summary}</p>
          {item.details ? (
            <p className="mt-2 text-sm text-slate-400">{item.details}</p>
          ) : null}
          <div className="mt-3 flex flex-wrap items-center gap-2 text-xs text-slate-500">
            <span>{item.category}</span>
            <span>•</span>
            <span>{formatTime(item.time)}</span>
            {item.tags.map((tag) => (
              <span
                key={tag}
                className="rounded-full border border-white/10 bg-white/5 px-2 py-0.5"
              >
                {tag}
              </span>
            ))}
          </div>
        </article>
      ))}
    </div>
  );
}
