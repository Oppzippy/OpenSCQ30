use nom::{
    bytes::complete::take,
    combinator::{map, map_res},
    number::complete::le_u8,
    IResult,
};

pub type ParseResult<'a, T> = IResult<&'a [u8], T>;

pub fn take_bool(input: &[u8]) -> ParseResult<bool> {
    map(le_u8, |value| value == 1)(input)
}

pub fn take_str(len: usize) -> impl Fn(&[u8]) -> ParseResult<&str> {
    move |input: &[u8]| map_res(take(len), |bytes| std::str::from_utf8(bytes))(input)
}
