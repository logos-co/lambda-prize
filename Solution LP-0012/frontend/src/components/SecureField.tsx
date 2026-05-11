import React, { useState } from "react";
import { Eye, EyeOff, Copy } from "lucide-react";

export function SecureField({
  label,
  value,
  visible = false,
  copyable = true,
  fallback = "—",
}: {
  label: string;
  value?: string;
  visible?: boolean;
  copyable?: boolean;
  fallback?: string;
}) {
  const [revealed, setRevealed] = useState(visible);
  const [copied, setCopied] = useState(false);

  const rendered = value
    ? revealed
      ? value
      : value.length > 24
        ? `${value.slice(0, 8)}…${value.slice(-6)}`
        : "••••••••"
    : fallback;

  async function copy() {
    if (!value || !copyable) return;
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      // ignore
    }
  }

  return (
    <div className="rounded-2xl border border-white/10 bg-white/5 p-4">
      <div className="flex items-center justify-between gap-2">
        <span className="text-xs font-medium uppercase tracking-[0.25em] text-slate-500">
          {label}
        </span>
        <div className="flex gap-2">
          {value ? (
            <button
              onClick={() => setRevealed((v) => !v)}
              className="text-slate-400 transition hover:text-white"
              aria-label={revealed ? "Hide value" : "Reveal value"}
            >
              {revealed ? (
                <EyeOff className="h-4 w-4" />
              ) : (
                <Eye className="h-4 w-4" />
              )}
            </button>
          ) : null}
          {copyable && value ? (
            <button
              onClick={copy}
              className="text-slate-400 transition hover:text-white"
              aria-label="Copy to clipboard"
            >
              <Copy className="h-4 w-4" />
            </button>
          ) : null}
        </div>
      </div>
      <div className="mt-2 break-all font-mono text-sm text-white">
        {copied ? (
          <span className="text-emerald-300">Copied!</span>
        ) : (
          rendered
        )}
      </div>
    </div>
  );
}
