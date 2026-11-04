import { useEffect, useState } from "react";
import { backend, toBackendError } from "@/services/backend";
import { logger } from "@/services/logger";
import type { RunningProgram } from "@/types/system";

const POLL_MS = 3000;

/** Reads the running programs only while something on screen needs them. */
export function useRunningPrograms() {
  const [programs, setPrograms] = useState<RunningProgram[]>([]);

  useEffect(() => {
    let cancelled = false;
    let failed = false;
    let timer: ReturnType<typeof setTimeout>;

    const tick = async () => {
      try {
        const next = await backend.getRunningPrograms();
        if (cancelled) return;
        failed = false;
        setPrograms(next);
      } catch (err) {
        if (cancelled) return;
        setPrograms([]);
        // The first failure of a streak only, so a dead backend doesn't spam the log.
        if (!failed) logger.warn(`program list failed: ${toBackendError(err).message}`, "games");
        failed = true;
      }
      timer = setTimeout(() => void tick(), POLL_MS);
    };

    void tick();
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, []);

  return programs;
}
