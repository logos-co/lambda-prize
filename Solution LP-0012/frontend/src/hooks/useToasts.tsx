import React, { createContext, useCallback, useContext, useState } from "react";

export type ToastKind = "info" | "success" | "warn" | "error";

export interface Toast {
  id: string;
  title: string;
  body?: string;
  kind: ToastKind;
  duration: number;
}

interface ToastCtx {
  toasts: Toast[];
  push: (t: Omit<Toast, "id">) => void;
  dismiss: (id: string) => void;
}

const Ctx = createContext<ToastCtx | null>(null);

export function ToastProvider({ children }: { children: React.ReactNode }) {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const push = useCallback((t: Omit<Toast, "id">) => {
    const id = `toast-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
    const duration = t.duration ?? 4000;
    setToasts((prev) => [...prev.slice(-4), { ...t, id, duration }]);
    if (duration > 0) {
      setTimeout(() => {
        setToasts((prev) => prev.filter((x) => x.id !== id));
      }, duration);
    }
  }, []);

  const dismiss = useCallback((id: string) => {
    setToasts((prev) => prev.filter((x) => x.id !== id));
  }, []);

  return (
    <Ctx.Provider value={{ toasts, push, dismiss }}>{children}</Ctx.Provider>
  );
}

export function useToasts(): ToastCtx {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("useToasts must be inside ToastProvider");
  return ctx;
}
