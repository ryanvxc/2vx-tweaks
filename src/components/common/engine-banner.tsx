import { Link } from "react-router";
import { Button } from "@/components/ui/button";
import { useEngineStatus } from "@/hooks/use-system";
import { isDesktop } from "@/services/backend";
import { Notice } from "./notice";

/**
 * Engine health above every screen that can change the system. Silent when all is well;
 * a broken journal or an interrupted batch must never be invisible.
 */
export function EngineBanner({
  showHistoryLink = true,
  previewNotice = true,
}: {
  showHistoryLink?: boolean;
  /** Home has no tweaks to caveat, and the title bar already says it's a preview. */
  previewNotice?: boolean;
}) {
  const { data: status, error } = useEngineStatus();

  if (!isDesktop) {
    if (!previewNotice) return null;
    return (
      <Notice tone="info" title="Browser preview">
        The optimisation engine runs only in the desktop app. Tweaks shown here are samples, and
        nothing in the preview can change this PC.
      </Notice>
    );
  }
  if (error) {
    return (
      <Notice tone="destructive" title="Couldn't read the engine status">
        {error.message}
      </Notice>
    );
  }
  if (!status) return null;
  if (!status.journalAvailable) {
    return (
      <Notice tone="destructive" title="Changes are disabled">
        The change journal can't be opened, so 2VX can't back up values before changing them.{" "}
        {status.journalError} The log folder in Settings has details.
      </Notice>
    );
  }
  if (status.interruptedSnapshots > 0) {
    const n = status.interruptedSnapshots;
    return (
      <Notice
        tone="warning"
        title={`${n} batch${n === 1 ? "" : "es"} of changes didn't finish`}
        action={
          showHistoryLink && (
            <Button asChild size="xs" variant="outline">
              <Link to="/restore">Open Restore Centre</Link>
            </Button>
          )
        }
      >
        The app closed while applying changes. Some values may have been changed; review and revert
        them in the Restore Centre.
      </Notice>
    );
  }
  return null;
}
