use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::packets::structures::BatteryLevel;

use super::ParseResult;

pub fn take_battery_level<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<BatteryLevel, E> {
    context("battery level", map(le_u8, BatteryLevel))(input)
}
