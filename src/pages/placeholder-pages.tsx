import { PageHeader } from "@/components/layout/page-header";
import { PlannedFeatures } from "@/components/common/planned-features";
import { TweakRegistryCard } from "./tweaks/tweak-registry-card";

// Sections whose functionality is scheduled for later phases. They render real
// engine state where it exists and list planned features explicitly.

export function GamingPage() {
  return (
    <>
      <PageHeader title="Gaming" description="Presets, game profiles and latency settings" />
      <div className="flex flex-col gap-4">
        <TweakRegistryCard categories={["gaming"]} />
        <PlannedFeatures
          phase="Phase 3"
          features={[
            "Gaming presets built from individually reviewed tweaks",
            "Windows Game Mode, Game Bar and capture settings",
            "Hardware-accelerated GPU scheduling (where supported)",
            "Per-game profiles applied on launch, reverted on exit",
            "Background activity management while gaming",
            "Before/after frame-time measurement, not promised FPS",
          ]}
        />
      </div>
    </>
  );
}

export function SystemTweaksPage() {
  return (
    <>
      <PageHeader
        title="System Tweaks"
        description="Windows, power, network and privacy configuration"
      />
      <div className="flex flex-col gap-4">
        <TweakRegistryCard
          categories={["performance", "power", "network", "privacy", "services", "windows"]}
        />
        <PlannedFeatures
          phase="Phase 2"
          features={[
            "Power plan management using documented powercfg schemes",
            "Visual effects and performance options",
            "Privacy and telemetry settings that Microsoft documents",
            "Service start-type review with explanations (never bulk-disable)",
            "Dry-run preview of every registry and setting change",
            "Automatic backup and one-click revert for each tweak",
          ]}
        />
      </div>
    </>
  );
}

export function MonitoringPage() {
  return (
    <>
      <PageHeader title="Monitoring" description="Live and historical utilisation" />
      <PlannedFeatures
        phase="Phase 2"
        features={[
          "Per-core CPU usage and clock speeds",
          "GPU engine usage and VRAM over time",
          "Temperatures where sensors are exposed to Windows",
          "Disk read/write throughput and queue length",
          "Per-adapter network throughput",
          "Longer history persisted to the local database",
        ]}
      />
    </>
  );
}

export function CleanerPage() {
  return (
    <>
      <PageHeader title="PC Cleaner" description="Reclaim storage from safe locations" />
      <PlannedFeatures
        phase="Phase 4"
        features={[
          "Scan before clean: nothing is removed without review",
          "User and system temp folders (files not in use)",
          "Browser and shader caches, listed per application",
          "Recycle Bin size and emptying",
          "Storage breakdown by folder",
          "Fixed allowlist of locations: never arbitrary paths",
        ]}
      />
    </>
  );
}

export function StartupPage() {
  return (
    <>
      <PageHeader title="Startup" description="Control what runs when Windows starts" />
      <PlannedFeatures
        phase="Phase 4"
        features={[
          "Startup entries from Run keys, Startup folders and scheduled tasks",
          "Enable or disable using the same mechanism Task Manager uses",
          "Publisher, path and signature details for each entry",
          "Startup impact where Windows reports it",
        ]}
      />
    </>
  );
}

export function ToolsPage() {
  return (
    <>
      <PageHeader title="Tools" description="Diagnostics, DNS and Windows utilities" />
      <PlannedFeatures
        phase="Phase 5"
        features={[
          "Ping, traceroute and latency testing",
          "DNS resolver benchmark and flush",
          "Detailed system and hardware report export",
          "Shortcuts to built-in Windows utilities",
        ]}
      />
    </>
  );
}
