use nom::error::{ContextError, ParseError};

use crate::devices::standard::packets::parsing::ParseResult;

use super::InboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetAmbientSoundModeCycleOkPacket {}

impl InboundPacket for SetAmbientSoundModeCycleOkPacket {
    fn header() -> [u8; 7] {
        [0x09, 0xFF, 0x00, 0x00, 0x01, 0x06, 0x82]
    }

    #[allow(dead_code)]
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<SetAmbientSoundModeCycleOkPacket, E> {
        Ok((input, SetAmbientSoundModeCycleOkPacket::default()))
    }
}

#[cfg(test)]
mod tests {
    use nom::error::VerboseError;

    use crate::devices::standard::packets::inbound::{take_inbound_packet_header, InboundPacket};

    use super::SetAmbientSoundModeCycleOkPacket;

    #[test]
    fn it_parses_an_example_ok_packet() {
        let input: &[u8] = &[0x09, 0xFF, 0x00, 0x00, 0x01, 0x06, 0x82, 0x0A, 0x00, 0x9B];
        let (body, _) = take_inbound_packet_header::<VerboseError<_>>(input).unwrap();
        SetAmbientSoundModeCycleOkPacket::take::<VerboseError<_>>(body).expect("should not error");
    }
}
