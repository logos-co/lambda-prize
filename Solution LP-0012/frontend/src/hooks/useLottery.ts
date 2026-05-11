import { useState, useEffect } from "react";
import type { LotterySlot } from "../types/privacy";

const DIFFICULTY = 0.004;

function randomHex(bytes = 8): string {
  return (
    "0x" +
    Array.from(
      { length: bytes },
      () => Math.floor(Math.random() * 256).toString(16).padStart(2, "0")
    ).join("")
  );
}

function makeSlot(slotNum: number): LotterySlot {
  const won = Math.random() < DIFFICULTY;
  return {
    slot: slotNum,
    epoch: Math.floor(slotNum / 8192),
    vrfOutput: randomHex(8),
    difficulty: DIFFICULTY,
    won,
    proposalId: won ? randomHex(6) : undefined,
  };
}

const INITIAL_SLOT = 8_429_173;

export function useLottery() {
  const [slots, setSlots] = useState<LotterySlot[]>(() =>
    Array.from({ length: 12 }, (_, i) =>
      makeSlot(INITIAL_SLOT - (11 - i))
    ).reverse()
  );
  const [currentSlot, setCurrentSlot] = useState(INITIAL_SLOT);
  const [isRunning, setIsRunning] = useState(true);

  useEffect(() => {
    if (!isRunning) return;
    const id = setInterval(() => {
      setCurrentSlot((prev) => {
        const next = prev + 1;
        setSlots((prevSlots) =>
          [makeSlot(next), ...prevSlots].slice(0, 24)
        );
        return next;
      });
    }, 4000);
    return () => clearInterval(id);
  }, [isRunning]);

  return { slots, currentSlot, isRunning, setIsRunning };
}
