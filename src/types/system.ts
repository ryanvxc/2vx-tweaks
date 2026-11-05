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
/** The app's own sign-in entry. `offInTaskManager` can only be undone in Task Manager. */
export type AutoLaunch = "off" | "on" | "offInTaskManager";

export type StartupSource = "runKey" | "runOnceKey" | "startupFolder" | "task";
export type StartupScope = "currentUser" | "allUsers";
export type StartupOrigin = "microsoft" | "thirdParty" | "unknown";

export interface StartupItem {
  id: string;
  name: string;
  /** What Windows runs. For a shortcut, the `.lnk` itself. */
  command: string;
  /** The program. For a shortcut, what it points at. */
  path: string;
  source: StartupSource;
  /** `source` in words: "Run key", "RunOnce key", "Startup folder" or "Task". */
  sourceLabel: string;
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

/** `elevationDeclined` is the UAC prompt being refused: nothing ran, and it isn't a 0-byte success. */
export type CleanupStatus = "cleared" | "elevationDeclined" | "failed";

export interface CleanupResult {
  id: string;
  freedBytes: number;
  removedFiles: number;
  /** Files left behind, nearly always because something has them open. */
  skippedFiles: number;
  status: CleanupStatus;
  error: string | null;
}

export type CleanupPhase = "started" | "clearing" | "finished" | "declined" | "failed";

/** One step of `runCleanup`. A location goes `started`, any number of `clearing`, then one more. */
export interface CleanupProgress {
  id: string;
  phase: CleanupPhase;
  /** Freed from this location so far. */
  freedBytes: number;
  /** Freed across the whole run so far. */
  totalFreedBytes: number;
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

export interface GpuDriver {
  name: string;
  version: string | null;
  /** `YYYY-MM-DD`. */
  date: string | null;
  /** Who published the driver, as it says of itself. */
  provider: string | null;
  /** Windows' fallback driver. Says nothing about any other driver: 2VX never claims "outdated". */
  isBasicDriver: boolean;
}

export type AdapterKind = "ethernet" | "wifi";

export interface NetworkAdapter {
  name: string;
  kind: AdapterKind;
  driverVersion: string | null;
  /** `null` when Windows has no interface for it or doesn't know its media state. */
  connected: boolean | null;
  /** The faster direction, while connected. */
  linkSpeedBps: number | null;
}

export type MsiGroup = "gpu" | "audio" | "network" | "nvme" | "sata" | "usb";

export interface InterruptSupport {
  lineBased: boolean;
  msi: boolean;
  msiX: boolean;
}

export interface MsiDevice {
  group: MsiGroup;
  name: string;
  /** What it's configured for. */
  enabled: boolean;
  /** What it can do. `null` when Windows doesn't report it. */
  interruptSupport: InterruptSupport | null;
}

export type ProtectionId = "memoryIntegrity" | "secureBoot" | "smartScreen" | "uac" | "firewall";
export type ProtectionState = "on" | "off" | "unknown";

export interface Protection {
  id: ProtectionId;
  state: ProtectionState;
}

export interface TimerResolution {
  currentMs: number;
  coarsestMs: number;
  finestMs: number;
}

export interface SleepSample {
  resolutionMs: number;
  sleptMs: number;
  deltaMs: number;
}

export interface DeviceReport {
  gpus: GpuDriver[];
  drives: string[];
  adapters: NetworkAdapter[];
  msi: MsiDevice[];
  protections: Protection[];
  timer: TimerResolution | null;
}

// Mirrors src-tauri/src/system/hardware_details.rs. Every `null` means Windows didn't report it.
export interface CpuDetails {
  performanceCores: number | null;
  /** `0` on a CPU with one kind of core. */
  efficientCores: number | null;
  logicalProcessors: number | null;
  l2CacheBytes: number | null;
  l3CacheBytes: number | null;
}

export interface MemoryModule {
  /** The board's label, such as `DIMM_A2`. */
  slot: string | null;
  capacityBytes: number | null;
  /** The speed it's running at, falling back to its rated speed. */
  speedMts: number | null;
  ratedSpeedMts: number | null;
  /** `DDR4`, `DDR5`, `LPDDR5`… */
  memoryType: string | null;
  manufacturer: string | null;
  partNumber: string | null;
}

export interface MemoryDetails {
  slotsUsed: number | null;
  slotsTotal: number | null;
  modules: MemoryModule[];
  /** Inferred from Windows' "Memory Compression" process. */
  compressionActive: boolean | null;
}

export interface DriveDetails {
  model: string | null;
  /** `NVMe`, `SATA`, `USB`, `SAS`, `SCSI`… */
  bus: string | null;
  media: StorageKind;
  sizeBytes: number | null;
  trimSupported: boolean | null;
  /** `Healthy`, `Warning` or `Unhealthy`, as Windows reports it. */
  health: string | null;
}

export type DeviceClass =
  "display" | "net" | "media" | "usb" | "bluetooth" | "mouse" | "keyboard" | "diskDrive";

export interface DeviceEntry {
  name: string;
  class: DeviceClass;
  /** Windows reports a problem. A device the user turned off isn't one. */
  problem: boolean;
}

export interface HardwareDetails {
  cpu: CpuDetails;
  memory: MemoryDetails;
  drives: DriveDetails[];
  /** Whether Windows sends TRIM, which is system-wide rather than per drive. */
  trimEnabled: boolean | null;
  gpuDrivers: GpuDriver[];
  devices: DeviceEntry[];
}

// Mirrors src-tauri/src/system/services.rs.
export type ServiceStartType = "automatic" | "automaticDelayed" | "manual" | "disabled" | "unknown";

export interface ServiceState {
  name: string;
  displayName: string;
  description: string | null;
  /** `null` when the Service Control Manager can't be asked. */
  running: boolean | null;
  startType: ServiceStartType;
}

// Mirrors src-tauri/src/system/ping.rs.
export type PingTarget = "gateway" | "internet";

export interface PingResult {
  /** Empty when there's no default gateway to ask. */
  address: string;
  /** `null` when no reply came within a second. */
  rttMs: number | null;
}

/** The Windows tools the app can open. Names, never commands. */
export type WindowsTool =
  | "resourceMonitor"
  | "reliabilityMonitor"
  | "systemInformation"
  | "directxDiagnostic"
  | "diskCleanup"
  | "eventViewer"
  | "taskManager"
  | "deviceManager"
  | "servicesConsole"
  | "networkConnections"
  | "systemRestore"
  | "systemProtection"
  | "optimizeDrives";

/** The backend's fixed list of pages. Names, never addresses. */
export type ExternalLink =
  | "nvidiaDrivers"
  | "amdDrivers"
  | "intelDrivers"
  | "bufferbloatTest"
  | "bufferbloatFixes"
  | "graphicsSettings"
  | "networkSettings"
  | "timerDocs"
  | "reportIssue"
  | "projectPage"
  | "troubleshoot"
  | "secureBootDocs"
  | "tpmDocs"
  | "memoryIntegrityDocs"
  | "resizableBarNvidia"
  | "resizableBarAmd"
  | "expoDocs"
  | "xmpDocs"
  | "intelInstabilityDocs"
  | "autoruns"
  | "processExplorer"
  | "hwinfo"
  | "latencyMon"
  | "capFrameX"
  | "crystalDiskInfo"
  | "occt"
  | "displayDriverUninstaller"
  | "nvCleanstall"
  | "nvidiaProfileInspector";

// Mirrors src-tauri/src/engine/restore_points.rs.
export type ProtectionStatus = "on" | "off" | "blocked" | "unknown";

export interface RestorePoint {
  sequence: number;
  description: string;
  createdMs: number | null;
}

export interface RestoreOverview {
  protection: ProtectionStatus;
  /** Newest first. `null` when Windows wouldn't list them, which isn't the same as having none. */
  points: RestorePoint[] | null;
}

/** `false` when Windows skipped it, because one was made recently. */
export interface CreatedPoint {
  created: boolean;
}

// Mirrors src-tauri/src/engine/drives.rs.
export interface DriveOutcome {
  drive: string;
  optimized: boolean;
  problem: string | null;
}

// Mirrors src-tauri/src/engine/repair.rs.
export type RepairTool = "systemFiles" | "windowsImage";

export interface RepairOutcome {
  tool: RepairTool;
  succeeded: boolean;
  output: string;
}

// Mirrors src-tauri/src/system/firmware.rs.
export type BootMode = "uefi" | "legacy" | "unknown";

export interface FirmwareReport {
  bootMode: BootMode;
  vendor: string | null;
  version: string | null;
  releaseDate: string | null;
  board: string | null;
  virtualization: boolean | null;
  /** 2 for TPM 2.0, 1 for 1.2. `null` when no TPM answers. */
  tpmVersion: number | null;
  /** `null` when no adapter's memory windows could be read. */
  resizableBar: boolean | null;
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
