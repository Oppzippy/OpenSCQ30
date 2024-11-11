use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::devices::{
    a3945::device_profile::A3945_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::state_update_packet::StateUpdatePacket,
            parsing::{take_bool, ParseResult},
        },
        quirks::TwoExtraEqBandsValues,
        structures::{
            BatteryLevel, CustomButtonModel, DualBattery, EqualizerConfiguration, FirmwareVersion,
            HostDevice, SerialNumber, StereoEqualizerConfiguration,
        },
    },
};

// A3945 only
// Despite EQ being 10 bands, only the first 8 seem to be used?
#[derive(Debug, Clone, PartialEq)]
pub struct A3945StateUpdatePacket {
    pub host_device: HostDevice,
    pub tws_status: bool,
    pub battery: DualBattery,
    pub left_firmware: FirmwareVersion,
    pub right_firmware: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub left_equalizer_configuration: EqualizerConfiguration,
    pub right_equalizer_configuration: EqualizerConfiguration,
    pub extra_band_values: TwoExtraEqBandsValues,
    pub custom_button_model: CustomButtonModel,
    pub touch_tone_switch: bool,
    pub wear_detection_switch: bool,
    pub game_mode_switch: bool,
    pub charging_case_battery_level: BatteryLevel,
    pub bass_up_switch: bool,
    pub device_color: u8,
}

impl From<A3945StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3945StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3945_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.left_equalizer_configuration,
            sound_modes: None,
            age_range: None,
            gender: None,
            hear_id: None,
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: Some(packet.left_firmware.min(packet.right_firmware)),
            serial_number: Some(packet.serial_number),
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl A3945StateUpdatePacket {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<A3945StateUpdatePacket, E> {
        context(
            "a3945 state update packet",
            all_consuming(map(
                tuple((
                    HostDevice::take,
                    take_bool,
                    DualBattery::take,
                    FirmwareVersion::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    StereoEqualizerConfiguration::take_with_two_extra_bands(8),
                    CustomButtonModel::take,
                    take_bool,
                    take_bool,
                    take_bool,
                    BatteryLevel::take,
                    take_bool,
                    le_u8,
                )),
                |(
                    host_device,
                    tws_status,
                    battery,
                    left_firmware,
                    right_firmware,
                    serial_number,
                    (equalizer_configuration, extra_band_values),
                    custom_button_model,
                    touch_tone_switch,
                    wear_detection_switch,
                    game_mode_switch,
                    charging_case_battery_level,
                    bass_up_switch,
                    device_color,
                )| {
                    A3945StateUpdatePacket {
                        host_device,
                        tws_status,
                        battery,
                        left_firmware,
                        right_firmware,
                        serial_number,
                        left_equalizer_configuration: equalizer_configuration.left,
                        right_equalizer_configuration: equalizer_configuration.right,
                        extra_band_values,
                        custom_button_model,
                        touch_tone_switch,
                        wear_detection_switch,
                        game_mode_switch,
                        charging_case_battery_level,
                        bass_up_switch,
                        device_color,
                    }
                },
            )),
        )(input)
    }
}
