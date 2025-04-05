use crate::{
    devices::soundcore::standard::structures::{
        DualBattery, EqualizerConfiguration, InternalMultiButtonConfiguration, SoundModes,
        TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3931StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3931State {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    button_configuration: InternalMultiButtonConfiguration,
    sound_modes: SoundModes,
    side_tone: bool,
    touch_tone: bool,
    auto_power_off_on: bool,
    auto_power_off_index: u8, // 0 to 3
}

impl_as_ref_for_field!(
    struct A3931State {
        tws_status: TwsStatus,
        battery: DualBattery,
        equalizer_configuration: EqualizerConfiguration,
        button_configuration: InternalMultiButtonConfiguration,
        sound_modes: SoundModes,
    }
);

impl From<A3931StateUpdatePacket> for A3931State {
    fn from(value: A3931StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            button_configuration: value.button_configuration,
            sound_modes: value.sound_modes,
            side_tone: value.side_tone,
            touch_tone: value.touch_tone,
            auto_power_off_on: value.auto_power_off_on,
            auto_power_off_index: value.auto_power_off_index,
        }
    }
}
