use bitflags::bitflags;
use nom::{
    combinator::map,
    error::{ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::standard::packets::parsing::ParseResult;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct AmbientSoundModeCycle: u8 {
        const NOISE_CANCELING_MODE = 1 << 0;
        const TRANSPARENCY_MODE    = 1 << 1;
        const NORMAL_MODE          = 1 << 2;
    }
}

pub fn take_ambient_sound_mode_cycle<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<AmbientSoundModeCycle, E> {
    map(le_u8, AmbientSoundModeCycle::from_bits_truncate)(input)
}
