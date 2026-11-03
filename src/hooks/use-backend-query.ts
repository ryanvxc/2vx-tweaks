import { useCallback, useEffect, useRef, useState } from "react";
import { toBackendError } from "@/services/backend";
import type { BackendError } from "@/types/system";

interface QueryState<T> {
  data: T | undefined;
  error: BackendError | undefined;
  loading: boolean;
}

// Results survive page switches so navigating doesn't flash loading states.
const cache = new Map<string, unknown>();
// Mounted queries per key, so a change in one place (an apply) refreshes every view of it.
const listeners = new Map<string, Set<() => void>>();

/**
 * Minimal async query hook for backend calls. `key` identifies the result in the
 * session cache; `refetch` bypasses it.
 */
export function useBackendQuery<T>(key: string, fetcher: () => Promise<T>) {
  const [state, setState] = useState<QueryState<T>>(() => ({
    data: cache.get(key) as T | undefined,
    error: undefined,
    loading: !cache.has(key),
  }));
  const mounted = useRef(true);
  const fetcherRef = useRef(fetcher);

  // Only updates state from promise callbacks, never synchronously.
  const load = useCallback(() => {
    fetcherRef.current().then(
      (data) => {
        cache.set(key, data);
        if (mounted.current) setState({ data, error: undefined, loading: false });
      },
      (err: unknown) => {
        if (mounted.current)
          setState((s) => ({ ...s, error: toBackendError(err), loading: false }));
      },
    );
  }, [key]);

  const refetch = useCallback(() => {
    setState((s) => ({ ...s, loading: true, error: undefined }));
    load();
  }, [load]);

  useEffect(() => {
    mounted.current = true;
    if (!cache.has(key)) load();
    let set = listeners.get(key);
    if (!set) listeners.set(key, (set = new Set()));
    set.add(refetch);
    return () => {
      mounted.current = false;
      set.delete(refetch);
    };
  }, [key, load, refetch]);

  return { ...state, refetch };
}

/** Drops cached results and reloads every mounted query for these keys. */
export function invalidateQueries(keys: string[]) {
  for (const key of keys) {
    cache.delete(key);
    listeners.get(key)?.forEach((reload) => reload());
  }
}

/** Test helper. */
export function clearQueryCache() {
  cache.clear();
}
