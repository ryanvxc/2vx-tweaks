import {
  EthernetPort,
  ExternalLink as ExternalLinkIcon,
  Wifi,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useDeviceReport } from "@/hooks/use-system";
import { backend, toBackendError } from "@/services/backend";
import type { AdapterKind, NetworkAdapter } from "@/types/system";

const ICON: Record<AdapterKind, LucideIcon> = { wifi: Wifi, ethernet: EthernetPort };

/** 1_000_000_000 becomes "1 Gbps". Link speeds are decimal, unlike file sizes. */
function formatLinkSpeed(bps: number | null | undefined): string | null {
  if (bps == null || !Number.isFinite(bps) || bps <= 0) return null;
  const [value, unit] = bps >= 1e9 ? [bps / 1e9, "Gbps"] : [bps / 1e6, "Mbps"];
  return `${Number.isInteger(value) ? value : value.toFixed(1)} ${unit}`;
}

/** Every physical adapter in one panel, above the tuning tweaks for the two kinds. */
export function AdapterList() {
  const { data, error, loading, refetch } = useDeviceReport();
  if (error) return <ErrorState message={error.message} onRetry={refetch} />;
  if (loading && !data) return <LoadingRows rows={2} />;
  const adapters = data?.adapters ?? [];

  return (
    <section data-stagger className="flex flex-col overflow-hidden panel">
      <header className="flex h-11 items-center gap-3 border-b px-4">
        <h2 className="flex-1 text-[14px] font-semibold">Adapters</h2>
        <DataSourceBadge source="live" />
        <Button
          size="sm"
          variant="outline"
          onClick={() =>
            void backend.openLink("networkSettings").catch((err: unknown) => {
              toast.error(toBackendError(err).message);
            })
          }
        >
          Open adapter settings <ExternalLinkIcon />
        </Button>
      </header>
      {adapters.length === 0 ? (
        <p className="flex h-14 items-center px-4 text-[13px] text-muted-foreground">
          No network adapters found
        </p>
      ) : (
        <ul className="divide-y">
          {adapters.map((adapter, i) => (
            <AdapterRow key={`${adapter.name}-${i}`} adapter={adapter} />
          ))}
        </ul>
      )}
    </section>
  );
}

function AdapterRow({ adapter }: { adapter: NetworkAdapter }) {
  const Icon = ICON[adapter.kind];
  const speed = adapter.connected ? formatLinkSpeed(adapter.linkSpeedBps) : null;
  return (
    <li className="flex h-12 items-center gap-3 px-4 text-[13px]">
      <Icon className="size-[18px] shrink-0 text-brand" strokeWidth={1.75} aria-hidden />
      <span data-selectable className="min-w-0 flex-1 truncate text-[14px] font-semibold">
        {adapter.name}
      </span>
      <span data-selectable className="w-44 shrink-0 truncate text-muted-foreground tabular">
        {adapter.driverVersion ? `Driver ${adapter.driverVersion}` : "Driver not reported"}
      </span>
      <span className="w-20 shrink-0 text-right text-muted-foreground tabular">{speed ?? ""}</span>
      <span className="flex w-32 shrink-0 justify-end">
        {adapter.connected === null ? (
          <StatusLabel tone="neutral">Not reported</StatusLabel>
        ) : (
          <StatusLabel tone={adapter.connected ? "success" : "neutral"}>
            {adapter.connected ? "Connected" : "Not connected"}
          </StatusLabel>
        )}
      </span>
    </li>
  );
}
