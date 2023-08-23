use openscq30_lib::packets::outbound::{
    OutboundPacketBytes, SetEqualizerAndCustomHearIdPacket as LibSetEqualizerAndCustomHearIdPacket,
};

use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::{
    packets::structures::EqualizerConfiguration, type_conversion, AgeRange, CustomHearId, Gender,
    OutboundPacket,
};

#[generate_interface_doc]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetEqualizerAndCustomHearIdPacket {
    packet: LibSetEqualizerAndCustomHearIdPacket,
}

impl SetEqualizerAndCustomHearIdPacket {
    #[generate_interface(constructor)]
    pub fn new(
        equalizer_configuration: EqualizerConfiguration,
        gender: Gender,
        age_range: AgeRange,
        custom_hear_id: CustomHearId,
    ) -> SetEqualizerAndCustomHearIdPacket {
        Self {
            packet: LibSetEqualizerAndCustomHearIdPacket {
                equalizer_configuration: equalizer_configuration.into(),
                gender: gender.into(),
                age_range: age_range.into(),
                custom_hear_id: custom_hear_id.into(),
            },
        }
    }
}

impl OutboundPacket for SetEqualizerAndCustomHearIdPacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        type_conversion::u8_slice_to_i8_slice(&self.packet.bytes()).to_vec()
    }
}
