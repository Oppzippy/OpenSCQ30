use nom::{
    IResult, Parser,
    combinator::map_opt,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::standard::packets::checksum::calculate_checksum;

pub(crate) fn take_checksum<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], (), E> {
    context(
        "checksum",
        map_opt(take_last_byte, |actual_checksum| {
            let expected_checksum = calculate_checksum(&input[..input.len() - 1]);
            if actual_checksum == expected_checksum {
                return Some(());
            }
            None
        }),
    )
    .parse_complete(input)
}

fn take_last_byte<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], u8, E> {
    let last_byte = le_u8(&input[input.len() - 1..])?.1;
    Ok((&input[..input.len() - 1], last_byte))
}
