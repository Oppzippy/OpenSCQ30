use crate::{
    devices::soundcore::standard::structures::{
        DualBattery, EqualizerConfiguration, MultiButtonConfiguration, SoundModes, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3031StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct A3031State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub button_configuration: MultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub touch_tone: bool,
    pub auto_power_off_on: bool,
    pub auto_power_off_on_index: u8,
}

impl_as_ref_for_field!(
    struct A3031State {
        tws_status: TwsStatus,
        battery: DualBattery,
        sound_modes: SoundModes,
        equalizer_configuration: EqualizerConfiguration<2, 8>,
        button_configuration: MultiButtonConfiguration,
    }
);

impl From<A3031StateUpdatePacket> for A3031State {
    fn from(value: A3031StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            equalizer_configuration: value.equalizer_configuration,
            button_configuration: value.button_configuration,
            sound_modes: value.sound_modes,
            side_tone: value.side_tone,
            touch_tone: value.touch_tone,
            auto_power_off_on: value.auto_power_off_on,
            auto_power_off_on_index: value.auto_power_off_on_index,
        }
    }
}
