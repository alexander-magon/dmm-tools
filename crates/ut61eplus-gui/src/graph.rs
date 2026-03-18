use eframe::egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;

/// Maximum number of points to keep in the history buffer.
const MAX_POINTS: usize = 10_000;

/// Real-time scrolling graph of measurement values.
pub struct Graph {
    /// (elapsed_seconds, value) pairs.
    history: VecDeque<(f64, f64)>,
    /// The mode string when points were recorded. Cleared on mode change.
    current_mode: Option<String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(MAX_POINTS),
            current_mode: None,
        }
    }

    /// Push a new data point. If mode changed, clear history.
    pub fn push(&mut self, elapsed_secs: f64, value: f64, mode: &str) {
        if self.current_mode.as_deref() != Some(mode) {
            self.history.clear();
            self.current_mode = Some(mode.to_string());
        }
        if self.history.len() >= MAX_POINTS {
            self.history.pop_front();
        }
        self.history.push_back((elapsed_secs, value));
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.current_mode = None;
    }

    /// Render the plot in the given UI region.
    pub fn show(&self, ui: &mut Ui, time_window_secs: f64) {
        let points: PlotPoints = self
            .history
            .iter()
            .map(|&(t, v)| [t, v])
            .collect();

        let line = Line::new(points);

        // Determine x-axis bounds: latest time - window .. latest time
        let x_max = self
            .history
            .back()
            .map(|&(t, _)| t)
            .unwrap_or(0.0);
        let x_min = (x_max - time_window_secs).max(0.0);

        Plot::new("measurement_plot")
            .height(ui.available_height().max(80.0))
            .include_x(x_min)
            .include_x(x_max)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .x_axis_label("time (s)")
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph_is_empty() {
        let g = Graph::new();
        assert!(g.is_empty());
        assert_eq!(g.len(), 0);
    }

    #[test]
    fn push_adds_point() {
        let mut g = Graph::new();
        g.push(0.0, 5.0, "DC V");
        assert_eq!(g.len(), 1);
        assert!(!g.is_empty());
    }

    #[test]
    fn mode_change_clears_history() {
        let mut g = Graph::new();
        g.push(0.0, 5.0, "DC V");
        g.push(1.0, 5.1, "DC V");
        assert_eq!(g.len(), 2);

        g.push(2.0, 100.0, "Ohm");
        assert_eq!(g.len(), 1); // cleared + new point
    }

    #[test]
    fn max_points_evicts_oldest() {
        let mut g = Graph::new();
        for i in 0..MAX_POINTS + 100 {
            g.push(i as f64, i as f64, "DC V");
        }
        assert_eq!(g.len(), MAX_POINTS);
    }

    #[test]
    fn clear_resets_everything() {
        let mut g = Graph::new();
        g.push(0.0, 5.0, "DC V");
        g.clear();
        assert!(g.is_empty());
        assert_eq!(g.current_mode, None);
    }
}
