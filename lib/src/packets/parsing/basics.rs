use nom::{
    bytes::complete::take,
    combinator::{map, map_opt},
    error::{ContextError, ParseError},
    number::complete::le_u8,
    IResult,
};

pub type ParseResult<'a, T, E> = IResult<&'a [u8], T, E>;

pub fn take_bool<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<bool, E> {
    map(le_u8, |value| value == 1)(input)
}

pub fn take_str<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    len: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<&'a str, E> {
    move |input| map_opt(take(len), |bytes| std::str::from_utf8(bytes).ok())(input)
}
