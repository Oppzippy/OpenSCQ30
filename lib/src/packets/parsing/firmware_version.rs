use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
};

use crate::packets::structures::FirmwareVersion;

use super::{take_str, ParseResult};

pub fn take_firmware_version<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
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

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::packets::parsing::take_firmware_version;

    use super::FirmwareVersion;

    #[test]
    fn test_parse_version_number() {
        let version_str = "12.34";
        let firmware_version = take_firmware_version::<VerboseError<&[u8]>>(version_str.as_bytes())
            .unwrap()
            .1;
        assert_eq!(FirmwareVersion::new(12, 34), firmware_version);
    }
}
