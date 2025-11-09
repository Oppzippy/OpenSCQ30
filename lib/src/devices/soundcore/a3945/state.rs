use openscq30_lib_macros::Has;

use crate::devices::soundcore::common::{
    modules::reset_button_configuration::ResetButtonConfigurationPending,
    structures::{
        CaseBatteryLevel, DualBattery, DualFirmwareVersion, EqualizerConfiguration, GamingMode,
        SerialNumber, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
    },
};

use super::packets::A3945StateUpdatePacket;

#[derive(Debug, Clone, PartialEq, Eq, Has)]
pub struct A3945State {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub case_battery_level: CaseBatteryLevel,
    pub touch_tone: TouchTone,
    pub gaming_mode: GamingMode,
    #[has(skip)]
    pub wear_detection_switch: bool,
    #[has(skip)]
    pub bass_up_switch: bool,
    #[has(skip)]
    pub device_color: u8,
    button_reset_pending: ResetButtonConfigurationPending,
}

impl From<A3945StateUpdatePacket> for A3945State {
    fn from(value: A3945StateUpdatePacket) -> Self {
        Self {
            tws_status: value.tws_status,
            battery: value.battery,
            dual_firmware_version: value.dual_firmware_version,
            serial_number: value.serial_number,
            equalizer_configuration: value.equalizer_configuration,
            button_configuration: value.button_configuration,
            touch_tone: value.touch_tone,
            wear_detection_switch: value.wear_detection_switch,
            gaming_mode: value.gaming_mode,
            case_battery_level: value.case_battery_level,
            bass_up_switch: value.bass_up_switch,
            device_color: value.device_color,
            button_reset_pending: ResetButtonConfigurationPending::default(),
        }
    }
}
