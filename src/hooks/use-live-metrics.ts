import { useEffect, useRef, useState } from "react";
import { backend, toBackendError } from "@/services/backend";
import { logger } from "@/services/logger";
import type { BackendError, MetricsSample } from "@/types/system";

const POLL_MS = 1000;
const MAX_SAMPLES = 300;

/**
 * Polls the backend sampler for new samples once per second. The backend keeps the
 * history (5 min), so the first poll back-fills the charts immediately.
 */
export function useLiveMetrics() {
  const [samples, setSamples] = useState<MetricsSample[]>([]);
  const [error, setError] = useState<BackendError>();
  const lastTs = useRef<number | undefined>(undefined);
  const failures = useRef(0);

  useEffect(() => {
    let cancelled = false;
    let timer: ReturnType<typeof setTimeout>;

    const tick = async () => {
      try {
        const fresh = await backend.getMetrics(lastTs.current);
        if (cancelled) return;
        failures.current = 0;
        setError(undefined);
        if (fresh.length > 0) {
          lastTs.current = fresh[fresh.length - 1]!.timestampMs;
          setSamples((prev) => {
            const next = prev.concat(fresh);
            return next.length > MAX_SAMPLES ? next.slice(next.length - MAX_SAMPLES) : next;
          });
        }
      } catch (err) {
        if (cancelled) return;
        failures.current += 1;
        const e = toBackendError(err);
        setError(e);
        // Log the first failure of a streak only, so a dead backend doesn't spam the log.
        if (failures.current === 1) logger.warn(`metrics poll failed: ${e.message}`, "metrics");
      }
      if (!cancelled) timer = setTimeout(tick, POLL_MS);
    };

    tick();
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, []);

  return { samples, latest: samples.at(-1), error };
}
