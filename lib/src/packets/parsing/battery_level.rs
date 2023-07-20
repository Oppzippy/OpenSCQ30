use nom::{combinator::map, error::context, number::complete::le_u8};

use crate::packets::structures::BatteryLevel;

use super::ParseResult;

pub fn take_battery_level(input: &[u8]) -> ParseResult<BatteryLevel> {
    context("battery level", map(le_u8, BatteryLevel))(input)
}
