import { Link, Navigate, useLocation } from "react-router";
import {
  Bell,
  BookOpenCheck,
  Database,
  FolderOpen,
  Globe,
  Info,
  LayoutList,
  ListChecks,
  Move,
  PanelLeft,
  Palette,
  RefreshCw,
  ScrollText,
  ShieldCheck,
  ShieldUser,
} from "lucide-react";
import { toast } from "sonner";
import { pageDescription, SETTINGS_TABS, type SettingsTab } from "@/app/navigation";
import { StatusPill } from "@/components/common/notice";
import { PageTabs } from "@/components/common/page-tabs";
import { SettingsGroup, SettingsRow } from "@/components/common/settings-row";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { useAppInfo, useEngineStatus } from "@/hooks/use-system";
import { useTabParam } from "@/hooks/use-tab-param";
import { isDesktop } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import { UpdateCentre } from "./update-centre";

// Sections of the old single-page Settings, so `/settings#updates` links keep working.
const OLD_SECTIONS: Record<string, SettingsTab> = {
  general: "general",
  behaviour: "general",
  appearance: "appearance",
  notifications: "updates",
  updates: "updates",
  security: "security",
  about: "about",
};

export function SettingsPage() {
  const [tab, setTab] = useTabParam(SETTINGS_TABS, "general");
  const { hash } = useLocation();
  const moved = OLD_SECTIONS[hash.slice(1)];
  if (moved) return <Navigate to={`/settings?tab=${moved}`} replace />;

  return (
    <>
      <PageHeader title="Settings" description={pageDescription("/settings")} />
      <div className="flex flex-col gap-4">
        <PageTabs label="Sections" tabs={SETTINGS_TABS} value={tab} onChange={setTab} />
        <div className="max-w-3xl">
          {tab === "general" && <GeneralTab />}
          {tab === "appearance" && <AppearanceTab />}
          {tab === "updates" && <UpdatesTab />}
          {tab === "security" && <SecurityTab />}
          {tab === "about" && <AboutTab />}
        </div>
      </div>
    </>
  );
}

function GeneralTab() {
  const { data: app } = useAppInfo();
  const ui = useUiStore();

  const copyLogPath = async () => {
    if (!app) return;
    try {
      await navigator.clipboard.writeText(app.logDir);
      toast.success("Log folder path copied");
    } catch {
      toast.error("Couldn't access the clipboard");
    }
  };

  return (
    <SettingsGroup>
      <SettingsRow
        icon={ShieldUser}
        title="Privileges"
        description={
          app?.isElevated
            ? "Running as administrator. 2VX only needs this for machine-wide changes."
            : "Running as a standard user. Windows asks for permission only when a change needs it."
        }
        control={
          app && (
            <StatusPill tone={app.isElevated ? "warning" : "success"}>
              {app.isElevated ? "Administrator" : "Standard user"}
            </StatusPill>
          )
        }
      />
      <SettingsRow
        icon={ListChecks}
        title="Review before applying"
        description="Every change shows the exact values first and is applied only after you confirm. This can't be turned off."
        control={<StatusPill tone="success">Always on</StatusPill>}
      />
      <SettingsRow
        icon={LayoutList}
        title="Quickstart on Home"
        htmlFor="pref-quickstart"
        control={
          <Switch
            id="pref-quickstart"
            checked={ui.showQuickstart}
            onCheckedChange={(v) => ui.set({ showQuickstart: v })}
          />
        }
      />
      <SettingsRow
        icon={FolderOpen}
        title="Log folder"
        description={
          <code data-selectable className="font-mono text-[11.5px] break-all">
            {app?.logDir ?? "…"}
          </code>
        }
        control={
          <Button size="sm" variant="outline" onClick={copyLogPath} disabled={!app}>
            Copy path
          </Button>
        }
      />
    </SettingsGroup>
  );
}

