import React from "react";
import { consentLabel } from "../lib/privacy-defaults";
import type { ConsentMatrix, ConsentValue, PrivacyScope } from "../lib/privacy-types";

const SCOPES: PrivacyScope[] = [
  "identity",
  "wallet",
  "balances",
  "transactions",
  "messages",
  "analytics",
  "support",
  "sharing",
];

export function ConsentMatrixEditor({
  consent,
  onChange,
}: {
  consent: ConsentMatrix;
  onChange: (scope: PrivacyScope, value: ConsentValue) => void;
}) {
  return (
    <div className="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
      {SCOPES.map((scope) => (
        <label
          key={scope}
          className="rounded-2xl border border-white/10 bg-white/5 p-4"
        >
          <div className="flex items-center justify-between gap-3">
            <span className="font-medium capitalize text-white">{scope}</span>
            <span className="text-xs text-slate-400">
              {consentLabel(consent[scope])}
            </span>
          </div>
          <select
            value={consent[scope]}
            onChange={(e) =>
              onChange(scope, e.target.value as ConsentValue)
            }
            className="mt-3 w-full rounded-xl border border-white/10 bg-slate-950 px-3 py-2 text-sm text-white outline-none"
          >
            <option value="allow">Allow</option>
            <option value="deny">Deny</option>
            <option value="ask">Ask each time</option>
          </select>
        </label>
      ))}
    </div>
  );
}
