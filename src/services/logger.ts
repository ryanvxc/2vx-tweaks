import { backend, type LogLevel } from "./backend";

/**
 * Frontend logger. Forwards to the backend log file (sanitised and length-limited
 * there) and mirrors to the console in development. Logging must never throw.
 */
function log(level: LogLevel, message: string, source?: string) {
  if (import.meta.env.DEV) {
    const fn = level === "debug" ? console.debug : console[level];
    fn(`[${source ?? "ui"}] ${message}`);
  }
  backend.logEvent(level, message, source).catch(() => {
    /* swallow: a logging failure must not cascade */
  });
}

export const logger = {
  debug: (message: string, source?: string) => log("debug", message, source),
  info: (message: string, source?: string) => log("info", message, source),
  warn: (message: string, source?: string) => log("warn", message, source),
  error: (message: string, source?: string) => log("error", message, source),
};

let installed = false;

/** Routes uncaught errors and unhandled rejections to the backend log. */
export function installGlobalErrorLogging() {
  if (installed) return;
  installed = true;
  window.addEventListener("error", (e) => {
    logger.error(`${e.message} @ ${e.filename}:${e.lineno}`, "window.onerror");
  });
  window.addEventListener("unhandledrejection", (e) => {
    const reason =
      e.reason instanceof Error ? (e.reason.stack ?? e.reason.message) : String(e.reason);
    logger.error(reason, "unhandledrejection");
  });
}
