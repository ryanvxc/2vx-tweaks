import {
  Bluetooth,
  ExternalLink as ExternalLinkIcon,
  HardDrive,
  Keyboard,
  Monitor,
  Mouse,
  Network,
  Usb,
  Volume2,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useHardwareDetails } from "@/hooks/use-system";
import { collapseDevices, deviceName } from "@/lib/devices";
import { backend, toBackendError } from "@/services/backend";
import type { DeviceClass } from "@/types/system";

const CLASS: Record<DeviceClass, { icon: LucideIcon; label: string }> = {
  display: { icon: Monitor, label: "Display" },
  net: { icon: Network, label: "Network" },
  media: { icon: Volume2, label: "Audio" },
  usb: { icon: Usb, label: "USB" },
  bluetooth: { icon: Bluetooth, label: "Bluetooth" },
  mouse: { icon: Mouse, label: "Mouse" },
  keyboard: { icon: Keyboard, label: "Keyboard" },
  diskDrive: { icon: HardDrive, label: "Drive" },
};

/** The devices Windows lists, with any it reports a problem for. Read-only. */
export function DeviceList() {
  const { data, error, loading, refetch } = useHardwareDetails();
  if (error) return <ErrorState message={error.message} onRetry={refetch} />;
  if (loading && !data) return <LoadingRows rows={3} />;
  const devices = data?.devices ?? [];
  const problems = devices.filter((d) => d.problem).length;

  return (
    <section data-stagger className="flex flex-col overflow-hidden panel">
      <header className="flex h-11 shrink-0 items-center gap-3 border-b px-4">
        <h2 className="text-[14px] font-semibold">Devices</h2>
        <span className="flex-1 text-[12.5px] text-muted-foreground tabular">
          {devices.length}
          {problems > 0 && ` · ${problems} with a problem`}
        </span>
        <DataSourceBadge source="live" />
        <Button
          size="sm"
          variant="outline"
          onClick={() =>
            void backend.openTool("deviceManager").catch((err: unknown) => {
              toast.error(toBackendError(err).message);
            })
          }
        >
          Open Device Manager <ExternalLinkIcon />
        </Button>
      </header>
      {devices.length === 0 ? (
        <p className="flex h-14 items-center px-4 text-[13px] text-muted-foreground">
          No devices reported
        </p>
      ) : (
        // A PC lists dozens: scroll here so the tweaks below stay in view.
        <ul className="max-h-[324px] divide-y overflow-y-auto">
          {collapseDevices(devices).map((d, i) => {
            const { icon: Icon, label } = CLASS[d.class];
            return (
              <li key={`${d.name}-${i}`} className="flex h-9 items-center gap-3 px-4 text-[13px]">
                <Icon className="size-4 shrink-0 text-brand" strokeWidth={1.75} aria-hidden />
                <span data-selectable className="min-w-0 flex-1 truncate font-medium">
                  {deviceName(d)}
                </span>
                <span className="w-24 shrink-0 text-muted-foreground">{label}</span>
                <span className="flex w-32 shrink-0 justify-end">
                  {d.problem && <StatusLabel tone="warning">Problem reported</StatusLabel>}
                </span>
              </li>
            );
          })}
        </ul>
      )}
    </section>
  );
}
