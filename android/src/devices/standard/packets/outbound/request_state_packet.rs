use openscq30_lib::devices::standard::packets::outbound::{
    OutboundPacketBytes, RequestStatePacket as LibRequestStatePacket,
};
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::{type_conversion, OutboundPacket};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestStatePacket {
    packet: LibRequestStatePacket,
}

impl RequestStatePacket {
    #[generate_interface(constructor)]
    pub fn new() -> RequestStatePacket {
        Self {
            packet: LibRequestStatePacket::new(),
        }
    }
}

impl OutboundPacket for RequestStatePacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}
