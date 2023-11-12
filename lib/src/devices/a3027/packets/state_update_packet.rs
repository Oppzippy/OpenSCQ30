use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::devices::{
    a3027::device_profile::A3027_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::state_update_packet::StateUpdatePacket,
            parsing::{
                take_age_range, take_basic_hear_id, take_bool, take_equalizer_configuration,
                take_firmware_version, take_gender, take_serial_number, take_single_battery,
                take_sound_modes, ParseResult,
            },
        },
        structures::{
            AgeRange, BasicHearId, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
            SingleBattery, SoundModes,
        },
    },
};

// A3027 and A3030
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct A3027StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
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
            device_profile: A3027_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.hear_id.into()),
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
        all_consuming(map(
            tuple((
                take_single_battery,
                take_equalizer_configuration(8),
                take_gender,
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
        )),
    )(input)
}
