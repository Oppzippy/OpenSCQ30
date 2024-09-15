use nom::{
    branch::alt,
    combinator::map,
    error::{ContextError, ParseError},
};

use crate::{
    device_profiles::DeviceProfile,
    devices::{
        a3027::packets::take_a3027_state_update_packet,
        a3028::packets::take_a3028_state_update_packet,
        a3033::packets::take_a3033_state_update_packet,
        a3926::packets::take_a3926_state_update_packet,
        a3930::packets::take_a3930_state_update_packet,
        a3931::packets::take_a3931_state_update_packet,
        a3933::packets::inbound::take_a3933_state_update_packet,
        a3945::packets::take_a3945_state_update_packet,
        a3951::packets::take_a3951_state_update_packet,
        standard::{
            packets::parsing::ParseResult,
            structures::{
                AgeRange, AmbientSoundModeCycle, Battery, CustomButtonModel,
                EqualizerConfiguration, FirmwareVersion, Gender, HearId, SerialNumber, SoundModes,
                SoundModesTypeTwo,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct StateUpdatePacket {
    pub device_profile: DeviceProfile,
    pub battery: Battery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub sound_modes: Option<SoundModes>,
    pub sound_modes_type_two: Option<SoundModesTypeTwo>,
    pub age_range: Option<AgeRange>,
    pub gender: Option<Gender>,
    pub hear_id: Option<HearId>,
    pub custom_button_model: Option<CustomButtonModel>,
    pub firmware_version: Option<FirmwareVersion>,
    pub serial_number: Option<SerialNumber>,
    pub ambient_sound_mode_cycle: Option<AmbientSoundModeCycle>,
}

pub fn take_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<StateUpdatePacket, E> {
    alt((
        map(take_a3027_state_update_packet, StateUpdatePacket::from),
        map(take_a3028_state_update_packet, StateUpdatePacket::from),
        map(take_a3033_state_update_packet, StateUpdatePacket::from),
        map(take_a3926_state_update_packet, StateUpdatePacket::from),
        map(take_a3930_state_update_packet, StateUpdatePacket::from),
        map(take_a3931_state_update_packet, StateUpdatePacket::from),
        map(take_a3951_state_update_packet, StateUpdatePacket::from),
        map(take_a3933_state_update_packet, StateUpdatePacket::from),
        map(take_a3945_state_update_packet, StateUpdatePacket::from),
    ))(input)
}
