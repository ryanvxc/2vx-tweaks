import { EngineBanner } from "@/components/common/engine-banner";
import { PresetsTab } from "./presets-tab";
import { TweakGrid } from "./tweak-grid";
import { TWEAKS_VIEW } from "./tweak-views";

export function TweaksPage() {
  return (
    <TweakGrid
      view={{
        ...TWEAKS_VIEW,
        banner: <EngineBanner />,
        body: (tab, context) => (tab === "presets" ? <PresetsTab {...context} /> : undefined),
      }}
    />
  );
}
