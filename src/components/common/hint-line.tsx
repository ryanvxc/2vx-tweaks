import type { ReactNode } from "react";
import { ExternalLink as ExternalLinkIcon, Info } from "lucide-react";
import { toast } from "sonner";
import { backend, toBackendError } from "@/services/backend";
import type { ExternalLink } from "@/types/system";

/** One line above a tab's content, for the single thing that could confuse. */
export function HintLine({
  children,
  link,
}: {
  children: ReactNode;
  link?: { label: string; to: ExternalLink };
}) {
  return (
    <div
      data-stagger
      data-selectable
      className="flex min-h-10 items-center gap-2.5 panel px-4 py-2 text-[13px] text-muted-foreground"
    >
      <Info className="size-4 shrink-0" aria-hidden />
      <p className="min-w-0 flex-1">{children}</p>
      {link && (
        <button
          type="button"
          onClick={() =>
            void backend.openLink(link.to).catch((err: unknown) => {
              toast.error(toBackendError(err).message);
            })
          }
          className="flex shrink-0 items-center gap-1.5 text-brand hover:underline"
        >
          {link.label} <ExternalLinkIcon className="size-3.5" aria-hidden />
        </button>
      )}
    </div>
  );
}
