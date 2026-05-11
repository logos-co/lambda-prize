import React, { useEffect, useRef, useState } from "react";

const STAGES = [
  { label: "Preparing inputs",       end: 0.12 },
  { label: "Generating commitment",  end: 0.40 },
  { label: "Creating nullifier",     end: 0.72 },
  { label: "Broadcasting via Blend", end: 0.91 },
  { label: "Confirming on-chain",    end: 1.00 },
];

export function ProofProgressBar({ durationMs }: { durationMs: number }) {
  const [pct, setPct] = useState(0);
  const rafRef = useRef(0);

  useEffect(() => {
    const start = Date.now();
    const tick = () => {
      const elapsed = (Date.now() - start) / durationMs;
      const next = Math.min(1, elapsed);
      setPct(next);
      if (next < 1) rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [durationMs]);

  const displayPct = Math.round(pct * 100);
  const stage = STAGES.find((s) => pct < s.end) ?? STAGES[STAGES.length - 1];

  return (
    <div className="mt-3 card-inner p-4 space-y-3">
      <div className="flex items-center justify-between">
        <span className="text-xs text-slate-400">{stage.label}…</span>
        <span className="tabular-nums text-xs font-semibold text-violet-400">
          {displayPct}%
        </span>
      </div>

      <div className="relative h-1.5 overflow-hidden rounded-full bg-slate-700">
        <div
          className="absolute inset-y-0 left-0 rounded-full bg-violet-500"
          style={{ width: `${displayPct}%`, transition: "width 80ms linear" }}
        />
      </div>

      <div className="flex gap-1.5">
        {STAGES.map((s, i) => {
          const stageStart = i === 0 ? 0 : STAGES[i - 1].end;
          const active = pct >= stageStart;
          return (
            <div
              key={i}
              title={s.label}
              className={`flex-1 h-1 rounded-full transition-colors duration-500 ${
                active ? "bg-violet-500" : "bg-slate-700"
              }`}
            />
          );
        })}
      </div>

      <p className="text-xs text-slate-500">
        Proof generated locally — private key never leaves your browser.
      </p>
    </div>
  );
}
