import { EngineBanner } from "@/components/common/engine-banner";
import { useDeviceReport, useSystemSnapshot } from "@/hooks/use-system";
import { TweakGrid } from "../tweaks/tweak-grid";
import { HARDWARE_VIEW } from "../tweaks/tweak-views";
import { DeviceChip } from "./device-chip";
import { GpuProfiles } from "./gpu-profiles";
import { HardwareFacts } from "./hardware-facts";

export function HardwarePage() {
  const { data: snapshot } = useSystemSnapshot();
  const { data: devices } = useDeviceReport();
  return (
    <TweakGrid
      view={{
        ...HARDWARE_VIEW,
        banner: <EngineBanner />,
        end: (tab) => <DeviceChip tab={tab} snapshot={snapshot} devices={devices} />,
        intro: (tab) => (
          <>
            <HardwareFacts tab={tab} />
            {tab === "gpu" && <GpuProfiles />}
          </>
        ),
      }}
    />
  );
}
