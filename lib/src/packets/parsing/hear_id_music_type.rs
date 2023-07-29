use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::packets::structures::HearIdMusicType;

use super::ParseResult;

pub fn take_hear_id_music_type<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<HearIdMusicType, E> {
    context("hear id music type", map(le_u8, HearIdMusicType))(input)
}
