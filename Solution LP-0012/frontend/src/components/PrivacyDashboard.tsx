import React, { useMemo, useState } from "react";
import type {
  AuditEvent,
  MessageEnvelope,
  PrivacyConsentMatrix,
  PrivacySettings,
  ShieldedBalance,
  WalletSession,
} from "../types/privacy";
import { ConsentBanner } from "./ConsentBanner";
import { ShieldedBalanceCard } from "./ShieldedBalanceCard";
import { TransactionPreviewCard } from "./TransactionPreviewCard";
import { EncryptedMessageComposer } from "./EncryptedMessageComposer";
import { createTransactionIntent, buildTransactionPreview } from "../lib/transactionPreview";
import { DEFAULT_CONSENT, setConsent } from "../lib/consent";
import { privacySummary, redactMessageEnvelope } from "../lib/redaction";

type Props = {
  wallet: WalletSession;
  settings: PrivacySettings;
  onSettingsChange: (patch: Partial<PrivacySettings>) => void;
  onSettingsReset: () => void;
  consent: PrivacyConsentMatrix;
  onConsentChange: (
    scope: keyof PrivacyConsentMatrix,
    value: "allow" | "deny" | "ask"
  ) => void;
  onConsentReset: () => void;
  balances: ShieldedBalance[];
  audits: AuditEvent[];
  onAppendAudit: (event: AuditEvent) => void;
  encryptionKey?: CryptoKey;
  onSubmitTransfer?: () => Promise<void> | void;
  onRevealBalance?: (assetId: string) => void;
  onMessageCreated?: (message: MessageEnvelope) => void;
};

