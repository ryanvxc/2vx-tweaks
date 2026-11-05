import { EngineBanner } from "@/components/common/engine-banner";
import { TweakGrid } from "../tweaks/tweak-grid";
import { NETWORK_VIEW } from "../tweaks/tweak-views";
import { AdapterList } from "./adapter-list";
import { BufferbloatTab } from "./bufferbloat-tab";

export function NetworkPage() {
  return (
    <TweakGrid
      view={{
        ...NETWORK_VIEW,
        banner: <EngineBanner />,
        intro: (tab) => (tab === "adapters" ? <AdapterList /> : null),
        // The test has its own controls, so it replaces the search row and the tweaks.
        body: (tab) => (tab === "bufferbloat" ? <BufferbloatTab /> : null),
      }}
    />
  );
}
