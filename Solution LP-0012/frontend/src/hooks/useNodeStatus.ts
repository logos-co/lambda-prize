import { useState, useEffect } from "react";
import type { NodeStatus } from "../types/privacy";

const HIST = 20;

export type NodeHistory = {
  participation: number[];
  latency: number[];
  pendingTx: number[];
};

const INITIAL: NodeStatus = {
  blockHeight: 2_847_391,
  slotNumber: 8_429_173,
  epochNumber: 87_803,
  validatorCount: 256,
  activeValidators: 196,
  networkParticipation: 76.6,
  pendingTxCount: 42,
  finalisedTxCount: 1_482_031,
  coverTrafficRate: 4.2,
  rpcLatencyMs: 23,
  isSynced: true,
};

function tick(prev: NodeStatus): NodeStatus {
  const newSlot = prev.slotNumber + 1;
  return {
    ...prev,
    blockHeight: prev.blockHeight + (Math.random() > 0.3 ? 1 : 0),
    slotNumber: newSlot,
    epochNumber: Math.floor(newSlot / 8192),
    pendingTxCount: Math.max(
      0,
      prev.pendingTxCount + Math.floor(Math.random() * 5 - 2)
    ),
    finalisedTxCount: prev.finalisedTxCount + Math.floor(Math.random() * 3),
    networkParticipation: Math.min(
      100,
      Math.max(60, prev.networkParticipation + (Math.random() - 0.5) * 1.5)
    ),
    rpcLatencyMs: Math.max(
      4,
      Math.min(250, prev.rpcLatencyMs + Math.floor(Math.random() * 16 - 8))
    ),
    coverTrafficRate: Math.max(
      0.5,
      Math.min(20, prev.coverTrafficRate + (Math.random() - 0.5) * 0.4)
    ),
    activeValidators: Math.max(
      150,
      Math.min(
        prev.validatorCount,
        prev.activeValidators + Math.floor(Math.random() * 3 - 1)
      )
    ),
  };
}

function makeInitialHistory(): NodeHistory {
  return {
    participation: Array(HIST).fill(INITIAL.networkParticipation),
    latency: Array(HIST).fill(INITIAL.rpcLatencyMs),
    pendingTx: Array(HIST).fill(INITIAL.pendingTxCount),
  };
}

export function useNodeStatus(intervalMs = 3000) {
  const [status, setStatus] = useState<NodeStatus>(INITIAL);
  const [history, setHistory] = useState<NodeHistory>(makeInitialHistory());

  useEffect(() => {
    const id = setInterval(() => {
      setStatus((prev) => {
        const next = tick(prev);
        setHistory((h) => ({
          participation: [...h.participation.slice(1), next.networkParticipation],
          latency: [...h.latency.slice(1), next.rpcLatencyMs],
          pendingTx: [...h.pendingTx.slice(1), next.pendingTxCount],
        }));
        return next;
      });
    }, intervalMs);
    return () => clearInterval(id);
  }, [intervalMs]);

  return { status, history, isLoading: false };
}
