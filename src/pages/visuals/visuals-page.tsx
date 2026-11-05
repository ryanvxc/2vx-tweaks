import { EngineBanner } from "@/components/common/engine-banner";
import { TweakGrid } from "../tweaks/tweak-grid";
import { VISUALS_VIEW } from "../tweaks/tweak-views";

export function VisualsPage() {
  return <TweakGrid view={{ ...VISUALS_VIEW, banner: <EngineBanner /> }} />;
}
