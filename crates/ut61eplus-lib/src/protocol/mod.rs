pub mod framing;
pub mod ut61eplus;

use crate::error::Result;
use crate::measurement::Measurement;
use crate::transport::Transport;

/// Protocol stability level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    /// Verified against real hardware.
    Verified,
    /// Based on reverse engineering, not yet verified against real hardware.
    Experimental,
}

/// Static profile information about a device.
pub struct DeviceProfile {
    pub family_name: &'static str,
    pub model_name: &'static str,
    pub stability: Stability,
    pub supported_commands: &'static [&'static str],
}

/// Device family selector for opening a connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceFamily {
    /// UT61E+, UT61B+, UT61D+, UT161B, UT161D, UT161E
    Ut61EPlus,
    /// UT8803 / UT8803E bench multimeter
    Ut8803,
    /// UT171A / UT171B / UT171C
    Ut171,
    /// UT181A
    Ut181a,
}

impl std::fmt::Display for DeviceFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceFamily::Ut61EPlus => write!(f, "ut61eplus"),
            DeviceFamily::Ut8803 => write!(f, "ut8803"),
            DeviceFamily::Ut171 => write!(f, "ut171"),
            DeviceFamily::Ut181a => write!(f, "ut181a"),
        }
    }
}

impl std::str::FromStr for DeviceFamily {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ut61eplus" | "ut61e+" | "ut61e" | "ut61bplus" | "ut61b+" | "ut61b" | "ut61dplus"
            | "ut61d+" | "ut61d" | "ut161b" | "ut161d" | "ut161e" | "ut161" => {
                Ok(DeviceFamily::Ut61EPlus)
            }
            "ut8803" | "ut8803e" => Ok(DeviceFamily::Ut8803),
            "ut171" | "ut171a" | "ut171b" | "ut171c" => Ok(DeviceFamily::Ut171),
            "ut181a" | "ut181" => Ok(DeviceFamily::Ut181a),
            _ => Err(format!("unknown device family: {s}")),
        }
    }
}

/// Each device family implements this trait. Object-safe.
///
/// The Protocol owns its internal state (rx buffer, streaming trigger state, etc).
/// I/O is performed through the Transport reference passed to each method.
pub trait Protocol: Send {
    /// Post-transport initialization (e.g. send streaming trigger, purge FIFOs).
    fn init(&mut self, transport: &dyn Transport) -> Result<()>;

    /// Get the next measurement.
    /// For polled protocols: sends request + reads response.
    /// For streaming protocols: reads the next frame from the stream.
    fn request_measurement(&mut self, transport: &dyn Transport) -> Result<Measurement>;

    /// Send a named command ("hold", "range", "auto", etc.).
    /// Returns UnsupportedCommand for unknown commands.
    fn send_command(&mut self, transport: &dyn Transport, command: &str) -> Result<()>;

    /// Request device name. Returns None if the protocol doesn't support it.
    fn get_name(&mut self, transport: &dyn Transport) -> Result<Option<String>>;

    /// Static device profile information.
    fn profile(&self) -> &DeviceProfile;
}
