import type { TweakStatus } from "@/types/engine";
import { profileProgress, type Profile, type ProfileProgress } from "./profiles";

/**
 * A preset is a named list of existing service tweaks, like a gaming profile: picking one opens
 * the normal review, whether it's active is read from detected state, and each tweak can be
 * undone by itself. Windows Update, Defender and other protected services can't be in a list:
 * the engine refuses those targets outright.
 */
export interface ServicePreset extends Profile {
  /** How much of Windows keeps working, out of five. A judgement, not a measurement. */
  functionality: number;
  /** Relative to the other preset. Nothing here is measured, so it stays a word. */
  background: string;
}

const BASIC = [
  "services.diagtrack-off",
  "services.wap-push-off",
  "services.sensor-off",
  "services.sensor-monitoring-off",
  "services.geolocation-off",
  "services.maps-broker-off",
  "services.sync-host-off",
  "services.compat-assistant-off",
];

const ADVANCED = [
  ...BASIC,
  "services.print-notify-off",
  "services.print-spooler-off",
  "services.bluetooth-off",
  "services.xbox-auth-off",
  "services.xbox-save-off",
];

export const SERVICE_PRESETS: ServicePreset[] = [
  {
    id: "basic",
    name: "Basic",
    summary:
      "Turns off telemetry, push routing, sensors, location, offline maps and app sync. Windows Update, Defender, networking, Bluetooth and printing keep working.",
    tweakIds: BASIC,
    functionality: 4,
    background: "Lower",
  },
  {
    id: "advanced",
    name: "Advanced",
    summary:
      "Basic, plus printing, Bluetooth and Xbox services. Printers, Bluetooth devices, Xbox sign-in and cloud saves stop working. Windows Update, Defender and networking keep working.",
    tweakIds: ADVANCED,
    functionality: 2,
    background: "Lowest",
  },
];

/** The fullest preset fully in effect: Advanced contains Basic, so only one reads as active. */
export function activePreset(statuses: TweakStatus[]): ServicePreset | null {
  return SERVICE_PRESETS.findLast((preset) => profileProgress(preset, statuses).active) ?? null;
}

/** The tweaks of a preset that 2VX changed and can put back. */
export const undoableIn = (progress: ProfileProgress): string[] =>
  progress.tweaks.filter((t) => t.undoable).map((t) => t.meta.id);
