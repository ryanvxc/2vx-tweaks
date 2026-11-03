import { invoke, isTauri } from "@tauri-apps/api/core";
import type { TweakMeta } from "@/types/engine";
import type { AppInfo, BackendError, MetricsSample, SystemSnapshot } from "@/types/system";
import { preview } from "./preview";

/**
 * The only module allowed to call `invoke`. Every backend command gets a typed
 * wrapper here; UI code never builds command names or arguments itself.
 *
 * Outside the desktop shell (plain `npm run dev` in a browser, or tests) calls are
 * served by `preview`, which returns clearly simulated data.
 */
export const isDesktop = isTauri();

export type LogLevel = "debug" | "info" | "warn" | "error";

function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args).catch((err: unknown) => {
    throw toBackendError(err);
  });
}

export function toBackendError(err: unknown): BackendError {
  if (err && typeof err === "object" && "code" in err && "message" in err) {
    return err as BackendError;
  }
  return { code: "internal", message: err instanceof Error ? err.message : String(err) };
}

export const backend = {
  getAppInfo: (): Promise<AppInfo> =>
    isDesktop ? call("get_app_info") : Promise.resolve(preview.appInfo()),

  getSystemSnapshot: (): Promise<SystemSnapshot> =>
    isDesktop ? call("get_system_snapshot") : Promise.resolve(preview.snapshot()),

  /** Samples newer than `sinceMs`, or the full backend history (up to 5 min) when omitted. */
  getMetrics: (sinceMs?: number): Promise<MetricsSample[]> =>
    isDesktop
      ? call("get_metrics", { sinceMs: sinceMs ?? null })
      : Promise.resolve(preview.metricsSince(sinceMs)),

  listTweaks: (): Promise<TweakMeta[]> => (isDesktop ? call("list_tweaks") : Promise.resolve([])),

  logEvent: (level: LogLevel, message: string, source?: string): Promise<void> =>
    isDesktop
      ? call("log_frontend_event", { level, message, source: source ?? null })
      : Promise.resolve(),
};
