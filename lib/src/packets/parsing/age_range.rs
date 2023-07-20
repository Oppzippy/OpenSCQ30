use nom::{combinator::map, error::context, number::complete::le_u8};

use crate::packets::structures::AgeRange;

use super::ParseResult;

pub fn take_age_range(input: &[u8]) -> ParseResult<AgeRange> {
    context("age range", map(le_u8, AgeRange))(input)
}
