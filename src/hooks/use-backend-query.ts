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

  useEffect(() => {
    mounted.current = true;
    if (!cache.has(key)) load();
    return () => {
      mounted.current = false;
    };
  }, [key, load]);

  const refetch = useCallback(() => {
    setState((s) => ({ ...s, loading: true, error: undefined }));
    load();
  }, [load]);

  return { ...state, refetch };
}

/** Test helper. */
export function clearQueryCache() {
  cache.clear();
}
