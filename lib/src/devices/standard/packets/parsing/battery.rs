use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::devices::standard::structures::{DualBattery, SingleBattery};

use super::{take_battery_level, take_is_battery_charging, ParseResult};

pub fn take_dual_battery<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<DualBattery, E> {
    context(
        "dual battery",
        map(
            tuple((
                take_battery_level,
                take_battery_level,
                take_is_battery_charging,
                take_is_battery_charging,
            )),
            |(left_level, right_level, is_left_charging, is_right_charging)| DualBattery {
                left: SingleBattery {
                    level: left_level,
                    is_charging: is_left_charging,
                },
                right: SingleBattery {
                    level: right_level,
                    is_charging: is_right_charging,
                },
            },
        ),
    )(input)
}

pub fn take_single_battery<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<SingleBattery, E> {
    context(
        "battery",
        map(
            tuple((take_battery_level, take_is_battery_charging)),
            |(level, is_charging)| SingleBattery { level, is_charging },
        ),
    )(input)
}
