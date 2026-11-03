import {
  BatteryCharging,
  ChevronRight,
  Gamepad2,
  History,
  Rocket,
  type LucideIcon,
} from "lucide-react";
import { RiskBadge } from "@/components/common/risk-badge";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState } from "@/components/common/states";
import type { RiskLevel } from "@/types/engine";

// SAMPLE DATA: placeholder recommendations to design the layout around. These are
// not computed from this PC. Real recommendations come from the engine (Phase 2+).
const SAMPLE_RECOMMENDATIONS: {
  icon: LucideIcon;
  title: string;
  detail: string;
  risk: RiskLevel;
}[] = [
  {
    icon: Gamepad2,
    title: "Turn on Game Mode",
    detail: "Lets Windows prioritise the foreground game.",
    risk: "low",
  },
  {
    icon: BatteryCharging,
    title: "Use a performance power plan while gaming",
    detail: "Switches back automatically when the game closes.",
    risk: "low",
  },
  {
    icon: Rocket,
    title: "Review 6 high-impact startup apps",
    detail: "These delay sign-in and stay resident in memory.",
    risk: "low",
  },
];

export function Recommendations() {
  return (
    <SectionCard title="Recommendations" description="Suggested next steps" source="sample">
      <ul className="-mx-2 flex flex-col">
        {SAMPLE_RECOMMENDATIONS.map(({ icon: Icon, title, detail, risk }) => (
          <li
            key={title}
            className="flex items-center gap-3 rounded-md px-2 py-2.5 transition-colors hover:bg-accent/50"
          >
            <div className="flex size-8 shrink-0 items-center justify-center rounded-md bg-muted">
              <Icon className="size-4 text-muted-foreground" />
            </div>
            <div className="min-w-0 flex-1">
              <div className="text-[13px] leading-snug font-medium">{title}</div>
              <div className="text-[12px] leading-snug text-muted-foreground">{detail}</div>
            </div>
            <RiskBadge risk={risk} />
            <ChevronRight className="size-4 text-muted-foreground/50" />
          </li>
        ))}
      </ul>
    </SectionCard>
  );
}

export function RecentActivity() {
  return (
    <SectionCard title="Recently applied" description="Changes made by 2VX" source="planned">
      <EmptyState
        icon={History}
        title="No changes yet"
        description="Every change 2VX makes will be listed here with a one-click undo, once the backup engine lands."
        className="py-6"
      />
    </SectionCard>
  );
}
