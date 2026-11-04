import { describe, expect, it } from "vitest";
import { latestNotes } from "./changelog";

const TEXT = [
  "# Changelog",
  "",
  "Intro that isn't a release.",
  "",
  "## 1.2.0 (unreleased)",
  "",
  "### Home",
  "- One line.",
  "- Two lines that",
  "  wrap onto the next.",
  "",
  "### `Safety`",
  "- Uses **bold** and `code`.",
  "",
  "## 1.1.0",
  "",
  "### Older",
  "- Not shown.",
].join("\r\n");

describe("latestNotes", () => {
  it("reads only the newest release", () => {
    expect(latestNotes(TEXT)).toEqual({
      version: "1.2.0",
      sections: [
        { title: "Home", items: ["One line.", "Two lines that wrap onto the next."] },
        { title: "Safety", items: ["Uses bold and code."] },
      ],
    });
  });

  it("has nothing to show without a release", () => {
    expect(latestNotes("# Changelog")).toBeNull();
  });

  it("reads the bundled changelog", () => {
    const notes = latestNotes();
    expect(notes?.version).toMatch(/^\d+\.\d+\.\d+$/);
    expect(notes?.sections.length).toBeGreaterThan(0);
  });
});
