use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::packets::structures::IsBatteryCharging;

use super::ParseResult;

pub fn take_is_battery_charging<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<IsBatteryCharging, E> {
    context(
        "is battery charging",
        map(le_u8, |value| -> IsBatteryCharging {
            if value == 1 {
                IsBatteryCharging::Yes
            } else {
                IsBatteryCharging::No
            }
        }),
    )(input)
}
