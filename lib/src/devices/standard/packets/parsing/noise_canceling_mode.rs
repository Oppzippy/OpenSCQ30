use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::structures::NoiseCancelingMode;

use super::ParseResult;

pub fn take_noise_canceling_mode<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<NoiseCancelingMode, E> {
    context(
        "noise canceling mode",
        map(le_u8, |noise_canceling_mode| {
            NoiseCancelingMode::from_id(noise_canceling_mode).unwrap_or_default()
        }),
    )(input)
}
