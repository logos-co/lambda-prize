import React from "react";
import { useNodeStatus } from "../hooks/useNodeStatus";
import type { NodeHistory } from "../hooks/useNodeStatus";

function fmt(n: number): string {
  return n.toLocaleString("en-US");
}

/* ── Trend ───────────────────────────────────────────────────────────── */

type Trend = "up" | "down" | "flat";

function getTrend(values: number[]): Trend {
  if (values.length < 4) return "flat";
  const last = values[values.length - 1];
  const ref  = values[Math.max(0, values.length - 5)];
  if (last > ref * 1.005) return "up";
  if (last < ref * 0.995) return "down";
  return "flat";
}

function TrendArrow({ trend }: { trend: Trend }) {
  if (trend === "up")   return <span className="trend-up"   aria-label="rising">↑</span>;
  if (trend === "down") return <span className="trend-down" aria-label="falling">↓</span>;
  return null;
}

/* ── Sparkline ───────────────────────────────────────────────────────── */

function Sparkline({
  values,
  color = "#a78bfa",
}: {
  values: number[];
  color?: string;
}) {
  if (values.length < 2) return null;
  const W = 72, H = 28, P = 3;
  const min = Math.min(...values);
  const max = Math.max(...values);
  const range = max - min || 1;
  const pts = values
    .map((v, i) => {
      const x = (i / (values.length - 1)) * W;
      const y = H - P - ((v - min) / range) * (H - P * 2);
      return `${x.toFixed(1)},${y.toFixed(1)}`;
    })
    .join(" ");
  const lastV = values[values.length - 1];
  const lastY = H - P - ((lastV - min) / range) * (H - P * 2);
  return (
    <svg
      width={W}
      height={H}
      viewBox={`0 0 ${W} ${H}`}
      className="overflow-visible shrink-0 opacity-70"
      aria-hidden="true"
    >
      <polyline
        points={pts}
        fill="none"
        stroke={color}
        strokeWidth="1.5"
        strokeLinejoin="round"
        strokeLinecap="round"
      />
      <circle cx={W} cy={lastY} r="2.5" fill={color} />
    </svg>
  );
}

/* ── StatCard ────────────────────────────────────────────────────────── */

type Accent = "emerald" | "violet" | "amber" | "rose" | "default";

type StatCardProps = {
  label: string;
  value: string;
  sub?: string;
  accent?: Accent;
  sparkline?: number[];
  sparklineColor?: string;
  progress?: number;
  trend?: Trend;
};

const ACCENT_VALUE: Record<Accent, string> = {
  emerald: "text-emerald-400",
  violet:  "text-violet-400",
  amber:   "text-amber-400",
  rose:    "text-rose-400",
  default: "text-slate-100",
};

const ACCENT_BAR: Record<Accent, string> = {
  emerald: "bg-emerald-500",
  violet:  "bg-violet-500",
  amber:   "bg-amber-500",
  rose:    "bg-rose-500",
  default: "bg-slate-500",
};

const ACCENT_TOP: Record<Accent, string> = {
  emerald: "border-t-emerald-500/60",
  violet:  "border-t-violet-500/60",
  amber:   "border-t-amber-500/60",
  rose:    "border-t-rose-500/60",
  default: "border-t-slate-700",
};

function StatCard({
  label,
  value,
  sub,
  accent = "default",
  sparkline,
  sparklineColor,
  progress,
  trend,
}: StatCardProps) {
  return (
    <div className={`card-inner p-4 border-t-2 ${ACCENT_TOP[accent]}`}>
      <div className="flex items-start justify-between gap-2">
        <span className="label-xs leading-tight">{label}</span>
        {sparkline && sparkline.length > 1 ? (
          <Sparkline values={sparkline} color={sparklineColor ?? "#a78bfa"} />
        ) : null}
      </div>
      <div className={`stat-value mt-2 flex items-baseline gap-1.5 ${ACCENT_VALUE[accent]}`}>
        {value}
        {trend ? <TrendArrow trend={trend} /> : null}
      </div>
      {progress !== undefined ? (
        <div className="mt-2 h-1 rounded-full bg-slate-700/80 overflow-hidden">
          <div
            className={`h-full rounded-full transition-all duration-1000 ${ACCENT_BAR[accent]}`}
            style={{
              width: `${Math.min(100, Math.max(0, progress))}%`,
              animation: "health-fill 1s ease-out",
            }}
          />
        </div>
      ) : null}
      {sub ? <div className="mt-1 text-xs text-slate-500">{sub}</div> : null}
    </div>
  );
}

