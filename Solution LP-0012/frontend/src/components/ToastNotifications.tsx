import React from "react";
import { useToasts } from "../hooks/useToasts";
import type { ToastKind } from "../hooks/useToasts";

const KIND: Record<
  ToastKind,
  { bar: string; icon: string; iconBg: string; iconColor: string }
> = {
  info: {
    bar: "bg-violet-500",
    icon: "ℹ",
    iconBg: "bg-violet-500/15",
    iconColor: "text-violet-400",
  },
  success: {
    bar: "bg-emerald-500",
    icon: "✓",
    iconBg: "bg-emerald-500/15",
    iconColor: "text-emerald-400",
  },
  warn: {
    bar: "bg-amber-500",
    icon: "⚠",
    iconBg: "bg-amber-500/15",
    iconColor: "text-amber-400",
  },
  error: {
    bar: "bg-rose-500",
    icon: "✕",
    iconBg: "bg-rose-500/15",
    iconColor: "text-rose-400",
  },
};

export function ToastNotifications() {
  const { toasts, dismiss } = useToasts();

  if (toasts.length === 0) return null;

  return (
    <div
      className="fixed bottom-4 right-4 z-50 flex flex-col gap-2"
      style={{ width: 320 }}
      aria-live="polite"
      aria-label="Notifications"
    >
      {/* Dismiss-all when stacking */}
      {toasts.length >= 2 ? (
        <div className="flex justify-end">
          <button
            onClick={() => toasts.forEach((t) => dismiss(t.id))}
            className="rounded-lg px-2.5 py-1 text-xs text-slate-500 hover:text-slate-300 hover:bg-slate-800 transition-colors"
          >
            Dismiss all ({toasts.length})
          </button>
        </div>
      ) : null}

      {toasts.map((toast) => {
        const s = KIND[toast.kind];
        return (
          <div
            key={toast.id}
            role="status"
            className="toast-enter relative flex items-start gap-3 overflow-hidden rounded-2xl border border-slate-700/70 bg-slate-900 p-3.5 shadow-2xl shadow-black/60"
          >
            {/* Kind icon */}
            <div
              className={`mt-0.5 flex h-6 w-6 shrink-0 items-center justify-center rounded-lg text-xs font-bold ${s.iconBg} ${s.iconColor}`}
            >
              {s.icon}
            </div>

            {/* Content */}
            <div className="min-w-0 flex-1">
              <div className="text-sm font-semibold text-slate-100 leading-snug">
                {toast.title}
              </div>
              {toast.body ? (
                <div className="mt-0.5 text-xs text-slate-400 line-clamp-2 leading-relaxed">
                  {toast.body}
                </div>
              ) : null}
            </div>

            {/* Dismiss */}
            <button
              onClick={() => dismiss(toast.id)}
              aria-label="Dismiss"
              className="mt-0.5 shrink-0 text-slate-600 hover:text-slate-200 leading-none text-lg transition-colors"
            >
              ×
            </button>

            {/* Countdown progress bar — shrinks from right to left over duration */}
            {toast.duration > 0 ? (
              <div
                className={`absolute bottom-0 left-0 right-0 h-[2px] origin-left ${s.bar}`}
                style={{
                  animation: `countdown ${toast.duration}ms linear forwards`,
                }}
              />
            ) : (
              <div className={`absolute bottom-0 left-0 right-0 h-[2px] ${s.bar} opacity-40`} />
            )}
          </div>
        );
      })}
    </div>
  );
}
