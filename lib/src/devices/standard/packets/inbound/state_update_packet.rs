use nom::{
    IResult,
    branch::alt,
    combinator::map,
    error::{ContextError, ParseError},
};

use crate::{
    device_profile::DeviceProfile,
    devices::{
        a3936::packets::A3936StateUpdatePacket,
        standard::structures::{
            AgeRange, AmbientSoundModeCycle, Battery, Command, EqualizerConfiguration,
            FirmwareVersion, Gender, HearId, MultiButtonConfiguration, SerialNumber, SoundModes,
            SoundModesTypeTwo, TwsStatus,
        },
    },
};

use super::InboundPacket;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StateUpdatePacket {
    pub device_profile: &'static DeviceProfile,
    pub tws_status: Option<TwsStatus>,
    pub battery: Battery,
    pub equalizer_configuration: EqualizerConfiguration,
    pub sound_modes: Option<SoundModes>,
    pub sound_modes_type_two: Option<SoundModesTypeTwo>,
    pub age_range: Option<AgeRange>,
    pub gender: Option<Gender>,
    pub hear_id: Option<HearId>,
    pub button_configuration: Option<MultiButtonConfiguration>,
    pub firmware_version: Option<FirmwareVersion>,
    pub serial_number: Option<SerialNumber>,
    pub ambient_sound_mode_cycle: Option<AmbientSoundModeCycle>,
}

impl InboundPacket for StateUpdatePacket {
    fn command() -> Command {
        Command::new([0x09, 0xff, 0x00, 0x00, 0x01, 0x01, 0x01])
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], StateUpdatePacket, E> {
        alt((map(A3936StateUpdatePacket::take, StateUpdatePacket::from),))(input)
    }
}
