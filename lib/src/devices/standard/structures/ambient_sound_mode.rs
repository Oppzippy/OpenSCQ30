use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, FromRepr, IntoStaticStr};

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
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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
    ) -> IResult<&'a [u8], AmbientSoundMode, E> {
        context(
            "ambient sound mode",
            map(le_u8, |ambient_sound_mode_id| {
                AmbientSoundMode::from_id(ambient_sound_mode_id).unwrap_or_default()
            }),
        )(input)
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
