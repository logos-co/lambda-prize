import { useMemo } from "react";
import { usePersistentState } from "./usePersistentState";
import { DEFAULT_SETTINGS } from "../lib/privacy-defaults";
import type { PrivacySettings } from "../lib/privacy-types";

export function usePrivacySettingsNew() {
  const [settings, setSettings] = usePersistentState<PrivacySettings>(
    "privacy2",
    "settings",
    DEFAULT_SETTINGS
  );

  return useMemo(
    () => ({
      settings,
      setSettings,
      patch(partial: Partial<PrivacySettings>) {
        setSettings((prev) => ({ ...prev, ...partial }));
      },
      reset() {
        setSettings(DEFAULT_SETTINGS);
      },
    }),
    [settings, setSettings]
  );
}
