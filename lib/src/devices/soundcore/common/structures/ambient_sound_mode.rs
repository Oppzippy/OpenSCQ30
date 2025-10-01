use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr, VariantArray};

#[repr(u8)]
#[derive(
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Display,
    Default,
    AsRefStr,
    IntoStaticStr,
    EnumIter,
    EnumString,
    Translate,
    VariantArray,
)]
pub enum AmbientSoundMode {
    #[default]
    NoiseCanceling = 0,
    Transparency = 1,
    Normal = 2,
}

impl AmbientSoundMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "ambient sound mode",
            map(le_u8, |ambient_sound_mode_id| {
                Self::from_id(ambient_sound_mode_id).unwrap_or_default()
            }),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for AmbientSoundMode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(AmbientSoundMode::VARIANTS).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::AmbientSoundMode;

    #[test]
    fn from_id_creates_with_valid_id() {
        let mode = AmbientSoundMode::from_id(0);
        assert!(mode.is_some());
    }

    #[test]
    fn from_id_returns_none_with_invalid_id() {
        let mode = AmbientSoundMode::from_id(100);
        assert!(mode.is_none());
    }

    #[test]
    fn getting_id_works() {
        assert_eq!(1, AmbientSoundMode::Transparency.id());
    }
}
