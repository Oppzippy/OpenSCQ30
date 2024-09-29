use nom::{
    combinator::map,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::devices::standard::packets::parsing::ParseResult;

use super::{BasicHearId, CustomHearId};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", tag = "type"))]
pub enum HearId {
    Basic(BasicHearId),
    Custom(CustomHearId),
}

impl From<BasicHearId> for HearId {
    fn from(basic_hear_id: BasicHearId) -> Self {
        Self::Basic(basic_hear_id)
    }
}

impl From<CustomHearId> for HearId {
    fn from(custom_hear_id: CustomHearId) -> Self {
        Self::Custom(custom_hear_id)
    }
}

// unsure what this is. values 0 to 2 are allowed. maybe switch to an enum when the meanings are determined.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HearIdType(pub u8);

impl HearIdType {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<HearIdType, E> {
        context("hear id type", map(le_u8, HearIdType))(input)
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HearIdMusicType(pub u8);

impl HearIdMusicType {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<HearIdMusicType, E> {
        context("hear id music type", map(le_u8, HearIdMusicType))(input)
    }
}
