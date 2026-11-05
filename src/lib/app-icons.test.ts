import { AppWindow } from "lucide-react";
import { describe, expect, it } from "vitest";
import { APP_ICON, appIcon } from "./app-icons";

const SOURCES = import.meta.glob<string>("/src-tauri/src/engine/apps.rs", {
  query: "?raw",
  import: "default",
  eager: true,
});
const source = Object.values(SOURCES).join("");
// Each row of the table starts `app(` with the ID on the next line.
const ids = [...source.matchAll(/\bapp\(\s*"([a-z0-9-]+)",/g)].map((m) => m[1]!);

describe("app icons", () => {
  it("covers every app in the backend's table", () => {
    expect(ids.length).toBeGreaterThan(30);
    expect(ids.filter((id) => !APP_ICON[id])).toEqual([]);
  });

  it("falls back for an unknown app", () => {
    expect(appIcon("something-new")).toBe(AppWindow);
  });
});
