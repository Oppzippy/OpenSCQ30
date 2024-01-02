use nom::{
    combinator::{all_consuming, map},
    error::{context, ContextError, ParseError},
    number::complete::le_u8,
    sequence::tuple,
};

use crate::devices::{
    a3931::device_profile::A3931_DEVICE_PROFILE,
    standard::{
        packets::{
            inbound::state_update_packet::StateUpdatePacket,
            parsing::{
                take_bool, take_custom_button_model, take_dual_battery, take_sound_modes,
                take_stereo_equalizer_configuration, ParseResult,
            },
        },
        structures::{CustomButtonModel, DualBattery, EqualizerConfiguration, SoundModes},
    },
};

// A3931 and A3935 and A3931XR and A3935W
#[derive(Debug, Clone, PartialEq)]
pub struct A3931StateUpdatePacket {
    host_device: u8,
    tws_status: bool,
    battery: DualBattery,
    equalizer_configuration: EqualizerConfiguration,
    custom_button_model: CustomButtonModel,
    sound_modes: SoundModes,
    side_tone: bool,
    touch_tone: bool,
    auto_power_off_on: bool,
    auto_power_off_index: u8, // 0 to 3
}

impl From<A3931StateUpdatePacket> for StateUpdatePacket {
    fn from(packet: A3931StateUpdatePacket) -> Self {
        Self {
            device_profile: A3931_DEVICE_PROFILE,
            battery: packet.battery.into(),
            equalizer_configuration: packet.equalizer_configuration,
            sound_modes: Some(packet.sound_modes),
            age_range: None,
            gender: None,
            hear_id: None,
            custom_button_model: Some(packet.custom_button_model),
            firmware_version: None,
            serial_number: None,
            ambient_sound_mode_cycle: None,
        }
    }
}

pub fn take_a3931_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<A3931StateUpdatePacket, E> {
    context(
        "a3931 state update packet",
        all_consuming(map(
            tuple((
                le_u8,
                take_bool,
                take_dual_battery,
                take_stereo_equalizer_configuration(8),
                take_custom_button_model,
                take_sound_modes,
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
                custom_button_model,
                sound_modes,
                side_tone,
                touch_tone,
                auto_power_off_on,
                auto_power_off_index,
            )| {
                A3931StateUpdatePacket {
                    host_device,
                    tws_status,
                    battery,
                    equalizer_configuration,
                    custom_button_model,
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

impl A3931StateUpdatePacket {}
