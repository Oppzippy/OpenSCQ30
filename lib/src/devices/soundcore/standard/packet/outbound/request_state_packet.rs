use crate::devices::soundcore::standard::packet::Command;

use super::outbound_packet::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestStatePacket {}

impl RequestStatePacket {
    pub const COMMAND: Command = Command([0x01, 0x01]);

    pub fn new() -> Self {
        RequestStatePacket {}
    }
}

impl OutboundPacket for RequestStatePacket {
    fn command(&self) -> Command {
        Self::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::devices::soundcore::standard::packet::outbound::{
        OutboundPacketBytesExt, RequestStatePacket,
    };

    #[test]
    fn it_matches_an_example_request_state_packet() {
        const EXPECTED: &[u8] = &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0a, 0x00, 0x02];
        let packet = RequestStatePacket::new();
        assert_eq!(EXPECTED, packet.bytes());
    }
}
