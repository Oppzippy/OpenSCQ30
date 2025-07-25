use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3004::state::A3004State,
        standard::{
            modules::ModuleCollection,
            packet::{
                Command, Packet,
                inbound::{InboundPacket, TryIntoInboundPacket, state_update_packet},
                outbound::OutboundPacket,
            },
            packet_manager::PacketHandler,
            structures::{
                EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery, SoundModes,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3004StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration<1, 10>,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
}

impl InboundPacket for A3004StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3004 state update packet",
            map(
                (
                    SingleBattery::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    SoundModes::take,
                ),
                |(
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    sound_modes,
                )| {
                    Self {
                        battery,
                        equalizer_configuration,
                        sound_modes,
                        firmware_version,
                        serial_number,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl OutboundPacket for A3004StateUpdatePacket {
    fn command(&self) -> Command {
        state_update_packet::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.battery.level.0, self.battery.is_charging as u8]
            .into_iter()
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.as_str().as_bytes().iter().cloned())
            .chain(self.equalizer_configuration.bytes())
            .chain(self.sound_modes.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3004State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3004State>,
        packet: &Packet,
    ) -> device::Result<()> {
        let packet: A3004StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3004State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            state_update_packet::COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use crate::devices::soundcore::standard::packet::{
        inbound::TryIntoInboundPacket, outbound::OutboundPacketBytesExt,
    };

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3004StateUpdatePacket::default().bytes();
        let (_, packet) = Packet::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3004StateUpdatePacket = packet.try_into_inbound_packet().unwrap();
    }
}
