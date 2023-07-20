use nom::{combinator::map, error::context};

use crate::packets::structures::SerialNumber;

use super::{take_str, ParseResult};

pub fn take_serial_number(input: &[u8]) -> ParseResult<SerialNumber> {
    context(
        "serial number",
        map(take_str(16usize), |serial_number| {
            SerialNumber(serial_number.to_owned())
        }),
    )(input)
}
