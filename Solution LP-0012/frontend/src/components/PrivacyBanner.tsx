import React from "react";
import { Link } from "react-router-dom";
import { Shield, Lock, Eye, MessageSquareText } from "lucide-react";

export function PrivacyBanner() {
  return (
    <div className="grid gap-4 rounded-[2rem] border border-cyan-300/20 bg-cyan-300/10 p-5 md:grid-cols-[1.2fr_0.8fr]">
      <div>
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-2xl bg-white/10">
            <Shield className="h-5 w-5 text-cyan-200" />
          </div>
          <div>
            <h3 className="text-lg font-semibold text-white">
              Privacy-first by default
            </h3>
            <p className="text-sm text-cyan-100/90">
              Keep raw values hidden unless the user opts in.
            </p>
          </div>
        </div>
        <p className="mt-4 max-w-2xl text-sm leading-7 text-cyan-50/90">
          The interface uses redaction, local encryption, and selective
          disclosure to make sensitive data easier to manage.
        </p>
      </div>

      <div className="grid gap-2 sm:grid-cols-2">
        <Link
          to="/privacy-center"
          className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10"
        >
          <div className="flex items-center gap-2 font-medium text-white">
            <Lock className="h-4 w-4" /> Privacy center
          </div>
          <div className="mt-1 text-xs text-slate-300">
            Review settings and defaults.
          </div>
        </Link>
        <Link
          to="/sharing"
          className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10"
        >
          <div className="flex items-center gap-2 font-medium text-white">
            <Eye className="h-4 w-4" /> Safe sharing
          </div>
          <div className="mt-1 text-xs text-slate-300">
            Export only what is needed.
          </div>
        </Link>
        <Link
          to="/consent"
          className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10"
        >
          <div className="flex items-center gap-2 font-medium text-white">
            <MessageSquareText className="h-4 w-4" /> Consent
          </div>
          <div className="mt-1 text-xs text-slate-300">
            Scope-level permissions.
          </div>
        </Link>
        <Link
          to="/vault"
          className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 text-sm transition hover:bg-white/10"
        >
          <div className="flex items-center gap-2 font-medium text-white">
            <Lock className="h-4 w-4" /> Vault
          </div>
          <div className="mt-1 text-xs text-slate-300">
            Keep notes sealed locally.
          </div>
        </Link>
      </div>
    </div>
  );
}
