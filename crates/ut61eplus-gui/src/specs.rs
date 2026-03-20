use eframe::egui::{self, RichText, Ui};
use ut61eplus_lib::protocol::ut61eplus::tables::{ModeSpecInfo, SpecInfo};

/// Full specs panel for the wide (side panel) layout.
pub fn show_specs(
    ui: &mut Ui,
    spec: &SpecInfo,
    mode_spec: Option<&ModeSpecInfo>,
    manual_url: Option<&str>,
    scale: f32,
) {
    let main_font = 12.0 * scale;
    let sub_font = 11.0 * scale;
    let weak = ui.visuals().weak_text_color();

    ui.label(
        RichText::new("Specifications")
            .strong()
            .font(egui::FontId::proportional(sub_font)),
    );

    // Resolution
    ui.label(
        RichText::new(format!("Resolution  {}", spec.resolution))
            .font(egui::FontId::proportional(main_font)),
    );

    // Accuracy
    if spec.accuracy.len() == 1 {
        ui.label(
            RichText::new(format!("Accuracy  \u{00B1}({})", spec.accuracy[0].accuracy))
                .font(egui::FontId::proportional(main_font)),
        );
    } else {
        ui.label(RichText::new("Accuracy").font(egui::FontId::proportional(main_font)));
        for band in spec.accuracy {
            let freq = band.freq_range.unwrap_or("???");
            ui.label(
                RichText::new(format!("  {freq}  \u{00B1}({})", band.accuracy))
                    .font(egui::FontId::proportional(sub_font))
                    .color(weak),
            );
        }
    }

    // Input impedance and notes
    if let Some(ms) = mode_spec {
        if let Some(z) = ms.input_impedance {
            ui.label(
                RichText::new(format!("Input Z  {z}")).font(egui::FontId::proportional(main_font)),
            );
        }
        for note in ms.notes {
            ui.label(
                RichText::new(*note)
                    .font(egui::FontId::proportional(sub_font))
                    .color(weak),
            );
        }
    }

    // Manual link
    if let Some(url) = manual_url {
        ui.hyperlink_to(
            RichText::new("Manual \u{2197}")
                .font(egui::FontId::proportional(sub_font))
                .color(weak),
            url,
        );
    }
}

/// Compact single-line specs for the narrow layout.
pub fn show_specs_compact(
    ui: &mut Ui,
    spec: &SpecInfo,
    mode_spec: Option<&ModeSpecInfo>,
    manual_url: Option<&str>,
) {
    let weak = ui.visuals().weak_text_color();
    let sub_font = 11.0;

    // Build a compact string: "Res: 0.01mV  Acc: ±(0.1%+5)  ~10MΩ"
    let acc_str = if spec.accuracy.len() == 1 {
        format!("\u{00B1}({})", spec.accuracy[0].accuracy)
    } else {
        // Show first band only in compact mode
        let band = &spec.accuracy[0];
        let freq = band.freq_range.unwrap_or("");
        format!("\u{00B1}({}) {freq}", band.accuracy)
    };

    let mut parts = vec![
        format!("Res: {}", spec.resolution),
        format!("Acc: {acc_str}"),
    ];
    if let Some(ms) = mode_spec
        && let Some(z) = ms.input_impedance
    {
        parts.push(z.to_string());
    }

    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(parts.join("  "))
                .font(egui::FontId::proportional(sub_font))
                .color(weak),
        );
        if let Some(url) = manual_url {
            ui.hyperlink_to(
                RichText::new("Manual \u{2197}")
                    .font(egui::FontId::proportional(sub_font))
                    .color(weak),
                url,
            );
        }
    });
}

/// Inline pipe-separated specs for big meter mode.
pub fn show_specs_inline(
    ui: &mut Ui,
    spec: &SpecInfo,
    mode_spec: Option<&ModeSpecInfo>,
    manual_url: Option<&str>,
    scale: f32,
) {
    let font_size = 12.0 * scale;
    let weak = ui.visuals().weak_text_color();

    let acc_str = if spec.accuracy.len() == 1 {
        format!("\u{00B1}({})", spec.accuracy[0].accuracy)
    } else {
        let band = &spec.accuracy[0];
        let freq = band.freq_range.unwrap_or("");
        format!("\u{00B1}({}) {freq}", band.accuracy)
    };

    let mut parts = vec![
        format!("Resolution {}", spec.resolution),
        format!("Accuracy {acc_str}"),
    ];
    if let Some(ms) = mode_spec
        && let Some(z) = ms.input_impedance
    {
        parts.push(z.to_string());
    }

    ui.horizontal_wrapped(|ui| {
        ui.label(
            RichText::new(parts.join("  |  "))
                .font(egui::FontId::proportional(font_size))
                .color(weak),
        );
        if let Some(url) = manual_url {
            ui.hyperlink_to(
                RichText::new("Manual \u{2197}")
                    .font(egui::FontId::proportional(font_size))
                    .color(weak),
                url,
            );
        }
    });
}

/// Render only the manual link (when no spec data is available but a URL exists).
pub fn show_manual_only(ui: &mut Ui, url: &str, scale: f32) {
    let font_size = 11.0 * scale;
    let weak = ui.visuals().weak_text_color();
    ui.hyperlink_to(
        RichText::new("Manual \u{2197}")
            .font(egui::FontId::proportional(font_size))
            .color(weak),
        url,
    );
}
