import type { UpdaterState } from "@/hooks/use-updater";

/** One line on where updating stands, from the same state the Update Centre shows. */
export function updateStatus(state: UpdaterState): string {
  switch (state.kind) {
    case "unsupported":
      return "Updates aren't available in this build.";
    case "upToDate":
      return "2VX is up to date.";
    case "available":
      return `2VX ${state.info.version} is available.`;
    case "downloading":
    case "installing":
      return `Getting 2VX ${state.info.version}.`;
    case "ready":
      return `2VX ${state.info.version} is ready to install.`;
    case "restartRequired":
      return "Restart 2VX to finish updating.";
    case "error":
      return "Couldn't check for updates.";
    default:
      return "Checking for updates.";
  }
}
