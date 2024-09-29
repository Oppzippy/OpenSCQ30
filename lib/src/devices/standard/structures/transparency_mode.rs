use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, FromRepr};

use crate::devices::standard::packets::parsing::ParseResult;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, FromRepr, Default, Display, AsRefStr)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum TransparencyMode {
    FullyTransparent = 0,
    #[default]
    VocalMode = 1,
}

impl TransparencyMode {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::from_repr(id)
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<TransparencyMode, E> {
        context(
            "transparency mode",
            map(le_u8, |transparency_mode| {
                TransparencyMode::from_id(transparency_mode).unwrap_or_default()
            }),
        )(input)
    }
}
