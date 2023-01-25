#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SetAmbientModeOkPacket {}

impl SetAmbientModeOkPacket {
    pub fn new(bytes: &[u8]) -> Option<Self> {
        const PREFIX: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a];
        if bytes == PREFIX {
            Some(Self {})
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::inbound::SetAmbientModeOkPacket;

    #[test]
    fn it_parses_an_example_ok_packet() {
        const PACKET_BYTES: &[u8] = &[0x09, 0xff, 0x00, 0x00, 0x01, 0x06, 0x81, 0x0a, 0x00, 0x9a];
        SetAmbientModeOkPacket::new(PACKET_BYTES).expect("should be an OkPacket");
    }

    #[test]
    fn it_does_not_parse_unknown_packet() {
        const PACKET_BYTES: &[u8] = &[0x01, 0x02, 0x03];
        let packet = SetAmbientModeOkPacket::new(PACKET_BYTES);
        assert_eq!(None, packet);
    }
}
