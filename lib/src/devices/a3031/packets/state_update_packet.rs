use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::devices::{
    a3031::device_profile::A3031_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
            parsing::{take_bool, ParseResult},
        },
        structures::{
            CustomButtonModel, DualBattery, EqualizerConfiguration, SoundModes,
            StereoEqualizerConfiguration,
        },
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct A3031StateUpdatePacket {
    pub host_device: u8,
    pub tws_status: bool,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub button_model: CustomButtonModel,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub touch_tone: bool,
    pub auto_power_off_on: bool,
    pub auto_power_off_on_index: u8,
}

impl From<A3031StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3031StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3031_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: None,
            gender: None,
            hear_id: None,
            custom_button_model: Some(packet.button_model),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3031StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> ParseResult<A3031StateUpdatePacket, E> {
        context(
            "a3031 state update packet",
            all_consuming(map(
                tuple((
                    le_u8,
                    take_bool,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    CustomButtonModel::take,
                    SoundModes::take,
                    take_bool,
                    take_bool,
                    take_bool,
                    le_u8,
                )),
                |(
                    host_device,
                    tws_status,
                    battery,
                    equalizer_configuration,
                    button_model,
                    sound_modes,
                    side_tone,
                    touch_tone,
                    auto_power_off_on,
                    auto_power_off_on_index,
                )| {
                    A3031StateUpdatePacket {
                        host_device,
                        tws_status,
                        battery,
                        equalizer_configuration,
                        button_model,
                        sound_modes,
                        side_tone,
                        touch_tone,
                        auto_power_off_on,
                        auto_power_off_on_index,
                    }
                },
            )),
        )(input)
    }
}
