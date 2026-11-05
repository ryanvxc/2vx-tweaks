import type { ReactNode } from "react";
import { ChevronDown } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Progress } from "@/components/ui/progress";
import { driverOf, primaryGpu } from "@/lib/devices";
import { formatBytes, formatFrequency, formatPercent, percentOf } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { DeviceReport, DiskInfo, SystemSnapshot } from "@/types/system";

interface Part {
  title: string;
  facts: [string, ReactNode][];
  /** One column, for values too long to share a row. */
  wide?: boolean;
  extra?: ReactNode;
}

interface Chip {
  label: string;
  /** Joined to the end of the chip, such as the driver version. */
  tag?: string;
  parts: Part[];
}

function gpuChip(s: SystemSnapshot, devices: DeviceReport | undefined): Chip {
  const primary = primaryGpu(s);
  const version = driverOf(primary, devices)?.version;
  return {
    label: primary?.model ?? "No GPU reported",
    tag: version ? `Driver ${version}` : undefined,
    parts: s.gpus.map((g) => {
      const driver = driverOf(g, devices);
      return {
        title: g.model ?? "Unknown GPU",
        facts: [
          ["Video memory", g.dedicatedMemoryBytes ? formatBytes(g.dedicatedMemoryBytes, 0) : "—"],
          ["Vendor", g.vendor ?? "—"],
          ["Driver", driver?.version ?? "—"],
          ["Driver date", driver?.date ?? "—"],
        ],
      };
    }),
  };
}

function cpuChip(s: SystemSnapshot): Chip {
  const board = s.motherboard;
  return {
    label: s.cpu.brand || "Unknown CPU",
    parts: [
      {
        title: s.cpu.brand || "CPU",
        facts: [
          ["Cores", s.cpu.physicalCores ?? "—"],
          ["Threads", s.cpu.logicalCores],
          ["Base clock", formatFrequency(s.cpu.frequencyMhz)],
          ["Vendor", s.cpu.vendor || "—"],
        ],
      },
      ...(board && (board.name || board.vendor)
        ? [
            {
              title: board.name ?? "Motherboard",
              facts: [["Vendor", board.vendor ?? "—"]] as [string, ReactNode][],
            },
          ]
        : []),
      {
        title: `${s.os.productName}${s.os.displayVersion ? ` ${s.os.displayVersion}` : ""}`,
        facts: [
          ["Build", `${s.os.build}${s.os.ubr != null ? `.${s.os.ubr}` : ""}`],
          ["Edition", s.os.editionId ?? "—"],
          ["Architecture", s.os.architecture],
          ["PC name", s.hostname ?? "—"],
        ],
      },
    ],
  };
}

function ramChip(s: SystemSnapshot): Chip {
  return {
    label: formatBytes(s.memory.totalBytes, 0),
    parts: [
      {
        title: "Memory",
        facts: [
          ["Installed", formatBytes(s.memory.totalBytes)],
          ["Page file", formatBytes(s.memory.swapTotalBytes)],
        ],
      },
    ],
  };
}

function diskPart(d: DiskInfo): Part {
  const used = d.totalBytes - d.availableBytes;
  const pct = percentOf(used, d.totalBytes);
  const free = 100 - pct;
  const label = d.name || (d.isRemovable ? "Removable disk" : "Local disk");
  return {
    title: `${d.mountPoint.replace(/\\$/, "")} ${label}`,
    facts: [
      [
        "Type",
        [d.kind !== "unknown" && d.kind.toUpperCase(), d.fileSystem].filter(Boolean).join(" · "),
      ],
      ["Size", formatBytes(d.totalBytes)],
      ["Free", formatBytes(d.availableBytes)],
      ["Used", formatPercent(pct)],
    ],
    extra: (
      <Progress
        value={pct}
        indicatorClassName={cn(
          free < 10 ? "bg-destructive" : free < 20 ? "bg-warning" : "bg-brand",
        )}
        aria-label={`${d.mountPoint} used`}
      />
    ),
  };
}

function storageChip(s: SystemSnapshot, devices: DeviceReport | undefined): Chip {
  const models = devices?.drives ?? [];
  return {
    label: models[0] ?? s.disks[0]?.name ?? "No drive reported",
    parts: [
      ...(models.length > 1
        ? [
            {
              title: "Drives",
              wide: true,
              facts: models.map((m, i): [string, ReactNode] => [`Drive ${i + 1}`, m]),
            },
          ]
        : []),
      ...s.disks.map(diskPart),
    ],
  };
}

/** The search row's device chip: this PC's part for the tab, with its details in the menu. */
export function DeviceChip({
  tab,
  snapshot,
  devices,
}: {
  tab: string;
  snapshot: SystemSnapshot | undefined;
  devices: DeviceReport | undefined;
}) {
  if (!snapshot || tab === "peripherals") return null;
  const chip =
    tab === "gpu"
      ? gpuChip(snapshot, devices)
      : tab === "cpu"
        ? cpuChip(snapshot)
        : tab === "ram"
          ? ramChip(snapshot)
          : storageChip(snapshot, devices);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          type="button"
          className="flex h-10 max-w-[24rem] min-w-0 items-center overflow-hidden rounded-lg border border-input bg-card text-[13.5px] outline-none hover:bg-raised focus-visible:ring-2 focus-visible:ring-ring"
        >
          <span className="flex min-w-0 items-center gap-2 px-3.5">
            <span className="truncate font-medium">{chip.label}</span>
            <ChevronDown className="size-4 shrink-0 text-muted-foreground" aria-hidden />
          </span>
          {chip.tag && (
            <span className="flex h-full shrink-0 items-center border-l px-3 text-[13px] text-muted-foreground tabular">
              {chip.tag}
            </span>
          )}
        </button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-[22rem] p-1.5">
        <div className="flex max-h-96 flex-col gap-3 overflow-y-auto p-2">
          {chip.parts.map((part) => (
            <section key={part.title} className="flex flex-col gap-2">
              <h3 className="truncate text-[13px] font-semibold">{part.title}</h3>
              <dl className={cn("grid gap-x-4 gap-y-1.5", !part.wide && "grid-cols-2")}>
                {part.facts.map(([label, value]) => (
                  <div key={label} className="min-w-0">
                    <dt className="text-[11.5px] text-muted-foreground">{label}</dt>
                    <dd data-selectable className="truncate text-[12.5px] font-medium tabular">
                      {value}
                    </dd>
                  </div>
                ))}
              </dl>
              {part.extra}
            </section>
          ))}
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
