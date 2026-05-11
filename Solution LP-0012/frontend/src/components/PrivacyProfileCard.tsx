import React from "react";
import { Lock, EyeOff, Zap } from "lucide-react";
import clsx from "clsx";

type Level = "public" | "private" | "confidential";

const profiles: { id: Level; title: string; icon: React.ReactNode; body: string }[] = [
  {
    id: "public",
    title: "Public",
    icon: <EyeOff className="h-4 w-4" />,
    body: "Show more detail — useful for demos and public dashboards.",
  },
  {
    id: "private",
    title: "Private",
    icon: <Lock className="h-4 w-4" />,
    body: "Prefer summaries and redaction with local-only details.",
  },
  {
    id: "confidential",
    title: "Confidential",
    icon: <Zap className="h-4 w-4" />,
    body: "Use the strictest settings for sensitive workflows.",
  },
];

export function PrivacyProfileCard({
  level,
  onSelect,
}: {
  level: Level;
  onSelect: (level: Level) => void;
}) {
  return (
    <div className="grid gap-4 md:grid-cols-3">
      {profiles.map((profile) => (
        <button
          key={profile.id}
          onClick={() => onSelect(profile.id)}
          className={clsx(
            "rounded-3xl border p-5 text-left transition",
            profile.id === level
              ? "border-cyan-300/30 bg-cyan-300/10"
              : "border-white/10 bg-white/5 hover:bg-white/[0.07]"
          )}
        >
          <div className="flex items-center gap-2 text-sm font-semibold text-white">
            {profile.icon} {profile.title}
          </div>
          <p className="mt-3 text-sm leading-7 text-slate-300">{profile.body}</p>
        </button>
      ))}
    </div>
  );
}
