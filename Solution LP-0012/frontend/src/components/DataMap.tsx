import React from "react";
import type { DataCategory } from "../lib/privacy-types";

export function DataMap({ items }: { items: DataCategory[] }) {
  return (
    <div className="grid gap-5 md:grid-cols-2">
      {items.map((item) => (
        <div
          key={item.name}
          className="rounded-3xl border border-white/10 bg-white/5 p-6"
        >
          <div className="flex items-center justify-between gap-3">
            <h3 className="text-xl font-semibold text-white">{item.name}</h3>
            <span
              className={`rounded-full px-3 py-1 text-xs uppercase tracking-[0.2em] ${
                item.sensitive
                  ? "bg-rose-500/10 text-rose-200"
                  : "bg-emerald-500/10 text-emerald-200"
              }`}
            >
              {item.sensitive ? "Sensitive" : "Low risk"}
            </span>
          </div>
          <p className="mt-3 text-sm leading-7 text-slate-300">{item.purpose}</p>
          <dl className="mt-5 grid gap-3 sm:grid-cols-2">
            <div className="rounded-2xl bg-slate-950/50 p-3">
              <dt className="text-xs uppercase tracking-[0.2em] text-slate-500">
                Stored locally
              </dt>
              <dd className="mt-1 text-sm text-white">
                {item.storedLocally ? "Yes" : "No"}
              </dd>
            </div>
            <div className="rounded-2xl bg-slate-950/50 p-3">
              <dt className="text-xs uppercase tracking-[0.2em] text-slate-500">
                Shared externally
              </dt>
              <dd className="mt-1 text-sm text-white">
                {item.sharedExternally ? "Sometimes" : "No"}
              </dd>
            </div>
            <div className="rounded-2xl bg-slate-950/50 p-3 sm:col-span-2">
              <dt className="text-xs uppercase tracking-[0.2em] text-slate-500">
                Retention
              </dt>
              <dd className="mt-1 text-sm text-white">{item.retention}</dd>
            </div>
          </dl>
        </div>
      ))}
    </div>
  );
}
