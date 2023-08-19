use openscq30_lib::packets::outbound::OutboundPacketBytes;
use openscq30_lib::packets::outbound::SetEqualizerWithDrcPacket as LibSetEqualizerWithDrcPacket;

use rifgen::rifgen_attr::generate_interface;
use rifgen::rifgen_attr::generate_interface_doc;

use crate::OutboundPacket;
use crate::{packets::structures::EqualizerConfiguration, type_conversion};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerWithDrcPacket {
    packet: LibSetEqualizerWithDrcPacket,
}

impl SetEqualizerWithDrcPacket {
    #[generate_interface(constructor)]
    pub fn new(
        left_configuration: &EqualizerConfiguration,
        right_configuration: Option<&EqualizerConfiguration>,
    ) -> SetEqualizerWithDrcPacket {
        Self {
            packet: LibSetEqualizerWithDrcPacket::new(
                left_configuration.to_owned().into(),
                right_configuration.copied().map(Into::into),
            ),
        }
    }
}

impl OutboundPacket for SetEqualizerWithDrcPacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}
