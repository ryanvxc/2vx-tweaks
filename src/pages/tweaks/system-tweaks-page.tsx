import { EngineBanner } from "@/components/common/engine-banner";
import { PageHeader } from "@/components/layout/page-header";
import { TweakGrid } from "./tweak-grid";

export function SystemTweaksPage() {
  return (
    <>
      <PageHeader
        title="System Tweaks"
        description="Documented Windows, privacy and browser settings. Switch one on to review it, off to undo it."
      />
      <div className="flex flex-col gap-4">
        <EngineBanner />
        <TweakGrid
          categories={["performance", "privacy", "windows", "power", "network", "services"]}
        />
      </div>
    </>
  );
}
