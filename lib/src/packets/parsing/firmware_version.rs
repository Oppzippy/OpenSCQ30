use nom::{combinator::map, error::context};

use crate::packets::structures::FirmwareVersion;

use super::{take_str, ParseResult};

pub fn take_firmware_version(input: &[u8]) -> ParseResult<FirmwareVersion> {
    context(
        "firmware version",
        map(take_str(5usize), |firmware_version| {
            FirmwareVersion(firmware_version.to_owned())
        }),
    )(input)
}
