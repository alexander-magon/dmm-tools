/// Tracks min/max/avg statistics for a series of measurements.
///
/// Used by both CLI and GUI to accumulate running statistics
/// over measurement values.
#[derive(Debug, Clone)]
pub struct RunningStats {
    pub min: Option<f64>,
    pub max: Option<f64>,
    sum: f64,
    pub count: u64,
}

impl RunningStats {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            sum: 0.0,
            count: 0,
        }
    }

    /// Record a new value, updating min/max/sum/count.
    pub fn push(&mut self, value: f64) {
        self.min = Some(self.min.map_or(value, |m: f64| m.min(value)));
        self.max = Some(self.max.map_or(value, |m: f64| m.max(value)));
        self.sum += value;
        self.count += 1;
    }

    /// Return the average, or `None` if no values have been pushed.
    pub fn avg(&self) -> Option<f64> {
        if self.count > 0 {
            Some(self.sum / self.count as f64)
        } else {
            None
        }
    }

    /// Reset all statistics to the initial empty state.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for RunningStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_stats() {
        let s = RunningStats::new();
        assert!(s.min.is_none());
        assert!(s.max.is_none());
        assert!(s.avg().is_none());
        assert_eq!(s.count, 0);
    }

    #[test]
    fn single_value() {
        let mut s = RunningStats::new();
        s.push(5.0);
        assert_eq!(s.min, Some(5.0));
        assert_eq!(s.max, Some(5.0));
        assert_eq!(s.avg(), Some(5.0));
        assert_eq!(s.count, 1);
    }

    #[test]
    fn multiple_values() {
        let mut s = RunningStats::new();
        s.push(1.0);
        s.push(3.0);
        s.push(5.0);
        assert_eq!(s.min, Some(1.0));
        assert_eq!(s.max, Some(5.0));
        assert_eq!(s.avg(), Some(3.0));
        assert_eq!(s.count, 3);
    }

    #[test]
    fn negative_values() {
        let mut s = RunningStats::new();
        s.push(-10.0);
        s.push(10.0);
        assert_eq!(s.min, Some(-10.0));
        assert_eq!(s.max, Some(10.0));
        assert_eq!(s.avg(), Some(0.0));
    }

    #[test]
    fn reset_clears_all() {
        let mut s = RunningStats::new();
        s.push(1.0);
        s.push(2.0);
        s.reset();
        assert!(s.min.is_none());
        assert!(s.max.is_none());
        assert!(s.avg().is_none());
        assert_eq!(s.count, 0);
    }
}
