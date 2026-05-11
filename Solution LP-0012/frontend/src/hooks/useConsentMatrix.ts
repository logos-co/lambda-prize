import { useMemo } from "react";
import { usePersistentState } from "./usePersistentState";
import { DEFAULT_CONSENT } from "../lib/privacy-defaults";
import type { ConsentMatrix, ConsentValue, PrivacyScope } from "../lib/privacy-types";

export function useConsentMatrix() {
  const [consent, setConsent] = usePersistentState<ConsentMatrix>(
    "privacy2",
    "consent",
    DEFAULT_CONSENT
  );

  return useMemo(
    () => ({
      consent,
      setConsent,
      set(scope: PrivacyScope, value: ConsentValue) {
        setConsent((prev) => ({ ...prev, [scope]: value }));
      },
      reset() {
        setConsent(DEFAULT_CONSENT);
      },
    }),
    [consent, setConsent]
  );
}
