use crate::error::Error;
use std::fmt;

/// Measurement modes reported by the UT61E+.
///
/// Values verified against real device captures and cross-checked with
/// ljakob/unit_ut61eplus (Python) and mwuertinger/ut61ep (Go).
///
/// The mode byte does NOT have a 0x30 high nibble — use the raw value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Mode {
    AcV = 0x00,
    AcMv = 0x01,
    DcV = 0x02,
    DcMv = 0x03,
    Hz = 0x04,
    DutyCycle = 0x05,
    Ohm = 0x06,
    Continuity = 0x07,
    Diode = 0x08,
    Capacitance = 0x09,
    TempC = 0x0A,
    TempF = 0x0B,
    DcUa = 0x0C,
    AcUa = 0x0D,
    DcMa = 0x0E,
    AcMa = 0x0F,
    DcA = 0x10,
    AcA = 0x11,
    Hfe = 0x12,
    Live = 0x13,
    Ncv = 0x14,
    LozV = 0x15,
    AcDcA = 0x16,
    AcDcDcA = 0x17,
    LpfV = 0x18,
    AcDcV = 0x19,
    LpfMv = 0x1A,
    AcDcMv = 0x1B,
    LpfA = 0x1C,
    AcDcA2 = 0x1D,
    Inrush = 0x1E,
}

impl Mode {
    pub fn from_byte(b: u8) -> Result<Self, Error> {
        match b {
            0x00 => Ok(Mode::AcV),
            0x01 => Ok(Mode::AcMv),
            0x02 => Ok(Mode::DcV),
            0x03 => Ok(Mode::DcMv),
            0x04 => Ok(Mode::Hz),
            0x05 => Ok(Mode::DutyCycle),
            0x06 => Ok(Mode::Ohm),
            0x07 => Ok(Mode::Continuity),
            0x08 => Ok(Mode::Diode),
            0x09 => Ok(Mode::Capacitance),
            0x0A => Ok(Mode::TempC),
            0x0B => Ok(Mode::TempF),
            0x0C => Ok(Mode::DcUa),
            0x0D => Ok(Mode::AcUa),
            0x0E => Ok(Mode::DcMa),
            0x0F => Ok(Mode::AcMa),
            0x10 => Ok(Mode::DcA),
            0x11 => Ok(Mode::AcA),
            0x12 => Ok(Mode::Hfe),
            0x13 => Ok(Mode::Live),
            0x14 => Ok(Mode::Ncv),
            0x15 => Ok(Mode::LozV),
            0x16 => Ok(Mode::AcDcA),
            0x17 => Ok(Mode::AcDcDcA),
            0x18 => Ok(Mode::LpfV),
            0x19 => Ok(Mode::AcDcV),
            0x1A => Ok(Mode::LpfMv),
            0x1B => Ok(Mode::AcDcMv),
            0x1C => Ok(Mode::LpfA),
            0x1D => Ok(Mode::AcDcA2),
            0x1E => Ok(Mode::Inrush),
            _ => Err(Error::UnknownMode(b)),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Mode::AcV => "AC V",
            Mode::AcMv => "AC mV",
            Mode::DcV => "DC V",
            Mode::DcMv => "DC mV",
            Mode::Hz => "Hz",
            Mode::DutyCycle => "Duty %",
            Mode::Ohm => "Ω",
            Mode::Continuity => "Continuity",
            Mode::Diode => "Diode",
            Mode::Capacitance => "Capacitance",
            Mode::TempC => "°C",
            Mode::TempF => "°F",
            Mode::DcUa => "DC µA",
            Mode::AcUa => "AC µA",
            Mode::DcMa => "DC mA",
            Mode::AcMa => "AC mA",
            Mode::DcA => "DC A",
            Mode::AcA => "AC A",
            Mode::Hfe => "hFE",
            Mode::Live => "Live",
            Mode::Ncv => "NCV",
            Mode::LozV => "LoZ V",
            Mode::AcDcA => "AC+DC A",
            Mode::AcDcDcA => "AC+DC/DC A",
            Mode::LpfV => "LPF V",
            Mode::AcDcV => "AC+DC V",
            Mode::LpfMv => "LPF mV",
            Mode::AcDcMv => "AC+DC mV",
            Mode::LpfA => "LPF A",
            Mode::AcDcA2 => "AC+DC A",
            Mode::Inrush => "Inrush",
        };
        write!(f, "{s}")
    }
}
