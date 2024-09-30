use nom::error::{ContextError, ParseError};

use crate::devices::standard::{
    packets::parsing::{take_checksum, ParseResult},
    structures::PacketHeader,
};

pub(crate) fn take_inbound_packet_header<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<[u8; 7], E> {
    let input = take_checksum(input)?.0;
    let (input, header) = PacketHeader::take(input)?;
    Ok((input, header.packet_type))
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::take_inbound_packet_header;
    #[test]
    fn it_errors_when_nothing_matches() {
        let result = take_inbound_packet_header::<VerboseError<_>>(&[1, 2, 3]);
        assert_eq!(true, result.is_err());
    }
}
