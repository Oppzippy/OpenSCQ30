use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3959::{
        packets::A3959StateUpdatePacket,
        structures::{A3959MultiButtonConfiguration, A3959SoundModes},
    },
    standard::structures::{
        AmbientSoundModeCycle, AutoPowerOff, DualBattery, DualFirmwareVersion,
        EqualizerConfiguration, SerialNumber, TwsStatus,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3959State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration<2, 10>,
    button_configuration: A3959MultiButtonConfiguration,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: A3959SoundModes,
    auto_power_off: AutoPowerOff,
}

impl From<A3959StateUpdatePacket> for A3959State {
    fn from(packet: A3959StateUpdatePacket) -> Self {
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
        }
    }
}
