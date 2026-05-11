import React, { useEffect, useRef } from "react";
import { useBlendMonitor } from "../hooks/useBlendMonitor";
import { useToasts } from "../hooks/useToasts";
import type { BlendPacketEvent } from "../types/privacy";

/* ── helpers ─────────────────────────────────────────────────────────── */

function timeStr(iso: string): string {
  return new Date(iso).toLocaleTimeString("en-US", {
    hour12: false,
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
    fractionalSecondDigits: 2,
  } as Intl.DateTimeFormatOptions);
}

/* ── SVG blend path ──────────────────────────────────────────────────── */

const NODES = [
  { cx: 60,  label: "ENTRY", icon: "⊕", edge: true },
  { cx: 175, label: "MIX-1", icon: "⊛", edge: false },
  { cx: 290, label: "MIX-2", icon: "⊛", edge: false },
  { cx: 405, label: "MIX-3", icon: "⊛", edge: false },
  { cx: 520, label: "EXIT",  icon: "⊖", edge: true },
];

const CY = 52;
const NODE_R = 22;
const PACKET_PATH = `M ${NODES[0].cx} ${CY} L ${NODES[NODES.length - 1].cx} ${CY}`;

function BlendPath({ coverRate }: { coverRate: number }) {
  const speed = Math.max(0.6, 1.8 - coverRate * 0.04);
  const packetDelays = [0, speed / 3, (speed / 3) * 2];

  return (
    <svg
      viewBox="0 0 580 100"
      className="w-full select-none"
      style={{ height: 100 }}
      aria-label="Blend mixnet path visualization"
      role="img"
    >
      {/* Connection lines */}
      {NODES.slice(0, -1).map((node, i) => (
        <line
          key={node.cx}
          x1={node.cx + NODE_R}
          y1={CY}
          x2={NODES[i + 1].cx - NODE_R}
          y2={CY}
          stroke="rgb(51 65 85)"
          strokeWidth="2"
          strokeDasharray="5 3"
        />
      ))}

      {/* Flowing packet dots */}
      {packetDelays.map((delay, i) => (
        <g key={i} aria-hidden="true">
          <circle r="5" fill="rgba(167,139,250,0.85)">
            <animateMotion
              dur={`${speed}s`}
              begin={`-${delay}s`}
              repeatCount="indefinite"
              path={PACKET_PATH}
            />
            <animate
              attributeName="opacity"
              values="0;1;1;0"
              dur={`${speed}s`}
              begin={`-${delay}s`}
              repeatCount="indefinite"
            />
          </circle>
        </g>
      ))}

      {/* Node circles */}
      {NODES.map((node) => (
        <g key={node.cx} role="img" aria-label={node.label}>
          <circle
            cx={node.cx}
            cy={CY}
            r={NODE_R}
            fill="rgb(15 23 42)"
            stroke={node.edge ? "rgb(100 116 139)" : "rgb(71 85 105)"}
            strokeWidth="1.5"
          />
          <text
            x={node.cx}
            y={CY + 5}
            textAnchor="middle"
            fill={node.edge ? "#cbd5e1" : "#94a3b8"}
            fontSize="15"
            aria-hidden="true"
          >
            {node.icon}
          </text>
          <text
            x={node.cx}
            y="88"
            textAnchor="middle"
            fill="rgb(71 85 105)"
            fontSize="9"
            letterSpacing="0.08em"
            aria-hidden="true"
          >
            {node.label}
          </text>
        </g>
      ))}
    </svg>
  );
}

/* ── Delay histogram ─────────────────────────────────────────────────── */

const BUCKETS = [0, 50, 100, 150, 200, 250];

