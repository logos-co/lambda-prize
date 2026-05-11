import React from "react";
import { consentLabel } from "../lib/consent";
import type { ConsentScope, PrivacyConsentMatrix } from "../types/privacy";

type ConsentValue = "allow" | "deny" | "ask";

type Props = {
  consent: PrivacyConsentMatrix;
  onChange: (scope: ConsentScope, value: ConsentValue) => void;
  onAcceptAll: () => void;
  onDenyAll: () => void;
};

const SCOPES: ConsentScope[] = [
  "identity",
  "balances",
  "transactions",
  "messages",
  "analytics",
  "support",
];

const SCOPE_META: Record<ConsentScope, { icon: string; desc: string }> = {
  identity:     { icon: "◎", desc: "Address & identity data" },
  balances:     { icon: "◈", desc: "Shielded balance values" },
  transactions: { icon: "⬡", desc: "Tx history & commitments" },
  messages:     { icon: "◻", desc: "Encrypted message content" },
  analytics:    { icon: "⊞", desc: "Usage & performance telemetry" },
  support:      { icon: "⊛", desc: "Debugging & support logs" },
};

const SEG_STYLES: Record<
  ConsentValue,
  { active: string; inactive: string }
> = {
  allow: {
    active: "bg-emerald-600/25 text-emerald-300 ring-1 ring-emerald-500/30",
    inactive: "text-slate-600 hover:text-slate-400",
  },
  ask: {
    active: "bg-amber-600/25 text-amber-300 ring-1 ring-amber-500/30",
    inactive: "text-slate-600 hover:text-slate-400",
  },
  deny: {
    active: "bg-rose-600/25 text-rose-400 ring-1 ring-rose-500/30",
    inactive: "text-slate-600 hover:text-slate-400",
  },
};

const OPTIONS: ConsentValue[] = ["allow", "ask", "deny"];
const OPTION_LABEL: Record<ConsentValue, string> = {
  allow: "Allow",
  ask:   "Ask",
  deny:  "Deny",
};

function SegmentedConsent({
  scope,
  value,
  onChange,
}: {
  scope: ConsentScope;
  value: ConsentValue;
  onChange: (v: ConsentValue) => void;
}) {
  const meta = SCOPE_META[scope];
  return (
    <div className="rounded-xl border border-slate-700/60 bg-slate-800/40 p-3">
      <div className="flex items-center gap-2 mb-2.5">
        <span className="text-slate-500 text-sm">{meta.icon}</span>
        <div>
          <span className="text-sm font-medium capitalize text-slate-200">
            {scope}
          </span>
          <span className="ml-2 text-xs text-slate-600">{meta.desc}</span>
        </div>
      </div>

      {/* 3-button segmented control */}
      <div
        className="flex rounded-lg overflow-hidden border border-slate-700/60 bg-slate-900/60"
        role="group"
        aria-label={`${scope} consent`}
      >
        {OPTIONS.map((opt, i) => {
          const isActive = value === opt;
          const styles = SEG_STYLES[opt];
          return (
            <button
              key={opt}
              onClick={() => onChange(opt)}
              aria-pressed={isActive}
              className={[
                "flex-1 py-1.5 text-xs font-medium capitalize transition-all duration-150",
                i !== 0 ? "border-l border-slate-700/60" : "",
                isActive ? styles.active : styles.inactive,
              ].join(" ")}
            >
              {OPTION_LABEL[opt]}
            </button>
          );
        })}
      </div>
    </div>
  );
}

export function ConsentBanner({ consent, onChange, onAcceptAll, onDenyAll }: Props) {
  const allowCount = SCOPES.filter((s) => consent[s] === "allow").length;
  const denyCount  = SCOPES.filter((s) => consent[s] === "deny").length;

  return (
    <div className="card p-5">
      <div className="flex flex-col gap-3 sm:flex-row sm:items-start sm:justify-between">
        <div>
          <h2 className="text-lg font-semibold text-slate-100">Privacy consent</h2>
          <p className="mt-1 text-sm text-slate-400">
            Choose what the app may read, store, or display.
          </p>
          <div className="mt-1.5 flex items-center gap-2">
            {allowCount > 0 && (
              <span className="badge-emerald">{allowCount} allowed</span>
            )}
            {denyCount > 0 && (
              <span className="badge-rose">{denyCount} denied</span>
            )}
            {allowCount === 0 && denyCount === 0 && (
              <span className="text-xs text-slate-600">Defaults are deny-first</span>
            )}
          </div>
        </div>
        <div className="flex gap-2 shrink-0">
          <button onClick={onAcceptAll} className="btn-primary text-xs px-3 py-1.5">
            Support only
          </button>
          <button onClick={onDenyAll} className="btn-ghost text-xs px-3 py-1.5">
            Deny all
          </button>
        </div>
      </div>

      <div className="mt-4 grid gap-3 sm:grid-cols-2 xl:grid-cols-3">
        {SCOPES.map((scope) => (
          <SegmentedConsent
            key={scope}
            scope={scope}
            value={consent[scope] as ConsentValue}
            onChange={(v) => onChange(scope, v)}
          />
        ))}
      </div>
    </div>
  );
}
