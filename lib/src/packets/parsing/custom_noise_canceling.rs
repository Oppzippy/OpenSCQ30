use nom::{combinator::map, error::context, number::complete::le_u8};

use crate::packets::structures::CustomNoiseCanceling;

use super::ParseResult;

pub fn take_custom_noise_canceling(input: &[u8]) -> ParseResult<CustomNoiseCanceling> {
    context(
        "custom noise canceling",
        map(le_u8, |custom_noise_canceling_level| {
            CustomNoiseCanceling::new(custom_noise_canceling_level)
        }),
    )(input)
}
