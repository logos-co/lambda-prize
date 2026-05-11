import React from "react";
import { AnimatePresence, motion } from "framer-motion";
import { X, CheckCircle2, AlertTriangle, Info, XCircle } from "lucide-react";
import { useToasts } from "../hooks/useToasts";
import type { ToastKind } from "../hooks/useToasts";

const ICONS: Record<ToastKind, React.ReactNode> = {
  success: <CheckCircle2 className="h-4 w-4 text-emerald-300 shrink-0" />,
  warn: <AlertTriangle className="h-4 w-4 text-amber-300 shrink-0" />,
  error: <XCircle className="h-4 w-4 text-rose-300 shrink-0" />,
  info: <Info className="h-4 w-4 text-cyan-300 shrink-0" />,
};

const BORDERS: Record<ToastKind, string> = {
  success: "border-emerald-500/30 bg-emerald-500/10",
  warn: "border-amber-500/30 bg-amber-500/10",
  error: "border-rose-500/30 bg-rose-500/10",
  info: "border-cyan-500/30 bg-cyan-500/10",
};

export function ToastHost() {
  const { toasts, dismiss } = useToasts();

  return (
    <div
      className="fixed bottom-6 right-6 z-50 flex flex-col gap-3"
      style={{ pointerEvents: "none" }}
      aria-live="polite"
      aria-atomic="false"
    >
      <AnimatePresence>
        {toasts.map((toast) => (
          <motion.div
            key={toast.id}
            initial={{ x: 60, opacity: 0, scale: 0.95 }}
            animate={{ x: 0, opacity: 1, scale: 1 }}
            exit={{ x: 60, opacity: 0, scale: 0.95 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
            style={{ pointerEvents: "auto" }}
            className={`flex items-start gap-3 rounded-2xl border px-4 py-3 shadow-2xl shadow-black/30 max-w-sm w-full ${BORDERS[toast.kind]}`}
            role="alert"
          >
            <div className="pt-0.5">{ICONS[toast.kind]}</div>
            <div className="flex-1 min-w-0">
              <p className="text-sm font-medium text-white leading-tight">
                {toast.title}
              </p>
              {toast.body ? (
                <p className="mt-0.5 text-xs leading-5 text-slate-300">
                  {toast.body}
                </p>
              ) : null}
            </div>
            <button
              onClick={() => dismiss(toast.id)}
              className="text-slate-400 transition hover:text-white shrink-0 mt-0.5"
              aria-label="Dismiss notification"
            >
              <X className="h-3.5 w-3.5" />
            </button>
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
