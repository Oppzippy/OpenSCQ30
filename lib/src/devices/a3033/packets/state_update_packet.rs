use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    sequence::tuple,
};

use crate::devices::{
    a3033::device_profile::A3033_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::state_update_packet::StateUpdatePacket,
            parsing::{take_bool, ParseResult},
        },
        structures::{EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery},
    },
};

// A3033 and A3033EU
#[derive(Debug, Clone, PartialEq)]
pub struct A3033StateUpdatePacket {
    battery: SingleBattery,
    equalizer_configuration: EqualizerConfiguration,
    firmware_version: FirmwareVersion,
    serial_number: SerialNumber,
    wear_detection: bool,
}

impl From<A3033StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3033StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3033_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: None,
            age_range: None,
            gender: None,
            hear_id: None,
            custom_button_model: None,
            firmware_version: Some(packet.firmware_version),
            serial_number: Some(packet.serial_number),
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl A3033StateUpdatePacket {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<A3033StateUpdatePacket, E> {
        context(
            "a3033 state update packet",
            all_consuming(map(
                tuple((
                    SingleBattery::take,
                    EqualizerConfiguration::take(8),
                    FirmwareVersion::take,
                    SerialNumber::take,
                    take_bool,
                )),
                |(
                    battery,
                    equalizer_configuration,
                    firmware_version,
                    serial_number,
                    wear_detection,
                )| {
                    A3033StateUpdatePacket {
                        battery,
                        equalizer_configuration,
                        firmware_version,
                        serial_number,
                        wear_detection,
                    }
                },
            )),
        )(input)
    }
}
