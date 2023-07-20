use nom::{
    combinator::map_res,
    error::{context, ErrorKind},
    number::complete::le_u8,
};

use crate::packets::structures::NoiseCancelingMode;

use super::ParseResult;

pub fn take_noise_canceling_mode(input: &[u8]) -> ParseResult<NoiseCancelingMode> {
    context(
        "noise canceling mode",
        map_res(le_u8, |noise_canceling_mode| {
            NoiseCancelingMode::from_id(noise_canceling_mode)
                .ok_or_else(|| nom::Err::Failure(nom::error::Error::new(input, ErrorKind::Alt)))
        }),
    )(input)
}
