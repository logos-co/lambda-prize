import React, { useState } from "react";
import type { PrivacyLevel, ShieldedBalance, StakePosition } from "../types/privacy";
import { ProofProgressBar } from "./ProofProgressBar";
import { useToasts } from "../hooks/useToasts";

type Props = {
  balances: ShieldedBalance[];
  onAppendAudit?: (title: string, desc: string) => void;
};

const PROOF_DURATION_MS = 1800;

function randomHex(bytes = 16): string {
  return (
    "0x" +
    Array.from(
      { length: bytes },
      () => Math.floor(Math.random() * 256).toString(16).padStart(2, "0")
    ).join("")
  );
}

const INITIAL_POSITIONS: StakePosition[] = [
  {
    id: "pos-1",
    commitment: "0xdeadbeefdeadbeefdeadbeef00000001",
    nullifier: "0xabcdef1234567890abcdef1234567890",
    amount: "1000",
    assetId: "DEMO",
    status: "active",
    createdAt: new Date(Date.now() - 86400_000).toISOString(),
    privacyLevel: "private",
  },
  {
    id: "pos-2",
    commitment: "0xdeadbeefdeadbeefdeadbeef00000002",
    nullifier: "0xfedcba9876543210fedcba9876543210",
    amount: "420",
    assetId: "PRIV",
    status: "active",
    createdAt: new Date(Date.now() - 3600_000).toISOString(),
    privacyLevel: "confidential",
  },
];

function shortHex(h: string): string {
  return h.slice(0, 10) + "…" + h.slice(-6);
}

function statusBadge(s: StakePosition["status"]) {
  if (s === "active") return <span className="badge-emerald">{s}</span>;
  if (s === "pending") return <span className="badge-amber">{s}</span>;
  return <span className="badge-rose">{s}</span>;
}

