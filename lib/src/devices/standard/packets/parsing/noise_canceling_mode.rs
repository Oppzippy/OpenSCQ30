use nom::{
    combinator::map_opt,
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
        map_opt(le_u8, |noise_canceling_mode| {
            NoiseCancelingMode::from_id(noise_canceling_mode)
        }),
    )(input)
}
