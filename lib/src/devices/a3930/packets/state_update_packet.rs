use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u16,
    sequence::tuple,
    IResult,
};

use crate::devices::{
    a3930::device_profile::A3930_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
            parsing::take_bool,
        },
        structures::{
            AgeRange, CustomHearId, DualBattery, EqualizerConfiguration, Gender,
            InternalCustomButtonModel, SoundModes, StereoEqualizerConfiguration, TwsStatus,
        },
    },
};

// A3930
#[derive(Debug, Clone, PartialEq)]
pub struct A3930StateUpdatePacket {
    tws_status: TwsStatus,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    gender: Gender,
    age_range: AgeRange,
    custom_hear_id: CustomHearId,
    custom_button_model: InternalCustomButtonModel,
    sound_modes: SoundModes,
    side_tone: bool,
    // length >= 94
    hear_id_eq_index: Option<u16>,
}

impl From<A3930StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3930StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3930_DEVICE_PROFILE,
            tws_status: Some(packet.tws_status),
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.custom_hear_id.into()),
            custom_button_model: Some(packet.custom_button_model.into()),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3930StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3930StateUpdatePacket, E> {
        context(
            "a3930 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    CustomHearId::take_with_all_fields,
                    InternalCustomButtonModel::take,
                    SoundModes::take,
                    take_bool,
                    opt(le_u16),
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    custom_hear_id,
                    custom_button_model,
                    sound_modes,
                    side_tone,
                    hear_id_eq_index,
                )| {
                    A3930StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        custom_button_model,
                        sound_modes,
                        side_tone,
                        hear_id_eq_index,
                    }
                },
            )),
        )(input)
    }
}
