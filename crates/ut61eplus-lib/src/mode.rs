use crate::error::Error;
use std::fmt;

/// Measurement modes reported by the UT61E+.
/// Values are the nibble values after masking with 0x0F.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Mode {
    DcV = 0x00,
    AcV = 0x01,
    DcMv = 0x02,
    AcMv = 0x03,
    Ohm = 0x04,
    Capacitance = 0x05,
    Hz = 0x06,
    DutyCycle = 0x07,
    TempC = 0x08,
    TempF = 0x09,
    Diode = 0x0A,
    Continuity = 0x0B,
    DcUa = 0x0C,
    AcUa = 0x0D,
    DcMa = 0x0E,
    AcMa = 0x0F,
    DcA = 0x10,
    AcA = 0x11,
    AcDcV = 0x12,
    AcDcMv = 0x13,
    AcDcUa = 0x14,
    AcDcMa = 0x15,
    AcDcA = 0x16,
    LpfV = 0x17,
    LpfMv = 0x18,
    LpfUa = 0x19,
    LpfMa = 0x1A,
    LpfA = 0x1B,
    Ncv = 0x1C,
    PeakV = 0x1D,
    PeakMv = 0x1E,
}

impl Mode {
    pub fn from_byte(b: u8) -> Result<Self, Error> {
        match b {
            0x00 => Ok(Mode::DcV),
            0x01 => Ok(Mode::AcV),
            0x02 => Ok(Mode::DcMv),
            0x03 => Ok(Mode::AcMv),
            0x04 => Ok(Mode::Ohm),
            0x05 => Ok(Mode::Capacitance),
            0x06 => Ok(Mode::Hz),
            0x07 => Ok(Mode::DutyCycle),
            0x08 => Ok(Mode::TempC),
            0x09 => Ok(Mode::TempF),
            0x0A => Ok(Mode::Diode),
            0x0B => Ok(Mode::Continuity),
            0x0C => Ok(Mode::DcUa),
            0x0D => Ok(Mode::AcUa),
            0x0E => Ok(Mode::DcMa),
            0x0F => Ok(Mode::AcMa),
            0x10 => Ok(Mode::DcA),
            0x11 => Ok(Mode::AcA),
            0x12 => Ok(Mode::AcDcV),
            0x13 => Ok(Mode::AcDcMv),
            0x14 => Ok(Mode::AcDcUa),
            0x15 => Ok(Mode::AcDcMa),
            0x16 => Ok(Mode::AcDcA),
            0x17 => Ok(Mode::LpfV),
            0x18 => Ok(Mode::LpfMv),
            0x19 => Ok(Mode::LpfUa),
            0x1A => Ok(Mode::LpfMa),
            0x1B => Ok(Mode::LpfA),
            0x1C => Ok(Mode::Ncv),
            0x1D => Ok(Mode::PeakV),
            0x1E => Ok(Mode::PeakMv),
            _ => Err(Error::UnknownMode(b)),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Mode::DcV => "DC V",
            Mode::AcV => "AC V",
            Mode::DcMv => "DC mV",
            Mode::AcMv => "AC mV",
            Mode::Ohm => "Ω",
            Mode::Capacitance => "Capacitance",
            Mode::Hz => "Hz",
            Mode::DutyCycle => "Duty Cycle",
            Mode::TempC => "°C",
            Mode::TempF => "°F",
            Mode::Diode => "Diode",
            Mode::Continuity => "Continuity",
            Mode::DcUa => "DC µA",
            Mode::AcUa => "AC µA",
            Mode::DcMa => "DC mA",
            Mode::AcMa => "AC mA",
            Mode::DcA => "DC A",
            Mode::AcA => "AC A",
            Mode::AcDcV => "AC+DC V",
            Mode::AcDcMv => "AC+DC mV",
            Mode::AcDcUa => "AC+DC µA",
            Mode::AcDcMa => "AC+DC mA",
            Mode::AcDcA => "AC+DC A",
            Mode::LpfV => "LPF V",
            Mode::LpfMv => "LPF mV",
            Mode::LpfUa => "LPF µA",
            Mode::LpfMa => "LPF mA",
            Mode::LpfA => "LPF A",
            Mode::Ncv => "NCV",
            Mode::PeakV => "Peak V",
            Mode::PeakMv => "Peak mV",
        };
        write!(f, "{s}")
    }
}
