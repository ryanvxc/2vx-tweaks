import { PageHeader } from "@/components/layout/page-header";
import { EngineBanner } from "@/components/common/engine-banner";
import { PlannedFeatures } from "@/components/common/planned-features";
import { TweakList } from "./tweaks/tweak-list";

// Sections that are not fully built yet. They show real engine state where it exists
// and list what is still planned explicitly, without promising dates.

export function GamingPage() {
  return (
    <>
      <PageHeader title="Gaming" description="Presets, game profiles and latency settings" />
      <div className="flex flex-col gap-4">
        <EngineBanner />
        <TweakList categories={["gaming"]} />
        <PlannedFeatures
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
        <EngineBanner />
        <TweakList
          categories={["performance", "power", "network", "privacy", "services", "windows"]}
        />
        <PlannedFeatures
          features={[
            "Power plans through the Windows power API",
            "Visual effects and performance options",
            "More privacy settings that Microsoft documents",
            "Service start-type review with explanations (never bulk-disable)",
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
        features={[
          "Per-core CPU usage and clock speeds",
          "GPU engine usage and VRAM over time",
          "Temperatures where sensors are exposed to Windows",
          "Disk read/write throughput and queue length",
          "Per-adapter network throughput",
          "Longer history kept on this PC",
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
        features={[
          "Startup entries from Run keys, Startup folders and scheduled tasks",
          "Disable and re-enable entries with an undo, using a documented method",
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
