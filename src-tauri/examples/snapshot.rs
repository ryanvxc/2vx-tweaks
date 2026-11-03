//! Prints the system snapshot as JSON — handy for checking what the UI receives.
//! Run with `cargo run --example snapshot`.

fn main() {
    match twovx_tweaks_lib::system::hardware::collect_snapshot() {
        Ok(snapshot) => println!("{}", serde_json::to_string_pretty(&snapshot).unwrap()),
        Err(e) => {
            eprintln!("snapshot failed: {e}");
            std::process::exit(1);
        }
    }
}
