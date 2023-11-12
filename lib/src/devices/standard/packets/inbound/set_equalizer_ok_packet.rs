use nom::error::{ContextError, ParseError};

use crate::devices::standard::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetEqualizerOkPacket {}

pub fn take_set_equalizer_ok_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<SetEqualizerOkPacket, E> {
    Ok((input, SetEqualizerOkPacket::default()))
}

#[cfg(test)]
mod tests {
    use crate::devices::standard::packets::inbound::InboundPacket;

    #[test]
    fn it_parses_an_example_ok_packet() {
        let input: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x02, 0x81, 0x0a, 0x00, 0x96];
        let InboundPacket::SetEqualizerOk(_packet) = InboundPacket::new(input).unwrap() else {
            panic!("wrong packet type");
        };
    }
}
