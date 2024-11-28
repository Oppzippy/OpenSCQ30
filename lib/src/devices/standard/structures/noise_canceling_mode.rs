use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    IResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, FromRepr};

#[repr(u8)]
#[derive(FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Default, AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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
    ) -> IResult<&'a [u8], NoiseCancelingMode, E> {
        context(
            "noise canceling mode",
            map(le_u8, |noise_canceling_mode| {
                NoiseCancelingMode::from_id(noise_canceling_mode).unwrap_or_default()
            }),
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::NoiseCancelingMode;

    #[test]
    fn from_id_creates_with_valid_id() {
        let mode = NoiseCancelingMode::from_id(0);
        assert_eq!(true, mode.is_some());
    }

    #[test]
    fn from_id_returns_none_with_invalid_id() {
        let mode = NoiseCancelingMode::from_id(100);
        assert_eq!(true, mode.is_none());
    }

    #[test]
    fn getting_id_works() {
        assert_eq!(1, NoiseCancelingMode::Outdoor.id());
    }
}
