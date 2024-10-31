use nom::error::{ContextError, ParseError};

use crate::devices::standard::{packets::parsing::ParseResult, structures::Command};

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetCustomButtonModelOkPacket {}

impl InboundPacket for SetCustomButtonModelOkPacket {
    fn header() -> Command {
        Command::new([0x09, 0xFF, 0x00, 0x00, 0x01, 0x04, 0x84])
    }

    #[allow(dead_code)]
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<SetCustomButtonModelOkPacket, E> {
        Ok((input, SetCustomButtonModelOkPacket::default()))
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::{take_inbound_packet_header, InboundPacket};

    use super::SetCustomButtonModelOkPacket;

    #[test]
    fn it_parses_an_example_ok_packet() {
        let input: &[u8] = &[0x09, 0xFF, 0x00, 0x00, 0x01, 0x04, 0x84, 0x0A, 0x00, 0x9B];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        SetCustomButtonModelOkPacket::take::<VerboseError<_>>(body).expect("should not error");
    }
}
