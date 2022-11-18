use super::outbound_packet::OutboundPacket;

pub struct RequestStatePacket {}

impl RequestStatePacket {
    pub fn new() -> Self {
        RequestStatePacket {}
    }
}

impl OutboundPacket for RequestStatePacket {
    fn bytes(&self) -> Vec<u8> {
        vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x01, 0x0A, 0x00, 0x02]
    }
}
