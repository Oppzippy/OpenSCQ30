use nom::{
    branch::alt,
    combinator::map,
    error::{ContextError, ParseError},
    IResult,
};

use crate::{
    device_profile::DeviceProfile,
    devices::{
        a3027::packets::A3027StateUpdatePacket,
        a3028::packets::A3028StateUpdatePacket,
        a3031::packets::A3031StateUpdatePacket,
        a3033::packets::A3033StateUpdatePacket,
        a3926::packets::A3926StateUpdatePacket,
        a3930::packets::A3930StateUpdatePacket,
        a3931::packets::A3931StateUpdatePacket,
        a3933::packets::inbound::A3933StateUpdatePacket,
        a3936::packets::A3936StateUpdatePacket,
        a3945::packets::A3945StateUpdatePacket,
        a3951::packets::A3951StateUpdatePacket,
        standard::structures::{
            AgeRange, AmbientSoundModeCycle, Battery, Command, EqualizerConfiguration,
            FirmwareVersion, Gender, HearId, MultiButtonConfiguration, SerialNumber, SoundModes,
            SoundModesTypeTwo, TwsStatus,
        },
    },
};

use super::InboundPacket;

#[derive(Debug, Clone)]
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
        alt((
            map(A3027StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3028StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3031StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3033StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3926StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3930StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3931StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3951StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3933StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3936StateUpdatePacket::take, StateUpdatePacket::from),
            map(A3945StateUpdatePacket::take, StateUpdatePacket::from),
        ))(input)
    }
}
