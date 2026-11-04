import { EngineBanner } from "@/components/common/engine-banner";
import { TweakGrid } from "./tweak-grid";

export function TweaksPage() {
  return (
    <TweakGrid
      browse
      banner={<EngineBanner />}
      categories={["gaming", "performance", "privacy", "windows", "power", "network", "services"]}
    />
  );
}
