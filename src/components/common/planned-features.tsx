import { CircleDashed } from "lucide-react";
import { SectionCard } from "./section-card";

/** Lists the features a page will hold, so unbuilt sections are explicit rather than faked. */
export function PlannedFeatures({ features }: { features: string[] }) {
  return (
    <SectionCard title="Planned for this section" source="planned">
      <ul className="grid grid-cols-2 gap-x-6 gap-y-2.5">
        {features.map((f) => (
          <li key={f} className="flex items-start gap-2 text-[13px] text-muted-foreground">
            <CircleDashed className="mt-0.5 size-3.5 shrink-0 text-muted-foreground/60" />
            {f}
          </li>
        ))}
      </ul>
    </SectionCard>
  );
}
