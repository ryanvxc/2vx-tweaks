import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "@/app/app";
import { installGlobalErrorLogging, logger } from "@/services/logger";
import "@/styles/globals.css";

installGlobalErrorLogging();
logger.info("frontend started", "main");

// Block the WebView's default context menu outside text inputs; it exposes
// "Reload" and "Inspect" which make no sense in a desktop app.
if (!import.meta.env.DEV) {
  window.addEventListener("contextmenu", (e) => {
    const target = e.target as HTMLElement;
    if (!target.closest("input, textarea, [data-selectable]")) e.preventDefault();
  });
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
