pub mod ut61e_plus;

use super::mode::Mode;

/// Information about a specific measurement range.
#[derive(Debug, Clone)]
pub struct RangeInfo {
    pub label: &'static str,
    pub unit: &'static str,
    pub overload_pos: f64,
    pub overload_neg: f64,
}

/// Trait for device-specific range/unit lookup tables.
pub trait DeviceTable: Send {
    fn range_info(&self, mode: Mode, range: u8) -> Option<&RangeInfo>;
    fn model_name(&self) -> &'static str;
}
