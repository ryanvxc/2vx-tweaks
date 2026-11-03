use tauri::State;

use crate::engine::{TweakMeta, TweakRegistry};

/// Lists registered tweaks. Read-only; returns an empty list in Phase 1.
#[tauri::command]
pub fn list_tweaks(registry: State<'_, TweakRegistry>) -> Vec<TweakMeta> {
    registry.list().cloned().collect()
}
