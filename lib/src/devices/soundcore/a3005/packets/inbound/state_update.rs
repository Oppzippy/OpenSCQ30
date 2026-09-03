use std::iter;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3005::state::A3005State,
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            state::Update,
            structures::{
                AutoPowerOff, CommonEqualizerConfiguration, FirmwareVersion, SerialNumber,
                SingleBattery,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3005StateUpdatePacket {
    pub battery: SingleBattery,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    pub dual_connections_enabled: bool,
    pub auto_power_off: AutoPowerOff,
}

impl FromPacketBody for A3005StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3005 state update packet",
            map(
                (
                    SingleBattery::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    CommonEqualizerConfiguration::take,
                    take(6usize),
                    take_bool,
                    take(4usize),
                    AutoPowerOff::take,
                ),
                |(
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _unknown1,
                    dual_connections_enabled,
                    _unknown2,
                    auto_power_off,
                )| Self {
                    battery,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    dual_connections_enabled,
                    auto_power_off,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3005StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.battery
            .bytes()
            .into_iter()
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::repeat_n(0, 6))
            .chain(iter::once(self.dual_connections_enabled.into()))
            .chain(iter::repeat_n(0, 4))
            .chain(self.auto_power_off.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<A3005State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3005State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3005StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update(packet));
        Ok(())
    }
}

impl ModuleCollection<A3005State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler),
        );
    }
}
