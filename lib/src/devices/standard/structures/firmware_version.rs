use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FirmwareVersion {
    major: u8,
    minor: u8,
}

impl FirmwareVersion {
    pub const fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    pub const fn major(&self) -> u8 {
        self.major
    }

    pub const fn minor(&self) -> u8 {
        self.minor
    }

    pub const fn number(&self) -> u16 {
        (self.major as u16) * 100 + (self.minor as u16)
    }
}

impl Display for FirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:02}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::FirmwareVersion;

    #[test]
    fn test_combined_version_number() {
        let firmware_version = FirmwareVersion::new(98, 76);
        assert_eq!(9876, firmware_version.number());
    }

    #[test]
    fn test_to_string() {
        let firmware_version = FirmwareVersion::new(12, 34);
        assert_eq!("12.34", firmware_version.to_string());
    }

    #[test]
    fn test_to_string_for_numbers_starting_with_zero() {
        let firmware_version = FirmwareVersion::new(01, 02);
        assert_eq!("01.02", firmware_version.to_string());
    }

    #[test]
    fn test_to_string_for_numbers_ending_with_zero() {
        let firmware_version = FirmwareVersion::new(10, 20);
        assert_eq!("10.20", firmware_version.to_string());
    }

    #[test]
    fn test_major_has_priority_in_ordering() {
        let bigger = FirmwareVersion::new(01, 00);
        let smaller = FirmwareVersion::new(00, 20);
        assert!(smaller < bigger)
    }
}
