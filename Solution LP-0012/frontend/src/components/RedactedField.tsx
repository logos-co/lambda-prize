import React, { useState } from "react";

type Props = {
  label: string;
  value?: string;
  visible?: boolean;
  copyable?: boolean;
  fallback?: string;
};

export function RedactedField({
  label,
  value,
  visible = false,
  copyable = false,
  fallback = "—",
}: Props) {
  const [copied, setCopied] = useState(false);

  const rendered = value
    ? visible
      ? value
      : value.length > 18
      ? `${value.slice(0, 8)}…${value.slice(-6)}`
      : value
    : fallback;

  async function copy() {
    if (!value || !copyable || typeof navigator === "undefined") return;
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch {
      // ignore clipboard failures
    }
  }

  return (
    <div className="card-inner p-3">
      <div className="flex items-center justify-between gap-2">
        <span className="label-xs">{label}</span>
        {copyable ? (
          <button
            onClick={copy}
            className="text-xs text-violet-400 hover:text-violet-300 transition-colors"
          >
            {copied ? "✓ copied" : "copy"}
          </button>
        ) : null}
      </div>
      <div className="mt-1 break-all mono text-sm text-slate-300">{rendered}</div>
    </div>
  );
}
