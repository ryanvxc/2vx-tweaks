import { backend } from "@/services/backend";
import { invalidateQueries, useBackendQuery } from "./use-backend-query";

export const useSystemSnapshot = () =>
  useBackendQuery("system-snapshot", backend.getSystemSnapshot);
export const useDeviceReport = () => useBackendQuery("device-report", backend.getDeviceReport);
export const useHardwareDetails = () =>
  useBackendQuery("hardware-details", backend.getHardwareDetails);
export const useAppInfo = () => useBackendQuery("app-info", backend.getAppInfo);
export const useAccount = () => useBackendQuery("account", backend.getAccount);
export const useTweaks = () => useBackendQuery("tweaks", backend.listTweaks);
export const useRetiredTweaks = () => useBackendQuery("retired-tweaks", backend.listRetiredTweaks);
export const useEngineStatus = () => useBackendQuery("engine-status", backend.getEngineStatus);
export const useTweakStatuses = () => useBackendQuery("tweak-statuses", backend.getTweakStatuses);
export const useStartupItems = () => useBackendQuery("startup-items", backend.listStartupItems);
export const useCleanupScan = () => useBackendQuery("cleanup-scan", backend.scanCleanupLocations);
export const useRemovableApps = () => useBackendQuery("removable-apps", backend.listRemovableApps);
// The whole journal, not a recent page: Home counts it and Game Mode looks up an old batch in it.
export const useSnapshots = () => useBackendQuery("snapshots", () => backend.listSnapshots(5_000));

export const refreshAccount = () => invalidateQueries(["account"]);

/** Call after anything that may have changed the system or the journal. */
export const refreshEngineState = () =>
  invalidateQueries(["tweak-statuses", "snapshots", "engine-status", "retired-tweaks"]);
