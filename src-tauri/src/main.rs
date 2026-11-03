// Prevents an additional console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use twovx_tweaks_lib::engine::broker;

fn main() {
    // The elevated helper is this same executable, started by the app with UAC (see broker.rs).
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some(broker::BROKER_FLAG) {
        std::process::exit(broker::run_elevated(&args[2..]));
    }
    twovx_tweaks_lib::run();
}
