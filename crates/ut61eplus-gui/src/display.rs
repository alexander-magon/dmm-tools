use eframe::egui::{Color32, FontId, RichText, Ui};
use ut61eplus_lib::measurement::{MeasuredValue, Measurement};

/// Render the large primary reading display.
pub fn show_reading(ui: &mut Ui, measurement: Option<&Measurement>) {
    match measurement {
        Some(m) => {
            // Primary value + unit
            let (value_text, value_color) = match &m.value {
                MeasuredValue::Normal(v) => {
                    (format!("{v}"), ui.visuals().text_color())
                }
                MeasuredValue::Overload => {
                    ("OL".to_string(), Color32::from_rgb(220, 50, 50))
                }
                MeasuredValue::NcvLevel(l) => {
                    (format!("NCV {l}"), ui.visuals().text_color())
                }
            };

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(&value_text)
                        .font(FontId::monospace(36.0))
                        .color(value_color),
                );
                ui.label(
                    RichText::new(&m.unit)
                        .font(FontId::proportional(20.0))
                        .color(ui.visuals().text_color()),
                );
            });

            // Mode, range, flags
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.label(
                    RichText::new(m.mode.to_string())
                        .color(ui.visuals().weak_text_color()),
                );
                if !m.range_label.is_empty() {
                    ui.label(
                        RichText::new(&m.range_label)
                            .color(ui.visuals().weak_text_color()),
                    );
                }
                show_flags(ui, m);
            });
        }
        None => {
            ui.label(
                RichText::new("---")
                    .font(FontId::monospace(36.0))
                    .color(ui.visuals().weak_text_color()),
            );
            ui.label(
                RichText::new("No reading")
                    .color(ui.visuals().weak_text_color()),
            );
        }
    }
}

/// Render the reading as a compact single line (for narrow layout).
pub fn show_reading_compact(ui: &mut Ui, measurement: Option<&Measurement>) {
    match measurement {
        Some(m) => {
            let value_text = match &m.value {
                MeasuredValue::Normal(v) => format!("{v}"),
                MeasuredValue::Overload => "OL".to_string(),
                MeasuredValue::NcvLevel(l) => format!("NCV {l}"),
            };

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(&value_text)
                        .font(FontId::monospace(28.0)),
                );
                ui.label(
                    RichText::new(&m.unit)
                        .font(FontId::proportional(16.0)),
                );
                ui.separator();
                ui.label(
                    RichText::new(m.mode.to_string())
                        .color(ui.visuals().weak_text_color())
                        .small(),
                );
                show_flags(ui, m);
            });
        }
        None => {
            ui.label(
                RichText::new("--- No reading")
                    .font(FontId::monospace(28.0))
                    .color(ui.visuals().weak_text_color()),
            );
        }
    }
}

fn show_flags(ui: &mut Ui, m: &Measurement) {
    let badge = |ui: &mut Ui, label: &str, color: Color32| {
        let text = RichText::new(label).small().color(color);
        ui.label(text);
    };

    let accent = Color32::from_rgb(100, 180, 255);
    let warning = Color32::from_rgb(230, 160, 40);

    if m.flags.auto_range {
        badge(ui, "AUTO", accent);
    }
    if m.flags.hold {
        badge(ui, "HOLD", accent);
    }
    if m.flags.rel {
        badge(ui, "REL", accent);
    }
    if m.flags.min {
        badge(ui, "MIN", accent);
    }
    if m.flags.max {
        badge(ui, "MAX", accent);
    }
    if m.flags.low_battery {
        badge(ui, "LOW BAT", warning);
    }
}
