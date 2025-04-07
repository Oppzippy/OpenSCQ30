use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

// unsure what this is. values 0 to 2 are allowed. maybe switch to an enum when the meanings are determined.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HearIdType(pub u8);

impl HearIdType {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], HearIdType, E> {
        context("hear id type", map(le_u8, HearIdType))(input)
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HearIdMusicType(pub u8);

impl HearIdMusicType {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], HearIdMusicType, E> {
        context("hear id music type", map(le_u8, HearIdMusicType))(input)
    }
}
