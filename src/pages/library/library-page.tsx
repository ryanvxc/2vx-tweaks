import { EngineBanner } from "@/components/common/engine-banner";
import { TweakGrid } from "../tweaks/tweak-grid";
import { LIBRARY_VIEW } from "../tweaks/tweak-views";
import { LibraryPresets } from "./library-presets";

export function LibraryPage() {
  return (
    <TweakGrid
      view={{
        ...LIBRARY_VIEW,
        banner: <EngineBanner />,
        body: (tab, context) => (tab === "presets" ? <LibraryPresets {...context} /> : undefined),
      }}
    />
  );
}
