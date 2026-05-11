import React from "react";
import {
  redactAddress,
  redactAmount,
  redactTxHash,
  redactMessage,
  redactWalletProvider,
} from "../lib/privacy-redact";
import type { PrivacySettings } from "../lib/privacy-types";

const RAW = {
  address: "0x1234567890abcdef1234567890abcdef12345678",
  amount: "12345.6789",
  txHash:
    "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcd",
  message:
    "This is a sensitive note that should stay hidden until explicitly revealed.",
  provider: "walletconnect",
};

export function RedactionPreview({
  settings,
}: {
  settings: PrivacySettings;
}) {
  return (
    <div className="grid gap-4 md:grid-cols-2">
      <div className="rounded-3xl border border-white/10 bg-white/5 p-5">
        <h3 className="text-lg font-semibold text-white">Raw values</h3>
        <div className="mt-4 space-y-3 break-all font-mono text-sm text-slate-300">
          <div>Address: {RAW.address}</div>
          <div>Amount: {RAW.amount}</div>
          <div>Tx hash: {RAW.txHash}</div>
          <div>Message: {RAW.message}</div>
          <div>Provider: {RAW.provider}</div>
        </div>
      </div>

      <div className="rounded-3xl border border-white/10 bg-white/5 p-5">
        <h3 className="text-lg font-semibold text-white">Redacted view</h3>
        <div className="mt-4 space-y-3 break-all font-mono text-sm text-slate-300">
          <div>
            Address:{" "}
            <span className={settings.showRawAddresses ? "text-amber-200" : ""}>
              {redactAddress(RAW.address, settings.showRawAddresses)}
            </span>
          </div>
          <div>
            Amount:{" "}
            <span className={settings.showRawAmounts ? "text-amber-200" : ""}>
              {redactAmount(RAW.amount, settings.showRawAmounts)}
            </span>
          </div>
          <div>
            Tx hash:{" "}
            <span className={settings.showTxHashes ? "text-amber-200" : ""}>
              {redactTxHash(RAW.txHash, settings.showTxHashes)}
            </span>
          </div>
          <div>
            Message:{" "}
            <span className={settings.showMessageBodies ? "text-amber-200" : ""}>
              {redactMessage(RAW.message, settings.showMessageBodies)}
            </span>
          </div>
          <div>
            Provider:{" "}
            <span className={settings.showWalletProvider ? "text-amber-200" : ""}>
              {redactWalletProvider(RAW.provider, settings.showWalletProvider)}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}
