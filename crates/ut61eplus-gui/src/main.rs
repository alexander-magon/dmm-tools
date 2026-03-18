mod app;
mod display;
mod graph;
mod recording;
mod settings;
mod stats;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "UT61E+ Multimeter",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
