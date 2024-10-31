use nom::{
    combinator::{all_consuming, map, opt},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::{pair, tuple},
};

use crate::devices::{
    a3030::device_profile::A3030_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
            parsing::{take_bool, ParseResult},
        },
        structures::{
            AgeRange, CustomButtonModel, CustomHearId, DualBattery, EqualizerConfiguration, Gender,
            SoundModes, StereoEqualizerConfiguration,
        },
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct A3030StateUpdatePacket {
    pub host_device: u8,
    pub tws_status: bool,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: CustomHearId,
    pub button_model: CustomButtonModel,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub hear_id_eq_index: Option<[u8; 2]>,
}

impl From<A3030StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3030StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3030_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: Some(packet.age_range),
            gender: Some(packet.gender),
            hear_id: Some(packet.hear_id.into()),
            custom_button_model: Some(packet.button_model),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3030StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<A3030StateUpdatePacket, E> {
        context(
            "a3030 state update packet",
            all_consuming(map(
                tuple((
                    le_u8,
                    take_bool,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    Gender::take,
                    AgeRange::take,
                    CustomHearId::take_with_all_fields,
                    CustomButtonModel::take,
                    SoundModes::take,
                    take_bool,
                    opt(pair(le_u8, le_u8)),
                )),
                |(
                    host_device,
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    button_model,
                    sound_modes,
                    side_tone,
                    hear_id_eq_index,
                )| {
                    A3030StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        hear_id,
                        sound_modes,
                        host_device,
                        button_model,
                        side_tone,
                        hear_id_eq_index: hear_id_eq_index.map(Into::into),
                    }
                },
            )),
        )(input)
    }
}