function AppearanceTab() {
  const ui = useUiStore();
  return (
    <SettingsGroup>
      <SettingsRow
        icon={Palette}
        title="Theme"
        description="A light theme isn't available yet."
        control={<StatusPill tone="neutral">Dark</StatusPill>}
      />
      <SettingsRow
        icon={PanelLeft}
        title="Compact sidebar"
        htmlFor="pref-sidebar"
        control={
          <Switch
            id="pref-sidebar"
            checked={ui.sidebarCollapsed}
            onCheckedChange={ui.toggleSidebar}
          />
        }
      />
      <SettingsRow
        icon={Move}
        title="Reduce motion"
        description="2VX also follows the Windows animation setting."
        htmlFor="pref-motion"
        control={
          <Switch
            id="pref-motion"
            checked={ui.reduceMotion}
            onCheckedChange={(v) => ui.set({ reduceMotion: v })}
          />
        }
      />
    </SettingsGroup>
  );
}

function UpdatesTab() {
  const { data: app } = useAppInfo();
  const ui = useUiStore();
  return (
    <SettingsGroup description="2VX never downloads or installs an update without you choosing to.">
      <UpdateCentre currentVersion={app?.version} />
      <SettingsRow
        icon={RefreshCw}
        title="Check for updates automatically"
        description="Once each time 2VX starts. It sends no data about this PC beyond a normal web request."
        htmlFor="pref-auto-update"
        control={
          <Switch
            id="pref-auto-update"
            checked={ui.autoCheckUpdates}
            onCheckedChange={(v) => ui.set({ autoCheckUpdates: v })}
          />
        }
      />
      <SettingsRow
        icon={Bell}
        title="Update notice on Home"
        htmlFor="pref-update-notice"
        control={
          <Switch
            id="pref-update-notice"
            checked={ui.showUpdateNotice}
            onCheckedChange={(v) => ui.set({ showUpdateNotice: v, dismissedUpdateVersion: null })}
          />
        }
      />
    </SettingsGroup>
  );
}

function SecurityTab() {
  return (
    <SettingsGroup>
      <EngineRows />
      <SettingsRow
        icon={Globe}
        title="Network use"
        description="The only network request 2VX makes is the update check (and a download when you choose one). No telemetry, no analytics."
      />
      <SettingsRow
        icon={ShieldCheck}
        title="Safety rules"
        description="2VX never turns off Defender, the firewall, Windows Update, UAC or SmartScreen, and every tweak cites the documentation it is based on."
      />
    </SettingsGroup>
  );
}

function AboutTab() {
  const { data: app } = useAppInfo();
  return (
    <SettingsGroup>
      <SettingsRow
        icon={Info}
        title="Version"
        description={
          app ? (
            <>
              2VX Tweaks {app.version}
              {app.debugBuild && " (development build)"}
            </>
          ) : (
            <Skeleton className="h-3.5 w-24" />
          )
        }
      />
      <SettingsRow
        icon={BookOpenCheck}
        title="Open-source licences"
        description="Icons: Lucide (ISC). Fonts: Inter and Manrope (SIL Open Font License 1.1), bundled with the app. Full list in docs/ICON_AND_FONT_LICENCES.md."
      />
      <SettingsRow
        icon={ScrollText}
        title="Change history"
        control={
          <Button asChild size="sm" variant="outline">
            <Link to="/restore">Open Restore Centre</Link>
          </Button>
        }
      />
    </SettingsGroup>
  );
}

function EngineRows() {
  const { data: status, error } = useEngineStatus();
  return (
    <>
      <SettingsRow
        icon={Database}
        title="Change journal"
        description={
          !isDesktop
            ? "Not available in browser preview."
            : error
              ? error.message
              : status?.journalAvailable
                ? `Records every value before 2VX changes it. ${status.tweakCount} tweaks available.`
                : (status?.journalError ?? "Checking…")
        }
        control={
          status &&
          (!isDesktop ? (
            <StatusPill tone="info">Preview</StatusPill>
          ) : status.journalAvailable ? (
            <StatusPill tone="success">Available</StatusPill>
          ) : (
            <StatusPill tone="destructive">Unavailable: changes disabled</StatusPill>
          ))
        }
      />
      {status && status.interruptedSnapshots > 0 && (
        <SettingsRow
          icon={Database}
          title="Interrupted batches"
          description="2VX stopped while applying. Review them in the Restore Centre to restore the old values."
          control={
            <Button asChild size="sm" variant="outline">
              <Link to="/restore">Review {status.interruptedSnapshots}</Link>
            </Button>
          }
        />
      )}
    </>
  );
}
