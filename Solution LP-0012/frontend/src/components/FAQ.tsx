import React from "react";

export function FAQ({ items }: { items: { q: string; a: string }[] }) {
  return (
    <div className="space-y-4">
      {items.map((faq) => (
        <details
          key={faq.q}
          className="group rounded-3xl border border-white/10 bg-white/5 p-6"
        >
          <summary className="cursor-pointer list-none text-lg font-semibold text-white">
            {faq.q}
          </summary>
          <p className="mt-4 text-sm leading-7 text-slate-300">{faq.a}</p>
        </details>
      ))}
    </div>
  );
}
