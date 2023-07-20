use crate::packets::parsing::ParseResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetSoundModeOkPacket {}

pub fn take_set_ambient_sound_mode_ok_packet(input: &[u8]) -> ParseResult<SetSoundModeOkPacket> {
    Ok((input, SetSoundModeOkPacket::default()))
}

#[cfg(test)]
mod tests {
    use crate::packets::{
        inbound::take_set_ambient_sound_mode_ok_packet, parsing::take_packet_header,
    };

    #[test]
    fn it_parses_an_example_ok_packet() {
        const PACKET_BYTES: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a];
        let input = take_packet_header(PACKET_BYTES).unwrap().0;
        take_set_ambient_sound_mode_ok_packet(input).expect("should parse correct ok packet");
    }
}
