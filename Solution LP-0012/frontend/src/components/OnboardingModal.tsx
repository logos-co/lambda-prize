import React, { useEffect, useRef, useState } from "react";

interface Props {
  open: boolean;
  onComplete: () => void;
  onConnectDemo: () => void;
  onConnectInjected: () => void;
}

const STEPS = [
  {
    icon: "⬡",
    title: "Welcome to Logos / LEZ",
    content: (
      <div className="space-y-4">
        <p className="text-sm text-slate-300 leading-relaxed">
          A privacy-first blockchain stack combining{" "}
          <span className="text-violet-300 font-medium">Cryptarchia</span>{" "}
          (Private Proof of Stake) and{" "}
          <span className="text-emerald-300 font-medium">Blend</span>{" "}
          (Proposer Anonymity via mixnet).
        </p>
        <div className="grid grid-cols-3 gap-3">
          {[
            { icon: "◎", label: "Live node data", color: "text-violet-400" },
            { icon: "⬡", label: "Mixnet routing", color: "text-emerald-400" },
            { icon: "⚄", label: "VRF lottery",    color: "text-amber-400"  },
          ].map((f) => (
            <div
              key={f.label}
              className="card-inner flex flex-col items-center gap-2 py-4 text-center"
            >
              <span className={`text-2xl ${f.color}`}>{f.icon}</span>
              <span className="text-xs text-slate-400">{f.label}</span>
            </div>
          ))}
        </div>
      </div>
    ),
  },
  {
    icon: "🔒",
    title: "Privacy by Default",
    content: (
      <div className="space-y-3">
        <p className="text-sm text-slate-400">
          Every design decision protects your identity — even from the node.
        </p>
        <ul className="space-y-3">
          {[
            {
              head: "Cryptarchia",
              body: "Stake-weighted block proposals without revealing which validator won.",
              color: "border-violet-500/40 bg-violet-500/5",
              dot: "bg-violet-400",
            },
            {
              head: "Blend mixnet",
              body: "Proposals are Sphinx-encrypted and forwarded through 5 independent mix nodes.",
              color: "border-emerald-500/40 bg-emerald-500/5",
              dot: "bg-emerald-400",
            },
            {
              head: "Local ZK proofs",
              body: "Commitments and nullifiers are computed in your browser. No keys ever leave.",
              color: "border-amber-500/40 bg-amber-500/5",
              dot: "bg-amber-400",
            },
          ].map((item) => (
            <li
              key={item.head}
              className={`flex gap-3 rounded-xl border p-3 ${item.color}`}
            >
              <span
                className={`mt-1 h-2 w-2 shrink-0 rounded-full ${item.dot}`}
              />
              <div>
                <div className="text-sm font-medium text-slate-200">
                  {item.head}
                </div>
                <div className="text-xs text-slate-400 mt-0.5">{item.body}</div>
              </div>
            </li>
          ))}
        </ul>
      </div>
    ),
  },
  {
    icon: "◎",
    title: "Get Started",
    content: null,
  },
  {
    icon: "✓",
    title: "You're all set!",
    content: (
      <div className="space-y-3">
        <p className="text-sm text-slate-400">Here's what to explore:</p>
        <ul className="space-y-2 text-sm">
          {[
            ["1  Node",    "Live block height, validators, network health", "text-violet-400"],
            ["2  Blend",   "Animated mixnet path + latency histogram",     "text-emerald-400"],
            ["3  Lottery", "VRF slot lottery with animated counter",        "text-amber-400"],
            ["4  Staking", "Create shielded stake notes with ZK proofs",   "text-violet-400"],
            ["5  Privacy", "Consent matrix, encrypted messages, transfers","text-slate-300"],
            ["6  Audit",   "Filterable security event log",                "text-slate-400"],
          ].map(([key, desc, color]) => (
            <li key={key} className="flex items-start gap-3">
              <kbd
                className={`shrink-0 rounded bg-slate-800 px-1.5 py-0.5 font-mono text-xs ring-1 ring-slate-700 ${color}`}
              >
                {key}
              </kbd>
              <span className="text-slate-400 text-xs pt-0.5">{desc}</span>
            </li>
          ))}
        </ul>
        <p className="text-xs text-slate-500 pt-1">
          Press <kbd className="rounded bg-slate-800 px-1 font-mono text-slate-300 ring-1 ring-slate-700">?</kbd> anytime to see keyboard shortcuts.
        </p>
      </div>
    ),
  },
];

