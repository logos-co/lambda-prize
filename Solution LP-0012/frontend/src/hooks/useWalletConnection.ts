import { useCallback, useEffect, useMemo, useState } from "react";
import type { WalletSession } from "../types/privacy";
import { PrivacyUiError, toAppErrorInfo } from "../lib/errors";
import { usePersistentState } from "./usePersistentState";

const EMPTY_SESSION: WalletSession = { connected: false, provider: "demo" };

export function useWalletConnection() {
  const [session, setSession] = usePersistentState<WalletSession>(
    "wallet",
    "session",
    EMPTY_SESSION
  );
  const [connecting, setConnecting] = useState(false);

  const connectInjected = useCallback(async () => {
    setConnecting(true);
    try {
      const eth = (
        window as Window & {
          ethereum?: { request: (args: { method: string }) => Promise<string[]> };
        }
      ).ethereum;
      if (!eth) {
        throw new PrivacyUiError(
          toAppErrorInfo("NO_WALLET", "No injected wallet was found.", {
            hint: "Install a browser wallet or continue in demo mode.",
          })
        );
      }

      const accounts = await eth.request({ method: "eth_requestAccounts" });
      const account = accounts[0];
      if (!account) {
        throw new PrivacyUiError(
          toAppErrorInfo("NO_ACCOUNT", "No wallet account was returned.")
        );
      }

      setSession({
        connected: true,
        account: account as `0x${string}`,
        chainId: 0,
        provider: "injected",
        lastConnectedAt: new Date().toISOString(),
      });
    } finally {
      setConnecting(false);
    }
  }, [setSession]);

  const connectDemo = useCallback(() => {
    setSession({
      connected: true,
      account: "0xd3m0000000000000000000000000000000000001" as `0x${string}`,
      chainId: 1,
      provider: "demo",
      lastConnectedAt: new Date().toISOString(),
    });
  }, [setSession]);

  const disconnect = useCallback(() => {
    setSession({ connected: false, provider: session.provider ?? "demo" });
  }, [setSession, session.provider]);

  useEffect(() => {
    if (!session.connected) return;
    const handle = setInterval(() => {
      if (session.lastConnectedAt) {
        const last = new Date(session.lastConnectedAt).getTime();
        const diffMinutes = (Date.now() - last) / 60_000;
        if (diffMinutes > 60) {
          setSession((prev) => ({ ...prev, connected: false }));
        }
      }
    }, 30_000);

    return () => clearInterval(handle);
  }, [session.connected, session.lastConnectedAt, setSession]);

  return useMemo(
    () => ({
      session,
      connecting,
      connectInjected,
      connectDemo,
      disconnect,
      isConnected: session.connected,
      account: session.account,
    }),
    [session, connecting, connectInjected, connectDemo, disconnect]
  );
}
