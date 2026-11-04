//! Tauri command handlers — the only entry points reachable from the frontend.
//!
//! Rules: validate every argument, never accept paths/commands/registry keys as free
//! text, keep handlers thin (delegate to `system` / `engine`), and return `AppResult`.

pub mod account;
pub mod app;
pub mod engine;
pub mod system;
