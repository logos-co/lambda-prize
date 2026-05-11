import React, { useState } from "react";
import type { ShieldedBalance, PrivacySettings } from "../types/privacy";
import { formatShieldedBalance, redactTxHash } from "../lib/redaction";

type Props = {
  balance: ShieldedBalance;
  settings: PrivacySettings;
  onReveal?: (assetId: string) => void;
};

const ASSET_ACCENT: Record<string, { text: string; bg: string; border: string }> = {
  DEMO: { text: "text-violet-300", bg: "bg-violet-500/10", border: "border-violet-500/25" },
  PRIV: { text: "text-emerald-300", bg: "bg-emerald-500/10", border: "border-emerald-500/25" },
};

const DEFAULT_ACCENT = {
  text: "text-slate-300",
  bg: "bg-slate-500/10",
  border: "border-slate-500/25",
};

function getAccent(assetId: string) {
  return ASSET_ACCENT[assetId] ?? DEFAULT_ACCENT;
}

function shortHash(h: string, chars = 10): string {
  if (h.length <= chars * 2 + 2) return h;
  return h.slice(0, chars) + "…" + h.slice(-6);
}

function CopyHint({ value }: { value: string }) {
  const [copied, setCopied] = useState(false);
  function copy() {
    navigator.clipboard.writeText(value).catch(() => {});
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  }
  return (
    <button
      onClick={copy}
      className="ml-1 rounded px-1 py-0.5 text-[10px] text-slate-700 hover:text-slate-300 hover:bg-slate-700/50 transition-colors"
      title="Copy to clipboard"
      aria-label="Copy hash"
    >
      {copied ? "✓" : "copy"}
    </button>
  );
}

export function ShieldedBalanceCard({ balance, settings, onReveal }: Props) {
  const accent = getAccent(balance.assetId);

  return (
    <article className={`card overflow-hidden border ${accent.border}`}>
      {/* Coloured top bar */}
      <div
        className="h-[3px]"
        style={{
          background: balance.shielded
            ? "linear-gradient(90deg, rgb(139 92 246 / 0.8), rgb(52 211 153 / 0.5))"
            : "linear-gradient(90deg, rgb(52 211 153 / 0.8), rgb(99 102 241 / 0.5))",
        }}
      />

      <div className="p-5">
        {/* Header */}
        <div className="flex items-start justify-between gap-3">
          <div className="flex items-center gap-2.5">
            <div
              className={`flex h-9 w-9 items-center justify-center rounded-xl text-lg ${accent.bg}`}
              aria-hidden="true"
            >
              {balance.shielded ? "⊗" : "◎"}
            </div>
            <div>
              <div className="text-sm font-semibold text-slate-100">
                {balance.shielded ? "Shielded balance" : "Public balance"}
              </div>
              <div className="text-xs text-slate-500 mt-0.5">
                {formatShieldedBalance(balance, settings)}
              </div>
            </div>
          </div>
          <span className={balance.shielded ? "badge-violet" : "badge-emerald"}>
            {balance.shielded ? "private" : "public"}
          </span>
        </div>

        {/* Large balance display */}
        <div className="mt-5 rounded-xl border border-slate-700/40 bg-slate-950/60 p-4">
          <div className="label-xs mb-2">Available balance</div>
          <div className="flex items-baseline gap-2">
            {settings.showRawAmounts ? (
              <>
                <span className={`text-3xl font-bold tabular-nums ${accent.text}`}>
                  {Number(balance.available).toLocaleString("en-US")}
                </span>
                <span className={`text-base font-semibold ${accent.text} opacity-60`}>
                  {balance.assetId}
                </span>
              </>
            ) : (
              <span className="text-2xl text-slate-600 font-medium tracking-widest">
                ● ● ● ●
              </span>
            )}
          </div>
          {balance.pending && balance.pending !== "0" ? (
            <div className="mt-1.5 text-xs text-amber-500/80">
              +{balance.pending} {balance.assetId} pending
            </div>
          ) : null}
        </div>

        {/* Commitments */}
        <dl className="mt-3 space-y-2">
          <div className="card-inner p-3">
            <dt className="label-xs mb-1 flex items-center justify-between">
              <span>Owner commitment</span>
              <CopyHint value={balance.ownerCommitment} />
            </dt>
            <dd className="mono text-xs text-slate-500 break-all leading-relaxed">
              {redactTxHash(balance.ownerCommitment, false)}
            </dd>
          </div>
          <div className="card-inner p-3">
            <dt className="label-xs mb-1 flex items-center justify-between">
              <span>Balance commitment</span>
              <CopyHint value={balance.balanceCommitment} />
            </dt>
            <dd className="mono text-xs text-slate-500 break-all leading-relaxed">
              {shortHash(balance.balanceCommitment)}
            </dd>
          </div>
        </dl>

        {/* Actions */}
        {onReveal ? (
          <div className="mt-4 flex items-center gap-2">
            <button
              onClick={() => onReveal(balance.assetId)}
              className="btn-ghost text-xs px-3 py-1.5"
            >
              Reveal to me
            </button>
            {!settings.showRawAmounts ? (
              <span className="text-xs text-slate-600">
                Amount hidden by privacy settings
              </span>
            ) : null}
          </div>
        ) : null}
      </div>
    </article>
  );
}
