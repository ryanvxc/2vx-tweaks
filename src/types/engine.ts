// Mirrors src-tauri/src/engine/mod.rs.

export type RiskLevel = "low" | "moderate" | "high";
export type TweakCategory =
  "gaming" | "performance" | "power" | "network" | "privacy" | "services" | "windows";
export type Permission = "user" | "administrator";
export type RestartRequirement = "none" | "signOut" | "reboot";

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
  recommended: boolean;
  restart: RestartRequirement;
  references: string[];
}
