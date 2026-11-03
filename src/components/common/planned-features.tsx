import { ChevronRight, CircleDashed } from "lucide-react";

/**
 * What a page can't do yet, closed by default: honest, but not what the page is for.
 * Unbuilt features stay explicit rather than faked.
 */
export function PlannedFeatures({ features }: { features: string[] }) {
  return (
    <details className="group rounded-xl border bg-card/50">
      <summary className="flex h-10 cursor-pointer list-none items-center gap-2 rounded-xl px-4 text-[13px] text-muted-foreground outline-none hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring [&::-webkit-details-marker]:hidden">
        <ChevronRight className="size-4 transition-transform group-open:rotate-90" />
        Planned ({features.length})
      </summary>
      <ul className="grid grid-cols-2 gap-x-6 gap-y-2 px-4 pb-3.5">
        {features.map((f) => (
          <li key={f} className="flex items-start gap-2 text-[13px] text-muted-foreground">
            <CircleDashed className="mt-0.5 size-3.5 shrink-0 text-muted-foreground/60" />
            {f}
          </li>
        ))}
      </ul>
    </details>
  );
}
