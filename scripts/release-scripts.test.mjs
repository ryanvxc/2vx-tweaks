// @vitest-environment node
import { describe, expect, it } from "vitest";
import { checkFeed } from "./check-update-feed.mjs";
import { compare, latestTag, nextVersion } from "./next-version.mjs";
import { notes } from "./release-notes.mjs";

describe("next-version", () => {
  it("starts from package.json when there is no tag", () => {
    expect(nextVersion("0.2.0", null)).toBe("0.2.0");
  });

  it("bumps the patch of the latest tag", () => {
    expect(nextVersion("0.2.0", "v0.2.0")).toBe("0.2.1");
    expect(nextVersion("0.2.0", "v0.2.7")).toBe("0.2.8");
  });

  it("takes a manual minor bump from package.json", () => {
    expect(nextVersion("0.3.0", "v0.2.7")).toBe("0.3.0");
  });

  it("orders versions numerically and ignores other tags", () => {
    expect(latestTag(["v0.2.9", "v0.2.10", "updater-main", "v1.0.0-beta", ""])).toBe("v0.2.10");
    expect(compare("0.10.0", "0.9.9")).toBeGreaterThan(0);
  });

  it("rejects versions that aren't plain semver", () => {
    expect(() => compare("0.2", "0.2.0")).toThrow();
  });
});

describe("release-notes", () => {
  it("lists subjects without merges or trailers", () => {
    const text = notes("0.2.1", ["Fix the thing", "Merge branch 'x'", ""], "v0.2.0");
    expect(text).toContain("Changes since v0.2.0:");
    expect(text).toContain("- Fix the thing");
    expect(text).not.toContain("Merge");
  });
});

describe("check-update-feed", () => {
  const feed = {
    version: "0.2.1",
    platforms: {
      "windows-x86_64": {
        signature: "x".repeat(200),
        url: "https://github.com/o/r/releases/download/v0.2.1/setup.exe",
      },
    },
  };

  it("accepts a signed HTTPS feed that is newer", () => {
    expect(checkFeed(feed, "0.2.1", { version: "0.2.0" })).toEqual([]);
  });

  it("refuses unsigned, plain HTTP, wrong-version and downgrade feeds", () => {
    const bad = {
      version: "0.2.1",
      platforms: { "windows-x86_64": { signature: "", url: "http://example.com/a.exe" } },
    };
    expect(checkFeed(bad, "0.2.1", null)).toEqual([
      "windows-x86_64: unsigned",
      "windows-x86_64: URL is not HTTPS",
    ]);
    expect(checkFeed(feed, "0.2.2", null)).toEqual(["version is 0.2.1, expected 0.2.2"]);
    expect(checkFeed(feed, "0.2.1", { version: "0.2.1" })).toEqual([
      "0.2.1 is not newer than the published 0.2.1",
    ]);
  });
});
