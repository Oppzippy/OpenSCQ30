use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, map_parser},
    error::{ContextError, ParseError, context},
    sequence::{pair, separated_pair},
};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DualFirmwareVersion {
    LeftOnly(FirmwareVersion),
    RightOnly(FirmwareVersion),
    Both {
        left: FirmwareVersion,
        right: FirmwareVersion,
    },
}

impl Default for DualFirmwareVersion {
    fn default() -> Self {
        Self::Both {
            left: FirmwareVersion::default(),
            right: FirmwareVersion::default(),
        }
    }
}

impl DualFirmwareVersion {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        let firmware_version_some = || map(FirmwareVersion::take, Some);
        let firmware_version_none = || map(tag::<&[u8], _, _>(&[0u8; 5]), |_| None);
        context(
            "dual firmware version",
            map(
                // Don't parse case where both sides are none
                alt((
                    pair(firmware_version_some(), firmware_version_some()),
                    pair(firmware_version_some(), firmware_version_none()),
                    pair(firmware_version_none(), firmware_version_some()),
                )),
                |firmware_versions| match firmware_versions {
                    (Some(left), Some(right)) => Self::Both { left, right },
                    (Some(left), None) => Self::LeftOnly(left),
                    (None, Some(right)) => Self::RightOnly(right),
                    (None, None) => unreachable!("parsing will fail in this case"),
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        match self {
            Self::LeftOnly(firmware_version) => firmware_version.bytes().into_iter().chain([0; 5]),
            Self::RightOnly(firmware_version) => {
                [0u8; 5].into_iter().chain(firmware_version.bytes())
            }
            Self::Both { left, right } => left.bytes().into_iter().chain(right.bytes()),
        }
    }

    pub fn left(&self) -> Option<FirmwareVersion> {
        match self {
            Self::LeftOnly(firmware_version) => Some(*firmware_version),
            Self::RightOnly(_) => None,
            Self::Both { left, right: _ } => Some(*left),
        }
    }

    pub fn right(&self) -> Option<FirmwareVersion> {
        match self {
            Self::LeftOnly(_) => None,
            Self::RightOnly(firmware_version) => Some(*firmware_version),
            Self::Both { left: _, right } => Some(*right),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FirmwareVersion {
    major: u8,
    minor: u8,
}

impl FirmwareVersion {
    pub const fn new(major: u8, minor: u8) -> Self {
        debug_assert!(major < 100, "major version must fit within in two digits");
        debug_assert!(minor < 100, "minor version must fit within in two digits");
        Self { major, minor }
    }

    pub const fn major(&self) -> u8 {
        self.major
    }

    pub const fn minor(&self) -> u8 {
        self.minor
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "firmware version",
            map(
                separated_pair(
                    map_parser(take(2usize), all_consuming(nom::character::complete::u8)),
                    tag("."),
                    map_parser(take(2usize), all_consuming(nom::character::complete::u8)),
                ),
                |(major, minor)| Self::new(major, minor),
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 5] {
        self
            .to_string()
            .as_bytes()
            .try_into()
            .expect(
                "ToString left pads major and minor with 0s to 2 digits, and they're numbers, so single byte characters. Plus the ., a total of 5 bytes is the only possibility."
            )
    }
}

impl Display for FirmwareVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:02}", self.major(), self.minor())
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use super::FirmwareVersion;

    #[test]
    fn test_to_string() {
        let firmware_version = FirmwareVersion::new(12, 34);
        assert_eq!("12.34", firmware_version.to_string());
    }

    #[test]
    fn test_to_string_for_numbers_starting_with_zero() {
        let firmware_version = FirmwareVersion::new(1, 2);
        assert_eq!("01.02", firmware_version.to_string());
    }

    #[test]
    fn test_to_string_for_numbers_ending_with_zero() {
        let firmware_version = FirmwareVersion::new(10, 20);
        assert_eq!("10.20", firmware_version.to_string());
    }

    #[test]
    fn test_major_has_priority_in_ordering() {
        let bigger = FirmwareVersion::new(1, 0);
        let smaller = FirmwareVersion::new(0, 20);
        assert!(smaller < bigger);
    }

    #[test]
    fn test_parse_version_number() {
        let version_str = "12.34";
        let firmware_version = FirmwareVersion::take::<VerboseError<&[u8]>>(version_str.as_bytes())
            .unwrap()
            .1;
        assert_eq!(FirmwareVersion::new(12, 34), firmware_version);
    }

    #[test]
    fn test_parsing_fails_with_non_numeric() {
        let version_str = "1a.23";
        let result = FirmwareVersion::take::<VerboseError<&[u8]>>(version_str.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn test_parsing_fails_with_incorrect_separator() {
        let version_str = "12_23";
        let result = FirmwareVersion::take::<VerboseError<&[u8]>>(version_str.as_bytes());
        assert!(result.is_err());
    }
}
