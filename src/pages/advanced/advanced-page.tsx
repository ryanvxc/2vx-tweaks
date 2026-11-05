import {
  BrickWallFire,
  KeyRound,
  ShieldCheck,
  ShieldHalf,
  UserRoundCheck,
  type LucideIcon,
} from "lucide-react";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { EngineBanner } from "@/components/common/engine-banner";
import { StatusLabel } from "@/components/common/notice";
import { OptimisationRow } from "@/components/common/optimisation-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { useDeviceReport } from "@/hooks/use-system";
import type { ProtectionId, ProtectionState } from "@/types/system";
import { TweakGrid } from "../tweaks/tweak-grid";
import { ADVANCED_VIEW } from "../tweaks/tweak-views";
import { DeviceList } from "./device-list";
import { MsiTab } from "./msi-tab";
import { TimerTab } from "./timer-tab";

export function AdvancedPage() {
  return (
    <TweakGrid
      view={{
        ...ADVANCED_VIEW,
        banner: <EngineBanner />,
        intro: (tab) =>
          tab === "security" ? <Protections /> : tab === "devices" ? <DeviceList /> : null,
        body: (tab) => (tab === "msi" ? <MsiTab /> : tab === "timer" ? <TimerTab /> : null),
      }}
    />
  );
}

const PROTECTIONS: Record<ProtectionId, { icon: LucideIcon; name: string; description: string }> = {
  memoryIntegrity: {
    icon: ShieldCheck,
    name: "Memory integrity",
    description: "Keeps malicious code out of the Windows kernel",
  },
  secureBoot: {
    icon: KeyRound,
    name: "Secure Boot",
    description: "Only trusted software can load while the PC starts",
  },
  smartScreen: {
    icon: ShieldHalf,
    name: "SmartScreen",
    description: "Checks apps and downloads for known threats",
  },
  uac: {
    icon: UserRoundCheck,
    name: "User Account Control",
    description: "Asks before an app gets administrator rights",
  },
  firewall: {
    icon: BrickWallFire,
    name: "Windows Firewall",
    description: "Blocks connections you didn't ask for",
  },
};

const STATE: Record<ProtectionState, { label: string; tone: "success" | "warning" | "neutral" }> = {
  on: { label: "On", tone: "success" },
  off: { label: "Off", tone: "warning" },
  unknown: { label: "Not reported", tone: "neutral" },
};

/** A read-out, on purpose: 2VX never turns a protection off (see docs/security.md). */
function Protections() {
  const { data, error, loading, refetch } = useDeviceReport();
  return (
    <>
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={5} />
      ) : (
        <section data-stagger className="flex flex-col divide-y overflow-hidden panel">
          <header className="flex h-11 shrink-0 items-center justify-between px-4">
            <h2 className="text-[14px] font-semibold">Protections</h2>
            <DataSourceBadge source="live" />
          </header>
          {data?.protections.map(({ id, state }) => {
            const { icon: Icon, name, description } = PROTECTIONS[id];
            const { label, tone } = STATE[state];
            return (
              <OptimisationRow
                key={id}
                flat
                leading={<Icon className="size-[18px] text-brand" strokeWidth={1.75} aria-hidden />}
                title={name}
                description={description}
                control={<StatusLabel tone={tone}>{label}</StatusLabel>}
              />
            );
          })}
        </section>
      )}
    </>
  );
}
