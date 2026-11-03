import type { ReactNode } from "react";
import { CircuitBoard, Cpu, Gpu, HardDrive, MemoryStick, type LucideIcon } from "lucide-react";
import { SectionCard } from "@/components/common/section-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Progress } from "@/components/ui/progress";
import { formatBytes, formatFrequency, percentOf } from "@/lib/format";
import { cn } from "@/lib/utils";
import type { BackendError, DiskInfo, SystemSnapshot } from "@/types/system";

export function HardwareCard({
  snapshot,
  error,
  onRetry,
}: {
  snapshot: SystemSnapshot | undefined;
  error: BackendError | undefined;
  onRetry: () => void;
}) {
  return (
    <SectionCard title="Hardware" description="Detected components" source="live">
      {error ? (
        <ErrorState
          title="Couldn't read hardware information"
          message={error.message}
          onRetry={onRetry}
        />
      ) : !snapshot ? (
        <LoadingRows rows={6} />
      ) : (
        <div className="flex flex-col">
          <Row icon={Cpu} label="Processor">
            <Primary>{snapshot.cpu.brand || "Unknown CPU"}</Primary>
            <Secondary>
              {snapshot.cpu.physicalCores ?? "?"} cores · {snapshot.cpu.logicalCores} threads ·{" "}
              {formatFrequency(snapshot.cpu.frequencyMhz)} base
            </Secondary>
          </Row>
          <Row icon={Gpu} label="Graphics">
            {snapshot.gpus.length === 0 ? (
              <Secondary>No GPU reported</Secondary>
            ) : (
              snapshot.gpus.map((g, i) => (
                <div key={i}>
                  <Primary>{g.model ?? "Unknown GPU"}</Primary>
                  <Secondary>
                    {g.dedicatedMemoryBytes
                      ? `${formatBytes(g.dedicatedMemoryBytes, 0)} dedicated`
                      : "—"}
                  </Secondary>
                </div>
              ))
            )}
          </Row>
          <Row icon={MemoryStick} label="Memory">
            <Primary>{formatBytes(snapshot.memory.totalBytes, 0)}</Primary>
            <Secondary>Page file {formatBytes(snapshot.memory.swapTotalBytes, 0)}</Secondary>
          </Row>
          {snapshot.motherboard && (snapshot.motherboard.name || snapshot.motherboard.vendor) && (
            <Row icon={CircuitBoard} label="Motherboard">
              <Primary>{snapshot.motherboard.name ?? "Unknown"}</Primary>
              <Secondary>{snapshot.motherboard.vendor ?? ""}</Secondary>
            </Row>
          )}
          <Row icon={HardDrive} label="Storage" last>
            <div className="flex flex-col gap-3">
              {snapshot.disks.map((d) => (
                <DiskRow key={d.mountPoint} disk={d} />
              ))}
            </div>
          </Row>
        </div>
      )}
    </SectionCard>
  );
}

function Row({
  icon: Icon,
  label,
  children,
  last,
}: {
  icon: LucideIcon;
  label: string;
  children: ReactNode;
  last?: boolean;
}) {
  return (
    <div className={cn("grid grid-cols-[132px_1fr] gap-4 py-3", !last && "border-b")}>
      <div className="flex items-center gap-2 self-start pt-px text-[12.5px] text-muted-foreground">
        <Icon className="size-4" />
        {label}
      </div>
      <div data-selectable className="min-w-0">
        {children}
      </div>
    </div>
  );
}

const Primary = ({ children }: { children: ReactNode }) => (
  <div className="truncate text-[13px] font-medium">{children}</div>
);
const Secondary = ({ children }: { children: ReactNode }) => (
  <div className="truncate text-[12px] text-muted-foreground tabular">{children}</div>
);

function DiskRow({ disk }: { disk: DiskInfo }) {
  const usedPct = percentOf(disk.totalBytes - disk.availableBytes, disk.totalBytes);
  const kind = disk.kind === "ssd" ? "SSD" : disk.kind === "hdd" ? "HDD" : null;
  return (
    <div>
      <div className="flex items-baseline justify-between gap-3">
        <div className="truncate text-[13px] font-medium">
          {disk.mountPoint.replace(/\\$/, "")}
          {disk.name && <span className="font-normal text-muted-foreground"> {disk.name}</span>}
          {kind && (
            <span className="ml-2 rounded border border-border px-1 py-px text-[10px] font-medium text-muted-foreground">
              {kind}
            </span>
          )}
        </div>
        <div className="shrink-0 text-[12px] text-muted-foreground tabular">
          {formatBytes(disk.availableBytes)} free of {formatBytes(disk.totalBytes)}
        </div>
      </div>
      <Progress
        value={usedPct}
        className="mt-1.5"
        indicatorClassName={cn(
          usedPct > 90 ? "bg-destructive" : usedPct > 80 ? "bg-warning" : "bg-brand",
        )}
      />
    </div>
  );
}
