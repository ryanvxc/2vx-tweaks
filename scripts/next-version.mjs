#!/usr/bin/env node
// Picks the next main-channel version and writes it into the three version files.
//
// Versions only ever go up: the result is the higher of
//   - the version in package.json (a person bumps this for a minor or major release), and
//   - the latest `v*` release tag with its patch number increased.
// Run in CI after a full-history checkout (`fetch-depth: 0`). The files are changed in the
// runner's working copy only; the release tag, not a commit, records the version.
//
// Usage: node scripts/next-version.mjs [--dry-run]
// Prints `version`, `tag` and `previous_tag` as `key=value` lines and appends them to
// $GITHUB_OUTPUT when it is set.

import { execFileSync } from "node:child_process";
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const SEMVER = /^(\d+)\.(\d+)\.(\d+)$/;

export function parse(version) {
  const m = SEMVER.exec(version);
  if (!m) throw new Error(`not a plain major.minor.patch version: ${version}`);
  return m.slice(1, 4).map(Number);
}

export function compare(a, b) {
  const [x, y] = [parse(a), parse(b)];
  for (let i = 0; i < 3; i++) if (x[i] !== y[i]) return x[i] - y[i];
  return 0;
}

export function bumpPatch(version) {
  const [major, minor, patch] = parse(version);
  return `${major}.${minor}.${patch + 1}`;
}

/** The highest `vX.Y.Z` tag, or null. Tags that aren't plain versions are ignored. */
export function latestTag(tags) {
  const versions = tags
    .map((t) => t.trim())
    .filter((t) => /^v\d+\.\d+\.\d+$/.test(t))
    .map((t) => t.slice(1))
    .sort(compare);
  return versions.length ? `v${versions.at(-1)}` : null;
}

export function nextVersion(packageVersion, previousTag) {
  if (!previousTag) return packageVersion;
  const bumped = bumpPatch(previousTag.slice(1));
  return compare(packageVersion, bumped) > 0 ? packageVersion : bumped;
}

function main() {
  const dryRun = process.argv.includes("--dry-run");
  const pkg = JSON.parse(readFileSync("package.json", "utf8"));
  const tags = execFileSync("git", ["tag", "--list", "v*"], { encoding: "utf8" }).split("\n");
  const previous = latestTag(tags);
  const version = nextVersion(pkg.version, previous);

  if (previous && compare(version, previous.slice(1)) <= 0) {
    throw new Error(`refusing to release ${version}: not newer than ${previous}`);
  }

  if (!dryRun) {
    pkg.version = version;
    writeFileSync("package.json", `${JSON.stringify(pkg, null, 2)}\n`);

    const confPath = "src-tauri/tauri.conf.json";
    const conf = JSON.parse(readFileSync(confPath, "utf8"));
    conf.version = version;
    writeFileSync(confPath, `${JSON.stringify(conf, null, 2)}\n`);

    // Only the [package] version: the first `version = ` line in Cargo.toml.
    const cargoPath = "src-tauri/Cargo.toml";
    const cargo = readFileSync(cargoPath, "utf8");
    const updated = cargo.replace(/^version = "[^"]*"/m, `version = "${version}"`);
    if (updated === cargo && !cargo.includes(`version = "${version}"`)) {
      throw new Error("couldn't find the package version in Cargo.toml");
    }
    writeFileSync(cargoPath, updated);
  }

  const out = { version, tag: `v${version}`, previous_tag: previous ?? "" };
  const lines = Object.entries(out).map(([k, v]) => `${k}=${v}`);
  console.log(lines.join("\n"));
  if (process.env.GITHUB_OUTPUT) appendFileSync(process.env.GITHUB_OUTPUT, `${lines.join("\n")}\n`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();
