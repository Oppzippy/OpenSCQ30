use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};

use crate::devices::soundcore::{
    a3134::{
        state::A3134State,
        structures::{self, A3134EqualizerConfiguration, BassUp, Lights},
    },
    common::{
        macros::state_update_packet_module,
        packet::{self, Command, inbound::FromPacketBody, outbound::ToPacket},
        structures::{FirmwareVersion, SerialNumber, SingleBattery},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3134StateUpdatePacket {
    pub battery: SingleBattery,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub bass_up: BassUp,
    pub equalizer_configuration: A3134EqualizerConfiguration,
    pub lights: Lights,
}

impl FromPacketBody for A3134StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3134 state update packet",
            map(
                (
                    take(1usize), // unknown
                    SingleBattery::take,
                    take(4usize), // unknown
                    FirmwareVersion::take,
                    SerialNumber::take,
                    BassUp::take,
                    take(4usize), // unknown
                    structures::take_equalizer_configuration,
                    take(4usize), // unknown
                    Lights::take,
                ),
                |(
                    _unknown1,
                    battery,
                    _unknown2,
                    firmware_version,
                    serial_number,
                    bass_up,
                    _unknown3,
                    equalizer_configuration,
                    _unknown4,
                    lights,
                )| Self {
                    battery,
                    firmware_version,
                    serial_number,
                    bass_up,
                    equalizer_configuration,
                    lights,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3134StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [0] // unknown
            .into_iter()
            .chain(self.battery.bytes())
            .chain([0, 1, 1, 2]) // unknown
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.bass_up.bytes())
            .chain([49, 255, 255, 255]) // unknown
            .chain([self.equalizer_configuration.preset_id() as u8])
            .chain(structures::equalizer_band_bytes(
                &self.equalizer_configuration,
            ))
            .chain([0, 0, 0, 86]) // unknown
            .chain(self.lights.bytes())
            .chain([0, 0, 0, 0]) // unknown
            .collect()
    }
}

state_update_packet_module!(A3134State, A3134StateUpdatePacket);
