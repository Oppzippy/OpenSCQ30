use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::structures::CustomNoiseCanceling;

use super::ParseResult;

pub fn take_custom_noise_canceling<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<CustomNoiseCanceling, E> {
    context(
        "custom noise canceling",
        map(le_u8, |custom_noise_canceling_level| {
            CustomNoiseCanceling::new(custom_noise_canceling_level)
        }),
    )(input)
}
