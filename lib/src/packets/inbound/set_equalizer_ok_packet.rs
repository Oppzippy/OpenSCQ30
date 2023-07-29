use nom::error::{ContextError, ParseError};

use crate::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetEqualizerOkPacket {}

pub fn take_set_equalizer_ok_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<SetEqualizerOkPacket, E> {
    Ok((input, SetEqualizerOkPacket::default()))
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::packets::{inbound::take_set_equalizer_ok_packet, parsing::take_packet_header};

    #[test]
    fn it_parses_an_example_ok_packet() {
        const PACKET_BYTES: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x02, 0x81, 0x0a, 0x00, 0x96];
        let input = take_packet_header::<VerboseError<&[u8]>>(PACKET_BYTES)
            .unwrap()
            .0;
        take_set_equalizer_ok_packet::<VerboseError<&[u8]>>(input)
            .expect("should parse a correct OkPacket");
    }
}
