import { Activity, EthernetPort, Router, Wifi, type LucideIcon } from "lucide-react";
import { ActionCard, InfoPanel } from "@/components/common/action-card";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { EngineBanner } from "@/components/common/engine-banner";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { useDeviceReport } from "@/hooks/use-system";
import type { AdapterKind, NetworkAdapter } from "@/types/system";
import { TweakGrid } from "../tweaks/tweak-grid";
import { NETWORK_VIEW } from "../tweaks/tweak-views";

export function NetworkPage() {
  return (
    <TweakGrid
      view={{
        ...NETWORK_VIEW,
        banner: <EngineBanner />,
        intro: (tab) =>
          tab === "priority" ? (
            <InfoPanel title="How it works">
              Each switch adds a Windows QoS rule that marks that game's traffic as high priority.
              Your router decides what to do with the mark, and many home routers ignore it.
            </InfoPanel>
          ) : null,
        body: (tab) =>
          tab === "adapters" ? <AdapterTab /> : tab === "bufferbloat" ? <BufferbloatTab /> : null,
      }}
    />
  );
}

const ADAPTERS: Record<AdapterKind, { icon: LucideIcon; title: string }> = {
  wifi: { icon: Wifi, title: "Wi-Fi adapter" },
  ethernet: { icon: EthernetPort, title: "Ethernet adapter" },
};

function AdapterTab() {
  const { data, error, loading, refetch } = useDeviceReport();
  return (
    <div className="flex flex-1 flex-col gap-3">
      <InfoPanel title="How it works">
        Adapters have their own driver settings, such as power saving. Each driver names them
        differently, so 2VX doesn't change them yet. These are the adapters in this PC.
      </InfoPanel>
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={3} />
      ) : (
        <div className="grid grid-cols-2 gap-3">
          {(["wifi", "ethernet"] as const).map((kind) => (
            <AdapterCard
              key={kind}
              kind={kind}
              adapter={data?.adapters.find((a) => a.kind === kind)}
            />
          ))}
        </div>
      )}
    </div>
  );
}

function AdapterCard({ kind, adapter }: { kind: AdapterKind; adapter?: NetworkAdapter }) {
  const { icon, title } = ADAPTERS[kind];
  return (
    <ActionCard
      icon={icon}
      title={title}
      status={<DataSourceBadge source="planned" />}
      description={
        adapter ? "Tuning for this adapter isn't built yet." : "This PC doesn't have one."
      }
      rows={[
        ["Adapter", adapter?.name ?? "None"],
        ["Driver", adapter?.driverVersion ?? "—"],
      ]}
      action={{ label: "Open adapter settings", to: "networkSettings" }}
    />
  );
}

function BufferbloatTab() {
  return (
    <div className="flex flex-1 flex-col gap-3">
      <InfoPanel
        title="What is bufferbloat?"
        link={{ label: "Bufferbloat test", to: "bufferbloatTest" }}
      >
        When your connection is busy, routers queue packets, and a long queue shows up as lag. The
        queue sits on your router, so a setting in Windows can't shorten it.
      </InfoPanel>
      <div className="grid grid-cols-2 gap-3">
        <ActionCard
          icon={Activity}
          title="Test your connection"
          description="Measures your latency while the line is idle and while it's full, then grades the difference."
          action={{ label: "Run the test", to: "bufferbloatTest" }}
        />
        <ActionCard
          icon={Router}
          title="Fix it on your router"
          description="Look for SQM or Smart Queue in your router's settings. It keeps the queue short by shaping traffic before it fills."
          action={{ label: "See what works", to: "bufferbloatFixes" }}
        />
      </div>
    </div>
  );
}
