use openscq30_lib::packets::outbound::OutboundPacket as _;
use openscq30_lib::packets::outbound::SetEqualizerPacket as LibSetEqualizerPacket;

use rifgen::rifgen_attr::generate_interface;

use crate::{packets::structures::EqualizerConfiguration, type_conversion};

use super::OutboundPacket;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerPacket {
    packet: LibSetEqualizerPacket,
}

impl SetEqualizerPacket {
    #[generate_interface(constructor)]
    pub fn new(
        left_configuration: &EqualizerConfiguration,
        right_configuration: Option<&EqualizerConfiguration>,
    ) -> SetEqualizerPacket {
        Self {
            packet: LibSetEqualizerPacket::new(
                left_configuration.to_owned().into(),
                right_configuration.copied().map(Into::into),
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
