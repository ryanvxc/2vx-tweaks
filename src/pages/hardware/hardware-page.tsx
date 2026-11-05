import { MonitorCog, Gpu } from "lucide-react";
import { ActionCard } from "@/components/common/action-card";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { EngineBanner } from "@/components/common/engine-banner";
import { useDeviceReport, useSystemSnapshot } from "@/hooks/use-system";
import { driverLink, driverOf, primaryGpu } from "@/lib/devices";
import type { DeviceReport, SystemSnapshot } from "@/types/system";
import { TweakGrid } from "../tweaks/tweak-grid";
import { HARDWARE_VIEW } from "../tweaks/tweak-views";
import { DeviceChip } from "./device-chip";

export function HardwarePage() {
  const { data: snapshot } = useSystemSnapshot();
  const { data: devices } = useDeviceReport();
  return (
    <TweakGrid
      view={{
        ...HARDWARE_VIEW,
        banner: <EngineBanner />,
        end: (tab) => (
          <>
            <DeviceChip tab={tab} snapshot={snapshot} devices={devices} />
            <DataSourceBadge source="live" />
          </>
        ),
        intro: (tab) =>
          tab === "gpu" ? <GpuPanels snapshot={snapshot} devices={devices} /> : null,
      }}
    />
  );
}

/** Where EXM offers NVIDIA profiles: 2VX can't start other programs, so it points to the real ones. */
function GpuPanels({
  snapshot,
  devices,
}: {
  snapshot: SystemSnapshot | undefined;
  devices: DeviceReport | undefined;
}) {
  const gpu = primaryGpu(snapshot);
  const driver = driverOf(gpu, devices);
  const link = driverLink(gpu);
  return (
    <div className="grid grid-cols-2 gap-3">
      <ActionCard
        icon={Gpu}
        title="Graphics driver"
        description="Drivers come from your GPU's maker. A clean, current one does more than any tweak here."
        rows={[
          ["GPU", gpu?.model ?? "Not reported"],
          ["Driver", driver?.version ?? "Not reported"],
        ]}
        action={link ? { label: "Get the latest driver", to: link } : undefined}
      />
      <ActionCard
        icon={MonitorCog}
        title="Windows graphics settings"
        description="Hardware-accelerated GPU scheduling and which GPU each game uses are set here, in Windows."
        action={{ label: "Open graphics settings", to: "graphicsSettings" }}
      />
    </div>
  );
}
