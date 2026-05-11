import { useState, useEffect, useCallback } from "react";
import type { BlendPacketEvent } from "../types/privacy";

function randomHex(bytes = 4): string {
  return Array.from(
    { length: bytes },
    () => Math.floor(Math.random() * 256).toString(16).padStart(2, "0")
  ).join("");
}

function makePacket(isReal: boolean): BlendPacketEvent {
  return {
    id: randomHex(8),
    timestamp: new Date().toISOString(),
    hopCount: 3 + Math.floor(Math.random() * 3),
    isReal,
    latencyMs: 40 + Math.floor(Math.random() * 220),
  };
}

export function useBlendMonitor() {
  const [events, setEvents] = useState<BlendPacketEvent[]>(() =>
    Array.from({ length: 8 }, (_, i) =>
      makePacket(i === 3)
    ).reverse()
  );
  const [animating, setAnimating] = useState<string[]>([]);
  const [coverRate, setCoverRate] = useState(4.2);

  const fire = useCallback(() => {
    const pkt = makePacket(Math.random() < 0.18);
    setEvents((prev) => [pkt, ...prev].slice(0, 60));
    setAnimating((prev) => [...prev, pkt.id]);
    setTimeout(() => {
      setAnimating((prev) => prev.filter((id) => id !== pkt.id));
    }, 1400);
    setCoverRate((prev) =>
      Math.max(0.5, Math.min(20, prev + (Math.random() - 0.5) * 0.35))
    );
  }, []);

  useEffect(() => {
    const id = setInterval(fire, 850);
    return () => clearInterval(id);
  }, [fire]);

  return { events, animating, coverRate };
}
