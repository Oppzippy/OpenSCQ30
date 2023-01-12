use openscq30_lib::packets::outbound::OutboundPacket as _;
use rifgen::rifgen_attr::generate_interface;

use crate::{packets::structures::EqualizerConfiguration, type_conversion};

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerPacket {
    packet: openscq30_lib::packets::outbound::SetEqualizerPacket,
}

impl SetEqualizerPacket {
    #[generate_interface(constructor)]
    pub fn new(configuration: &EqualizerConfiguration) -> SetEqualizerPacket {
        Self {
            packet: openscq30_lib::packets::outbound::SetEqualizerPacket::new(
                configuration.to_owned().into(),
            ),
        }
    }
}

impl OutboundPacket for SetEqualizerPacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}
