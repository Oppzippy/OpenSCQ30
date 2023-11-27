use openscq30_lib::devices::standard::packets::outbound::{
    OutboundPacketBytes, SetEqualizerAndCustomHearIdPacket as LibSetEqualizerAndCustomHearIdPacket,
};

use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use crate::{
    devices::standard::structures::EqualizerConfiguration, type_conversion, AgeRange, CustomHearId,
    Gender, OutboundPacket,
};

#[generate_interface_doc]
#[derive(Debug, Clone, PartialEq)]
pub struct SetEqualizerAndCustomHearIdPacket {
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId,
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
            equalizer_configuration,
            gender,
            age_range,
            custom_hear_id,
        }
    }
}

impl OutboundPacket for SetEqualizerAndCustomHearIdPacket {
    #[generate_interface]
    fn bytes(&self) -> Vec<i8> {
        let bytes = LibSetEqualizerAndCustomHearIdPacket {
            equalizer_configuration: &self.equalizer_configuration.to_owned().into(),
            custom_hear_id: &self.custom_hear_id.to_owned().into(),
            gender: self.gender.into(),
            age_range: self.age_range.into(),
        }
        .bytes();
        type_conversion::u8_slice_to_i8_slice(&bytes).to_vec()
    }
}
