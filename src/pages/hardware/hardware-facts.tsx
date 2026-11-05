import { useState, type ReactNode } from "react";
import { ExternalLink as ExternalLinkIcon } from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import type { Tone } from "@/lib/engine-text";
import { formatBytes } from "@/lib/format";
import { useHardwareDetails, useSystemSnapshot } from "@/hooks/use-system";
import { collapseDevices, deviceName } from "@/lib/devices";
import { backend, toBackendError } from "@/services/backend";
import type {
  CpuDetails,
  DeviceClass,
  DeviceEntry,
  DriveDetails,
  ExternalLink,
  GpuDriver,
  GpuInfo,
  HardwareDetails,
  MemoryDetails,
  MemoryModule,
} from "@/types/system";

/** Devices shown before the list folds. */
const DEVICE_LIMIT = 8;

const live = <DataSourceBadge source="live" />;

const open = (link: ExternalLink) =>
  backend.openLink(link).catch((err: unknown) => {
    toast.error(toBackendError(err).message);
  });

type Fact = [label: string, value: ReactNode | null | undefined];

/** Label over value. A fact without a value is left out, never printed as "Unknown". */
function FactList({ facts }: { facts: Fact[] }) {
  const known = facts.filter((f): f is [string, ReactNode] => f[1] != null && f[1] !== "");
  if (known.length === 0) return null;
  return (
    <dl className="grid min-w-0 flex-1 grid-cols-[repeat(auto-fit,minmax(8rem,1fr))] gap-x-6 gap-y-1.5">
      {known.map(([label, value]) => (
        <div key={label} className="min-w-0">
          <dt className="text-[11.5px] text-muted-foreground">{label}</dt>
          <dd data-selectable className="truncate text-[13px] font-medium tabular">
            {value}
          </dd>
        </div>
      ))}
    </dl>
  );
}

interface Column<T> {
  label: string;
  cell: (row: T) => ReactNode | null | undefined;
  /** Twice the width of the others, for names. */
  wide?: boolean;
}

/** Rows under shared column labels. A column no row has a value for is dropped. */
function FactTable<T>({ columns, rows }: { columns: Column<T>[]; rows: T[] }) {
  const hasValue = (value: ReactNode) => value != null && value !== "" && value !== false;
  const shown = columns.filter((c) => rows.some((r) => hasValue(c.cell(r))));
  if (shown.length === 0) return null;
  const template = shown.map((c) => (c.wide ? "minmax(0,2fr)" : "minmax(0,1fr)")).join(" ");
  const line = "grid items-center gap-x-4";
  return (
    <div className="flex flex-col divide-y">
      <div
        className={`${line} pb-1.5 text-[11.5px] text-muted-foreground`}
        style={{ gridTemplateColumns: template }}
      >
        {shown.map((c) => (
          <span key={c.label} className="truncate">
            {c.label}
          </span>
        ))}
      </div>
      {rows.map((row, i) => (
        <div
          key={i}
          className={`${line} min-h-8 text-[13px]`}
          style={{ gridTemplateColumns: template }}
        >
          {shown.map((c) => (
            <span key={c.label} data-selectable className="min-w-0 truncate font-medium tabular">
              {c.cell(row)}
            </span>
          ))}
        </div>
      ))}
    </div>
  );
}

function Panel({ children, badge = true }: { children: ReactNode; badge?: boolean }) {
  return (
    <section data-stagger className="relative flex flex-col gap-2 panel px-4 py-3">
      {badge && <span className="absolute top-3 right-4">{live}</span>}
      {children}
    </section>
  );
}

/** The maker, read from the adapter's name: the only place a driver report says it. */
function gpuMaker(name: string): string | null {
  const n = name.toLowerCase();
  if (n.includes("nvidia") || n.includes("geforce")) return "NVIDIA";
  if (n.includes("radeon") || /\bamd\b/.test(n) || /\bati\b/.test(n)) return "AMD";
  if (n.includes("intel")) return "Intel";
  if (n.includes("microsoft")) return "Microsoft";
  return null;
}

const DRIVER_LINK: Record<string, { label: string; to: ExternalLink }> = {
  NVIDIA: { label: "NVIDIA drivers", to: "nvidiaDrivers" },
  AMD: { label: "AMD drivers", to: "amdDrivers" },
  Intel: { label: "Intel drivers", to: "intelDrivers" },
};

