use nom::{
    bytes::complete::take,
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::devices::{
    a3933::device_profile::A3933_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::state_update_packet::StateUpdatePacket,
            parsing::{
                take_ambient_sound_mode_cycle, take_battery_level, take_bool,
                take_custom_button_model, take_custom_hear_id_without_music_type,
                take_dual_battery, take_equalizer_configuration, take_firmware_version,
                take_serial_number, take_sound_modes, ParseResult,
            },
        },
        structures::{
            AmbientSoundModeCycle, BatteryLevel, CustomButtonModel, CustomHearId, DualBattery,
            EqualizerConfiguration, FirmwareVersion, SerialNumber, SoundModes,
        },
    },
};

// A3933 and A3939
// Despite EQ being 10 bands, only the first 8 seem to be used?
#[derive(Debug, Clone, PartialEq)]
pub struct A3933StateUpdatePacket {
    host_device: u8,
    tws_status: bool,
    battery: DualBattery,
    left_firmware: FirmwareVersion,
    right_firmware: FirmwareVersion,
    serial_number: SerialNumber,
    equalizer_configuration: EqualizerConfiguration, // 10 bands mono
    age_range: u8,
    hear_id: CustomHearId, // 10 bands
    custom_button_model: CustomButtonModel,
    ambient_sound_mode_cycle: AmbientSoundModeCycle,
    sound_modes: SoundModes,
    touch_tone_switch: bool,
    wear_detection_switch: bool,
    game_mode_switch: bool,
    charging_case_battery_level: BatteryLevel,
    device_color: u8,
    wind_noise_detection: bool,
}

impl From<A3933StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3933StateUpdatePacket) -> Self {
        Self {
            device_profile: A3933_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: None,
            gender: None,
            hear_id: None,
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: Some(packet.left_firmware.min(packet.right_firmware)),
            serial_number: Some(packet.serial_number),
            ambient_sound_mode_cycle: Some(packet.ambient_sound_mode_cycle),
        }
    }
}

pub fn take_a3933_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3933StateUpdatePacket, E> {
    context(
        "a3933 state update packet",
        all_consuming(map(
            tuple((
                le_u8,
                take_bool,
                take_dual_battery,
                take_firmware_version,
                take_firmware_version,
                take_serial_number,
                take_equalizer_configuration(10),
                take(10usize), // ??, maybe stereo eq?
                le_u8,
                take_custom_hear_id_without_music_type(10),
                take_custom_button_model,
                take_ambient_sound_mode_cycle,
                take_sound_modes,
                opt(take_optional_extra_data),
            )),
            |(
                host_device,
                tws_status,
                battery,
                left_firmware,
                right_firmware,
                serial_number,
                equalizer_configuration, // 10 bands mono
                _,
                age_range,
                hear_id,
                custom_button_model,
                ambient_sound_mode_cycle,
                sound_modes,
                extra,
            )| {
                A3933StateUpdatePacket {
                    host_device,
                    tws_status,
                    battery,
                    left_firmware,
                    right_firmware,
                    serial_number,
                    equalizer_configuration,
                    age_range,
                    hear_id,
                    custom_button_model,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    touch_tone_switch: extra.map(|e| e.0).unwrap_or_default(),
                    wear_detection_switch: extra.map(|e| e.1).unwrap_or_default(),
                    game_mode_switch: extra.map(|e| e.2).unwrap_or_default(),
                    charging_case_battery_level: extra.map(|e| e.3).unwrap_or_default(),
                    device_color: extra.map(|e| e.5).unwrap_or_default(),
                    wind_noise_detection: extra.map(|e| e.6).unwrap_or_default(),
                }
            },
        )),
    )(input)
}

fn take_optional_extra_data<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<(bool, bool, bool, BatteryLevel, u8, u8, bool), E> {
    tuple((
        take_bool, // touch tone
        take_bool, // wear detection
        take_bool, // game mode
        take_battery_level,
        le_u8,     // what is this byte?
        le_u8,     // device color
        take_bool, // wind noise detection
    ))(input)
}
