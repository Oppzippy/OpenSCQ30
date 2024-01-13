use nom::{
    bytes::complete::take,
    combinator::all_consuming,
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
};

use crate::devices::{
    a3936::device_profile::A3936_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::state_update_packet::StateUpdatePacket,
            parsing::{
                take_age_range, take_ambient_sound_mode_cycle, take_battery_level, take_bool,
                take_custom_button_model, take_custom_hear_id_without_music_type,
                take_dual_battery, take_firmware_version, take_gender, take_serial_number,
                ParseResult,
            },
        },
        quirks::{take_stereo_equalizer_configuration_with_two_extra_bands, TwoExtraEqBandsValues},
        structures::{
            take_sound_modes_type_two, AgeRange, AmbientSoundModeCycle, BatteryLevel,
            CustomButtonModel, CustomHearId, CustomNoiseCanceling, DualBattery, FirmwareVersion,
            Gender, NoiseCancelingMode, SerialNumber, SoundModes, SoundModesTypeTwo,
            StereoEqualizerConfiguration,
        },
    },
};

// A3936
#[derive(Debug, Clone, PartialEq)]
pub struct A3936StateUpdatePacket {
    // TODO replace host device with enum, HostDevice::Left and HostDevice::Right?
    pub host_device: u8,
    pub tws_status: bool,
    pub battery: DualBattery,
    pub left_firmware: FirmwareVersion,
    pub right_firmware: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: StereoEqualizerConfiguration,
    pub extra_bands: TwoExtraEqBandsValues,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId,
    pub sound_modes: SoundModesTypeTwo,
    pub gender: Gender,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub custom_button_model: CustomButtonModel,
    pub touch_tone: bool,
    pub charging_case_battery: BatteryLevel,
    pub color: u8,
    pub ldac: bool,
    pub supports_two_cnn_switch: bool,
    pub auto_power_off_switch: bool,
    pub auto_power_off_index: bool,
    pub game_mode_switch: bool,
    pub wear_detection: bool,
    pub side_tone: bool,
}

impl From<A3936StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3936StateUpdatePacket) -> Self {
        Self {
            device_profile: A3936_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration.left,
            sound_modes: Some(SoundModes {
                ambient_sound_mode: packet.sound_modes.ambient_sound_mode,
                transparency_mode: packet.sound_modes.transparency_mode,
                noise_canceling_mode: NoiseCancelingMode::default(),
                custom_noise_canceling: CustomNoiseCanceling::default(),
            }),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.custom_hear_id.into()),
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
        }
    }
}

pub fn take_a3936_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3936StateUpdatePacket, E> {
    context(
        "a3936 state update packet",
        all_consuming(|input| {
            let (input, host_device) = le_u8(input)?;
            let (input, tws_status) = take_bool(input)?;
            let (input, battery) = take_dual_battery(input)?;
            let (input, left_firmware) = take_firmware_version(input)?;
            let (input, right_firmware) = take_firmware_version(input)?;
            let (input, serial_number) = take_serial_number(input)?;
            let (input, (equalizer_configuration, extra_bands)) =
                take_stereo_equalizer_configuration_with_two_extra_bands(10)(input)?;
            let (input, gender) = take_gender(input)?;
            let (input, age_range) = take_age_range(input)?;
            let (input, custom_hear_id) = take_custom_hear_id_without_music_type(8)(input)?;

            // For some reason, an offset value is taken before the custom button model, which refers to how many bytes
            // until the next data to be read. This offset includes the length of the custom button model. Presumably,
            // there are some extra bytes between the button model and the beginning of the next data to be parsed?
            let (input, skip_offset) = le_u8(input)?;
            let remaining_before_button_model = input.len();
            let (input, custom_button_model) = take_custom_button_model(input)?;
            let button_model_size = remaining_before_button_model - input.len();
            let (input, _) = take(
                (skip_offset as usize)
                    .checked_sub(button_model_size)
                    .unwrap_or_default(),
            )(input)?;

            let (input, ambient_sound_mode_cycle) = take_ambient_sound_mode_cycle(input)?;
            let (input, sound_modes) = take_sound_modes_type_two(input)?;
            let (input, touch_tone) = take_bool(input)?;
            let (input, charging_case_battery) = take_battery_level(input)?;
            let (input, color) = le_u8(input)?;
            let (input, ldac) = take_bool(input)?;
            let (input, supports_two_cnn_switch) = take_bool(input)?;
            let (input, auto_power_off_switch) = take_bool(input)?;
            let (input, auto_power_off_index) = take_bool(input)?;
            let (input, game_mode_switch) = take_bool(input)?;
            let (input, wear_detection) = take_bool(input)?;
            let (input, side_tone) = take_bool(input)?;
            Ok((
                input,
                A3936StateUpdatePacket {
                    host_device,
                    tws_status,
                    battery,
                    left_firmware,
                    right_firmware,
                    serial_number,
                    equalizer_configuration,
                    extra_bands,
                    age_range,
                    custom_hear_id,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    gender,
                    custom_button_model,
                    touch_tone,
                    charging_case_battery,
                    color,
                    ldac,
                    supports_two_cnn_switch,
                    auto_power_off_switch,
                    auto_power_off_index,
                    game_mode_switch,
                    wear_detection,
                    side_tone,
                },
            ))
        }),
    )(input)
}

impl A3936StateUpdatePacket {}
