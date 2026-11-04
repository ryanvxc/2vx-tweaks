import { CLEANER_TABS } from "@/app/navigation";
import { PageTabs } from "@/components/common/page-tabs";
import { RecommendedAction } from "@/components/common/recommended-action";
import { PageHero } from "@/components/layout/page-hero";
import { useTabParam } from "@/hooks/use-tab-param";
import { AutorunsTab } from "./autoruns-tab";
import { ServicesTab } from "./services-tab";
import { SystemCleanerTab } from "./system-cleaner-tab";
import { UninstallTab } from "./uninstall-tab";

export function CleanerPage() {
  const [tab, setTab] = useTabParam(CLEANER_TABS, "system");
  return (
    <div className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]">
      <PageHero
        title="PC Cleaner"
        subtitle="Free up space and choose what stays on this PC."
        action={<RecommendedAction otherwise={{ to: "/tweaks", label: "Browse tweaks" }} />}
      />
      <PageTabs label="PC Cleaner" tabs={CLEANER_TABS} value={tab} onChange={setTab} filled />
      {tab === "system" && <SystemCleanerTab />}
      {tab === "services" && <ServicesTab />}
      {tab === "uninstall" && <UninstallTab />}
      {tab === "autoruns" && <AutorunsTab />}
    </div>
  );
}
