import React, { useEffect, useRef } from "react";
import { useLottery } from "../hooks/useLottery";
import { useToasts } from "../hooks/useToasts";

function fmt(n: number): string {
  return n.toLocaleString("en-US");
}

function shortHex(h: string): string {
  return h.length > 14 ? h.slice(0, 10) + "…" + h.slice(-4) : h;
}

/* ── WinRateGauge ────────────────────────────────────────────────────── */

function WinRateGauge({ won, total }: { won: number; total: number }) {
  const pct = total > 0 ? (won / total) * 100 : 0;
  const difficultyPct = 0.4;
  const ratio = pct / difficultyPct;
  const color =
    ratio > 1.5 ? "text-emerald-400" : ratio < 0.5 ? "text-rose-400" : "text-violet-400";
  const barColor =
    ratio > 1.5 ? "bg-emerald-500" : ratio < 0.5 ? "bg-rose-500" : "bg-violet-500";

  return (
    <div className="card-inner p-3 text-center">
      <div className="label-xs mb-1">Win rate</div>
      <AnimatedValue
        value={pct.toFixed(2) + "%"}
        className={`text-xl font-bold tabular-nums ${color}`}
      />
      <div className="mt-2 h-1 rounded-full bg-slate-700 overflow-hidden">
        <div
          className={`h-full rounded-full transition-all duration-700 ${barColor}`}
          style={{
            width: `${Math.min(100, (pct / Math.max(pct, difficultyPct * 2)) * 100)}%`,
          }}
        />
      </div>
      <div className="mt-1 text-xs text-slate-600">
        target ~{difficultyPct}%
      </div>
    </div>
  );
}

/* ── AnimatedValue ───────────────────────────────────────────────────── */

function AnimatedValue({
  value,
  className,
}: {
  value: string;
  className?: string;
}) {
  return (
    <div key={value} className={`slot-tick-anim ${className ?? ""}`}>
      {value}
    </div>
  );
}

/* ── VrfHash ─────────────────────────────────────────────────────────── */

function VrfHash({ hash }: { hash: string }) {
  return (
    <div key={hash} className="vrf-update-anim mono break-all text-sm text-slate-200">
      {hash}
    </div>
  );
}

/* ── LotteryViewer ───────────────────────────────────────────────────── */

export function LotteryViewer() {
  const { slots, currentSlot, isRunning, setIsRunning } = useLottery();
  const { push } = useToasts();
  const prevLen = useRef(0);
  const won = slots.filter((s) => s.won).length;

  useEffect(() => {
    if (slots.length > prevLen.current && slots[0]?.won) {
      push({
        kind: "success",
        title: "Lottery won! 🎉",
        body: `Slot ${fmt(slots[0].slot)} · proposal ${shortHex(slots[0].proposalId ?? "")}`,
        duration: 6000,
      });
    }
    prevLen.current = slots.length;
  }, [slots, push]);

  const current = slots[0];

  return (
    <div className="space-y-4">
      {/* Hero card */}
      <div className="card p-5">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold text-slate-100">
            Block proposal lottery
          </h2>
          <div className="flex items-center gap-3">
            <button
              onClick={() => setIsRunning((v) => !v)}
              className="btn-ghost text-xs px-3 py-1.5"
              aria-pressed={isRunning}
            >
              {isRunning ? "⏸ Pause" : "▶ Resume"}
            </button>
            {isRunning ? (
              <span
                className="flex items-center gap-1.5 text-sm text-slate-400"
                aria-live="polite"
              >
                <span className="live-dot" aria-hidden="true" /> RUNNING
              </span>
            ) : (
              <span className="badge-amber">paused</span>
            )}
          </div>
        </div>

        <p className="mt-2 text-sm text-slate-500">
          Each slot runs an independent VRF lottery. Winners broadcast a
          commitment without revealing their identity until block finalisation.
        </p>

        {/* Main display */}
        <div className="mt-5 grid gap-4 sm:grid-cols-2">
          <div className="card-inner p-5 flex flex-col justify-between">
            <div className="label-xs mb-3">Current slot</div>
            <AnimatedValue
              value={fmt(currentSlot)}
              className="stat-value text-violet-400 text-3xl"
            />
            {current ? (
              <div className="mt-2 text-xs text-slate-500 font-medium">
                Epoch{" "}
                <span className="text-slate-400 font-mono">
                  {fmt(current.epoch)}
                </span>
                &nbsp;·&nbsp;4 s per slot
              </div>
            ) : null}
          </div>

          <div className="card-inner p-5 flex flex-col justify-between">
            <div className="label-xs mb-3">Latest VRF output</div>
            {current ? (
              <VrfHash hash={current.vrfOutput} />
            ) : (
              <span className="mono text-slate-500">—</span>
            )}
            <div className="mt-2 text-xs text-slate-500">
              Difficulty{" "}
              <span className="text-violet-400 font-medium">
                {current ? (current.difficulty * 100).toFixed(2) : "—"}%
              </span>{" "}
              ≈ 1 win per{" "}
              {Math.round(1 / (current?.difficulty ?? 0.004))} slots
            </div>
          </div>
        </div>

        {/* Stats row */}
        <div className="mt-4 grid grid-cols-3 gap-3">
          <div className="card-inner p-3 text-center">
            <div className="label-xs mb-1">Slots observed</div>
            <AnimatedValue
              value={String(slots.length)}
              className="text-xl font-bold tabular-nums text-slate-200"
            />
          </div>
          <div className="card-inner p-3 text-center">
            <div className="label-xs mb-1">Proposals won</div>
            <AnimatedValue
              value={String(won)}
              className={`text-xl font-bold tabular-nums ${
                won > 0 ? "text-emerald-400" : "text-slate-400"
              }`}
            />
          </div>
          <WinRateGauge won={won} total={slots.length} />
        </div>
      </div>

      {/* Slot history */}
      <div className="card p-5">
        <h3 className="text-sm font-semibold text-slate-200 mb-3">
          Slot history
        </h3>
        {slots.length === 0 ? (
          <div className="rounded-xl border border-dashed border-slate-700 p-8 text-center">
            <div className="text-2xl mb-2 opacity-40">⚄</div>
            <p className="text-sm text-slate-500">Waiting for first slot…</p>
          </div>
        ) : (
          <div className="max-h-80 space-y-1 overflow-y-auto" role="log" aria-label="Slot history">
            {slots.map((slot, i) => (
              <div
                key={slot.slot}
                className={[
                  "grid grid-cols-[88px_1fr_72px_1fr] items-center gap-3 rounded-xl px-3 py-2 text-xs transition-colors",
                  i === 0 ? "slot-new" : "",
                  slot.won
                    ? "slot-won border border-emerald-500/25 bg-emerald-500/5"
                    : "hover:bg-slate-800/40",
                ].join(" ")}
              >
                <span className="mono text-slate-400 tabular-nums">
                  {fmt(slot.slot)}
                </span>
                <span className="mono text-slate-500 truncate">
                  {shortHex(slot.vrfOutput)}
                </span>
                {slot.won ? (
                  <span className="badge-emerald font-bold" aria-label="Won">WON ✓</span>
                ) : (
                  <span className="text-slate-700" aria-label="Lost">—</span>
                )}
                {slot.proposalId ? (
                  <span className="mono text-emerald-400 truncate">
                    {shortHex(slot.proposalId)}
                  </span>
                ) : (
                  <span />
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
