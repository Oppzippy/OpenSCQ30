mod sound_modes;

use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
pub use sound_modes::*;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

#[derive(
    Debug, Default, Eq, PartialEq, Clone, Copy, EnumIter, EnumString, IntoStaticStr, Translate,
)]
pub enum ImmersiveExperience {
    #[default]
    Disabled,
    Music,
    Podcast,
    Movie,
}

impl ImmersiveExperience {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "immersive experience",
            map(
                (le_u8, le_u8, le_u8, le_u8),
                |(enabled, _unknown0, mode, _unknown1)| {
                    if enabled == 0 {
                        Self::Disabled
                    } else {
                        match mode {
                            1 => Self::Podcast,
                            2 => Self::Movie,
                            _ => Self::Music,
                        }
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    // [enabled, unknown, mode]. The unknown byte is always 1 in captures.
    // shortcut: hardcoded to 1; preserve it if a head tracking mode is found.
    pub fn bytes(&self) -> impl Iterator<Item = u8> {
        let (enabled, mode) = match self {
            Self::Disabled => (0, 0),
            Self::Music => (1, 0),
            Self::Podcast => (1, 1),
            Self::Movie => (1, 2),
        };
        [enabled, 1, mode].into_iter()
    }
}

#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Clone,
    Copy,
    EnumIter,
    EnumString,
    IntoStaticStr,
    FromRepr,
    Translate,
)]
#[repr(u8)]
pub enum PressureSensitivity {
    Softest = 0,
    #[default]
    Medium = 1,
    Firmest = 2,
}

impl PressureSensitivity {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "pressure sensitivity",
            map(le_u8, |v| Self::from_repr(v).unwrap_or_default()),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::ImmersiveExperience;

    #[test]
    fn dolby_modes_round_trip() {
        for (mode, wire) in [
            (ImmersiveExperience::Disabled, [0, 1, 0]),
            (ImmersiveExperience::Music, [1, 1, 0]),
            (ImmersiveExperience::Podcast, [1, 1, 1]),
            (ImmersiveExperience::Movie, [1, 1, 2]),
        ] {
            assert_eq!(mode.bytes().collect::<Vec<_>>(), wire);
            let state = [wire[0], wire[1], wire[2], 0];
            let (_, parsed) =
                ImmersiveExperience::take::<nom::error::Error<&[u8]>>(&state).unwrap();
            assert_eq!(parsed, mode);
        }
    }
}
