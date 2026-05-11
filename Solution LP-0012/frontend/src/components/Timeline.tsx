import React from "react";

export function Timeline({
  items,
}: {
  items: { phase: string; title: string; body: string }[];
}) {
  return (
    <div className="grid gap-5 lg:grid-cols-2">
      {items.map((item) => (
        <div
          key={item.phase}
          className="rounded-3xl border border-white/10 bg-white/5 p-6"
        >
          <div className="flex items-start gap-4">
            <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-emerald-400/10 text-emerald-300 ring-1 ring-emerald-300/20">
              <span className="text-sm font-bold">{item.phase}</span>
            </div>
            <div>
              <h3 className="text-xl font-semibold text-white">{item.title}</h3>
              <p className="mt-3 text-sm leading-7 text-slate-300">{item.body}</p>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