function LinkButton({ label, to }: { label: string; to: ExternalLink }) {
  return (
    <Button size="sm" variant="ghost" className="text-brand" onClick={() => void open(to)}>
      {label} <ExternalLinkIcon />
    </Button>
  );
}

function GpuBlock({
  gpu,
  memory,
  badge,
}: {
  gpu: GpuDriver;
  memory: number | null | undefined;
  badge: boolean;
}) {
  const maker = gpuMaker(gpu.name);
  const link = maker ? DRIVER_LINK[maker] : undefined;
  return (
    <div className="flex flex-col gap-1.5 py-2 first:pt-0 last:pb-0">
      <div className="flex items-center gap-3">
        <h3 data-selectable className="min-w-0 truncate text-[14px] font-semibold">
          {gpu.name}
        </h3>
        {gpu.isBasicDriver && (
          <StatusLabel
            tone="warning"
            title="Windows' fallback driver, not one from the GPU's maker"
          >
            Microsoft Basic Display Driver
          </StatusLabel>
        )}
        <span className="ml-auto flex shrink-0 items-center gap-1">
          {link && <LinkButton {...link} />}
          <LinkButton label="Graphics settings" to="graphicsSettings" />
        </span>
        {badge && live}
      </div>
      <FactList
        facts={[
          ["Video memory", memory ? formatBytes(memory, 0) : null],
          ["Driver version", gpu.isBasicDriver ? null : gpu.version],
          ["Driver date", gpu.isBasicDriver ? null : gpu.date],
          // The GPU's name already says who made it, unless someone else wrote the driver.
          ["Provider", gpu.provider === maker ? null : gpu.provider],
        ]}
      />
    </div>
  );
}

export function GpuFacts({ drivers, gpus = [] }: { drivers: GpuDriver[]; gpus?: GpuInfo[] }) {
  if (drivers.length === 0) return null;
  return (
    <Panel badge={false}>
      <div className="flex flex-col divide-y">
        {drivers.map((gpu, i) => (
          <GpuBlock
            key={`${gpu.name}:${i}`}
            gpu={gpu}
            memory={gpus.find((g) => g.model === gpu.name)?.dedicatedMemoryBytes}
            badge={i === 0}
          />
        ))}
      </div>
    </Panel>
  );
}

export function CpuFacts({ cpu }: { cpu: CpuDetails }) {
  const cores = (n: number | null) => (n ? n : null);
  return (
    <section data-stagger className="flex items-start gap-4 panel px-4 py-3">
      <FactList
        facts={[
          ["Performance cores", cores(cpu.performanceCores)],
          // Only a hybrid CPU has a second kind of core.
          ["Efficient cores", cores(cpu.efficientCores)],
          ["Logical processors", cpu.logicalProcessors],
          ["L2 cache", cpu.l2CacheBytes ? formatBytes(cpu.l2CacheBytes, 0) : null],
          ["L3 cache", cpu.l3CacheBytes ? formatBytes(cpu.l3CacheBytes, 0) : null],
        ]}
      />
      {live}
    </section>
  );
}

const onOff = (on: boolean | null) => (on == null ? null : on ? "On" : "Off");

function speed(m: MemoryModule): string | null {
  if (m.speedMts == null) return null;
  const rated = m.ratedSpeedMts != null && m.ratedSpeedMts !== m.speedMts;
  return `${m.speedMts} MT/s${rated ? ` (rated ${m.ratedSpeedMts})` : ""}`;
}

export function RamFacts({ memory }: { memory: MemoryDetails }) {
  const slots =
    memory.slotsUsed != null && memory.slotsTotal != null
      ? `${memory.slotsUsed} of ${memory.slotsTotal}`
      : null;
  return (
    <Panel>
      <FactList
        facts={[
          ["Slots in use", slots],
          ["Memory compression", onOff(memory.compressionActive)],
        ]}
      />
      <FactTable
        rows={memory.modules}
        columns={[
          { label: "Slot", cell: (m) => m.slot },
          {
            label: "Size",
            cell: (m) => (m.capacityBytes ? formatBytes(m.capacityBytes, 0) : null),
          },
          { label: "Type", cell: (m) => m.memoryType },
          { label: "Speed", cell: speed },
          { label: "Maker", cell: (m) => m.manufacturer },
          { label: "Part number", cell: (m) => m.partNumber, wide: true },
        ]}
      />
    </Panel>
  );
}

