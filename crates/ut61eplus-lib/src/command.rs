/// Remote commands (button presses) that can be sent to the meter.
///
/// Encoding: [0xAB, 0xCD, 0x03, cmd, (cmd+379)>>8, (cmd+379)&0xFF]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    /// Request a measurement reading.
    GetMeasurement = 0x5E,
    /// Toggle HOLD mode.
    Hold = 0x48,
    /// Toggle MIN/MAX mode.
    MinMax = 0x4D,
    /// Toggle REL (relative) mode.
    Rel = 0x52,
    /// Toggle range (auto/manual).
    Range = 0x41,
    /// Press SELECT button.
    Select = 0x53,
    /// Toggle backlight.
    Light = 0x4C,
}

impl Command {
    /// Encode this command into the 6-byte wire format.
    pub fn encode(self) -> [u8; 6] {
        let cmd = self as u8;
        let check = cmd as u16 + 379;
        [
            0xAB,
            0xCD,
            0x03,
            cmd,
            (check >> 8) as u8,
            (check & 0xFF) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_get_measurement() {
        // 0x5E + 379 = 94 + 379 = 473 = 0x01D9
        assert_eq!(
            Command::GetMeasurement.encode(),
            [0xAB, 0xCD, 0x03, 0x5E, 0x01, 0xD9]
        );
    }

    #[test]
    fn encode_hold() {
        // 0x48 + 379 = 72 + 379 = 451 = 0x01C3
        assert_eq!(
            Command::Hold.encode(),
            [0xAB, 0xCD, 0x03, 0x48, 0x01, 0xC3]
        );
    }
}
