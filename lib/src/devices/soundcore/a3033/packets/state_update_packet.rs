use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    sequence::tuple,
};
use tokio::sync::watch;

use crate::devices::soundcore::{
    a3033::state::A3033State,
    standard::{
        modules::ModuleCollection,
        packet_manager::PacketHandler,
        packets::{
            Packet,
            inbound::{InboundPacket, TryIntoInboundPacket, state_update_packet},
            outbound::OutboundPacket,
            parsing::take_bool,
        },
        structures::{EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery},
    },
};

// A3033 and A3033EU
#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3033StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: EqualizerConfiguration<1, 8>,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub wear_detection: bool,
}

impl InboundPacket for A3033StateUpdatePacket {
    fn command() -> crate::devices::soundcore::standard::structures::Command {
        state_update_packet::COMMAND
    }

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3033StateUpdatePacket, E> {
        context(
            "a3033 state update packet",
            all_consuming(map(
                tuple((
                    SingleBattery::take,
                    EqualizerConfiguration::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    take_bool,
                )),
                |(
                    battery,
                    equalizer_configuration,
                    firmware_version,
                    serial_number,
                    wear_detection,
                )| {
                    A3033StateUpdatePacket {
                        battery,
                        equalizer_configuration,
                        firmware_version,
                        serial_number,
                        wear_detection,
                    }
                },
            )),
        )(input)
    }
}

impl OutboundPacket for A3033StateUpdatePacket {
    fn command(&self) -> crate::devices::soundcore::standard::structures::Command {
        state_update_packet::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.battery.is_charging as u8, self.battery.level.0]
            .into_iter()
            .chain(self.equalizer_configuration.bytes())
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.0.as_bytes().iter().cloned())
            .chain([self.wear_detection as u8])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3033State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3033State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3033StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3033State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            state_update_packet::COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
