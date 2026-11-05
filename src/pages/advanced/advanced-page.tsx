import {
  BrickWallFire,
  KeyRound,
  ShieldCheck,
  ShieldHalf,
  UserRoundCheck,
  type LucideIcon,
} from "lucide-react";
import { InfoPanel } from "@/components/common/action-card";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { EngineBanner } from "@/components/common/engine-banner";
import { Notice, StatusLabel } from "@/components/common/notice";
import { OptimisationRow } from "@/components/common/optimisation-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { useDeviceReport } from "@/hooks/use-system";
import type { MsiGroup, ProtectionId, ProtectionState } from "@/types/system";
import { TweakGrid } from "../tweaks/tweak-grid";
import { ADVANCED_VIEW } from "../tweaks/tweak-views";
import { TimerTab } from "./timer-tab";

export function AdvancedPage() {
  return (
    <TweakGrid
      view={{
        ...ADVANCED_VIEW,
        banner: <EngineBanner />,
        intro: (tab) => (tab === "security" ? <Protections /> : null),
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
      <Notice tone="info" title="2VX never turns these off.">
        Disabling them leaves your PC open to attack, for no measured gain.
      </Notice>
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={5} />
      ) : (
        <div className="flex flex-col gap-2">
          {data?.protections.map(({ id, state }) => {
            const { icon: Icon, name, description } = PROTECTIONS[id];
            const { label, tone } = STATE[state];
            return (
              <OptimisationRow
                key={id}
                leading={<Icon className="size-[22px] text-brand" strokeWidth={1.75} aria-hidden />}
                title={name}
                description={description}
                control={<StatusLabel tone={tone}>{label}</StatusLabel>}
              />
            );
          })}
        </div>
      )}
    </>
  );
}

const MSI_GROUP: Record<MsiGroup, string> = {
  gpu: "GPU",
  audio: "Audio",
  network: "Network",
  nvme: "NVMe",
  sata: "SATA",
  usb: "USB",
};

function MsiTab() {
  const { data, error, loading, refetch } = useDeviceReport();
  return (
    <div className="flex flex-1 flex-col gap-3">
      <InfoPanel title="What is MSI mode?">
        Message-signaled interrupts let a device reach the CPU directly instead of sharing a line.
        The driver sets this when it installs. 2VX shows it but doesn't change it: forcing it on for
        a device that can't use it can stop Windows starting.
      </InfoPanel>
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={6} />
      ) : (
        <div className="overflow-hidden panel">
          <div className="flex h-11 items-center gap-4 border-b px-5 text-[13px] font-medium">
            <span className="w-20 shrink-0">Type</span>
            <span className="flex-1">Device</span>
            <DataSourceBadge source="live" />
            <span className="w-20 shrink-0 text-right whitespace-nowrap">MSI mode</span>
          </div>
          <ul className="divide-y">
            {data?.msi.map((d, i) => (
              <li
                key={`${d.name}-${i}`}
                className="flex h-12 items-center gap-4 px-5 text-[13.5px]"
              >
                <span className="w-20 shrink-0 text-muted-foreground">{MSI_GROUP[d.group]}</span>
                <span data-selectable className="min-w-0 flex-1 truncate font-medium">
                  {d.name}
                </span>
                <StatusLabel
                  className="w-20 shrink-0 justify-end"
                  tone={d.enabled ? "success" : "neutral"}
                >
                  {d.enabled ? "On" : "Off"}
                </StatusLabel>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
