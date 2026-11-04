import changelog from "../../CHANGELOG.md?raw";

export interface ReleaseNotes {
  version: string;
  sections: { title: string; items: string[] }[];
}

const clean = (text: string) => text.replace(/[`*]/g, "").trim();

/** The newest release in CHANGELOG.md. It's bundled, so Patch Notes needs no network. */
export function latestNotes(text = changelog): ReleaseNotes | null {
  const lines = text.split(/\r?\n/);
  const start = lines.findIndex((line) => line.startsWith("## "));
  if (start === -1) return null;

  const notes: ReleaseNotes = {
    version: clean(lines[start]!.slice(3).replace(/\(.*\)/, "")),
    sections: [],
  };
  for (const line of lines.slice(start + 1)) {
    if (line.startsWith("## ")) break;
    const section = notes.sections.at(-1);
    if (line.startsWith("### ")) notes.sections.push({ title: clean(line.slice(4)), items: [] });
    else if (line.startsWith("- ")) section?.items.push(clean(line.slice(2)));
    else if (line.startsWith("  ") && section?.items.length) {
      // A bullet wrapped onto the next line.
      section.items[section.items.length - 1] += ` ${clean(line)}`;
    }
  }
  return notes;
}
