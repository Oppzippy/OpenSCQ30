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
        a3948::state::A3948State,
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command, Packet,
                inbound::{InboundPacket, TryIntoInboundPacket},
                outbound::OutboundPacket,
            },
            packet_manager::PacketHandler,
            structures::{
                DualBattery, DualFirmwareVersion, EqualizerConfiguration, SerialNumber, TouchTone,
                TwsStatus,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3948StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub touch_tone: TouchTone,
}

impl InboundPacket for A3948StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3948 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    EqualizerConfiguration::take,
                    take(1usize),  // padding
                    take(12usize), // TODO button configuration
                    take(5usize),  // padding
                    TouchTone::take,
                    take(15usize), // padding
                ),
                |(
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _padding0,
                    todo_button_configuration,
                    _padding1,
                    touch_tone,
                    _padding2,
                )| {
                    Self {
                        tws_status,
                        battery,
                        firmware_version,
                        serial_number,
                        equalizer_configuration,
                        touch_tone,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl OutboundPacket for A3948StateUpdatePacket {
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
            .chain(std::iter::once(0))
            .chain([0; 12]) // TODO button configuration
            .chain([0; 5]) // padding
            .chain(self.touch_tone.bytes())
            .chain([0; 15]) // padding
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3948State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3948State>,
        packet: &Packet,
    ) -> device::Result<()> {
        let packet: A3948StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3948State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
