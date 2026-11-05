import { EngineBanner } from "@/components/common/engine-banner";
import { TweakGrid } from "./tweak-grid";
import { TWEAKS_VIEW } from "./tweak-views";

export function TweaksPage() {
  return <TweakGrid view={{ ...TWEAKS_VIEW, banner: <EngineBanner /> }} />;
}
