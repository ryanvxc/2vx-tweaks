import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { StatusLabel } from "@/components/common/notice";
import type { Tone } from "@/lib/engine-text";

export interface PresetFact {
  label: string;
  value: ReactNode;
  /** What the fact means, when the label alone can't say. */
  title?: string;
}

/**
 * A named list of tweaks applied together: Tweaks' presets, the service presets and the GPU profiles.
 * The state top-right is read from the PC; the facts are counts of real tweaks, never a score.
 */
export function PresetCard({
  icon: Icon,
  title,
  summary,
  status,
  facts,
  children,
}: {
  icon: LucideIcon;
  title: string;
  summary: string;
  status: { tone: Tone; label: string };
  facts: PresetFact[];
  /** The buttons along the bottom. */
  children?: ReactNode;
}) {
  return (
    <article
      data-stagger
      data-spot=""
      className="group spot relative flex flex-col gap-3 panel p-5"
    >
      <header className="flex items-center gap-3">
        <Icon
          className="size-[22px] shrink-0 text-brand group-hover:animate-nudge"
          strokeWidth={1.75}
          aria-hidden
        />
        <h2 className="min-w-0 flex-1 truncate text-[15px] font-semibold">{title}</h2>
        <StatusLabel tone={status.tone}>{status.label}</StatusLabel>
      </header>
      <p className="truncate text-[13px] text-muted-foreground" title={summary}>
        {summary}
      </p>
      <dl className="flex flex-col gap-1.5 text-[13px]">
        {facts.map((fact) => (
          <div key={fact.label} className="flex items-center justify-between gap-3">
            <dt className="text-muted-foreground">{fact.label}</dt>
            <dd className="font-medium tabular" title={fact.title}>
              {fact.value}
            </dd>
          </div>
        ))}
      </dl>
      {children && (
        <div className="mt-auto flex gap-2 pt-1 [&>*]:h-10 [&>*]:flex-1">{children}</div>
      )}
    </article>
  );
}
