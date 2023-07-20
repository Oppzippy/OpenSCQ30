use nom::{
    combinator::map_res,
    error::{context, ErrorKind},
    number::complete::le_u8,
};

use crate::packets::structures::TransparencyMode;

use super::ParseResult;

pub fn take_transparency_mode(input: &[u8]) -> ParseResult<TransparencyMode> {
    context(
        "transparency mode",
        map_res(le_u8, |transparency_mode| {
            TransparencyMode::from_id(transparency_mode)
                .ok_or_else(|| nom::Err::Failure(nom::error::Error::new(input, ErrorKind::Alt)))
        }),
    )(input)
}
