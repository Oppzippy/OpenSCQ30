use nom::{
    combinator::map,
    error::{ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::{packets::parsing::ParseResult, structures::AmbientSoundModeCycle};

pub fn take_ambient_sound_mode_cycle<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<AmbientSoundModeCycle, E> {
    map(le_u8, AmbientSoundModeCycle::from)(input)
}
