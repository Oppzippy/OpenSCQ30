use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
};
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::standard::packets::parsing::{take_str, ParseResult};

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

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<FirmwareVersion, E> {
        context(
            "firmware version",
            map(take_str(5usize), |version| {
                // format is xx.xx, so skip the decimal
                let major_str = &version[0..2];
                let minor_str = &version[3..5];

                let major = major_str.parse::<u8>().unwrap_or_default();
                let minor = minor_str.parse::<u8>().unwrap_or_default();

                FirmwareVersion::new(major, minor)
            }),
        )(input)
    }
}

impl Display for FirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:02}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

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

    #[test]
    fn test_parse_version_number() {
        let version_str = "12.34";
        let firmware_version = FirmwareVersion::take::<VerboseError<&[u8]>>(version_str.as_bytes())
            .unwrap()
            .1;
        assert_eq!(FirmwareVersion::new(12, 34), firmware_version);
    }
}
