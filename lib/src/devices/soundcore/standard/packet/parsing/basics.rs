use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{map, map_opt},
    error::{ContextError, ParseError},
    number::complete::le_u8,
};

pub fn take_bool<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], bool, E> {
    map(le_u8, |value| value == 1).parse_complete(input)
}

pub fn take_str<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    len: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a str, E> {
    move |input| map_opt(take(len), |bytes| std::str::from_utf8(bytes).ok()).parse_complete(input)
}
