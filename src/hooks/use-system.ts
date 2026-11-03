import { backend } from "@/services/backend";
import { invalidateQueries, useBackendQuery } from "./use-backend-query";

export const useSystemSnapshot = () =>
  useBackendQuery("system-snapshot", backend.getSystemSnapshot);
export const useAppInfo = () => useBackendQuery("app-info", backend.getAppInfo);
export const useTweaks = () => useBackendQuery("tweaks", backend.listTweaks);
export const useEngineStatus = () => useBackendQuery("engine-status", backend.getEngineStatus);
export const useTweakStatuses = () => useBackendQuery("tweak-statuses", backend.getTweakStatuses);
export const useSnapshots = () => useBackendQuery("snapshots", () => backend.listSnapshots(50));

/** Call after anything that may have changed the system or the journal. */
export const refreshEngineState = () =>
  invalidateQueries(["tweak-statuses", "snapshots", "engine-status"]);
