use crate::{
    devices::soundcore::standard::structures::{
        AgeRange, CustomHearId, DualBattery, EqualizerConfiguration, Gender,
        InternalMultiButtonConfiguration, SoundModes, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3951StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3951State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    gender: Gender,
    age_range: AgeRange,
    custom_hear_id: CustomHearId,
    button_configuration: InternalMultiButtonConfiguration,
    sound_modes: SoundModes,
    side_tone: bool,
    wear_detection: bool,
    touch_tone: bool,
    hear_id_eq_preset: Option<u16>,
    supports_new_battery: bool, // yes if packet is >98, don't parse
    left_new_battery: u8,       // 0 to 9
    right_new_battery: u8,      // 0 to 9
}

impl_as_ref_for_field!(
    struct A3951State {
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

impl From<A3951StateUpdatePacket> for A3951State {
    fn from(value: A3951StateUpdatePacket) -> Self {
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
            wear_detection: value.wear_detection,
            touch_tone: value.touch_tone,
            hear_id_eq_preset: value.hear_id_eq_preset,
            supports_new_battery: value.supports_new_battery,
            left_new_battery: value.left_new_battery,
            right_new_battery: value.right_new_battery,
        }
    }
}