/* ── HealthBar ───────────────────────────────────────────────────────── */

function HealthBar({ score, latency }: { score: number; latency: number }) {
  const { bar, text, label, bg } =
    score >= 80
      ? { bar: "bg-emerald-500", text: "text-emerald-300", label: "Healthy",  bg: "bg-emerald-500/10 border-emerald-500/20" }
      : score >= 60
      ? { bar: "bg-violet-500",  text: "text-violet-300",  label: "Degraded", bg: "bg-violet-500/10 border-violet-500/20" }
      : { bar: "bg-amber-500",   text: "text-amber-300",   label: "Warning",  bg: "bg-amber-500/10 border-amber-500/20" };

  return (
    <div className={`mb-4 rounded-xl border p-3 ${bg}`}>
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-2">
          <span className="label-xs">Node health</span>
          <span className={`rounded-full px-2 py-0.5 text-xs font-semibold border ${bg} ${text}`}>
            {label}
          </span>
        </div>
        <div className="flex items-center gap-3 text-xs">
          <span className={`font-bold tabular-nums ${text}`}>{score} / 100</span>
          <span className="text-slate-500">
            RPC{" "}
            <span
              className={
                latency < 50
                  ? "text-emerald-400"
                  : latency < 150
                  ? "text-amber-400"
                  : "text-rose-400"
              }
            >
              {latency} ms
            </span>
          </span>
        </div>
      </div>
      <div className="h-1.5 rounded-full bg-slate-800/60 overflow-hidden">
        <div
          className={`h-full rounded-full transition-all duration-1000 ${bar}`}
          style={{ width: `${score}%` }}
        />
      </div>
    </div>
  );
}

/* ── MiniBar ─────────────────────────────────────────────────────────── */

function MiniBar({
  label,
  value,
  max,
  color = "bg-violet-500",
}: {
  label: string;
  value: number;
  max: number;
  color?: string;
}) {
  const pct = Math.min(100, (value / max) * 100);
  return (
    <div className="flex items-center gap-2 text-xs">
      <span className="w-24 shrink-0 text-slate-400">{label}</span>
      <div className="flex-1 h-1.5 rounded-full bg-slate-700/80 overflow-hidden">
        <div
          className={`h-full rounded-full transition-all duration-700 ${color}`}
          style={{ width: `${pct}%` }}
        />
      </div>
      <span className="w-8 shrink-0 text-right tabular-nums text-slate-500">
        {Math.round(pct)}%
      </span>
    </div>
  );
}

/* ── NodeStatusDashboard ─────────────────────────────────────────────── */

