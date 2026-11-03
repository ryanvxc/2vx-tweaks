import { useEffect } from "react";
import { Link, useLocation } from "react-router";
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
import { StatusPill } from "@/components/common/notice";
import { SettingsGroup, SettingsRow } from "@/components/common/settings-row";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { useAppInfo, useEngineStatus } from "@/hooks/use-system";
import { isDesktop } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import { UpdateCentre } from "./update-centre";

const SECTIONS = [
  ["general", "General"],
  ["appearance", "Appearance"],
  ["notifications", "Notifications"],
  ["updates", "Updates"],
  ["behaviour", "Behaviour"],
  ["security", "Security"],
  ["about", "About"],
] as const;

export function SettingsPage() {
  const { data: app } = useAppInfo();
  const ui = useUiStore();
  const { hash } = useLocation();

  // Deep links such as the Home update notice (#updates).
  useEffect(() => {
    if (hash) document.getElementById(hash.slice(1))?.scrollIntoView({ block: "start" });
  }, [hash]);

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
    <>
      <PageHeader title="Settings" />
      <div className="grid grid-cols-[160px_minmax(0,1fr)] gap-6">
        <nav aria-label="Settings sections" className="sticky top-0 flex h-fit flex-col gap-px">
          {SECTIONS.map(([id, label]) => (
            <button
              key={id}
              type="button"
              onClick={() => document.getElementById(id)?.scrollIntoView({ behavior: "smooth" })}
              className="rounded-md px-2.5 py-1.5 text-left text-[13px] text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
            >
              {label}
            </button>
          ))}
        </nav>

        <div className="flex max-w-3xl flex-col gap-6">
          <SettingsGroup id="general" title="General">
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

          <SettingsGroup id="appearance" title="Appearance">
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

          <SettingsGroup id="notifications" title="Notifications">
            <SettingsRow
              icon={Bell}
              title="Update notice on Home"
              htmlFor="pref-update-notice"
              control={
                <Switch
                  id="pref-update-notice"
                  checked={ui.showUpdateNotice}
                  onCheckedChange={(v) =>
                    ui.set({ showUpdateNotice: v, dismissedUpdateVersion: null })
                  }
                />
              }
            />
          </SettingsGroup>

          <SettingsGroup
            id="updates"
            title="Updates"
            description="2VX never downloads or installs an update without you choosing to."
          >
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
          </SettingsGroup>

          <SettingsGroup id="behaviour" title="Behaviour">
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
          </SettingsGroup>

          <SettingsGroup id="security" title="Security">
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

          <SettingsGroup id="about" title="About">
            <SettingsRow
              icon={BookOpenCheck}
              title="Open-source licences"
              description="Icons: Lucide (ISC). Font: Inter (SIL Open Font License 1.1), bundled with the app. Full list in docs/ICON_AND_FONT_LICENCES.md."
            />
            <SettingsRow
              icon={ScrollText}
              title="Change history"
              control={
                <Button asChild size="sm" variant="outline">
                  <Link to="/history">Open History</Link>
                </Button>
              }
            />
          </SettingsGroup>
        </div>
      </div>
    </>
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
          description="2VX stopped while applying. Review them in History to restore the old values."
          control={
            <Button asChild size="sm" variant="outline">
              <Link to="/history">Review {status.interruptedSnapshots}</Link>
            </Button>
          }
        />
      )}
    </>
  );
}
