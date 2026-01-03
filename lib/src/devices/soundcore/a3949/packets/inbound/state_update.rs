use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    device,
    devices::soundcore::{
        a3949::{self, state::A3949State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
            },
            packet_manager::PacketHandler,
            structures::{
                CommonEqualizerConfiguration, DualBattery, DualFirmwareVersion, GamingMode,
                SerialNumber, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3949StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub gaming_mode: GamingMode,
    pub touch_tone: TouchTone,
}

impl Default for A3949StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: a3949::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            touch_tone: Default::default(),
            gaming_mode: Default::default(),
        }
    }
}

impl FromPacketBody for A3949StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3949 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    CommonEqualizerConfiguration::take,
                    // it's not a right channel, since when setting the eq, the soundcore app only sends one channel
                    take(11usize),
                    ButtonStatusCollection::take(
                        a3949::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    take(4usize),
                    GamingMode::take,
                    TouchTone::take,
                ),
                |(
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _unknown1,
                    button_configuration,
                    _unknown2,
                    gaming_mode,
                    touch_tone,
                )| {
                    Self {
                        tws_status,
                        battery,
                        firmware_version,
                        serial_number,
                        equalizer_configuration,
                        button_configuration,
                        gaming_mode,
                        touch_tone,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3949StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain([0; 11])
            .chain(
                self.button_configuration
                    .bytes(a3949::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain([0; 4])
            .chain(self.gaming_mode.bytes())
            .chain(self.touch_tone.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3949State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3949State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3949StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3949State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::common::packet::inbound::TryToPacket;

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3949StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3949StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
