import { Channel, invoke, isTauri } from "@tauri-apps/api/core";
import type {
  ChangeRecord,
  EngineStatus,
  OperationOutcome,
  Plan,
  RetiredTweak,
  SnapshotRecord,
  TweakMeta,
  TweakStatus,
} from "@/types/engine";
import type {
  Account,
  AppInfo,
  AutoLaunch,
  BackendError,
  MetricsSample,
  CleanupLocation,
  CleanupProgress,
  CleanupResult,
  DeviceReport,
  ExternalLink,
  HardwareDetails,
  PingResult,
  PingTarget,
  RemovableApp,
  ServiceState,
  SleepSample,
  StartupInventory,
  SystemSnapshot,
  UpdateSupport,
  RunningProgram,
  WindowsTool,
} from "@/types/system";
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

  /** Read-only, for Game Mode. Takes no process names: the game list stays in the UI. */
  getRunningPrograms: (): Promise<RunningProgram[]> =>
    isDesktop ? call("get_running_programs") : Promise.resolve(preview.runningPrograms()),

  /** The local account, if there is one. It stays on this PC. */
  getAccount: (): Promise<Account | null> =>
    isDesktop ? call("get_account") : Promise.resolve(preview.account()),

  setAccount: (email: string): Promise<Account> =>
    isDesktop ? call("set_account", { email }) : preview.setAccount(email),

  clearAccount: (): Promise<void> =>
    isDesktop ? call("clear_account") : Promise.resolve(preview.clearAccount()),

  /** Desktop only; `services/updater` serves the preview answer. */
  getUpdateSupport: (): Promise<UpdateSupport> => call("get_update_support"),

  getSystemSnapshot: (): Promise<SystemSnapshot> =>
    isDesktop ? call("get_system_snapshot") : Promise.resolve(preview.snapshot()),

  /** Drivers, adapters, interrupt modes, protections and the timer. Read-only. */
  getDeviceReport: (): Promise<DeviceReport> =>
    isDesktop ? call("get_device_report") : Promise.resolve(preview.deviceReport()),

  /** CPU layout, memory modules, drives, GPU drivers and devices. Read-only, needs no prompt. */
  getHardwareDetails: (): Promise<HardwareDetails> =>
    isDesktop ? call("get_hardware_details") : Promise.resolve(preview.hardwareDetails()),

  /** How long a one-millisecond sleep really takes. Only sleeps. */
  measureSleep: (): Promise<SleepSample[]> =>
    isDesktop ? call("measure_sleep") : Promise.resolve(preview.sleepSamples()),

  /** State of the services the catalogue's tweaks name. Read-only. */
  getServiceStates: (): Promise<ServiceState[]> =>
    isDesktop ? call("get_service_states") : Promise.resolve(preview.serviceStates()),

  /** One ICMP echo with a 1 s timeout. Only ever started by the user. */
  pingTarget: (target: PingTarget): Promise<PingResult> =>
    isDesktop ? call("ping_target", { target }) : Promise.resolve(preview.ping(target)),

  /** Opens a page from the backend's fixed list. Takes an entry name, never an address. */
  openLink: (link: ExternalLink): Promise<void> =>
    isDesktop ? call("open_link", { link }) : Promise.resolve(),

  /** Opens a Windows tool from the backend's fixed list. Takes a name, never a command. */
  openTool: (tool: WindowsTool): Promise<void> =>
    isDesktop ? call("open_tool", { tool }) : Promise.resolve(),

  openLogFolder: (): Promise<void> => (isDesktop ? call("open_log_folder") : Promise.resolve()),

  /** Shows the save dialog. Resolves to the file's name, or `null` if the user cancels. */
  exportReport: (text: string): Promise<string | null> =>
    isDesktop ? call("export_report", { text }) : preview.refuse(),

  /** Whether 2VX opens at sign-in. Off unless the user turned it on. */
  getAutoLaunch: (): Promise<AutoLaunch> =>
    isDesktop ? call("get_auto_launch") : Promise.resolve("off"),

  setAutoLaunch: (on: boolean): Promise<void> =>
    isDesktop ? call("set_auto_launch", { enabled: on }) : preview.refuse(),

  /** Samples newer than `sinceMs`, or the full backend history (up to 5 min) when omitted. */
  getMetrics: (sinceMs?: number): Promise<MetricsSample[]> =>
    isDesktop
      ? call("get_metrics", { sinceMs: sinceMs ?? null })
      : Promise.resolve(preview.metricsSince(sinceMs)),

  /** Run keys and Startup folders. Read-only. */
  listStartupItems: (): Promise<StartupInventory> =>
    isDesktop ? call("list_startup_items") : Promise.resolve(preview.startup()),

  /** Sizes of the fixed cleanup allowlist. Read-only; can take a few seconds. */
  scanCleanupLocations: (): Promise<CleanupLocation[]> =>
    isDesktop ? call("scan_cleanup_locations") : preview.cleanup(),

  /**
   * Clears allowlisted locations. Can't be undone, and may show one UAC prompt. Refusing it doesn't
   * reject: those locations come back with status `elevationDeclined`.
   */
  runCleanup: (
    ids: string[],
    onProgress?: (progress: CleanupProgress) => void,
  ): Promise<CleanupResult[]> => {
    if (!isDesktop) return preview.refuse();
    const channel = new Channel<CleanupProgress>();
    if (onProgress) channel.onmessage = onProgress;
    return call("clean_locations", { ids, onProgress: channel });
  },

  /** Switches a startup entry by the ID a scan gave it. May show one UAC prompt. */
  setStartupEnabled: (id: string, enabled: boolean): Promise<void> =>
    isDesktop ? call("set_startup_enabled", { id, enabled }) : preview.refuse(),

  listRemovableApps: (): Promise<RemovableApp[]> =>
    isDesktop ? call("list_removable_apps") : Promise.resolve(preview.apps()),

  uninstallApp: (appId: string): Promise<void> =>
    isDesktop ? call("uninstall_app", { appId }) : preview.refuse(),

  reinstallApp: (appId: string): Promise<void> =>
    isDesktop ? call("reinstall_app", { appId }) : preview.refuse(),

  listTweaks: (): Promise<TweakMeta[]> =>
    isDesktop ? call("list_tweaks") : Promise.resolve(preview.tweaks()),

  /** Tweaks 2VX no longer offers: names for old history, and what can still be undone. */
  listRetiredTweaks: (): Promise<RetiredTweak[]> =>
    isDesktop ? call("list_retired_tweaks") : Promise.resolve([]),

  getEngineStatus: (): Promise<EngineStatus> =>
    isDesktop ? call("get_engine_status") : Promise.resolve(preview.engineStatus()),

  /** Reads every target of every tweak; can take a moment, runs on a backend worker thread. */
  getTweakStatuses: (): Promise<TweakStatus[]> =>
    isDesktop ? call("get_tweak_statuses") : Promise.resolve(preview.tweakStatuses()),

  /** Dry run: writes nothing. Unknown IDs are rejected with `invalid_input`. */
  planTweaks: (tweakIds: string[]): Promise<Plan> =>
    isDesktop ? call("plan_tweaks", { tweakIds }) : preview.refuse(),

  /** Consumes the plan. Fails with `conflict` if it expired or a target changed since planning. */
  applyPlan: (planId: string): Promise<OperationOutcome> =>
    isDesktop ? call("apply_plan", { planId }) : preview.refuse(),

  revertSnapshot: (snapshotId: number): Promise<OperationOutcome> =>
    isDesktop ? call("revert_snapshot", { snapshotId }) : preview.refuse(),

  /** Undoes 2VX's recorded changes for one tweak, whichever batches made them. */
  revertTweak: (tweakId: string): Promise<OperationOutcome> =>
    isDesktop ? call("revert_tweak", { tweakId }) : preview.refuse(),

  /** One batch, so one prompt covers them all. */
  revertTweaks: (tweakIds: string[]): Promise<OperationOutcome> =>
    isDesktop ? call("revert_tweaks", { tweakIds }) : preview.refuse(),

  /** Newest first; the backend defaults to 50 and returns at most 5,000. */
  listSnapshots: (limit?: number): Promise<SnapshotRecord[]> =>
    isDesktop ? call("list_snapshots", { limit: limit ?? null }) : Promise.resolve([]),

  getSnapshotChanges: (snapshotId: number): Promise<ChangeRecord[]> =>
    isDesktop ? call("get_snapshot_changes", { snapshotId }) : Promise.resolve([]),

  logEvent: (level: LogLevel, message: string, source?: string): Promise<void> =>
    isDesktop
      ? call("log_frontend_event", { level, message, source: source ?? null })
      : Promise.resolve(),
};
