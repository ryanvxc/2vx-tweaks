import { EngineBanner } from "@/components/common/engine-banner";
import { PowerProfiles } from "../hardware/gpu-profiles";
import { PresetsTab } from "./presets-tab";
import { TweakGrid } from "./tweak-grid";
import { TWEAKS_VIEW } from "./tweak-views";

export function TweaksPage() {
  return (
    <TweakGrid
      view={{
        ...TWEAKS_VIEW,
        banner: <EngineBanner />,
        intro: (tab) => (tab === "power" ? <PowerProfiles /> : null),
        body: (tab, context) => (tab === "presets" ? <PresetsTab {...context} /> : undefined),
      }}
    />
  );
}
