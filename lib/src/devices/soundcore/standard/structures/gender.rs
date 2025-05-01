use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Gender(pub u8);

impl Gender {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Gender, E> {
        context("gender", map(le_u8, Gender)).parse_complete(input)
    }
}