export function StakeManager({ balances, onAppendAudit }: Props) {
  const { push } = useToasts();
  const [positions, setPositions] = useState<StakePosition[]>(INITIAL_POSITIONS);
  const [amount, setAmount] = useState("500");
  const [assetId, setAssetId] = useState("DEMO");
  const [privacyLevel, setPrivacyLevel] = useState<PrivacyLevel>("private");
  const [isStaking, setIsStaking] = useState(false);
  const [lastCommitment, setLastCommitment] = useState<string | null>(null);
  const [lastNullifier, setLastNullifier] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const assetOptions = [
    "DEMO",
    "PRIV",
    ...balances.map((b) => b.assetId).filter((a) => a !== "DEMO" && a !== "PRIV"),
  ];

  async function handleStake() {
    setError(null);
    if (!amount || Number(amount) <= 0) {
      setError("Enter a positive stake amount.");
      return;
    }
    setIsStaking(true);
    try {
      await new Promise((r) => setTimeout(r, PROOF_DURATION_MS + 200));
      const commitment = randomHex(16);
      const nullifier = randomHex(16);
      const pos: StakePosition = {
        id: crypto.randomUUID(),
        commitment,
        nullifier,
        amount,
        assetId,
        status: "pending",
        createdAt: new Date().toISOString(),
        privacyLevel,
      };
      setPositions((prev) => [pos, ...prev]);
      setLastCommitment(commitment);
      setLastNullifier(nullifier);
      onAppendAudit?.(
        "Stake submitted",
        `Shielded ${amount} ${assetId} — commitment ${shortHex(commitment)}`
      );
      push({
        kind: "success",
        title: "Stake note created",
        body: `${amount} ${assetId} · commitment ${shortHex(commitment)}`,
        duration: 5000,
      });
      setTimeout(() => {
        setPositions((prev) =>
          prev.map((p) => (p.id === pos.id ? { ...p, status: "active" } : p))
        );
      }, 2500);
      setAmount("");
    } catch (e) {
      const msg = e instanceof Error ? e.message : "Stake failed.";
      setError(msg);
      push({ kind: "error", title: "Stake failed", body: msg, duration: 5000 });
    } finally {
      setIsStaking(false);
    }
  }

  function handleWithdraw(id: string) {
    setPositions((prev) =>
      prev.map((p) => (p.id === id ? { ...p, status: "withdrawn" } : p))
    );
    onAppendAudit?.("Withdraw requested", `Position ${id} marked withdrawn.`);
    push({ kind: "warn", title: "Withdraw initiated", body: `Position ${id.slice(-6)}`, duration: 4000 });
  }

  return (
    <div className="space-y-4">
      <div className="card p-5">
        <h2 className="text-lg font-semibold text-slate-100">Private staking</h2>
        <p className="mt-1 text-sm text-slate-400">
          Create a shielded stake note using the Cryptarchia commitment scheme.
          The commitment is published on-chain; the nullifier stays local.
        </p>

        <div className="mt-4 grid gap-3 sm:grid-cols-3">
          <div>
            <label htmlFor="stake-amount" className="label-xs block mb-1.5">
              Amount
            </label>
            <input
              id="stake-amount"
              className="input-dark w-full"
              type="number"
              min="1"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="500"
              disabled={isStaking}
              aria-describedby={error ? "stake-error" : undefined}
            />
          </div>
          <div>
            <label htmlFor="stake-asset" className="label-xs block mb-1.5">
              Asset
            </label>
            <select
              id="stake-asset"
              className="input-dark w-full"
              value={assetId}
              onChange={(e) => setAssetId(e.target.value)}
              disabled={isStaking}
            >
              {assetOptions.map((a) => (
                <option key={a} value={a}>{a}</option>
              ))}
            </select>
          </div>
          <div>
            <label htmlFor="stake-privacy" className="label-xs block mb-1.5">
              Privacy level
            </label>
            <select
              id="stake-privacy"
              className="input-dark w-full"
              value={privacyLevel}
              onChange={(e) => setPrivacyLevel(e.target.value as PrivacyLevel)}
              disabled={isStaking}
            >
              <option value="public">Public</option>
              <option value="private">Private</option>
              <option value="confidential">Confidential</option>
            </select>
          </div>
        </div>

        {error ? (
          <p id="stake-error" role="alert" className="mt-3 text-sm text-rose-400">
            {error}
          </p>
        ) : null}

        <div className="mt-4 flex items-center gap-3">
          <button
            onClick={handleStake}
            disabled={isStaking}
            className="btn-primary"
            aria-busy={isStaking}
          >
            {isStaking ? "Generating proof…" : "Create stake"}
          </button>
          {!isStaking && (
            <span className="text-xs text-slate-500">
              Commitment and nullifier generated locally
            </span>
          )}
        </div>

        {isStaking ? (
          <ProofProgressBar durationMs={PROOF_DURATION_MS} />
        ) : null}

        {lastCommitment && !isStaking ? (
          <div className="mt-4 grid gap-2 sm:grid-cols-2">
            <div className="card-inner p-3">
              <div className="label-xs mb-1">Commitment (public)</div>
              <div className="mono text-xs text-violet-300 break-all">
                {lastCommitment}
              </div>
            </div>
            <div className="card-inner p-3">
              <div className="label-xs mb-1">Nullifier (private)</div>
              <div className="mono text-xs text-amber-300 break-all">
                {lastNullifier}
              </div>
            </div>
          </div>
        ) : null}
      </div>

      <div className="card p-5">
        <h3 className="text-sm font-semibold text-slate-200 mb-3">
          Active positions ({positions.length})
        </h3>
        {positions.length === 0 ? (
          <div className="rounded-xl border border-dashed border-slate-700 p-8 text-center">
            <div className="text-2xl mb-2 opacity-40">◈</div>
            <p className="text-sm text-slate-500">No stake positions yet.</p>
            <p className="text-xs text-slate-600 mt-1">
              Create your first shielded note above.
            </p>
          </div>
        ) : (
          <div className="space-y-2">
            {positions.map((pos) => (
              <div
                key={pos.id}
                className={[
                  "grid gap-3 rounded-xl border p-4 transition-all",
                  pos.status === "active"
                    ? "border-slate-700/50 bg-slate-800/40"
                    : pos.status === "pending"
                    ? "border-amber-500/20 bg-amber-500/5"
                    : "border-slate-800 bg-slate-900/40 opacity-50",
                ].join(" ")}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3 flex-wrap">
                    <span className="text-base font-bold text-slate-100">
                      {pos.amount}
                    </span>
                    <span className="badge-violet">{pos.assetId}</span>
                    {statusBadge(pos.status)}
                    <span className="badge-violet text-xs">{pos.privacyLevel}</span>
                  </div>
                  {pos.status === "active" ? (
                    <button
                      onClick={() => handleWithdraw(pos.id)}
                      className="btn-ghost text-xs px-3 py-1"
                    >
                      Withdraw
                    </button>
                  ) : null}
                </div>
                <div className="grid gap-2 sm:grid-cols-2">
                  <div>
                    <div className="label-xs mb-0.5">Commitment</div>
                    <div className="mono text-xs text-slate-400">
                      {shortHex(pos.commitment)}
                    </div>
                  </div>
                  <div>
                    <div className="label-xs mb-0.5">Nullifier</div>
                    <div className="mono text-xs text-slate-500">
                      {shortHex(pos.nullifier)}
                    </div>
                  </div>
                </div>
                <div className="text-xs text-slate-600">
                  Created {new Date(pos.createdAt).toLocaleString()}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
