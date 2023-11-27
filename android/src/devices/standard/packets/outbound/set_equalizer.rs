use openscq30_lib::devices::standard::packets::outbound::{
    OutboundPacketBytes, SetEqualizerPacket as LibSetEqualizerPacket,
};

use rifgen::rifgen_attr::generate_interface;
use rifgen::rifgen_attr::generate_interface_doc;

use crate::OutboundPacket;
use crate::{devices::standard::structures::EqualizerConfiguration, type_conversion};

#[generate_interface_doc]
#[derive(Debug, Clone, PartialEq)]
pub struct SetEqualizerPacket {
    left_configuration: EqualizerConfiguration,
    right_configuration: Option<EqualizerConfiguration>,
}

impl SetEqualizerPacket {
    #[generate_interface(constructor)]
    pub fn new(
        left_configuration: &EqualizerConfiguration,
        right_configuration: Option<&EqualizerConfiguration>,
    ) -> SetEqualizerPacket {
        Self {
            left_configuration: left_configuration.to_owned(),
            right_configuration: right_configuration.cloned(),
        }
    }
}

impl OutboundPacket for SetEqualizerPacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        let bytes = LibSetEqualizerPacket::new(
            &self.left_configuration.to_owned().into(),
            self.right_configuration.to_owned().map(Into::into).as_ref(),
        )
        .bytes();
        type_conversion::u8_slice_to_i8_slice(&bytes).to_vec()
    }
}
