use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
    IResult,
};

use crate::devices::{
    a3931::device_profile::A3931_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::{state_update_packet::StateUpdatePacket, InboundPacket},
            parsing::take_bool,
        },
        structures::{
            DualBattery, EqualizerConfiguration, InternalMultiButtonConfiguration, SoundModes,
            StereoEqualizerConfiguration, TwsStatus,
        },
    },
};

// A3931 and A3935 and A3931XR and A3935W
#[derive(Debug, Clone, PartialEq)]
pub struct A3931StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub button_configuration: InternalMultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub touch_tone: bool,
    pub auto_power_off_on: bool,
    pub auto_power_off_index: u8, // 0 to 3
}

impl From<A3931StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3931StateUpdatePacket) -> Self {
        Self {
            device_profile: &A3931_DEVICE_PROFILE,
            tws_status: Some(packet.tws_status),
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: None,
            gender: None,
            hear_id: None,
            button_configuration: Some(packet.button_configuration.into()),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
            sound_modes_type_two: None,
        }
    }
}

impl InboundPacket for A3931StateUpdatePacket {
    fn command() -> crate::devices::standard::structures::Command {
        StateUpdatePacket::command()
    }
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3931StateUpdatePacket, E> {
        context(
            "a3931 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    StereoEqualizerConfiguration::take(8),
                    InternalMultiButtonConfiguration::take,
                    SoundModes::take,
                    take_bool,
                    take_bool,
                    take_bool,
                    le_u8,
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    button_configuration,
                    sound_modes,
                    side_tone,
                    touch_tone,
                    auto_power_off_on,
                    auto_power_off_index,
                )| {
                    A3931StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        touch_tone,
                        auto_power_off_on,
                        auto_power_off_index,
                    }
                },
            )),
        )(input)
    }
}
