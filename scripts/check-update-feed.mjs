#!/usr/bin/env node
// Validates a freshly built updater feed (latest.json) before it is published to the
// rolling `updater-main` release that installed copies poll.
//
// Refuses a feed that is unsigned, not HTTPS, for the wrong version, or not newer than the
// feed currently published (so a re-run or an old branch can't downgrade users).
//
// Usage: node scripts/check-update-feed.mjs <new latest.json> <expected version> [current latest.json]

import { existsSync, readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { compare } from "./next-version.mjs";

export function checkFeed(feed, expectedVersion, current) {
  const problems = [];
  const version = String(feed.version ?? "").replace(/^v/, "");
  if (version !== expectedVersion)
    problems.push(`version is ${version}, expected ${expectedVersion}`);

  const platforms = Object.entries(feed.platforms ?? {});
  if (!platforms.some(([name]) => name.startsWith("windows-x86_64"))) {
    problems.push("no windows-x86_64 platform");
  }
  for (const [name, entry] of platforms) {
    if (!entry?.signature || String(entry.signature).length < 40)
      problems.push(`${name}: unsigned`);
    if (!String(entry?.url ?? "").startsWith("https://"))
      problems.push(`${name}: URL is not HTTPS`);
  }

  if (current?.version) {
    const published = String(current.version).replace(/^v/, "");
    if (compare(version, published) <= 0) {
      problems.push(`${version} is not newer than the published ${published}`);
    }
  }
  return problems;
}

function main() {
  const [feedPath, expected, currentPath] = process.argv.slice(2);
  if (!feedPath || !expected) {
    throw new Error("usage: check-update-feed.mjs <latest.json> <version> [current.json]");
  }
  const feed = JSON.parse(readFileSync(feedPath, "utf8"));
  const current =
    currentPath && existsSync(currentPath) ? JSON.parse(readFileSync(currentPath, "utf8")) : null;
  const problems = checkFeed(feed, expected, current);
  if (problems.length) {
    for (const p of problems) console.error(`::error::update feed: ${p}`);
    process.exit(1);
  }
  console.log(`update feed for ${expected} is signed, HTTPS and newer than what is published`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();
