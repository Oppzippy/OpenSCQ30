use crate::{
    devices::soundcore::standard::structures::{
        BatteryLevel, DualBattery, EqualizerConfiguration, FirmwareVersion,
        MultiButtonConfiguration, SerialNumber, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3945StateUpdatePacket;

#[derive(Debug, Clone, PartialEq)]
pub struct A3945State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub left_firmware: FirmwareVersion,
    pub right_firmware: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration,
    pub button_configuration: MultiButtonConfiguration,
    pub touch_tone_switch: bool,
    pub wear_detection_switch: bool,
    pub game_mode_switch: bool,
    pub charging_case_battery_level: BatteryLevel,
    pub bass_up_switch: bool,
    pub device_color: u8,
}

impl_as_ref_for_field!(
    struct A3945State {
        tws_status: TwsStatus,
        battery: DualBattery,
        serial_number: SerialNumber,
        equalizer_configuration: EqualizerConfiguration,
        button_configuration: MultiButtonConfiguration,
    }
);

impl From<A3945StateUpdatePacket> for A3945State {
    fn from(value: A3945StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            left_firmware: value.left_firmware,
            right_firmware: value.right_firmware,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            button_configuration: value.button_configuration,
            touch_tone_switch: value.touch_tone_switch,
            wear_detection_switch: value.wear_detection_switch,
            game_mode_switch: value.game_mode_switch,
            charging_case_battery_level: value.charging_case_battery_level,
            bass_up_switch: value.bass_up_switch,
            device_color: value.device_color,
        }
    }
}
