//! Prints what the read-only commands report on this PC: hardware details, devices, the catalogue's
//! services and the Autoruns list. Reads only, and asks for no elevation.
//!
//! `cargo run --manifest-path src-tauri/Cargo.toml --example hardware_details`

use std::time::Instant;

use twovx_tweaks_lib::engine::startup;
use twovx_tweaks_lib::engine::{Hive, RegTarget, TweakRegistry};
use twovx_tweaks_lib::system::devices::{self, AdapterKind};
use twovx_tweaks_lib::system::{hardware_details, privilege, services};

fn timed<T>(label: &str, read: impl FnOnce() -> T) -> T {
    let started = Instant::now();
    let out = read();
    eprintln!("{label}: {} ms", started.elapsed().as_millis());
    out
}

fn main() -> Result<(), serde_json::Error> {
    eprintln!("elevated: {}", privilege::is_elevated());
    let details = timed("hardware details", hardware_details::collect);
    println!("{}", serde_json::to_string_pretty(&details)?);

    let report = timed("device report", devices::collect_report);
    println!("{}", serde_json::to_string_pretty(&report)?);

    let names = TweakRegistry::with_builtin_tweaks().service_names();
    eprintln!("catalogue services: {}", names.len());
    let states = timed("service states", || services::states(&names));
    println!("{}", serde_json::to_string_pretty(&states)?);

    let inventory = timed("autoruns", startup::inventory);
    println!("{}", serde_json::to_string_pretty(&inventory)?);

    // What a status pass pays: the store resolves the adapter for each adapter target.
    for kind in [AdapterKind::Ethernet, AdapterKind::Wifi] {
        let key = timed("adapter key", || devices::adapter_key(kind));
        eprintln!("{kind:?}: {key:?}");
    }
    let target = RegTarget::new(Hive::Adapter, "Ethernet", "*EEE");
    eprintln!("adapter target: {target}");
    Ok(())
}