function DelayHistogram({ events }: { events: BlendPacketEvent[] }) {
  const counts = BUCKETS.map((lo, i) => {
    const hi = BUCKETS[i + 1] ?? Infinity;
    return events.filter((e) => e.latencyMs >= lo && e.latencyMs < hi).length;
  });
  const peak = Math.max(...counts, 1);

  return (
    <div className="card-inner p-4">
      <div className="label-xs mb-3">Latency distribution (ms)</div>
      <div
        className="flex items-end gap-1.5 h-14"
        role="img"
        aria-label="Latency distribution chart"
      >
        {counts.map((n, i) => (
          <div
            key={i}
            className="flex flex-1 flex-col items-center gap-1"
            title={`${BUCKETS[i]}–${BUCKETS[i + 1] ?? "∞"} ms: ${n} packets`}
          >
            <div
              className="w-full min-h-[2px] rounded-t-sm bg-violet-500/55 transition-all duration-700"
              style={{ height: `${Math.max(2, (n / peak) * 48)}px` }}
            />
            <span className="text-xs text-slate-600">{BUCKETS[i]}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

/* ── PacketRow ───────────────────────────────────────────────────────── */

function PacketRow({
  event,
  isNew,
}: {
  event: BlendPacketEvent;
  isNew: boolean;
}) {
  return (
    <div
      className={[
        "grid grid-cols-[1fr_56px_52px_60px] gap-x-3 rounded-lg px-3 py-1.5 text-xs",
        isNew ? "slot-new" : "",
        event.isReal ? "text-emerald-300" : "text-slate-400",
      ].join(" ")}
    >
      <span className="mono">{timeStr(event.timestamp)}</span>
      <span className={event.isReal ? "font-semibold text-emerald-400" : ""}>
        {event.isReal ? "REAL" : "cover"}
      </span>
      <span>{event.hopCount} hops</span>
      <span className="text-right tabular-nums">{event.latencyMs} ms</span>
    </div>
  );
}

/* ── BlendMonitor ────────────────────────────────────────────────────── */

export function BlendMonitor() {
  const { events, coverRate } = useBlendMonitor();
  const { push } = useToasts();
  const prevRealRef = useRef(0);

  const realCount = events.filter((e) => e.isReal).length;
  const coverCount = events.length - realCount;

  useEffect(() => {
    if (realCount > prevRealRef.current && realCount % 5 === 0 && realCount > 0) {
      const latest = events.find((e) => e.isReal);
      push({
        kind: "info",
        title: "Real messages detected",
        body: `${realCount} real packets via ${latest?.hopCount ?? 5} hops`,
        duration: 3000,
      });
    }
    prevRealRef.current = realCount;
  }, [realCount, events, push]);

  return (
    <div className="space-y-4">
      {/* Main path card */}
      <div className="card p-5">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold text-slate-100">
            Blend anonymity gateway
          </h2>
          <div className="flex items-center gap-4 text-sm">
            <span className="text-slate-400">
              Cover{" "}
              <span className="text-emerald-400 font-medium">
                {coverRate.toFixed(1)} pkt/s
              </span>
            </span>
            <span className="flex items-center gap-1.5 text-slate-400" aria-live="polite">
              <span className="live-dot" aria-hidden="true" /> LIVE
            </span>
          </div>
        </div>

        <p className="mt-2 text-sm text-slate-500">
          Real messages are indistinguishable from cover traffic. Each packet
          is Sphinx-encrypted and forwarded through independently-delayed mix nodes.
        </p>

        <div className="mt-4">
          <BlendPath coverRate={coverRate} />
        </div>

        <div className="mt-4 grid grid-cols-3 gap-3">
          <div className="card-inner p-3 text-center">
            <div className="label-xs mb-1">Observed</div>
            <div className="text-xl font-bold tabular-nums text-violet-400">
              {events.length}
            </div>
          </div>
          <div className="card-inner p-3 text-center">
            <div className="label-xs mb-1">Real</div>
            <div className="text-xl font-bold tabular-nums text-emerald-400">
              {realCount}
            </div>
          </div>
          <div className="card-inner p-3 text-center">
            <div className="label-xs mb-1">Cover</div>
            <div className="text-xl font-bold tabular-nums text-slate-300">
              {coverCount}
            </div>
          </div>
        </div>
      </div>

      {/* Histogram + event log */}
      <div className="grid gap-4 xl:grid-cols-[320px_1fr]">
        <DelayHistogram events={events} />

        <div className="card p-5">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-sm font-semibold text-slate-200">
              Packet event log
            </h3>
            <div className="grid grid-cols-4 gap-3 text-xs text-slate-600 pr-1">
              <span>Time</span>
              <span>Type</span>
              <span>Hops</span>
              <span className="text-right">Latency</span>
            </div>
          </div>
          <div
            className="max-h-60 space-y-0.5 overflow-y-auto"
            role="log"
            aria-label="Packet event log"
            aria-live="polite"
            aria-relevant="additions"
          >
            {events.length === 0 ? (
              <p className="py-6 text-center text-xs text-slate-600">
                Waiting for packets…
              </p>
            ) : (
              events.map((evt, i) => (
                <PacketRow key={evt.id} event={evt} isNew={i === 0} />
              ))
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