export function OnboardingModal({
  open,
  onComplete,
  onConnectDemo,
  onConnectInjected,
}: Props) {
  const [step, setStep] = useState(0);
  const firstBtnRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (open) {
      setStep(0);
      setTimeout(() => firstBtnRef.current?.focus(), 50);
    }
  }, [open]);

  useEffect(() => {
    if (!open) return;
    function onKey(e: KeyboardEvent) {
      if (e.key === "Escape") onComplete();
      if (e.key === "ArrowRight" && step < STEPS.length - 1) setStep((s) => s + 1);
      if (e.key === "ArrowLeft" && step > 0) setStep((s) => s - 1);
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [open, step, onComplete]);

  if (!open) return null;

  const current = STEPS[step];
  const isLast = step === STEPS.length - 1;
  const isConnect = step === 2;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-4"
      style={{ background: "rgba(2,6,23,0.85)", backdropFilter: "blur(6px)" }}
      role="dialog"
      aria-modal="true"
      aria-labelledby="onboarding-title"
      onClick={isLast ? onComplete : undefined}
    >
      <div
        className="card w-full max-w-md shadow-2xl shadow-black/60 overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Top accent bar */}
        <div className="h-1 bg-gradient-to-r from-violet-600 via-violet-400 to-emerald-500" />

        <div className="p-6 space-y-5">
          {/* Step icon + title */}
          <div className="flex items-center gap-3">
            <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-violet-600/20 text-xl ring-1 ring-violet-500/30">
              {current.icon}
            </div>
            <h2
              id="onboarding-title"
              className="text-lg font-semibold text-slate-100"
            >
              {current.title}
            </h2>
          </div>

          {/* Step content */}
          {isConnect ? (
            <div className="space-y-3">
              <p className="text-sm text-slate-400">
                Choose how you'd like to explore the app.
              </p>
              <button
                ref={firstBtnRef}
                onClick={() => { onConnectDemo(); setStep((s) => s + 1); }}
                className="btn-primary w-full flex items-center justify-center gap-2 py-3"
              >
                <span>▶</span>
                Start in Demo Mode
              </button>
              <p className="text-center text-xs text-slate-500">
                Explore safely with simulated live data — no wallet required.
              </p>
              <div className="flex items-center gap-3">
                <div className="h-px flex-1 bg-slate-800" />
                <span className="text-xs text-slate-600">or</span>
                <div className="h-px flex-1 bg-slate-800" />
              </div>
              <button
                onClick={() => { onConnectInjected(); setStep((s) => s + 1); }}
                className="btn-ghost w-full py-3"
              >
                Connect Injected Wallet
              </button>
            </div>
          ) : (
            current.content
          )}

          {/* Step dots + navigation */}
          <div className="flex items-center justify-between pt-2">
            <div className="flex gap-1.5">
              {STEPS.map((_, i) => (
                <button
                  key={i}
                  onClick={() => setStep(i)}
                  aria-label={`Go to step ${i + 1}`}
                  className={`h-2 rounded-full transition-all duration-300 ${
                    i === step
                      ? "w-6 bg-violet-500"
                      : i < step
                      ? "w-2 bg-violet-500/40"
                      : "w-2 bg-slate-700"
                  }`}
                />
              ))}
            </div>

            <div className="flex items-center gap-2">
              {step > 0 ? (
                <button
                  onClick={() => setStep((s) => s - 1)}
                  className="btn-ghost text-xs px-3 py-1.5"
                >
                  ← Back
                </button>
              ) : (
                <button
                  onClick={onComplete}
                  className="text-xs text-slate-600 hover:text-slate-400 px-2"
                >
                  Skip
                </button>
              )}

              {!isConnect ? (
                isLast ? (
                  <button
                    ref={step === 0 ? firstBtnRef : undefined}
                    onClick={onComplete}
                    className="btn-primary text-xs px-4 py-1.5"
                  >
                    Open App →
                  </button>
                ) : (
                  <button
                    ref={step === 0 ? firstBtnRef : undefined}
                    onClick={() => setStep((s) => s + 1)}
                    className="btn-primary text-xs px-4 py-1.5"
                  >
                    Next →
                  </button>
                )
              ) : null}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
