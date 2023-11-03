use openscq30_lib::packets::outbound::OutboundPacketBytes;
use openscq30_lib::packets::outbound::SetEqualizerWithDrcPacket as LibSetEqualizerWithDrcPacket;

use rifgen::rifgen_attr::generate_interface;
use rifgen::rifgen_attr::generate_interface_doc;

use crate::OutboundPacket;
use crate::{packets::structures::EqualizerConfiguration, type_conversion};

#[generate_interface_doc]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetEqualizerWithDrcPacket {
    left_configuration: EqualizerConfiguration,
    right_configuration: Option<EqualizerConfiguration>,
}

impl SetEqualizerWithDrcPacket {
    #[generate_interface(constructor)]
    pub fn new(
        left_configuration: &EqualizerConfiguration,
        right_configuration: Option<&EqualizerConfiguration>,
    ) -> SetEqualizerWithDrcPacket {
        Self {
            left_configuration: left_configuration.to_owned(),
            right_configuration: right_configuration.cloned(),
        }
    }
}

impl OutboundPacket for SetEqualizerWithDrcPacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        let bytes = LibSetEqualizerWithDrcPacket::new(
            &self.left_configuration.to_owned().into(),
            self.right_configuration.to_owned().map(Into::into).as_ref(),
        )
        .bytes();
        type_conversion::u8_slice_to_i8_slice(&bytes).to_vec()
    }
}
