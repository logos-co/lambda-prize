import { useEffect, useMemo, useState } from "react";
import { createJsonStore, createStorageAdapter } from "../lib/storage";

export function usePersistentState<T>(
  namespace: string,
  key: string,
  fallback: T
): [T, React.Dispatch<React.SetStateAction<T>>] {
  const store = useMemo(
    () => createJsonStore(createStorageAdapter(), namespace),
    [namespace]
  );
  const [value, setValue] = useState<T>(() => store.get(key, fallback));

  useEffect(() => {
    store.set(key, value);
  }, [key, store, value]);

  return [value, setValue];
}
