import { backend } from "@/services/backend";
import { useBackendQuery } from "./use-backend-query";

export const useSystemSnapshot = () =>
  useBackendQuery("system-snapshot", backend.getSystemSnapshot);
export const useAppInfo = () => useBackendQuery("app-info", backend.getAppInfo);
export const useTweaks = () => useBackendQuery("tweaks", backend.listTweaks);
