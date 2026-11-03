import type {
  ChangeRecord,
  EngineStatus,
  OperationOutcome,
  Plan,
  SnapshotRecord,
  TweakMeta,
  TweakStatus,
} from "@/types/engine";

// Engine responses shaped like the Rust serde output, for component tests that mock
// `services/backend`.

const base: Omit<TweakMeta, "id" | "name" | "permission"> = {
  description: "Test description.",
  details: "Test details.",
  category: "privacy",
  risk: "low",
  supportedBuilds: { minBuild: 10240, maxBuild: null },
  device: "any",
  recommended: false,
  restart: "none",
  references: ["https://learn.microsoft.com/test"],
};

export const USER_TWEAK: TweakMeta = {
  ...base,
  id: "privacy.user-tweak",
  name: "User tweak",
  permission: "user",
};
export const ADMIN_TWEAK: TweakMeta = {
  ...base,
  id: "performance.admin-tweak",
  name: "Admin tweak",
  category: "performance",
  permission: "administrator",
};
export const APPLIED_TWEAK: TweakMeta = {
  ...base,
  id: "privacy.applied-tweak",
  name: "Applied tweak",
  permission: "user",
};

const USER_LABEL = String.raw`HKCU\Software\Test\User`;
const ADMIN_LABEL = String.raw`HKLM\SOFTWARE\Test\Admin`;

function status(meta: TweakMeta, label: string, applied: boolean): TweakStatus {
  return {
    meta,
    compatibility: meta.permission === "administrator" ? "needsElevation" : "compatible",
    state: applied ? "applied" : "notApplied",
    settings: [
      {
        target: {
          hive: meta.permission === "administrator" ? "localMachine" : "currentUser",
          key: "Software\\Test",
          value: meta.name,
        },
        targetLabel: label,
        current: applied ? { type: "dword", data: 0 } : null,
        currentLabel: applied ? "0 (REG_DWORD)" : "(not set)",
        desired: { type: "dword", data: 0 },
        desiredLabel: "0 (REG_DWORD)",
        matches: applied,
        error: null,
      },
    ],
  };
}

export const STATUSES: TweakStatus[] = [
  status(USER_TWEAK, USER_LABEL, false),
  status(ADMIN_TWEAK, ADMIN_LABEL, false),
  status(APPLIED_TWEAK, String.raw`HKCU\Software\Test\Applied`, true),
];

export const HEALTHY: EngineStatus = {
  tweakCount: 3,
  journalAvailable: true,
  journalError: null,
  interruptedSnapshots: 0,
};

export function plan(ids: string[]): Plan {
  const changes = ids.map((id) => {
    const admin = id === ADMIN_TWEAK.id;
    return {
      tweakId: id,
      target: {
        hive: admin ? ("localMachine" as const) : ("currentUser" as const),
        key: "Software\\Test",
        value: id,
      },
      targetLabel: admin ? ADMIN_LABEL : USER_LABEL,
      from: null,
      to: { type: "dword" as const, data: 0 },
      fromLabel: "(not set)",
      toLabel: "0 (REG_DWORD)",
    };
  });
  return {
    id: "a".repeat(32),
    createdAtMs: 0,
    expiresAtMs: 300_000,
    tweakIds: ids,
    changes,
    alreadyApplied: [],
    skipped: [],
    highestRisk: "low",
    restart: "none",
    needsElevation: ids.includes(ADMIN_TWEAK.id),
  };
}

export function change(overrides: Partial<ChangeRecord> = {}): ChangeRecord {
  return {
    id: 1,
    snapshotId: 7,
    seq: 0,
    tweakId: USER_TWEAK.id,
    target: { hive: "currentUser", key: "Software\\Test", value: "User" },
    targetLabel: USER_LABEL,
    before: null,
    beforeLabel: "(not set)",
    after: { type: "dword", data: 0 },
    afterLabel: "0 (REG_DWORD)",
    status: "verified",
    error: null,
    updatedAtMs: 0,
    ...overrides,
  };
}

export function outcome(overrides: Partial<OperationOutcome> = {}): OperationOutcome {
  return {
    snapshotId: 7,
    status: "completed",
    error: null,
    changes: [change()],
    restart: "none",
    ...overrides,
  };
}

export const SNAPSHOT: SnapshotRecord = {
  id: 7,
  kind: "apply",
  tweakIds: [USER_TWEAK.id],
  reverts: null,
  status: "completed",
  error: null,
  appVersion: "0.1.0",
  createdAtMs: Date.UTC(2026, 10, 3, 12, 0),
  finishedAtMs: Date.UTC(2026, 10, 3, 12, 0),
  changeCount: 1,
};
