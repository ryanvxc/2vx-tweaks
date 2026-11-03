// Mirrors src-tauri/src/engine/{mod,setting,plan,service,journal}.rs.

export type RiskLevel = "low" | "moderate" | "high";
export type TweakCategory =
  "gaming" | "performance" | "power" | "network" | "privacy" | "services" | "windows";
export type Permission = "user" | "administrator";
export type RestartRequirement = "none" | "signOut" | "reboot";
export type DeviceScope = "any" | "desktopOnly" | "laptopOnly";

export interface BuildRange {
  minBuild: number;
  maxBuild: number | null;
}

export interface TweakMeta {
  id: string;
  name: string;
  description: string;
  details: string;
  category: TweakCategory;
  risk: RiskLevel;
  permission: Permission;
  supportedBuilds: BuildRange;
  device: DeviceScope;
  recommended: boolean;
  restart: RestartRequirement;
  references: string[];
}

/** `needsElevation` is informational: applying shows a UAC prompt, it is not blocked. */
export type Compatibility = "compatible" | "unsupportedBuild" | "wrongDevice" | "needsElevation";

export type Hive = "currentUser" | "localMachine";

export interface RegTarget {
  hive: Hive;
  key: string;
  /** Empty string means the key's default value. */
  value: string;
}

/** `null` wherever a `RegValue | null` appears means "the value does not exist". */
export type RegValue =
  | { type: "dword"; data: number }
  // u64 from Rust; values above 2^53 lose precision in JS. Display the backend's label instead.
  | { type: "qword"; data: number }
  | { type: "string"; data: string }
  | { type: "expandString"; data: string };

export type TweakState = "applied" | "notApplied" | "partial" | "unknown";

export interface SettingStatus {
  target: RegTarget;
  targetLabel: string;
  current: RegValue | null;
  currentLabel: string;
  desired: RegValue | null;
  desiredLabel: string;
  matches: boolean;
  error: string | null;
}

export interface TweakStatus {
  meta: TweakMeta;
  compatibility: Compatibility;
  state: TweakState;
  settings: SettingStatus[];
}

export interface PlannedChange {
  tweakId: string;
  target: RegTarget;
  targetLabel: string;
  from: RegValue | null;
  to: RegValue | null;
  fromLabel: string;
  toLabel: string;
}

export interface SkippedTweak {
  tweakId: string;
  reason: Compatibility;
}

/** Dry-run result. Single use; `applyPlan` refuses it after `expiresAtMs` or if a target moved. */
export interface Plan {
  id: string;
  createdAtMs: number;
  expiresAtMs: number;
  tweakIds: string[];
  changes: PlannedChange[];
  alreadyApplied: string[];
  skipped: SkippedTweak[];
  highestRisk: RiskLevel | null;
  restart: RestartRequirement;
  needsElevation: boolean;
}

export type SnapshotKind = "apply" | "revert";
export type SnapshotStatus =
  "inProgress" | "completed" | "rolledBack" | "failed" | "interrupted" | "reverted";
export type ChangeStatus =
  "intent" | "verified" | "rolledBack" | "reverted" | "failed" | "conflict" | "unchanged";

export interface SnapshotRecord {
  id: number;
  kind: SnapshotKind;
  tweakIds: string[];
  /** For a revert snapshot: the apply snapshot it undid. */
  reverts: number | null;
  status: SnapshotStatus;
  error: string | null;
  appVersion: string;
  createdAtMs: number;
  finishedAtMs: number | null;
  changeCount: number;
}

export interface ChangeRecord {
  id: number;
  snapshotId: number;
  seq: number;
  tweakId: string;
  target: RegTarget;
  before: RegValue | null;
  after: RegValue | null;
  status: ChangeStatus;
  error: string | null;
  updatedAtMs: number;
}

export interface OperationOutcome {
  /** `null` when there was nothing to do. */
  snapshotId: number | null;
  status: SnapshotStatus;
  error: string | null;
  changes: ChangeRecord[];
  restart: RestartRequirement;
}

export interface EngineStatus {
  tweakCount: number;
  /** Applying and reverting are disabled while the journal is unavailable. */
  journalAvailable: boolean;
  journalError: string | null;
  interruptedSnapshots: number;
}