export function PrivacyDashboard({
  wallet,
  settings,
  onSettingsChange,
  onSettingsReset,
  consent,
  onConsentChange,
  onConsentReset,
  balances,
  onAppendAudit,
  encryptionKey,
  onSubmitTransfer,
  onRevealBalance,
  onMessageCreated,
}: Props) {
  const [amount, setAmount] = useState("1.0");
  const [recipient, setRecipient] = useState(
    "0x0000000000000000000000000000000000000000"
  );
  const [memo, setMemo] = useState("private transfer");

  const intent = useMemo(
    () =>
      createTransactionIntent({
        chainId: wallet.chainId ?? 0,
        kind: "transfer",
        sender: wallet.account,
        recipient: recipient as `0x${string}`,
        amount,
        memo,
        privacyLevel: settings.preferredPrivacyLevel,
      }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [wallet.account, wallet.chainId, amount, recipient, memo, settings.preferredPrivacyLevel]
  );

  const preview = useMemo(
    () => buildTransactionPreview(intent, settings),
    [intent, settings]
  );

  const sampleMessage = useMemo<MessageEnvelope | null>(() => {
    if (!onMessageCreated) return null;
    return redactMessageEnvelope(
      {
        id: "sample-message",
        createdAt: new Date().toISOString(),
        from: wallet.account,
        to: recipient as `0x${string}`,
        subject: "Status update",
        plaintextPreview: "We moved the funds privately.",
        privacyLevel: settings.preferredPrivacyLevel,
        tags: ["sample"],
      },
      settings
    );
  }, [wallet.account, recipient, settings, onMessageCreated]);

  function log(
    label: string,
    description: string,
    level: AuditEvent["level"] = "info",
    category: AuditEvent["category"] = "security"
  ) {
    onAppendAudit({
      id: crypto.randomUUID(),
      timestamp: new Date().toISOString(),
      category,
      level,
      title: label,
      description,
    });
  }

  return (
    <div className="space-y-6">
      <div className="card p-5">
        <div className="flex flex-col gap-4 lg:flex-row lg:items-start lg:justify-between">
          <div>
            <h2 className="text-xl font-semibold text-slate-100">
              Privacy settings
            </h2>
            <p className="mt-1 text-sm text-slate-400">
              Redacted by default — only reveal what you explicitly allow.
            </p>
            <p className="mt-1 text-xs text-slate-500">{privacySummary(settings)}</p>
          </div>

          <div className="rounded-xl border border-slate-700 bg-slate-800/60 p-3 text-sm">
            <div className="flex gap-2 text-slate-400">
              <span>Wallet:</span>
              <span className={wallet.connected ? "text-emerald-400" : "text-rose-400"}>
                {wallet.connected ? "connected" : "disconnected"}
              </span>
            </div>
            {wallet.account ? (
              <div className="mt-1 mono text-xs text-slate-500">
                {wallet.account}
              </div>
            ) : null}
          </div>
        </div>

        <div className="mt-4 grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
          {(
            [
              { key: "showRawAddresses", label: "Raw addresses" },
              { key: "showRawAmounts", label: "Raw amounts" },
              { key: "redactMessageBodies", label: "Redact messages" },
              { key: "localEncryptionEnabled", label: "Local encryption" },
            ] as Array<{ key: keyof typeof settings; label: string }>
          ).map(({ key, label }) => (
            <label
              key={key}
              className="flex cursor-pointer items-center justify-between rounded-xl border border-slate-700 bg-slate-800/40 px-4 py-3 hover:bg-slate-800/70 transition-colors"
            >
              <span className="text-sm text-slate-300">{label}</span>
              <div className="relative">
                <input
                  type="checkbox"
                  checked={settings[key] as boolean}
                  onChange={(e) =>
                    onSettingsChange({ [key]: e.target.checked })
                  }
                  className="sr-only"
                />
                <div
                  className={[
                    "h-5 w-9 rounded-full transition-colors",
                    (settings[key] as boolean)
                      ? "bg-violet-600"
                      : "bg-slate-600",
                  ].join(" ")}
                />
                <div
                  className={[
                    "absolute top-0.5 h-4 w-4 rounded-full bg-white shadow transition-transform",
                    (settings[key] as boolean) ? "translate-x-4" : "translate-x-0.5",
                  ].join(" ")}
                />
              </div>
            </label>
          ))}
        </div>

        <div className="mt-4 flex gap-2">
          <button onClick={onSettingsReset} className="btn-ghost text-xs">
            Reset settings
          </button>
          <button
            onClick={() => log("settings", "User reviewed privacy settings")}
            className="btn-primary text-xs"
          >
            Log review
          </button>
        </div>
      </div>

      <ConsentBanner
        consent={consent}
        onChange={onConsentChange}
        onAcceptAll={() => {
          onConsentChange("support", "allow");
          onConsentChange("transactions", "ask");
          onConsentChange("balances", "ask");
          log("consent", "Support-only consent selected");
        }}
        onDenyAll={() => {
          onConsentReset();
          log("consent", "Consent matrix reset to deny-first");
        }}
      />

      <section className="grid gap-6 xl:grid-cols-2">
        <div className="space-y-4">
          {balances.length === 0 ? (
            <div className="rounded-2xl border border-dashed border-slate-700 bg-slate-900/40 p-8 text-center text-sm text-slate-500">
              No shielded balances. Connect a wallet to load data.
            </div>
          ) : (
            balances.map((balance) => (
              <ShieldedBalanceCard
                key={`${balance.assetId}-${balance.ownerCommitment}`}
                balance={balance}
                settings={settings}
                onReveal={onRevealBalance}
              />
            ))
          )}
        </div>

        <div className="space-y-4">
          <TransactionPreviewCard
            preview={preview}
            onSubmit={() => {
              onSubmitTransfer?.();
              log("tx", "Private transfer accepted", "info", "tx");
            }}
          />

          <div className="card p-5">
            <h3 className="text-base font-semibold text-slate-100 mb-4">
              Prepare transfer
            </h3>
            <div className="space-y-3">
              <input
                value={recipient}
                onChange={(e) => setRecipient(e.target.value)}
                className="input-dark w-full"
                placeholder="Recipient address"
              />
              <input
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                className="input-dark w-full"
                placeholder="Amount"
              />
              <input
                value={memo}
                onChange={(e) => setMemo(e.target.value)}
                className="input-dark w-full"
                placeholder="Memo"
              />
            </div>
          </div>

          <EncryptedMessageComposer
            settings={settings}
            encryptionKey={encryptionKey}
            onCreate={(message) => {
              onMessageCreated?.(message);
              log("message", "Encrypted message created", "info", "message");
            }}
          />

          {sampleMessage ? (
            <div className="card p-5">
              <h3 className="text-base font-semibold text-slate-100 mb-3">
                Message preview
              </h3>
              <pre className="overflow-auto rounded-xl bg-slate-950 p-4 text-xs text-slate-400">
                {JSON.stringify(sampleMessage, null, 2)}
              </pre>
            </div>
          ) : null}
        </div>
      </section>
    </div>
  );
}
