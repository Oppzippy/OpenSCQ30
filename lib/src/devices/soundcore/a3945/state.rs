use openscq30_lib_macros::Has;

use crate::{
    devices::soundcore::standard::structures::{
        BatteryLevel, DualBattery, DualFirmwareVersion, EqualizerConfiguration,
        MultiButtonConfiguration, SerialNumber, TwsStatus,
    },
    macros::impl_as_ref_for_field,
};

use super::packets::A3945StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3945State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub button_configuration: MultiButtonConfiguration,
    pub charging_case_battery_level: BatteryLevel,
    #[has(skip)]
    pub touch_tone_switch: bool,
    #[has(skip)]
    pub wear_detection_switch: bool,
    #[has(skip)]
    pub game_mode_switch: bool,
    #[has(skip)]
    pub bass_up_switch: bool,
    #[has(skip)]
    pub device_color: u8,
}

impl_as_ref_for_field!(
    struct A3945State {
        tws_status: TwsStatus,
        battery: DualBattery,
        dual_firmware_version: DualFirmwareVersion,
        serial_number: SerialNumber,
        equalizer_configuration: EqualizerConfiguration<2, 10>,
        button_configuration: MultiButtonConfiguration,
    }
);

impl From<A3945StateUpdatePacket> for A3945State {
    fn from(value: A3945StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            dual_firmware_version: value.dual_firmware_version,
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
