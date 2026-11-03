import { PackageOpen } from "lucide-react";
import { RiskBadge } from "@/components/common/risk-badge";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { useTweaks } from "@/hooks/use-system";
import type { TweakCategory } from "@/types/engine";

/** Shows tweaks registered in the backend engine for the given categories. */
export function TweakRegistryCard({ categories }: { categories: TweakCategory[] }) {
  const { data, error, loading, refetch } = useTweaks();
  const tweaks = (data ?? []).filter((t) => categories.includes(t.category));

  return (
    <SectionCard
      title="Available optimisations"
      description="From the optimisation engine"
      source="live"
    >
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading ? (
        <LoadingRows />
      ) : tweaks.length === 0 ? (
        <EmptyState
          icon={PackageOpen}
          title="No optimisations registered yet"
          description="The engine is connected and reports an empty registry. Each tweak is added only after its effect, risk and undo path have been researched and tested."
        />
      ) : (
        <ul className="divide-y">
          {tweaks.map((t) => (
            <li key={t.id} className="flex items-center gap-3 py-3">
              <div className="min-w-0 flex-1">
                <div className="text-[13px] font-medium">{t.name}</div>
                <div className="text-[12px] text-muted-foreground">{t.description}</div>
              </div>
              <RiskBadge risk={t.risk} />
            </li>
          ))}
        </ul>
      )}
    </SectionCard>
  );
}
