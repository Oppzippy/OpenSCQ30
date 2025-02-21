use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    sequence::tuple,
};

use crate::devices::{
    a3926::device_profile::A3926_DEVICE_PROFILE,
    standard::{
        packets::inbound::{InboundPacket, state_update_packet::StateUpdatePacket},
        structures::{
            AgeRange, BasicHearId, DualBattery, EqualizerConfiguration, Gender,
            InternalMultiButtonConfiguration, StereoEqualizerConfiguration, TwsStatus,
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
    pub button_configuration: InternalMultiButtonConfiguration,
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
            button_configuration: Some(packet.button_configuration.into()),
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
                    InternalMultiButtonConfiguration::take,
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    button_configuration,
                )| {
                    A3926StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        hear_id,
                        button_configuration,
                    }
                },
            )),
        )(input)
    }
}
