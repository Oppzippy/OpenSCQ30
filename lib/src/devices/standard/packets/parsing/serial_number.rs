use nom::{
    combinator::map_opt,
    error::{context, ContextError, ParseError},
};

use crate::devices::standard::structures::SerialNumber;

use super::{take_str, ParseResult};

pub fn take_serial_number<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<SerialNumber, E> {
    context(
        "serial number",
        map_opt(take_str(16usize), |s| {
            // Serial number is 4 digit model number followed by mac address with pairs in reverse order.
            // ie. device model 1234 with max address 55:66:77:88:99:AA would be 1234AA9988776655
            // The mac address is hex, and the model number is base 10 digits only, so we can use that
            // to try to avoid parsing things that aren't serial numbers as one.
            if s.chars().all(|c| c.is_ascii_hexdigit()) {
                Some(SerialNumber::from(s))
            } else {
                None
            }
        }),
    )(input)
}
