import { EngineBanner } from "@/components/common/engine-banner";
import { PageHeader } from "@/components/layout/page-header";
import { TweakGrid } from "./tweak-grid";

export function TweaksPage() {
  return (
    <>
      <PageHeader title="Tweaks" />
      <div className="flex flex-col gap-4">
        <EngineBanner />
        <TweakGrid
          browse
          categories={["performance", "privacy", "windows", "power", "network", "services"]}
        />
      </div>
    </>
  );
}
