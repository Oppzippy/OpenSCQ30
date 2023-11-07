use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    number::complete::{le_u16, le_u8},
    sequence::tuple,
};

use crate::packets::{
    parsing::{
        take_age_range, take_bool, take_custom_button_model, take_custom_hear_id_with_all_fields,
        take_dual_battery, take_gender, take_sound_modes, take_stereo_equalizer_configuration,
        ParseResult,
    },
    structures::{
        AgeRange, CustomButtonModel, CustomHearId, DeviceFeatureFlags, DualBattery,
        EqualizerConfiguration, Gender, SoundModes,
    },
};

use super::StateUpdatePacket;

// A3930
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct A3930StateUpdatePacket {
    host_device: u8,
    tws_status: bool,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    gender: Gender,
    age_range: AgeRange,
    custom_hear_id: CustomHearId,
    custom_button_model: CustomButtonModel,
    sound_modes: SoundModes,
    side_tone: bool,
    wear_detection: bool,
    touch_tone: bool,
    // length >= 96
    hear_id_eq_index: Option<u16>,
}

impl From<A3930StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3930StateUpdatePacket) -> Self {
        Self {
            feature_flags: DeviceFeatureFlags::SOUND_MODES
                | DeviceFeatureFlags::HEAR_ID
                | DeviceFeatureFlags::EQUALIZER
                | DeviceFeatureFlags::TWO_CHANNEL_EQUALIZER
                | DeviceFeatureFlags::DYNAMIC_RANGE_COMPRESSION,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.custom_hear_id.into()),
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: None,
            serial_number: None,
            dynamic_range_compression_min_firmware_version: None,
        }
    }
}

pub fn take_a3930_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3930StateUpdatePacket, E> {
    context(
        "StateUpdatePacket",
        all_consuming(map(
            tuple((
                le_u8,
                take_bool,
                take_dual_battery,
                take_stereo_equalizer_configuration(8),
                take_gender,
                take_age_range,
                take_custom_hear_id_with_all_fields,
                take_custom_button_model,
                take_sound_modes,
                take_bool,
                take_bool,
                take_bool,
                opt(le_u16),
            )),
            |(
                host_device,
                tws_status,
                battery,
                equalizer_configuration,
                gender,
                age_range,
                custom_hear_id,
                custom_button_model,
                sound_modes,
                side_tone,
                wear_detection,
                touch_tone,
                hear_id_eq_index,
            )| {
                A3930StateUpdatePacket {
                    host_device,
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    custom_hear_id,
                    custom_button_model,
                    sound_modes,
                    side_tone,
                    wear_detection,
                    touch_tone,
                    hear_id_eq_index,
                }
            },
        )),
    )(input)
}

impl A3930StateUpdatePacket {}
