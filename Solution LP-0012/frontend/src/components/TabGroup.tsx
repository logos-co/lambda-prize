import React, { useMemo, useState } from "react";
import clsx from "clsx";

export function TabGroup({
  tabs,
}: {
  tabs: { id: string; label: string; content: React.ReactNode }[];
}) {
  const [active, setActive] = useState(tabs[0]?.id ?? "");
  const activeTab = useMemo(
    () => tabs.find((t) => t.id === active) ?? tabs[0],
    [tabs, active]
  );

  return (
    <div className="rounded-[2rem] border border-white/10 bg-white/5 p-4">
      <div className="flex flex-wrap gap-2">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActive(tab.id)}
            className={clsx(
              "rounded-full px-4 py-2 text-sm transition",
              tab.id === active
                ? "bg-white text-slate-950 font-medium"
                : "text-slate-300 hover:bg-white/10 hover:text-white"
            )}
          >
            {tab.label}
          </button>
        ))}
      </div>
      <div className="mt-4 rounded-3xl border border-white/10 bg-slate-950/60 p-5">
        {activeTab?.content}
      </div>
    </div>
  );
}
