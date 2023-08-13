use nom::error::{ContextError, ParseError};

use crate::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetSoundModeOkPacket {}

pub fn take_set_ambient_sound_mode_ok_packet<
    'a,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
>(
    input: &'a [u8],
) -> ParseResult<SetSoundModeOkPacket, E> {
    Ok((input, SetSoundModeOkPacket::default()))
}

#[cfg(test)]
mod tests {
    use crate::packets::inbound::InboundPacket;

    #[test]
    fn it_parses_an_example_ok_packet() {
        let input: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a];
        let InboundPacket::SetSoundModeOk(_packet) = InboundPacket::new(input).unwrap() else {
            panic!("wrong packet type");
        };
    }
}
