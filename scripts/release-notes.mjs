#!/usr/bin/env node
// Release notes for a main-channel build: the commit subjects since the previous release tag.
// The Update Centre shows these as plain text before the user chooses to install.
//
// Usage: node scripts/release-notes.mjs <version> [previous-tag] > notes.md

import { execFileSync } from "node:child_process";
import { pathToFileURL } from "node:url";

// Commit trailers and bot noise that say nothing to someone deciding whether to update.
const SKIP = [/^Merge /, /^Co-Authored-By:/i, /^\s*$/];

export function notes(version, subjects, previousTag) {
  const items = subjects.map((s) => s.trim()).filter((s) => !SKIP.some((re) => re.test(s)));
  const since = previousTag
    ? `Changes since ${previousTag}:`
    : "First release on the main channel.";
  const body = items.length ? items.map((s) => `- ${s}`).join("\n") : "- Maintenance build.";
  return `## 2VX Tweaks ${version}\n\n${since}\n\n${body}\n`;
}

function main() {
  const [version, previousTag] = process.argv.slice(2);
  if (!version) throw new Error("usage: release-notes.mjs <version> [previous-tag]");
  const range = previousTag ? [`${previousTag}..HEAD`] : ["HEAD"];
  const log = execFileSync("git", ["log", "--no-merges", "--format=%s", ...range], {
    encoding: "utf8",
  });
  // Cap the list: the notes are read in a small panel, and the full history is on GitHub.
  const subjects = log.split("\n").slice(0, 40);
  process.stdout.write(notes(version, subjects, previousTag));
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();
