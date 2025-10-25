use openscq30_lib_macros::Has;

use crate::devices::soundcore::{
    a3948::packets::inbound::A3948StateUpdatePacket,
    common::{
        modules::reset_button_configuration::ResetButtonConfigurationPending,
        structures::{
            DualBattery, DualFirmwareVersion, EqualizerConfiguration, SerialNumber, TouchTone,
            TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3948State {
    tws_status: TwsStatus,
    battery: DualBattery,
    firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration<1, 10>,
    button_configuration: ButtonStatusCollection<6>,
    touch_tone: TouchTone,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl From<A3948StateUpdatePacket> for A3948State {
    fn from(value: A3948StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            firmware_version: value.firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            button_configuration: value.button_configuration,
            touch_tone: value.touch_tone,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}
