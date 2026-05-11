import { useMemo } from "react";
import { usePersistentState } from "./usePersistentState";
import type { PrivacySettings } from "../types/privacy";

const DEFAULT_SETTINGS: PrivacySettings = {
  showRawAddresses: false,
  showRawAmounts: false,
  showTxHashes: false,
  showCounterpartyNames: false,
  redactMessageBodies: true,
  localEncryptionEnabled: true,
  autoLockMinutes: 10,
  preferredPrivacyLevel: "private",
};

export function usePrivacySettings() {
  const [settings, setSettings] = usePersistentState<PrivacySettings>(
    "privacy",
    "settings",
    DEFAULT_SETTINGS
  );

  const api = useMemo(() => {
    return {
      settings,
      setSettings,
      patch(partial: Partial<PrivacySettings>) {
        setSettings((prev) => ({ ...prev, ...partial }));
      },
      reset() {
        setSettings(DEFAULT_SETTINGS);
      },
    };
  }, [settings, setSettings]);

  return api;
}
