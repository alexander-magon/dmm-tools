mod app;
mod display;
mod graph;
mod recording;
mod settings;
mod specs;
mod theme;

/// Placeholder string for missing/unavailable data values in the UI.
pub(crate) const NO_DATA: &str = "---";

/// Version string for the app (shown in top bar, right side).
pub fn version_label() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let hash = env!("GIT_HASH");
    if version.contains("-dev") {
        format!("v{version} ({hash})")
    } else {
        format!("v{version}")
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();

    let device_override = parse_device_arg();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dmm-tools",
        options,
        Box::new(move |cc| Ok(Box::new(app::App::new(cc, device_override)))),
    )
}

/// Parse `--device <id>` or `--device=<id>` from argv without pulling in clap.
fn parse_device_arg() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--device" {
            return iter.next().cloned();
        }
        if let Some(val) = arg.strip_prefix("--device=") {
            return Some(val.to_string());
        }
    }
    None
}