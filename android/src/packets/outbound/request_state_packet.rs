use openscq30_lib::packets::outbound::OutboundPacketBytes;
use rifgen::rifgen_attr::generate_interface;

use crate::{type_conversion, OutboundPacket};

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
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}
