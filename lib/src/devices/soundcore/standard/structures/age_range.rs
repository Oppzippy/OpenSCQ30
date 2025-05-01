use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AgeRange(pub u8);

impl AgeRange {
    pub fn supports_hear_id(&self) -> bool {
        self.0 != u8::MAX
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], AgeRange, E> {
        context("age range", map(le_u8, AgeRange)).parse_complete(input)
    }
}
