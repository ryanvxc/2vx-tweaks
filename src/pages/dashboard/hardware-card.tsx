import { Link } from "react-router";
import { Cpu, Gpu, HardDrive, MemoryStick, type LucideIcon } from "lucide-react";
import { SectionCard } from "@/components/common/section-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { formatBytes, percentOf } from "@/lib/format";
import type { BackendError, SystemSnapshot } from "@/types/system";

/** Four lines on Home; the full sheet is the Hardware page. */
export function HardwareCard({
  snapshot,
  error,
  onRetry,
}: {
  snapshot: SystemSnapshot | undefined;
  error: BackendError | undefined;
  onRetry: () => void;
}) {
  const drive = snapshot?.disks.find((d) => /^c:/i.test(d.mountPoint)) ?? snapshot?.disks[0];
  return (
    <SectionCard
      title="This PC"
      source="live"
      className="h-full"
      actions={
        <Link to="/hardware" className="text-[12px] font-medium text-brand hover:underline">
          Details
        </Link>
      }
    >
      {error ? (
        <ErrorState
          title="Couldn't read hardware information"
          message={error.message}
          onRetry={onRetry}
        />
      ) : !snapshot ? (
        <LoadingRows rows={4} />
      ) : (
        <ul className="flex flex-col gap-2">
          <Row icon={Cpu} text={snapshot.cpu.brand.trim() || "Unknown CPU"} />
          <Row icon={Gpu} text={snapshot.gpus[0]?.model ?? "No GPU reported"} />
          <Row icon={MemoryStick} text={`${formatBytes(snapshot.memory.totalBytes, 0)} memory`} />
          {drive && (
            <Row
              icon={HardDrive}
              text={`${formatBytes(drive.availableBytes)} free on ${drive.mountPoint.replace(/\\$/, "")}`}
              hint={`${Math.round(percentOf(drive.availableBytes, drive.totalBytes))}%`}
            />
          )}
        </ul>
      )}
    </SectionCard>
  );
}

function Row({ icon: Icon, text, hint }: { icon: LucideIcon; text: string; hint?: string }) {
  return (
    <li className="flex items-center gap-2.5 text-[13px]">
      <Icon className="size-4 shrink-0 text-muted-foreground" aria-hidden />
      <span data-selectable className="min-w-0 flex-1 truncate font-medium" title={text}>
        {text}
      </span>
      {hint && <span className="text-[12px] text-muted-foreground tabular">{hint}</span>}
    </li>
  );
}
