//! Prints the tweak catalogue as JSON lines, one tweak each, for audits and docs.
//! Run with `cargo run --example catalogue`.

use twovx_tweaks_lib::engine::TweakRegistry;

fn main() {
    let registry = TweakRegistry::with_builtin_tweaks();
    for tweak in registry.iter() {
        let settings: Vec<String> = tweak
            .settings()
            .iter()
            .map(|s| format!("{} = {:?}", s.target, s.value))
            .collect();
        let line = serde_json::json!({ "meta": tweak.meta(), "settings": settings });
        println!("{line}");
    }
}
