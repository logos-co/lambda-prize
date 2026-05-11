import { PrivacyUiError, toAppErrorInfo } from "./errors";
import { safeJsonParse } from "./validation";

export interface StorageAdapter {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
  keys(): string[];
}

class MemoryStorageAdapter implements StorageAdapter {
  private store = new Map<string, string>();

  getItem(key: string): string | null {
    return this.store.get(key) ?? null;
  }

  setItem(key: string, value: string): void {
    this.store.set(key, value);
  }

  removeItem(key: string): void {
    this.store.delete(key);
  }

  keys(): string[] {
    return Array.from(this.store.keys());
  }
}

class BrowserStorageAdapter implements StorageAdapter {
  constructor(private readonly storage: Storage) {}

  getItem(key: string): string | null {
    return this.storage.getItem(key);
  }

  setItem(key: string, value: string): void {
    this.storage.setItem(key, value);
  }

  removeItem(key: string): void {
    this.storage.removeItem(key);
  }

  keys(): string[] {
    return Object.keys(this.storage);
  }
}

export function createStorageAdapter(): StorageAdapter {
  if (typeof window === "undefined" || typeof window.localStorage === "undefined") {
    return new MemoryStorageAdapter();
  }

  try {
    const probe = "__privacy_probe__";
    window.localStorage.setItem(probe, "1");
    window.localStorage.removeItem(probe);
    return new BrowserStorageAdapter(window.localStorage);
  } catch {
    return new MemoryStorageAdapter();
  }
}

export function createSessionStorageAdapter(): StorageAdapter {
  if (typeof window === "undefined" || typeof window.sessionStorage === "undefined") {
    return new MemoryStorageAdapter();
  }

  try {
    const probe = "__privacy_probe__";
    window.sessionStorage.setItem(probe, "1");
    window.sessionStorage.removeItem(probe);
    return new BrowserStorageAdapter(window.sessionStorage);
  } catch {
    return new MemoryStorageAdapter();
  }
}

export function createJsonStore(adapter: StorageAdapter, namespace: string) {
  const prefix = `${namespace}::`;

  return {
    get<T>(key: string, fallback: T): T {
      const raw = adapter.getItem(prefix + key);
      if (!raw) return fallback;
      return safeJsonParse(raw, fallback);
    },
    set<T>(key: string, value: T): void {
      try {
        adapter.setItem(prefix + key, JSON.stringify(value));
      } catch (cause) {
        throw new PrivacyUiError(
          toAppErrorInfo("STORAGE_WRITE_FAILED", "Unable to save local data.", {
            hint: "Clear some browser storage or disable strict privacy mode for local data.",
          }),
          cause
        );
      }
    },
    remove(key: string): void {
      adapter.removeItem(prefix + key);
    },
    listKeys(): string[] {
      return adapter
        .keys()
        .filter((k) => k.startsWith(prefix))
        .map((k) => k.slice(prefix.length));
    },
  };
}
