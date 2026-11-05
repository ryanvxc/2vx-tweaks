import type { ReactNode } from "react";
import { CircleHelp, ExternalLink as ExternalLinkIcon, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { backend, toBackendError } from "@/services/backend";
import type { ExternalLink } from "@/types/system";

const open = (link: ExternalLink) =>
  backend.openLink(link).catch((err: unknown) => {
    toast.error(toBackendError(err).message);
  });

/** Full-width explainer above a tab's content. */
export function InfoPanel({
  icon: Icon = CircleHelp,
  title,
  link,
  children,
}: {
  icon?: LucideIcon;
  title: string;
  /** A page to open, at the right of the title. */
  link?: { label: string; to: ExternalLink };
  children: ReactNode;
}) {
  return (
    <section data-stagger className="panel p-4">
      <div className="flex items-center justify-between gap-4">
        <h2 className="flex items-center gap-2 text-[14.5px] font-semibold">
          <Icon className="size-4" aria-hidden /> {title}
        </h2>
        {link && (
          <button
            type="button"
            onClick={() => void open(link.to)}
            className="flex shrink-0 items-center gap-1.5 text-[13px] text-brand hover:underline"
          >
            {link.label} <ExternalLinkIcon className="size-3.5" aria-hidden />
          </button>
        )}
      </div>
      <p data-selectable className="mt-1.5 text-[13px] leading-relaxed text-muted-foreground">
        {children}
      </p>
    </section>
  );
}

/** A card with one job: what it is, the facts behind it, and a page to open. */
export function ActionCard({
  icon: Icon,
  title,
  status,
  description,
  rows,
  action,
}: {
  icon: LucideIcon;
  title: string;
  status?: ReactNode;
  description: string;
  rows?: [string, ReactNode][];
  action?: { label: string; to: ExternalLink };
}) {
  return (
    <article data-stagger className="flex flex-col gap-4 panel p-5">
      <header className="flex items-center gap-3">
        <span className="flex size-10 shrink-0 items-center justify-center rounded-lg bg-brand/12 text-brand">
          <Icon className="size-5" aria-hidden />
        </span>
        <h2 className="min-w-0 flex-1 truncate text-[16px] font-semibold">{title}</h2>
        {status}
      </header>
      <p data-selectable className="text-[13px] leading-relaxed text-muted-foreground">
        {description}
      </p>
      {rows && (
        <dl className="flex flex-col gap-2 text-[13px]">
          {rows.map(([label, value]) => (
            <div key={label} className="flex items-center justify-between gap-4">
              <dt className="shrink-0 text-muted-foreground">{label}</dt>
              <dd data-selectable className="min-w-0 truncate font-medium tabular">
                {value}
              </dd>
            </div>
          ))}
        </dl>
      )}
      {action && (
        <Button className="mt-auto h-11" onClick={() => void open(action.to)}>
          {action.label} <ExternalLinkIcon />
        </Button>
      )}
    </article>
  );
}
