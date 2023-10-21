mod a3027;
mod a3028;
mod a3033;
mod a3926;
mod a3930;
mod a3931;
mod a3951;

pub(crate) use a3027::*;
pub(crate) use a3028::*;
pub(crate) use a3033::*;
pub(crate) use a3926::*;
pub(crate) use a3930::*;
pub(crate) use a3931::*;
pub(crate) use a3951::*;
use nom::{
    branch::alt,
    combinator::map,
    error::{context, ContextError, ParseError},
};

use crate::packets::{
    parsing::ParseResult,
    structures::{
        AgeRange, Battery, CustomButtonModel, DeviceFeatureFlags, EqualizerConfiguration,
        FirmwareVersion, Gender, HearId, SerialNumber, SoundModes,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateUpdatePacket {
    pub feature_flags: DeviceFeatureFlags,
    pub battery: Battery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub sound_modes: Option<SoundModes>,
    pub age_range: Option<AgeRange>,
    pub gender: Option<Gender>,
    pub hear_id: Option<HearId>,
    pub custom_button_model: Option<CustomButtonModel>,
    pub firmware_version: Option<FirmwareVersion>,
    pub serial_number: Option<SerialNumber>,
    pub dynamic_range_compression_min_firmware_version: Option<FirmwareVersion>,
}

pub fn take_state_update_packet<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> ParseResult<StateUpdatePacket, E> {
    context("state update packet", |input| {
        alt((
            map(take_a3027_state_update_packet, StateUpdatePacket::from),
            map(take_a3028_state_update_packet, StateUpdatePacket::from),
            map(take_a3033_state_update_packet, StateUpdatePacket::from),
            map(take_a3926_state_update_packet, StateUpdatePacket::from),
            map(take_a3930_state_update_packet, StateUpdatePacket::from),
            map(take_a3931_state_update_packet, StateUpdatePacket::from),
            map(take_a3951_state_update_packet, StateUpdatePacket::from),
        ))(input)
    })(input)
}
