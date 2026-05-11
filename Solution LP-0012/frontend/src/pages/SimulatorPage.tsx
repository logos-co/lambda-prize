import React, { useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { GlassCard } from "../components/GlassCard";
import { SectionHeading } from "../components/SectionHeading";

function hashLike(input: string): number {
  let h = 2166136261;
  for (let i = 0; i < input.length; i++) {
    h ^= input.charCodeAt(i);
    h = Math.imul(h, 16777619) >>> 0;
  }
  return h;
}

function pickWinner(
  seed: string,
  slot: number,
  stake: number,
  totalStake: number
) {
  const score = hashLike(`${seed}:${slot}:${stake}:${totalStake}`) % 1_000_000;
  const threshold = Math.floor((stake / Math.max(totalStake, 1)) * 300_000);
  return { score, threshold, won: score <= threshold };
}

export function SimulatorPage() {
  const [seed, setSeed] = useState("cryptarchia-seed");
  const [stake, setStake] = useState(12_500);
  const [totalStake, setTotalStake] = useState(250_000);
  const [slots, setSlots] = useState(12);

  const rows = useMemo(
    () =>
      Array.from({ length: Math.max(1, Math.min(slots, 64)) }, (_, i) => ({
        slot: i + 1,
        ...pickWinner(seed, i + 1, stake, totalStake),
      })),
    [seed, stake, totalStake, slots]
  );

  const wins = rows.filter((r) => r.won).length;

  return (
    <AppShell>
      <PageHero
        badge="Simulator"
        title="A front-end leadership lottery preview"
        description="Adjust stake, total stake, and seed to see how a slot-by-slot VRF selection story would unfold. This is a browser-only mock — no Rust required."
        primary={{ to: "/docs", label: "Open docs" }}
        secondary={{ to: "/leadership", label: "Back to leadership" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-20 lg:px-8">
        <SectionHeading
          eyebrow="Inputs"
          title="Play with the lottery parameters"
          description="Score and threshold are computed in the browser using a deterministic hash. Lower stake-to-total ratios mean fewer wins."
        />

        <div className="mt-14 grid gap-5 sm:grid-cols-2 lg:grid-cols-4">
          <GlassCard>
            <label className="text-sm text-slate-300">Epoch seed</label>
            <input
              value={seed}
              onChange={(e) => setSeed(e.target.value)}
              className="mt-3 w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none focus:border-cyan-400/40 transition"
            />
          </GlassCard>
          <GlassCard>
            <label className="text-sm text-slate-300">Validator stake</label>
            <input
              type="number"
              min={1}
              value={stake}
              onChange={(e) => setStake(Number(e.target.value))}
              className="mt-3 w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none focus:border-cyan-400/40 transition"
            />
          </GlassCard>
          <GlassCard>
            <label className="text-sm text-slate-300">Total stake</label>
            <input
              type="number"
              min={1}
              value={totalStake}
              onChange={(e) => setTotalStake(Number(e.target.value))}
              className="mt-3 w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none focus:border-cyan-400/40 transition"
            />
          </GlassCard>
          <GlassCard>
            <label className="text-sm text-slate-300">Slots (max 64)</label>
            <input
              type="number"
              min={1}
              max={64}
              value={slots}
              onChange={(e) => setSlots(Number(e.target.value))}
              className="mt-3 w-full rounded-2xl border border-white/10 bg-slate-950 px-4 py-3 text-sm text-white outline-none focus:border-cyan-400/40 transition"
            />
          </GlassCard>
        </div>

        <div className="mt-8 flex gap-6 text-sm text-slate-400">
          <span>
            Slots evaluated:{" "}
            <span className="font-semibold text-white">{rows.length}</span>
          </span>
          <span>
            Wins:{" "}
            <span className="font-semibold text-emerald-300">{wins}</span>
          </span>
          <span>
            Win rate:{" "}
            <span className="font-semibold text-white">
              {rows.length ? ((wins / rows.length) * 100).toFixed(1) : 0}%
            </span>
          </span>
        </div>
      </section>

      <section className="mx-auto max-w-5xl px-6 py-8 pb-20 lg:px-8">
        <div className="overflow-hidden rounded-[2rem] border border-white/10 bg-white/5">
          <div className="grid grid-cols-4 border-b border-white/10 bg-white/5 px-6 py-4 text-xs uppercase tracking-[0.25em] text-slate-400">
            <div>Slot</div>
            <div>Score</div>
            <div>Threshold</div>
            <div>Result</div>
          </div>
          {rows.map((row) => (
            <div
              key={row.slot}
              className="grid grid-cols-4 border-b border-white/10 px-6 py-4 text-sm last:border-b-0"
            >
              <div className="text-slate-300">{row.slot}</div>
              <div className="font-mono text-slate-300">{row.score.toLocaleString()}</div>
              <div className="font-mono text-slate-300">{row.threshold.toLocaleString()}</div>
              <div
                className={
                  row.won
                    ? "font-semibold text-emerald-300"
                    : "text-slate-500"
                }
              >
                {row.won ? "✓ Winner" : "No win"}
              </div>
            </div>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
