use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3959,
    common::structures::{
        AmbientSoundModeCycle, AutoPowerOff, DualBattery, DualFirmwareVersion,
        EqualizerConfiguration, SerialNumber, TouchTone, TwsStatus,
        button_configuration_v2::ButtonStatusCollection,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3959State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration<1, 10>,
    button_configuration: ButtonStatusCollection<8>,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: a3959::structures::SoundModes,
    auto_power_off: AutoPowerOff,
    touch_tone: TouchTone,
}

impl From<a3959::packets::inbound::A3959State> for A3959State {
    fn from(packet: a3959::packets::inbound::A3959State) -> Self {
        Self {
            tws_status: packet.tws_status,
            dual_battery: packet.dual_battery,
            dual_firmware_version: packet.dual_firmware_version,
            serial_number: packet.serial_number,
            equalizer_configuration: packet.equalizer_configuration,
            button_configuration: packet.button_configuration,
            ambient_sound_mode_cycle: packet.ambient_sound_mode_cycle,
            sound_modes: packet.sound_modes,
            auto_power_off: packet.auto_power_off,
            touch_tone: packet.touch_tone,
        }
    }
}
