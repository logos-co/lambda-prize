import React from "react";

export type TabId =
  | "dashboard"
  | "blend"
  | "lottery"
  | "staking"
  | "privacy"
  | "audit";

const TABS: { id: TabId; label: string; icon: string; key: string }[] = [
  { id: "dashboard", label: "Node",    icon: "◎", key: "1" },
  { id: "blend",     label: "Blend",   icon: "⬡", key: "2" },
  { id: "lottery",   label: "Lottery", icon: "⚄", key: "3" },
  { id: "staking",   label: "Staking", icon: "◈", key: "4" },
  { id: "privacy",   label: "Privacy", icon: "⊗", key: "5" },
  { id: "audit",     label: "Audit",   icon: "⊞", key: "6" },
];

type Props = {
  active: TabId;
  onChange: (id: TabId) => void;
};

export function NavTabs({ active, onChange }: Props) {
  return (
    <nav role="tablist" aria-label="Application sections" className="flex gap-1 overflow-x-auto">
      {TABS.map((tab) => {
        const isActive = tab.id === active;
        return (
          <button
            key={tab.id}
            role="tab"
            aria-selected={isActive}
            aria-label={`${tab.label} (press ${tab.key})`}
            onClick={() => onChange(tab.id)}
            className={[
              "relative flex items-center gap-1.5 rounded-xl px-4 py-2 text-sm font-medium transition-colors whitespace-nowrap",
              isActive
                ? "bg-violet-600/20 text-violet-300 ring-1 ring-violet-500/30"
                : "text-slate-400 hover:bg-slate-800 hover:text-slate-200",
            ].join(" ")}
          >
            <span className="text-base leading-none" aria-hidden="true">{tab.icon}</span>
            {tab.label}
            <span
              className={`ml-0.5 text-xs leading-none font-mono ${
                isActive ? "text-violet-500" : "text-slate-700"
              }`}
              aria-hidden="true"
            >
              {tab.key}
            </span>
          </button>
        );
      })}
    </nav>
  );
}