export function NodeStatusDashboard() {
  const { status, history } = useNodeStatus(3000);

  const participation  = status.networkParticipation;
  const validatorRatio = status.activeValidators / status.validatorCount;

  const partAccent: Accent =
    participation > 80 ? "emerald" : participation > 60 ? "violet" : "amber";

  const healthScore = Math.round(
    (participation / 100) * 50 +
      Math.max(0, (250 - status.rpcLatencyMs) / 250) * 30 +
      validatorRatio * 20
  );

  const blockTrend = getTrend(history.pendingTx);
  const partTrend  = getTrend(history.participation);

  return (
    <div className="space-y-4">
      <div className="card p-5">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold text-slate-100">Node status</h2>
          <div className="flex items-center gap-3">
            {status.isSynced ? (
              <span className="badge-emerald">synced</span>
            ) : (
              <span className="badge-amber">syncing</span>
            )}
            <span className="flex items-center gap-1.5 text-sm text-slate-400" aria-live="polite">
              <span className="live-dot" aria-hidden="true" /> LIVE
            </span>
          </div>
        </div>

        <HealthBar score={healthScore} latency={status.rpcLatencyMs} />

        <div className="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
          <StatCard
            label="Block height"
            value={fmt(status.blockHeight)}
            sub={`Slot ${fmt(status.slotNumber)}`}
            accent="violet"
            sparkline={history.pendingTx}
            sparklineColor="#a78bfa"
            trend={blockTrend}
          />
          <StatCard
            label="Active validators"
            value={`${status.activeValidators} / ${status.validatorCount}`}
            sub="Anonymous identities"
            accent="default"
            progress={validatorRatio * 100}
            sparkline={history.participation}
            sparklineColor="#64748b"
          />
          <StatCard
            label="Network participation"
            value={participation.toFixed(1) + "%"}
            sub="Weighted stake"
            accent={partAccent}
            progress={participation}
            trend={partTrend}
            sparkline={history.participation}
            sparklineColor={
              partAccent === "emerald"
                ? "#34d399"
                : partAccent === "violet"
                ? "#a78bfa"
                : "#f59e0b"
            }
          />
          <StatCard
            label="Pending transactions"
            value={fmt(status.pendingTxCount)}
            sub={`${fmt(status.finalisedTxCount)} finalised`}
            accent={status.pendingTxCount > 80 ? "amber" : "default"}
            sparkline={history.pendingTx}
            sparklineColor={status.pendingTxCount > 80 ? "#f59e0b" : "#64748b"}
          />
        </div>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        {/* Consensus */}
        <div className="card p-4">
          <div className="label-xs mb-3">Consensus</div>
          <dl className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-slate-400">Epoch</span>
              <span className="mono text-slate-200">{fmt(status.epochNumber)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-400">Slot</span>
              <span className="mono text-slate-200">{fmt(status.slotNumber)}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-400">Slot interval</span>
              <span className="mono text-slate-200">4 s</span>
            </div>
            <div className="mt-3 pt-3 border-t border-slate-800 space-y-2">
              <MiniBar
                label="Participation"
                value={participation}
                max={100}
                color={partAccent === "emerald" ? "bg-emerald-500" : "bg-violet-500"}
              />
              <MiniBar
                label="Validator uptime"
                value={validatorRatio * 100}
                max={100}
                color="bg-slate-400"
              />
            </div>
          </dl>
        </div>

        {/* Blend / Cover traffic */}
        <div className="card p-4">
          <div className="label-xs mb-3">Blend / cover traffic</div>
          <dl className="space-y-2 text-sm">
            <div className="flex justify-between items-center">
              <span className="text-slate-400">Cover rate</span>
              <span className="mono text-emerald-400">
                {status.coverTrafficRate.toFixed(1)} pkt/s
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-400">Mix nodes</span>
              <span className="mono text-slate-200">5</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-400">Max hops</span>
              <span className="mono text-slate-200">5</span>
            </div>
            <div className="mt-3 pt-3 border-t border-slate-800 space-y-2">
              <MiniBar
                label="Cover rate"
                value={status.coverTrafficRate}
                max={20}
                color="bg-emerald-500"
              />
              <MiniBar
                label="Mix capacity"
                value={75}
                max={100}
                color="bg-violet-500"
              />
            </div>
          </dl>
        </div>

        {/* Network health */}
        <div className="card p-4">
          <div className="label-xs mb-3">Network health</div>
          <dl className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-slate-400">RPC</span>
              <span className="badge-emerald">healthy</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-400">WebSocket</span>
              <span className="badge-emerald">healthy</span>
            </div>
            <div className="flex justify-between">
              <span className="text-slate-400">Latency</span>
              <span
                className={`mono font-medium ${
                  status.rpcLatencyMs < 50
                    ? "text-emerald-400"
                    : status.rpcLatencyMs < 150
                    ? "text-amber-400"
                    : "text-rose-400"
                }`}
              >
                {status.rpcLatencyMs} ms
              </span>
            </div>
            <div className="mt-3 pt-3 border-t border-slate-800 space-y-2">
              <MiniBar
                label="RPC latency"
                value={Math.min(250, status.rpcLatencyMs)}
                max={250}
                color={
                  status.rpcLatencyMs < 50
                    ? "bg-emerald-500"
                    : status.rpcLatencyMs < 150
                    ? "bg-amber-500"
                    : "bg-rose-500"
                }
              />
              <MiniBar
                label="Health score"
                value={healthScore}
                max={100}
                color={
                  healthScore >= 80
                    ? "bg-emerald-500"
                    : healthScore >= 60
                    ? "bg-violet-500"
                    : "bg-amber-500"
                }
              />
            </div>
          </dl>
        </div>
      </div>
    </div>
  );
}
