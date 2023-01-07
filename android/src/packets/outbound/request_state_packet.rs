use openscq30_lib::packets::outbound::OutboundPacket as _;
use rifgen::rifgen_attr::generate_interface;

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestStatePacket {
    packet: openscq30_lib::packets::outbound::RequestStatePacket,
}

impl RequestStatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> RequestStatePacket {
        Self {
            packet: openscq30_lib::packets::outbound::RequestStatePacket::new(),
        }
    }
}

impl OutboundPacket for RequestStatePacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i16> {
        self.packet
            .bytes()
            .into_iter()
            .map(|x| i16::from(x))
            .collect()
    }
}
