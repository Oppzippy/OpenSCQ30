use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
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
    Translate,
)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub enum NoiseCancelingMode {
    #[default]
    Transport = 0,
    Outdoor = 1,
    Indoor = 2,
    Custom = 3,
}

impl NoiseCancelingMode {
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
            "noise canceling mode",
            map(le_u8, |noise_canceling_mode| {
                Self::from_id(noise_canceling_mode).unwrap_or_default()
            }),
        )
        .parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::NoiseCancelingMode;

    #[test]
    fn from_id_creates_with_valid_id() {
        let mode = NoiseCancelingMode::from_id(0);
        assert!(mode.is_some());
    }

    #[test]
    fn from_id_returns_none_with_invalid_id() {
        let mode = NoiseCancelingMode::from_id(100);
        assert!(mode.is_none());
    }

    #[test]
    fn getting_id_works() {
        assert_eq!(1, NoiseCancelingMode::Outdoor.id());
    }
}
