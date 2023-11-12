use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::devices::{
    a3926::device_profile::A3926_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::state_update_packet::StateUpdatePacket,
            parsing::{
                take_age_range, take_basic_hear_id, take_bool, take_custom_button_model,
                take_dual_battery, take_gender, take_stereo_equalizer_configuration, ParseResult,
            },
        },
        structures::{
            AgeRange, BasicHearId, CustomButtonModel, DualBattery, EqualizerConfiguration, Gender,
        },
    },
};

// A3926 and A3926Z11
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct A3926StateUpdatePacket {
    host_device: u8,
    tws_status: bool,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    gender: Gender,
    age_range: AgeRange,
    hear_id: BasicHearId,
    custom_button_model: CustomButtonModel,
}

impl From<A3926StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3926StateUpdatePacket) -> Self {
        Self {
            device_profile: A3926_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: None,
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.hear_id.into()),
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: None,
            serial_number: None,
        }
    }
}

pub fn take_a3926_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3926StateUpdatePacket, E> {
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
                take_basic_hear_id,
                take_custom_button_model,
            )),
            |(
                host_device,
                tws_status,
                battery,
                equalizer_configuration,
                gender,
                age_range,
                hear_id,
                custom_button_model,
            )| {
                A3926StateUpdatePacket {
                    host_device,
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    custom_button_model,
                }
            },
        )),
    )(input)
}

impl A3926StateUpdatePacket {}
