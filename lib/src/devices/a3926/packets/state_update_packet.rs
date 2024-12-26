use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    sequence::tuple,
    IResult,
};

use crate::devices::{
    a3926::device_profile::A3926_DEVICE_PROFILE,
    standard::{
        packets::inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
        structures::{
            AgeRange, BasicHearId, DualBattery, EqualizerConfiguration, Gender,
            InternalCustomButtonModel, StereoEqualizerConfiguration, TwsStatus,
        },
    },
};

// A3926 and A3926Z11
#[derive(Debug, Clone, PartialEq)]
pub struct A3926StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId,
    pub custom_button_model: InternalCustomButtonModel,
}

impl From<A3926StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3926StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3926_DEVICE_PROFILE,
            tws_status: Some(packet.tws_status),
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: None,
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.hear_id.into()),
            custom_button_model: Some(packet.custom_button_model.into()),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3926StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3926StateUpdatePacket, E> {
        context(
            "a3926 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    BasicHearId::take,
                    InternalCustomButtonModel::take,
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    custom_button_model,
                )| {
                    A3926StateUpdatePacket {
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
}
