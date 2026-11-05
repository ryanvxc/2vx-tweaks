import { EngineBanner } from "@/components/common/engine-banner";
import { TweakGrid } from "../tweaks/tweak-grid";
import { CLEANER_VIEW } from "../tweaks/tweak-views";
import { AutorunsTab } from "./autoruns-tab";
import { ServicePresets, ServiceReadout } from "./services-tab";
import { SystemCleanerTab } from "./system-cleaner-tab";
import { UninstallTab } from "./uninstall-tab";

export function CleanerPage() {
  return (
    <TweakGrid
      view={{
        ...CLEANER_VIEW,
        banner: <EngineBanner />,
        intro: (tab) =>
          tab === "services" ? (
            <div className="flex flex-col gap-3">
              <ServicePresets />
              <ServiceReadout />
            </div>
          ) : null,
        body: (tab) =>
          tab === "system" ? (
            <SystemCleanerTab />
          ) : tab === "uninstall" ? (
            <UninstallTab />
          ) : tab === "autoruns" ? (
            <AutorunsTab />
          ) : null,
      }}
    />
  );
}
