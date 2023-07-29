use nom::{
    combinator::{map, opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::packets::{
    parsing::{
        take_age_range, take_basic_hear_id, take_bool, take_equalizer_configuration,
        take_firmware_version, take_serial_number, take_single_battery, take_sound_modes,
        ParseResult,
    },
    structures::{
        AgeRange, BasicHearId, DeviceFeatureFlags, EqualizerConfiguration, FirmwareVersion,
        SerialNumber, SingleBattery, SoundModes,
    },
};

use super::StateUpdatePacket;

// A3027 and A3030
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct A3027StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: u8,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub wear_detection: bool,
    // if length >= 72
    pub touch_func: bool,
}

impl From<A3027StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3027StateUpdatePacket) -> Self {
        Self {
            feature_flags: DeviceFeatureFlags::SOUND_MODES
                | DeviceFeatureFlags::NOISE_CANCELING_MODE
                | DeviceFeatureFlags::EQUALIZER
                | DeviceFeatureFlags::WEAR_DETECTION,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            custom_hear_id: Some(packet.hear_id.into()),
            custom_button_model: None,
            firmware_version: Some(packet.firmware_version),
            serial_number: Some(packet.serial_number),
        }
    }
}

pub fn take_a3027_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3027StateUpdatePacket, E> {
    context(
        "StateUpdatePacket",
        map(
            tuple((
                take_single_battery,
                take_equalizer_configuration,
                le_u8,
                take_age_range,
                take_basic_hear_id,
                take_sound_modes,
                take_firmware_version,
                take_serial_number,
                take_bool,
                opt(take_bool),
            )),
            |(
                battery,
                equalizer_configuration,
                gender,
                age_range,
                hear_id,
                sound_modes,
                firmware_version,
                serial_number,
                wear_detection,
                touch_func,
            )| {
                A3027StateUpdatePacket {
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    sound_modes,
                    firmware_version,
                    serial_number,
                    wear_detection,
                    touch_func: touch_func.unwrap_or_default(),
                }
            },
        ),
    )(input)
}
