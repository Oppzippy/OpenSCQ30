use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3949::packets::inbound::A3949StateUpdatePacket,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            CommonEqualizerConfiguration, DualBattery, DualFirmwareVersion, GamingMode,
            SerialNumber, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3949State {
    tws_status: TwsStatus,
    battery: DualBattery,
    firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    button_configuration: ButtonStatusCollection<6>,
    gaming_mode: GamingMode,
    touch_tone: TouchTone,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl From<A3949StateUpdatePacket> for A3949State {
    fn from(value: A3949StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            button_configuration: value.button_configuration,
            touch_tone: value.touch_tone,
            gaming_mode: value.gaming_mode,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}
