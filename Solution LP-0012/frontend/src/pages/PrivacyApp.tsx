import React, { useEffect, useMemo, useState } from "react";
import { NavTabs } from "../components/NavTabs";
import type { TabId } from "../components/NavTabs";
import { NodeStatusDashboard } from "../components/NodeStatusDashboard";
import { BlendMonitor } from "../components/BlendMonitor";
import { LotteryViewer } from "../components/LotteryViewer";
import { StakeManager } from "../components/StakeManager";
import { PrivacyDashboard } from "../components/PrivacyDashboard";
import { AuditTrailPanel } from "../components/AuditTrailPanel";
import { OnboardingModal } from "../components/OnboardingModal";
import { ToastNotifications } from "../components/ToastNotifications";
import { ToastProvider, useToasts } from "../hooks/useToasts";
import { usePrivacySettings } from "../hooks/usePrivacySettings";
import { useWalletConnection } from "../hooks/useWalletConnection";
import { DEFAULT_CONSENT, setConsent } from "../lib/consent";
import { createJsonStore, createStorageAdapter } from "../lib/storage";
import type {
  AuditEvent,
  PrivacyConsentMatrix,
  ShieldedBalance,
  MessageEnvelope,
} from "../types/privacy";
import { deriveKeyFromPassphrase, randomId } from "../lib/crypto";
import { PrivacyBlockchainClient } from "../lib/blockchainClient";
import { PrivacyUiError, toAppErrorInfo } from "../lib/errors";

const consentStore = createJsonStore(createStorageAdapter(), "privacy-app");
const auditStore = createJsonStore(createStorageAdapter(), "privacy-app-audit");

const DEMO_BALANCES: ShieldedBalance[] = [
  {
    assetId: "DEMO",
    ownerCommitment:
      "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
    balanceCommitment:
      "0xbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdead",
    available: "1500",
    pending: "25",
    shielded: true,
  },
  {
    assetId: "PRIV",
    ownerCommitment:
      "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    balanceCommitment:
      "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    available: "420",
    pending: "0",
    shielded: true,
  },
];

function shortAddr(addr?: string): string {
  if (!addr) return "—";
  return addr.slice(0, 6) + "…" + addr.slice(-4);
}

/* ── Shortcuts overlay ───────────────────────────────────────────────── */

function ShortcutsOverlay({ onClose }: { onClose: () => void }) {
  const rows = [
    { key: "1 – 6", desc: "Switch tabs (Node → Audit)" },
    { key: "?",     desc: "Toggle this panel" },
    { key: "Esc",   desc: "Close panels & modals" },
  ];
  return (
    <div
      className="fixed inset-0 z-40 flex items-center justify-center p-4"
      style={{ background: "rgba(2,6,23,0.75)", backdropFilter: "blur(4px)" }}
      onClick={onClose}
      role="dialog"
      aria-modal="true"
      aria-label="Keyboard shortcuts"
    >
      <div
        className="card w-full max-w-xs shadow-2xl shadow-black/60"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="h-0.5 rounded-t-2xl bg-gradient-to-r from-violet-600 to-emerald-500" />
        <div className="p-5">
          <div className="flex items-center justify-between mb-4">
            <span className="text-sm font-semibold text-slate-100">
              Keyboard shortcuts
            </span>
            <button
              onClick={onClose}
              className="text-slate-500 hover:text-slate-200 text-xl leading-none transition-colors"
              aria-label="Close"
            >
              ×
            </button>
          </div>
          <div className="space-y-3">
            {rows.map(({ key, desc }) => (
              <div key={key} className="flex items-center gap-4">
                <kbd className="shrink-0 rounded bg-slate-800 px-2 py-1 font-mono text-xs text-slate-200 ring-1 ring-slate-700">
                  {key}
                </kbd>
                <span className="text-sm text-slate-400">{desc}</span>
              </div>
            ))}
          </div>
          <p className="mt-4 text-xs text-slate-600">
            Shortcuts are inactive while typing in input fields.
          </p>
        </div>
      </div>
    </div>
  );
}

/* ── PrivacyAppInner (needs ToastProvider above it) ──────────────────── */