const MEDIA = { ssd: "SSD", hdd: "HDD", unknown: null } as const;
const HEALTH: Record<string, Tone> = {
  Healthy: "success",
  Warning: "warning",
  Unhealthy: "destructive",
};

export function StorageFacts({
  drives,
  trimEnabled,
}: {
  drives: DriveDetails[];
  trimEnabled: boolean | null;
}) {
  return (
    <Panel>
      <FactList facts={[["TRIM in Windows", onOff(trimEnabled)]]} />
      <FactTable
        rows={drives}
        columns={[
          { label: "Drive", cell: (d) => d.model, wide: true },
          { label: "Bus", cell: (d) => d.bus },
          { label: "Media", cell: (d) => MEDIA[d.media] },
          { label: "Size", cell: (d) => (d.sizeBytes ? formatBytes(d.sizeBytes, 0) : null) },
          {
            label: "TRIM",
            cell: (d) => (d.trimSupported == null ? null : d.trimSupported ? "Supported" : "No"),
          },
          {
            label: "Health",
            cell: (d) =>
              d.health && (
                <StatusLabel tone={HEALTH[d.health] ?? "neutral"} className="text-[13px]">
                  {d.health}
                </StatusLabel>
              ),
          },
        ]}
      />
    </Panel>
  );
}

const DEVICE_TYPE: Partial<Record<DeviceClass, string>> = {
  mouse: "Mouse",
  keyboard: "Keyboard",
  media: "Sound and video",
  bluetooth: "Bluetooth",
  usb: "USB",
};
const DEVICE_ORDER = Object.keys(DEVICE_TYPE);

export function PeripheralFacts({ devices }: { devices: DeviceEntry[] }) {
  const [all, setAll] = useState(false);
  const list = collapseDevices(devices.filter((d) => d.class in DEVICE_TYPE)).toSorted(
    (a, b) => DEVICE_ORDER.indexOf(a.class) - DEVICE_ORDER.indexOf(b.class),
  );
  if (list.length === 0) return null;
  const rows = all ? list : list.slice(0, DEVICE_LIMIT);
  return (
    <Panel>
      <FactTable
        rows={rows}
        columns={[
          { label: "Device", cell: deviceName, wide: true },
          { label: "Type", cell: (d) => DEVICE_TYPE[d.class] },
          {
            label: "Status",
            cell: (d) =>
              d.problem && (
                <StatusLabel tone="warning" className="text-[13px]">
                  Windows reports a problem
                </StatusLabel>
              ),
          },
        ]}
      />
      {list.length > DEVICE_LIMIT && (
        <Button
          size="sm"
          variant="ghost"
          className="self-start"
          onClick={() => setAll((v) => !v)}
          aria-expanded={all}
        >
          {all ? "Show fewer" : `Show ${list.length - DEVICE_LIMIT} more`}
        </Button>
      )}
    </Panel>
  );
}

/** One tab's facts, from an already-read report. */
export function FactsPanel({
  tab,
  details,
  gpus,
}: {
  tab: string;
  details: HardwareDetails;
  gpus?: GpuInfo[];
}) {
  switch (tab) {
    case "gpu":
      return <GpuFacts drivers={details.gpuDrivers} gpus={gpus} />;
    case "cpu":
      return <CpuFacts cpu={details.cpu} />;
    case "ram":
      return <RamFacts memory={details.memory} />;
    case "storage":
      return <StorageFacts drives={details.drives} trimEnabled={details.trimEnabled} />;
    case "peripherals":
      return <PeripheralFacts devices={details.devices} />;
    default:
      return null;
  }
}

/** Reads the report (slow, so cached for the session) and shows the tab's part of it. */
export function HardwareFacts({ tab }: { tab: string }) {
  const { data, error, loading, refetch } = useHardwareDetails();
  const gpus = useSystemSnapshot().data?.gpus;
  if (data) return <FactsPanel tab={tab} details={data} gpus={gpus} />;
  if (error) {
    return (
      <section className="flex items-center gap-3 panel px-4 py-3 text-[13px]" role="alert">
        <StatusLabel tone="warning">Couldn't read the hardware details</StatusLabel>
        <span className="min-w-0 flex-1 truncate text-muted-foreground" title={error.message}>
          {error.message}
        </span>
        <Button size="sm" variant="outline" onClick={refetch}>
          Retry
        </Button>
      </section>
    );
  }
  return loading ? <Skeleton className="h-[72px] rounded-xl" /> : null;
}
