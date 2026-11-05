/**
 * Direct dependencies and the licence each one declares. Read from `node_modules/<pkg>/package.json`
 * and the crates' own `Cargo.toml`, at the versions in `package.json` and `src-tauri/Cargo.toml`.
 * Build and test tools are left out: none of them ships in the app.
 */
export interface Licence {
  name: string;
  licence: string;
}

export const WEB_LICENCES: Licence[] = [
  { name: "@fontsource-variable/geist", licence: "OFL-1.1" },
  { name: "@fontsource/poppins", licence: "OFL-1.1" },
  { name: "@tauri-apps/api", licence: "Apache-2.0 OR MIT" },
  { name: "@tauri-apps/plugin-process", licence: "MIT OR Apache-2.0" },
  { name: "@tauri-apps/plugin-updater", licence: "MIT OR Apache-2.0" },
  { name: "class-variance-authority", licence: "Apache-2.0" },
  { name: "clsx", licence: "MIT" },
  { name: "cn", licence: "MIT" },
  { name: "lucide-react", licence: "ISC" },
  { name: "radix-ui", licence: "MIT" },
  { name: "react", licence: "MIT" },
  { name: "react-dom", licence: "MIT" },
  { name: "react-router", licence: "MIT" },
  { name: "recharts", licence: "MIT" },
  { name: "sonner", licence: "MIT" },
  { name: "tailwind-merge", licence: "MIT" },
  { name: "tw-animate-css", licence: "MIT" },
  { name: "zustand", licence: "MIT" },
];

export const RUST_LICENCES: Licence[] = [
  { name: "getrandom", licence: "MIT OR Apache-2.0" },
  { name: "rusqlite", licence: "MIT" },
  { name: "serde", licence: "MIT OR Apache-2.0" },
  { name: "serde_json", licence: "MIT OR Apache-2.0" },
  { name: "sysinfo", licence: "MIT" },
  { name: "tauri", licence: "Apache-2.0 OR MIT" },
  { name: "tauri-plugin-process", licence: "Apache-2.0 OR MIT" },
  { name: "tauri-plugin-updater", licence: "Apache-2.0 OR MIT" },
  { name: "thiserror", licence: "MIT OR Apache-2.0" },
  { name: "tracing", licence: "MIT" },
  { name: "tracing-appender", licence: "MIT" },
  { name: "tracing-subscriber", licence: "MIT" },
  { name: "windows", licence: "MIT OR Apache-2.0" },
  { name: "windows-collections", licence: "MIT OR Apache-2.0" },
  { name: "windows-registry", licence: "MIT OR Apache-2.0" },
  { name: "windows-result", licence: "MIT OR Apache-2.0" },
];
