use nom::{combinator::map, error::context, number::complete::le_u8};

use crate::packets::structures::IsBatteryCharging;

use super::ParseResult;

pub fn take_is_battery_charging(input: &[u8]) -> ParseResult<IsBatteryCharging> {
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
