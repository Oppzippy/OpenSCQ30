use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    structures::{
        CommonEqualizerConfiguration, DualBattery, DualFirmwareVersion, SerialNumber, TouchTone,
        TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::packets::inbound::A3944StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3944State {
    tws_status: TwsStatus,
    dual_battery: DualBattery,
    dual_firmware_version: DualFirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    button_configuration: ButtonStatusCollection<6>,
    touch_tone: TouchTone,
    reset_button_configuration_pending: ResetButtonConfigurationPending,
}

impl From<A3944StateUpdatePacket> for A3944State {
    fn from(value: A3944StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            dual_battery: value.dual_battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            button_configuration: value.button_configuration,
            touch_tone: value.touch_tone,
            reset_button_configuration_pending: ResetButtonConfigurationPending::default(),
        }
    }
}
