import { ExternalLink as ExternalLinkIcon } from "lucide-react";
import { toast } from "sonner";
import { HintLine } from "@/components/common/hint-line";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { Button } from "@/components/ui/button";
import { useStaggerIn } from "@/hooks/use-motion";
import { useSystemSnapshot } from "@/hooks/use-system";
import { downloadGroups } from "@/lib/downloads";
import { backend, toBackendError } from "@/services/backend";
import type { ExternalLink } from "@/types/system";

const open = (link: ExternalLink) => {
  backend.openLink(link).catch((err: unknown) => toast.error(toBackendError(err).message));
};

/** Links to each maker's own page. 2VX bundles none of these and downloads nothing itself. */
export function DownloadsTab() {
  const { data: snapshot } = useSystemSnapshot();
  const list = useStaggerIn<HTMLDivElement>(snapshot ? "downloads" : null);
  return (
    <div ref={list} className="flex flex-1 flex-col gap-3">
      <HintLine>Each one opens its maker's site in your browser. 2VX doesn't make these.</HintLine>
      {downloadGroups(snapshot).map((group) => (
        <section key={group.title} aria-label={group.title} className="flex flex-col gap-2">
          <h2 className="text-[13px] font-semibold">{group.title}</h2>
          <CardGrid>
            {group.items.map((item) => (
              <OptimisationCard
                key={item.id}
                icon={item.icon}
                title={item.name}
                description={item.description}
                footer={
                  <Button variant="outline" className="w-full" onClick={() => open(item.link)}>
                    Open site <ExternalLinkIcon />
                  </Button>
                }
              />
            ))}
          </CardGrid>
        </section>
      ))}
    </div>
  );
}