function PrivacyAppInner() {
  const { push } = useToasts();
  const settingsApi = usePrivacySettings();
  const walletApi = useWalletConnection();

  const [activeTab, setActiveTab] = useState<TabId>("dashboard");
  const [consent, setConsentMatrix] = useState<PrivacyConsentMatrix>(() =>
    consentStore.get("consent", DEFAULT_CONSENT)
  );
  const [audits, setAudits] = useState<AuditEvent[]>(() =>
    auditStore.get("events", [])
  );
  const [balances, setBalances] = useState<ShieldedBalance[]>([]);
  const [secretKey, setSecretKey] = useState<CryptoKey | undefined>(undefined);
  const [unlockPassphrase, setUnlockPassphrase] = useState("");
  const [status, setStatus] = useState<string>("Ready");
  const [error, setError] = useState<string | null>(null);
  const [showUnlock, setShowUnlock] = useState(false);
  const [showShortcuts, setShowShortcuts] = useState(false);
  const [onboardingOpen, setOnboardingOpen] = useState(() => {
    try { return !localStorage.getItem("lez-onboarded"); }
    catch { return false; }
  });

  const client = useMemo(
    () =>
      new PrivacyBlockchainClient({
        baseUrl: "/api",
        chainId: walletApi.session.chainId ?? 1,
      }),
    [walletApi.session.chainId]
  );

  /* persist state */
  useEffect(() => { consentStore.set("consent", consent); }, [consent]);
  useEffect(() => { auditStore.set("events", audits); }, [audits]);

  /* auto-dismiss error after 5 s */
  useEffect(() => {
    if (!error) return;
    const t = setTimeout(() => setError(null), 5000);
    return () => clearTimeout(t);
  }, [error]);

  /* load balances */
  useEffect(() => {
    if (!walletApi.session.connected || !walletApi.session.account) return;
    client
      .getShieldedBalances(walletApi.session.account)
      .then(setBalances)
      .catch(() => setBalances(DEMO_BALANCES));
  }, [client, walletApi.session.account, walletApi.session.connected]);

  /* keyboard shortcuts */
  useEffect(() => {
    const TAB_MAP: Record<string, TabId> = {
      "1": "dashboard",
      "2": "blend",
      "3": "lottery",
      "4": "staking",
      "5": "privacy",
      "6": "audit",
    };
    function handler(e: KeyboardEvent) {
      if (
        e.target instanceof HTMLInputElement ||
        e.target instanceof HTMLTextAreaElement ||
        e.target instanceof HTMLSelectElement
      ) return;
      if (e.metaKey || e.ctrlKey || e.altKey) return;

      if (TAB_MAP[e.key]) {
        e.preventDefault();
        setActiveTab(TAB_MAP[e.key] as TabId);
      }
      if (e.key === "?") setShowShortcuts((v) => !v);
      if (e.key === "Escape") {
        setShowShortcuts(false);
        setShowUnlock(false);
      }
    }
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, []);

  /* ── handlers ─────────────────────────────────────────────────────── */

  function completeOnboarding() {
    try { localStorage.setItem("lez-onboarded", "1"); } catch {}
    setOnboardingOpen(false);
  }

  async function unlockLocalSecrets() {
    try {
      setError(null);
      const key = await deriveKeyFromPassphrase(
        unlockPassphrase,
        "privacy-app-local-salt"
      );
      setSecretKey(key);
      setShowUnlock(false);
      appendAudit("security", "Local key derived", "Key kept in memory only.");
      push({ kind: "success", title: "Local key unlocked", body: "Key kept in memory only", duration: 4000 });
    } catch (cause) {
      const msg = cause instanceof Error ? cause.message : "Unable to derive local key";
      setError(msg);
      appendAudit("security", "Local unlock failed", "Passphrase derivation failed.", "warn");
      push({ kind: "error", title: "Unlock failed", body: msg, duration: 5000 });
    }
  }

  function appendAudit(
    category: AuditEvent["category"],
    title: string,
    description: string,
    level: AuditEvent["level"] = "info"
  ) {
    setAudits((prev) => [
      ...prev,
      {
        id: randomId("audit"),
        timestamp: new Date().toISOString(),
        category,
        level,
        title,
        description,
      },
    ]);
  }

  async function submitTransfer() {
    try {
      setError(null);
      if (!walletApi.session.account) {
        throw new PrivacyUiError(
          toAppErrorInfo("NO_WALLET", "Connect a wallet before submitting.")
        );
      }
      appendAudit("tx", "Transfer submitted", "Preparing private transfer.");
      setStatus("Submitting private transfer…");
      const result = await client.submitPrivateTransfer({
        chainId: walletApi.session.chainId ?? 1,
        assetId: balances[0]?.assetId ?? "DEMO",
        fromCommitment:
          balances[0]?.ownerCommitment ??
          "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        toCommitment:
          "0x1111111111111111111111111111111111111111111111111111111111111111",
        amount: "1",
        memo: "frontend private transfer",
        privacyLevel: settingsApi.settings.preferredPrivacyLevel,
      });
      appendAudit("tx", "Transfer submitted", `Tx ${result.txHash}`, "info");
      setStatus(`Submitted: ${result.txHash}`);
      push({ kind: "success", title: "Transfer submitted", body: `Tx ${result.txHash.slice(0, 14)}…`, duration: 5000 });
    } catch (cause) {
      const msg = cause instanceof Error ? cause.message : "Transfer failed";
      setError(msg);
      appendAudit("tx", "Transfer failed", msg, "error");
      setStatus("Transfer failed.");
      push({ kind: "error", title: "Transfer failed", body: msg, duration: 5000 });
    }
  }

  function handleReveal(assetId: string) {
    appendAudit("security", "Reveal request", `Reveal balance for ${assetId}`);
    setStatus(`Reveal requested for ${assetId}`);
    push({ kind: "info", title: "Reveal requested", body: assetId, duration: 3000 });
  }

  function handleMessage(message: MessageEnvelope) {
    appendAudit("message", "Message created", message.subject ?? "No subject");
    push({ kind: "success", title: "Message created", body: message.subject ?? "No subject", duration: 4000 });
  }

  /* ── render ───────────────────────────────────────────────────────── */

  return (
    <div className="min-h-screen" style={{ background: "rgb(2 6 23)" }}>
      {/* ── Header ─────────────────────────────────────────────────── */}
      <header className="sticky top-0 z-30 border-b border-slate-800 bg-slate-950/90 backdrop-blur-md">
        <div className="mx-auto flex max-w-7xl items-center justify-between gap-4 px-4 py-3">
          {/* Brand */}
          <div className="flex items-center gap-3 shrink-0">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-violet-600/20 text-lg ring-1 ring-violet-500/30">
              ⬡
            </div>
            <span className="text-sm font-semibold text-slate-100 hidden sm:block">
              Logos / LEZ
            </span>
          </div>

          {/* Nav */}
          <div className="flex-1 overflow-x-auto">
            <NavTabs active={activeTab} onChange={setActiveTab} />
          </div>

          {/* Actions */}
          <div className="flex items-center gap-2 shrink-0">
            {!walletApi.isConnected ? (
              <>
                <button
                  onClick={() => { walletApi.connectDemo(); push({ kind: "info", title: "Demo mode active", body: "Simulated data only", duration: 3000 }); }}
                  className="btn-primary text-xs px-3 py-1.5"
                >
                  Demo
                </button>
                <button
                  onClick={walletApi.connectInjected}
                  disabled={walletApi.connecting}
                  className="btn-ghost text-xs px-3 py-1.5"
                >
                  Connect
                </button>
              </>
            ) : (
              <div className="flex items-center gap-2">
                <div className="hidden sm:flex items-center gap-2 rounded-xl border border-slate-700 bg-slate-800 px-3 py-1.5">
                  <span className="live-dot" aria-hidden="true" />
                  <span className="mono text-xs text-slate-300">
                    {shortAddr(walletApi.session.account)}
                  </span>
                </div>
                <button
                  onClick={walletApi.disconnect}
                  className="btn-ghost text-xs px-3 py-1.5"
                >
                  Disconnect
                </button>
              </div>
            )}

            <button
              onClick={() => setShowUnlock((v) => !v)}
              title={secretKey ? "Local key unlocked" : "Unlock local encryption"}
              aria-label={secretKey ? "Local key unlocked" : "Unlock local encryption"}
              aria-pressed={!!secretKey}
              className={[
                "btn-ghost text-xs px-3 py-1.5",
                secretKey ? "text-emerald-400 border-emerald-500/30" : "",
              ].join(" ")}
            >
              {secretKey ? "🔓" : "🔐"}
            </button>

            <button
              onClick={() => setShowShortcuts((v) => !v)}
              title="Keyboard shortcuts (?)"
              aria-label="Keyboard shortcuts"
              aria-expanded={showShortcuts}
              className="btn-ghost text-xs px-2.5 py-1.5 font-mono"
            >
              ?
            </button>

            <button
              onClick={() => setOnboardingOpen(true)}
              title="Open onboarding guide"
              aria-label="Open guide"
              className="btn-ghost text-xs px-2.5 py-1.5"
            >
              ☰
            </button>
          </div>
        </div>

        {/* Unlock row */}
        {showUnlock ? (
          <div className="border-t border-slate-800 px-4 py-3 animate-[slot-flash_0.4s_ease-out_forwards]">
            <div className="mx-auto flex max-w-7xl items-center gap-3">
              <label htmlFor="passphrase" className="text-xs text-slate-500 shrink-0">
                Local passphrase
              </label>
              <input
                id="passphrase"
                value={unlockPassphrase}
                onChange={(e) => setUnlockPassphrase(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") unlockLocalSecrets();
                  if (e.key === "Escape") setShowUnlock(false);
                }}
                placeholder="Enter passphrase to unlock local encryption"
                className="input-dark flex-1 text-xs"
                type="password"
                autoFocus
                aria-describedby="passphrase-hint"
              />
              <button onClick={unlockLocalSecrets} className="btn-primary text-xs">
                Unlock
              </button>
            </div>
            <p id="passphrase-hint" className="sr-only">
              Your passphrase is used to derive a local encryption key. It never leaves your browser.
            </p>
          </div>
        ) : null}

        {/* Error banner (auto-dismisses) */}
        {error ? (
          <div
            role="alert"
            className="border-t border-rose-500/20 bg-rose-500/5 px-4 py-2"
          >
            <div className="mx-auto max-w-7xl flex items-center justify-between gap-3 text-xs">
              <span className="text-rose-400">{error}</span>
              <button
                onClick={() => setError(null)}
                className="text-rose-500/60 hover:text-rose-400 transition-colors text-base leading-none"
                aria-label="Dismiss error"
              >
                ×
              </button>
            </div>
          </div>
        ) : null}
      </header>

      {/* ── Overlays ────────────────────────────────────────────────── */}
      {showShortcuts ? (
        <ShortcutsOverlay onClose={() => setShowShortcuts(false)} />
      ) : null}

      <OnboardingModal
        open={onboardingOpen}
        onComplete={completeOnboarding}
        onConnectDemo={() => {
          walletApi.connectDemo();
          push({ kind: "info", title: "Demo mode active", body: "Simulated live data", duration: 3000 });
        }}
        onConnectInjected={walletApi.connectInjected}
      />

      {/* ── Main content ─────────────────────────────────────────────── */}
      <main
        className="mx-auto max-w-7xl px-4 py-6"
        role="main"
        aria-label={`${activeTab} panel`}
      >
        {activeTab === "dashboard" && <NodeStatusDashboard />}
        {activeTab === "blend"     && <BlendMonitor />}
        {activeTab === "lottery"   && <LotteryViewer />}

        {activeTab === "staking" && (
          <StakeManager
            balances={balances.length > 0 ? balances : DEMO_BALANCES}
            onAppendAudit={(title, desc) => appendAudit("tx", title, desc)}
          />
        )}

        {activeTab === "privacy" && (
          <PrivacyDashboard
            wallet={walletApi.session}
            settings={settingsApi.settings}
            onSettingsChange={settingsApi.patch}
            onSettingsReset={settingsApi.reset}
            consent={consent}
            onConsentChange={(scope, value) => {
              setConsentMatrix((prev) => {
                const next = setConsent(prev, scope, value);
                consentStore.set("consent", next);
                return next;
              });
              appendAudit("consent", `Consent: ${scope}`, `Set ${scope} to ${value}`);
            }}
            onConsentReset={() => {
              setConsentMatrix(DEFAULT_CONSENT);
              consentStore.set("consent", DEFAULT_CONSENT);
              appendAudit("consent", "Consent reset", "Restored to defaults.");
            }}
            balances={balances.length > 0 ? balances : DEMO_BALANCES}
            audits={audits}
            onAppendAudit={(event) => setAudits((prev) => [...prev, event])}
            encryptionKey={secretKey}
            onSubmitTransfer={submitTransfer}
            onRevealBalance={handleReveal}
            onMessageCreated={handleMessage}
          />
        )}

        {activeTab === "audit" && <AuditTrailPanel events={audits} />}
      </main>

      {/* ── Toasts ───────────────────────────────────────────────────── */}
      <ToastNotifications />
    </div>
  );
}

/* ── Public export ───────────────────────────────────────────────────── */

export function PrivacyApp() {
  return (
    <ToastProvider>
      <PrivacyAppInner />
    </ToastProvider>
  );
}
