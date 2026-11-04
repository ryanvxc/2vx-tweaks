// Mirrors the serde types in src-tauri/src/system and src-tauri/src/commands/app.rs.
// Keep field names in sync (Rust uses `rename_all = "camelCase"`).

export interface OsInfo {
  productName: string;
  displayVersion: string | null;
  build: number;
  ubr: number | null;
  editionId: string | null;
  isWindows11: boolean;
  architecture: string;
}

export interface CpuInfo {
  brand: string;
  vendor: string;
  physicalCores: number | null;
  logicalCores: number;
  frequencyMhz: number;
}

export interface MemoryInfo {
  totalBytes: number;
  swapTotalBytes: number;
}

export interface GpuInfo {
  vendor: string | null;
  model: string | null;
  dedicatedMemoryBytes: number | null;
}

export type StorageKind = "ssd" | "hdd" | "unknown";

export interface DiskInfo {
  name: string;
  mountPoint: string;
  fileSystem: string;
  kind: StorageKind;
  totalBytes: number;
  availableBytes: number;
  isRemovable: boolean;
}

export interface MotherboardInfo {
  vendor: string | null;
  name: string | null;
}

export interface SystemSnapshot {
  hostname: string | null;
  uptimeSecs: number | null;
  os: OsInfo;
  cpu: CpuInfo;
  memory: MemoryInfo;
  gpus: GpuInfo[];
  disks: DiskInfo[];
  motherboard: MotherboardInfo | null;
}

export interface GpuSample {
  model: string | null;
  usagePercent: number | null;
  // Used VRAM is deliberately absent; see GpuSample in src-tauri/src/system/metrics.rs.
  memoryTotalBytes: number | null;
}

export interface MetricsSample {
  timestampMs: number;
  cpuPercent: number;
  perCorePercent: number[];
  memoryUsedBytes: number;
  memoryTotalBytes: number;
  gpus: GpuSample[];
  networkRxBytesPerSec: number;
  networkTxBytesPerSec: number;
}

// Mirrors src-tauri/src/system/processes.rs.
export interface RunningProgram {
  fileName: string;
  runningSeconds: number;
  /** The game's folder, when it runs from a game library. */
  libraryGame: string | null;
}

// Mirrors src-tauri/src/account.rs.
export interface Account {
  email: string;
}

export interface AppInfo {
  name: string;
  version: string;
  isElevated: boolean;
  logDir: string;
  debugBuild: boolean;
}

// Mirrors src-tauri/src/engine/startup.rs.
export type StartupSource = "runKey" | "runOnceKey" | "startupFolder";
export type StartupScope = "currentUser" | "allUsers";
export type StartupOrigin = "microsoft" | "thirdParty" | "unknown";

export interface StartupItem {
  id: string;
  name: string;
  command: string;
  path: string;
  source: StartupSource;
  scope: StartupScope;
  location: string;
  /** `false` when it won't run: switched off here or in Task Manager. */
  enabled: boolean | null;
  offInTaskManager: boolean;
  changeable: boolean;
  /** Who the program says made it. Self-reported, so a label and never a safety verdict. */
  origin: StartupOrigin;
  description: string | null;
  publisher: string | null;
}

export interface StartupInventory {
  items: StartupItem[];
  unreadable: string[];
}

// Mirrors src-tauri/src/engine/cleanup.rs.
export type ScanStatus = "measured" | "partial" | "missing" | "denied";

export interface CleanupLocation {
  id: string;
  label: string;
  description: string;
  path: string;
  bytes: number;
  files: number;
  status: ScanStatus;
}

export interface CleanupResult {
  id: string;
  freedBytes: number;
  removedFiles: number;
  /** Files left behind, nearly always because something has them open. */
  skippedFiles: number;
  error: string | null;
}

// Mirrors src-tauri/src/engine/apps.rs.
export interface RemovableApp {
  id: string;
  name: string;
  description: string;
  installed: boolean;
  /** The logo from the installed package, as a data URL. */
  icon: string | null;
}

/** Mirrors `UpdateSupport` in src-tauri/src/commands/app.rs. */
export interface UpdateSupport {
  available: boolean;
  reason: string | null;
  endpoint: string | null;
  channel: string;
}

/** Serialised `AppError` from the backend. */
export interface BackendError {
  code:
    | "invalid_input"
    | "unsupported"
    | "elevation_required"
    | "system"
    | "io"
    | "internal"
    | "storage"
    | "conflict"
    | "policy"
    | "preview";
  message: string;
}
