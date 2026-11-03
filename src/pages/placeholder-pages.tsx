import { PageHeader } from "@/components/layout/page-header";
import { EngineBanner } from "@/components/common/engine-banner";
import { PlannedFeatures } from "@/components/common/planned-features";
import { TweakGrid } from "./tweaks/tweak-grid";

// Sections that are not fully built yet. They show real engine state where it exists
// and list what is still planned explicitly, without promising dates.

export function GamingPage() {
  return (
    <>
      <PageHeader
        title="Gaming"
        description="Gaming-related settings, added once each one is documented and measurable"
      />
      <div className="flex flex-col gap-4">
        <EngineBanner />
        <TweakGrid categories={["gaming"]} />
        <PlannedFeatures
          features={[
            "Gaming presets built from individually reviewed tweaks",
            "Windows Game Mode, Game Bar and capture settings",
            "Hardware-accelerated GPU scheduling (where supported)",
            "Per-game profiles applied on launch, reverted on exit",
            "Before/after frame-time measurement, not promised FPS",
          ]}
        />
      </div>
    </>
  );
}
