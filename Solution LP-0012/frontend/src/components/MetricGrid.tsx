import React from "react";

export function MetricGrid({
  items,
}: {
  items: { label: string; value: string }[];
}) {
  return (
    <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {items.map((item) => (
        <div
          key={item.label}
          className="rounded-3xl border border-white/10 bg-white/5 p-5 backdrop-blur-sm"
        >
          <p className="text-sm text-slate-400">{item.label}</p>
          <p className="mt-3 text-3xl font-semibold tracking-tight text-white">
            {item.value}
          </p>
        </div>
      ))}
    </div>
  );
}
