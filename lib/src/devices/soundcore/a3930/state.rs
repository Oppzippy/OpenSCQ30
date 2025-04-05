use crate::{
    devices::soundcore::standard::structures::{
        AgeRange, CustomHearId, DualBattery, EqualizerConfiguration, Gender,
        InternalMultiButtonConfiguration, SoundModes, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3930StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3930State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    gender: Gender,
    age_range: AgeRange,
    custom_hear_id: CustomHearId,
    button_configuration: InternalMultiButtonConfiguration,
    sound_modes: SoundModes,
    side_tone: bool,
    hear_id_eq_index: Option<u16>,
}

impl_as_ref_for_field!(
    struct A3930State {
        tws_status: TwsStatus,
        battery: DualBattery,
        equalizer_configuration: EqualizerConfiguration,
        gender: Gender,
        age_range: AgeRange,
        custom_hear_id: CustomHearId,
        button_configuration: InternalMultiButtonConfiguration,
        sound_modes: SoundModes,
    }
);

impl From<A3930StateUpdatePacket> for A3930State {
    fn from(value: A3930StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            gender: value.gender,
            age_range: value.age_range,
            custom_hear_id: value.custom_hear_id,
            button_configuration: value.button_configuration,
            sound_modes: value.sound_modes,
            side_tone: value.side_tone,
            hear_id_eq_index: value.hear_id_eq_index,
        }
    }
}
