//! List all connected CP2110 (UT61E+) devices.

fn main() {
    env_logger::init();

    match ut61eplus_lib::list_devices() {
        Ok(devices) => {
            if devices.is_empty() {
                eprintln!("No UT61E+ devices found.");
                eprintln!("Check USB connection and udev rules (see udev/99-cp2110-unit.rules).");
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
