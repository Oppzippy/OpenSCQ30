use nom::{
    IResult, Parser,
    bytes::complete::take,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::{
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::SerialNumber,
    },
    d5103::{
        state::D5103State,
        structures::{
            BassUp, D5103EqualizerConfiguration, D5103FirmwareVersion, Lights,
            equalizer_band_bytes, take_equalizer_configuration, take_serial_number,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D5103StateUpdatePacket {
    pub unknown1: u8,
    pub battery_level: u8,
    pub unknown2: Vec<u8>,
    pub firmware_version: D5103FirmwareVersion,
    pub serial_number: SerialNumber,
    pub bass_up: BassUp,
    pub unknown3: Vec<u8>,
    pub equalizer_configuration: D5103EqualizerConfiguration,
    pub unknown4: Vec<u8>,
    pub lights: Lights,
    pub unknown5: Vec<u8>,
}

impl Default for D5103StateUpdatePacket {
    fn default() -> Self {
        Self {
            unknown1: 0,
            battery_level: 0,
            unknown2: vec![0, 1, 1, 1, 2],
            firmware_version: D5103FirmwareVersion::default(),
            serial_number: SerialNumber::from("00000000000000000"),
            bass_up: BassUp::default(),
            unknown3: vec![0; 5],
            equalizer_configuration: D5103EqualizerConfiguration::default(),
            unknown4: vec![0; 4],
            lights: Lights::default(),
            unknown5: vec![0; 16],
        }
    }
}

impl FromPacketBody for D5103StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d5103 state update packet",
            nom::combinator::map(
                (
                    take(1usize),
                    le_u8,
                    take(5usize),
                    D5103FirmwareVersion::take,
                    take_serial_number,
                    BassUp::take,
                    take(5usize),
                    take_equalizer_configuration,
                    take(4usize),
                    Lights::take,
                    take(16usize),
                ),
                |(
                    unknown1,
                    battery_level,
                    unknown2,
                    firmware_version,
                    serial_number,
                    bass_up,
                    unknown3,
                    equalizer_configuration,
                    unknown4,
                    lights,
                    unknown5,
                )| Self {
                    unknown1: unknown1[0],
                    battery_level,
                    unknown2: unknown2.to_vec(),
                    firmware_version,
                    serial_number,
                    bass_up,
                    unknown3: unknown3.to_vec(),
                    equalizer_configuration,
                    unknown4: unknown4.to_vec(),
                    lights,
                    unknown5: unknown5.to_vec(),
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for D5103StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.unknown1, self.battery_level]
            .into_iter()
            .chain(self.unknown2.iter().copied())
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.bass_up.bytes())
            .chain(self.unknown3.iter().copied())
            .chain(std::iter::once(
                self.equalizer_configuration.preset_id() as u8
            ))
            .chain(equalizer_band_bytes(&self.equalizer_configuration))
            .chain(self.unknown4.iter().copied())
            .chain(self.lights.bytes())
            .chain(self.unknown5.iter().copied())
            .collect()
    }
}

state_update_packet_module!(D5103State, D5103StateUpdatePacket);
