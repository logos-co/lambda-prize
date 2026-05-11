import React from "react";
import type { TransactionPreview } from "../types/privacy";
import { redactTxHash } from "../lib/redaction";

type Props = {
  preview: TransactionPreview;
  onSubmit?: () => void;
};

export function TransactionPreviewCard({ preview, onSubmit }: Props) {
  return (
    <section className="card p-5">
      <div className="flex flex-col gap-2 sm:flex-row sm:items-start sm:justify-between">
        <div>
          <h3 className="text-base font-semibold text-slate-100">
            Transaction preview
          </h3>
          <p className="mt-1 text-sm text-slate-400">{preview.rawSummary}</p>
        </div>
        <div className="text-right shrink-0">
          <div className="text-sm font-medium text-slate-200">
            {preview.estimatedFee}
          </div>
          <div className="label-xs mt-0.5">estimated fee</div>
        </div>
      </div>

      <div className="mt-4 grid gap-2 grid-cols-2 xl:grid-cols-4">
        {[
          { label: "Intent", value: preview.intent.kind },
          { label: "Gas limit", value: preview.gasLimit.toLocaleString() },
          { label: "Nonce", value: preview.intent.nonce, mono: true },
          { label: "Tx ID", value: redactTxHash(preview.intent.id), mono: true },
        ].map(({ label, value, mono }) => (
          <div key={label} className="card-inner p-3">
            <div className="label-xs mb-1">{label}</div>
            <div className={`text-sm text-slate-200 ${mono ? "font-mono" : ""}`}>
              {value}
            </div>
          </div>
        ))}
      </div>

      {preview.warnings.length > 0 ? (
        <ul className="mt-4 space-y-1.5 rounded-xl border border-amber-500/20 bg-amber-500/5 p-4 text-sm text-amber-300">
          {preview.warnings.map((w) => (
            <li key={w} className="flex items-start gap-2">
              <span className="shrink-0">⚠</span>
              {w}
            </li>
          ))}
        </ul>
      ) : null}

      <div className="mt-4 flex items-center justify-between">
        <span
          className={`text-sm ${
            preview.canSubmit ? "text-emerald-400" : "text-rose-400"
          }`}
        >
          {preview.canSubmit ? "✓ Ready to submit" : "✗ Blocked by policy"}
        </span>
        <button
          disabled={!preview.canSubmit}
          onClick={onSubmit}
          className="btn-primary"
        >
          Submit
        </button>
      </div>
    </section>
  );
}
