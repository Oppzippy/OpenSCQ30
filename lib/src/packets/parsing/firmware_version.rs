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
        map(take_str(5usize), |firmware_version| {
            FirmwareVersion(firmware_version.to_owned())
        }),
    )(input)
}
