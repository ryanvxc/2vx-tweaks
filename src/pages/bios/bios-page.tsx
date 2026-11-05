import { useState } from "react";
import { ExternalLink as ExternalLinkIcon } from "lucide-react";
import { toast } from "sonner";
import { BIOS_TABS, type BiosTab } from "@/app/navigation";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { HintLine } from "@/components/common/hint-line";
import { StatusLabel } from "@/components/common/notice";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { PageTabs } from "@/components/common/page-tabs";
import { RecommendedAction } from "@/components/common/recommended-action";
import { PageHero } from "@/components/layout/page-hero";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useStaggerIn } from "@/hooks/use-motion";
import {
  useDeviceReport,
  useFirmwareReport,
  useHardwareDetails,
  useSystemSnapshot,
} from "@/hooks/use-system";
import { useTabParam } from "@/hooks/use-tab-param";
import { biosItems, type BiosFacts, type BiosItem } from "@/lib/bios-guide";
import { backend, toBackendError } from "@/services/backend";

/**
 * What to check in the BIOS, with what Windows can see of each. It's a guide and nothing more: 2VX
 * has no way to write firmware settings, and `docs/security.md` says why it never will.
 */
export function BiosPage() {
  const [tab, setTab] = useTabParam<BiosTab>(BIOS_TABS, "general");
  const [open, setOpen] = useState<BiosItem | null>(null);
  const { data: firmware } = useFirmwareReport();
  const { data: devices } = useDeviceReport();
  const { data: hardware } = useHardwareDetails();
  const { data: snapshot } = useSystemSnapshot();
  const facts: BiosFacts = { firmware, devices, hardware, snapshot };
  const list = useStaggerIn<HTMLDivElement>(tab);
  const state = open?.state(facts);
  const link = open?.link?.(facts);

  return (
    <div className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]">
      <PageHero
        title="BIOS"
        subtitle="What to check in your BIOS. 2VX never changes it."
        action={<RecommendedAction />}
      />
      <PageTabs
        label="BIOS"
        tabs={BIOS_TABS}
        value={tab}
        onChange={setTab}
        end={<DataSourceBadge source="live" spelled />}
        filled
      />
      <HintLine>
        {[firmware?.board, firmware?.version && `BIOS ${firmware.version}`, firmware?.releaseDate]
          .filter(Boolean)
          .join(" · ") || "Reading the BIOS"}
      </HintLine>
      <div ref={list}>
        <CardGrid>
          {biosItems(tab, facts).map((item) => {
            const detected = item.state(facts);
            return (
              <OptimisationCard
                key={item.id}
                icon={item.icon}
                title={item.title}
                description={item.summary}
                onOpen={() => setOpen(item)}
                indicators={
                  detected ? (
                    <StatusLabel tone={detected.tone}>{detected.label}</StatusLabel>
                  ) : (
                    <StatusLabel tone="neutral">Check in the BIOS</StatusLabel>
                  )
                }
              />
            );
          })}
        </CardGrid>
      </div>

      <Dialog open={open !== null} onOpenChange={(next) => !next && setOpen(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{open?.title}</DialogTitle>
            <DialogDescription>{open?.summary}</DialogDescription>
          </DialogHeader>
          <p data-selectable className="text-[13px] leading-relaxed">
            {open?.detail}
          </p>
          <dl className="flex flex-col gap-1.5 text-[13px]">
            {state && (
              <div className="flex items-center justify-between gap-4">
                <dt className="text-muted-foreground">On this PC</dt>
                <dd>
                  <StatusLabel tone={state.tone}>{state.label}</StatusLabel>
                </dd>
              </div>
            )}
            {open?.lookFor && (
              <div className="flex items-center justify-between gap-4">
                <dt className="shrink-0 text-muted-foreground">Look for</dt>
                <dd data-selectable className="truncate text-right font-medium">
                  {open.lookFor}
                </dd>
              </div>
            )}
          </dl>
          {link && (
            <DialogFooter>
              <Button
                variant="outline"
                onClick={() => {
                  backend
                    .openLink(link)
                    .catch((err: unknown) => toast.error(toBackendError(err).message));
                }}
              >
                Read more <ExternalLinkIcon />
              </Button>
            </DialogFooter>
          )}
        </DialogContent>
      </Dialog>
    </div>
  );
}
