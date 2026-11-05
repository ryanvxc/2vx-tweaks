import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { useDeviceReport } from "@/hooks/use-system";
import type { InterruptSupport, MsiGroup } from "@/types/system";

const MSI_GROUP: Record<MsiGroup, string> = {
  gpu: "GPU",
  audio: "Audio",
  network: "Network",
  nvme: "NVMe",
  sata: "SATA",
  usb: "USB",
};

/** What the device can do, as Windows lists it. */
function describeSupport(support: InterruptSupport | null): string {
  const modes = [
    support?.lineBased && "Line-based",
    support?.msi && "MSI",
    support?.msiX && "MSI-X",
  ].filter(Boolean);
  return modes.length > 0 ? modes.join(", ") : "Not reported";
}

const COLUMNS = "flex items-center gap-4 px-5";

/** Read-only: forcing MSI on a device that can't use it can stop Windows starting. */
export function MsiTab() {
  const { data, error, loading, refetch } = useDeviceReport();
  return (
    <div className="flex flex-1 flex-col gap-3">
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={6} />
      ) : (
        <div className="flex min-h-0 flex-1 flex-col overflow-hidden panel">
          <div
            className={`${COLUMNS} h-10 shrink-0 border-b text-[12.5px] font-medium text-muted-foreground`}
          >
            <span className="w-20 shrink-0">Type</span>
            <span className="min-w-0 flex-1">Device</span>
            <span className="w-44 shrink-0">Supports</span>
            <span className="flex w-28 shrink-0 justify-end">
              <DataSourceBadge source="live" />
            </span>
          </div>
          {data?.msi.length === 0 ? (
            <p className="flex h-14 items-center px-5 text-[13px] text-muted-foreground">
              No devices reported
            </p>
          ) : (
            <ul className="divide-y">
              {data?.msi.map((d, i) => (
                <li key={`${d.name}-${i}`} className={`${COLUMNS} h-12 text-[13.5px]`}>
                  <span className="w-20 shrink-0 text-muted-foreground">{MSI_GROUP[d.group]}</span>
                  <span data-selectable className="min-w-0 flex-1 truncate font-medium">
                    {d.name}
                  </span>
                  <span className="w-44 shrink-0 truncate text-[13px] text-muted-foreground">
                    {describeSupport(d.interruptSupport)}
                  </span>
                  <span className="flex w-28 shrink-0 justify-end">
                    <StatusLabel tone={d.enabled ? "success" : "neutral"}>
                      {d.enabled ? "MSI" : "Line-based"}
                    </StatusLabel>
                  </span>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
}
