/// Status flags parsed from payload bytes 11-13 (after & 0x0F masking).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatusFlags {
    pub hold: bool,
    pub rel: bool,
    pub auto_range: bool,
    pub min: bool,
    pub max: bool,
    pub low_battery: bool,
}

impl StatusFlags {
    /// Parse flags from the three flag bytes (already masked with & 0x0F).
    ///
    /// Byte 11 (flag1): bit0 = Hold, bit1 = REL
    /// Byte 12 (flag2): bit0 = Auto, bit1 = Min, bit2 = Max
    /// Byte 13 (flag3): bit0 = Low Battery
    pub fn parse(flag1: u8, flag2: u8, flag3: u8) -> Self {
        Self {
            hold: flag1 & 0x01 != 0,
            rel: flag1 & 0x02 != 0,
            auto_range: flag2 & 0x01 != 0,
            min: flag2 & 0x02 != 0,
            max: flag2 & 0x04 != 0,
            low_battery: flag3 & 0x01 != 0,
        }
    }
}

impl std::fmt::Display for StatusFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if self.hold {
            parts.push("HOLD");
        }
        if self.rel {
            parts.push("REL");
        }
        if self.auto_range {
            parts.push("AUTO");
        }
        if self.min {
            parts.push("MIN");
        }
        if self.max {
            parts.push("MAX");
        }
        if self.low_battery {
            parts.push("LOW BAT");
        }
        write!(f, "{}", parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_flags() {
        let flags = StatusFlags::parse(0x00, 0x00, 0x00);
        assert_eq!(flags, StatusFlags::default());
    }

    #[test]
    fn parse_hold_and_auto() {
        let flags = StatusFlags::parse(0x01, 0x01, 0x00);
        assert!(flags.hold);
        assert!(!flags.rel);
        assert!(flags.auto_range);
        assert!(!flags.min);
        assert!(!flags.max);
        assert!(!flags.low_battery);
    }

    #[test]
    fn parse_all_flags() {
        let flags = StatusFlags::parse(0x03, 0x07, 0x01);
        assert!(flags.hold);
        assert!(flags.rel);
        assert!(flags.auto_range);
        assert!(flags.min);
        assert!(flags.max);
        assert!(flags.low_battery);
    }

    #[test]
    fn display_flags() {
        let flags = StatusFlags::parse(0x01, 0x01, 0x00);
        assert_eq!(flags.to_string(), "HOLD AUTO");
    }
}
