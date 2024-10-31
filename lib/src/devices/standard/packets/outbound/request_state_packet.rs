use crate::devices::standard::structures::Command;

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestStatePacket {}

impl RequestStatePacket {
    pub fn new() -> Self {
        RequestStatePacket {}
    }
}

impl OutboundPacket for RequestStatePacket {
    fn command(&self) -> Command {
        Command::new([0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x01])
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::standard::packets::outbound::{RequestStatePacket, SendableBytes};

    #[test]
    fn it_matches_an_example_request_state_packet() {
        const EXPECTED: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0a, 0x00, 0x02];
        let packet = RequestStatePacket::new();
        assert_eq!(EXPECTED, packet.bytes());
    }
}
