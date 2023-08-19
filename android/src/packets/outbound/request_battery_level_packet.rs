use openscq30_lib::packets::outbound::{
    OutboundPacketBytes, RequestBatteryLevelPacket as LibRequestBatteryLevelPacket,
};
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::{type_conversion, OutboundPacket};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RequestBatteryLevelPacket {
    packet: LibRequestBatteryLevelPacket,
}

impl RequestBatteryLevelPacket {
    #[generate_interface(constructor)]
    pub fn new() -> RequestBatteryLevelPacket {
        Self {
            packet: LibRequestBatteryLevelPacket::new(),
        }
    }
}

impl OutboundPacket for RequestBatteryLevelPacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}
