//! List all connected CP2110 (UT61E+) devices.

fn main() {
    env_logger::init();

    match ut61eplus_lib::list_devices() {
        Ok(devices) => {
            if devices.is_empty() {
                eprintln!("No UT61E+ devices found.");
                eprintln!("Check USB connection.");
                #[cfg(target_os = "linux")]
                eprintln!("Ensure udev rules are installed (see udev/99-cp2110-unit.rules).");
                #[cfg(target_os = "macos")]
                eprintln!(
                    "On macOS, the CP2110 should be recognized automatically (no driver needed)."
                );
                std::process::exit(1);
            }
            for (i, dev) in devices.iter().enumerate() {
                println!("[{i}] {dev}");
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
