use std::iter;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3116::{self, state::A3116State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
            },
            packet_manager::PacketHandler,
            structures::{EqualizerConfiguration, FirmwareVersion, SerialNumber, SingleBattery},
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3116StateUpdatePacket {
    pub battery: SingleBattery,
    pub volume: a3116::structures::Volume,
    pub auto_power_off_duration: a3116::structures::AutoPowerOffDuration,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<1, 9, -6, 6, 0>,
}

impl FromPacketBody for A3116StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3116 state update packet",
            map(
                (
                    SingleBattery::take,
                    a3116::structures::Volume::take,
                    le_u8, // unknown
                    a3116::structures::AutoPowerOffDuration::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    a3116::structures::take_equalizer_configuration,
                ),
                |(
                    battery,
                    volume,
                    _unknown1,
                    auto_power_off_duration,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                )| {
                    Self {
                        battery,
                        volume,
                        auto_power_off_duration,
                        firmware_version,
                        serial_number,
                        equalizer_configuration,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3116StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    // SingleBattery::take,
    // a3116::structures::Volume::take,
    // le_u8, // unknown
    // a3116::structures::AutoPowerOffDuration::take,
    // FirmwareVersion::take,
    // SerialNumber::take,
    // a3116::structures::EqualizerConfiguration::take,
    fn body(&self) -> Vec<u8> {
        [self.battery.is_charging as u8, self.battery.level.0]
            .into_iter()
            .chain(self.volume.bytes())
            .chain(iter::once(0)) // unknown
            .chain(self.auto_power_off_duration.bytes())
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.as_str().as_bytes().iter().copied())
            .chain(iter::once(self.equalizer_configuration.preset_id() as u8))
            .chain(
                self.equalizer_configuration
                    .volume_adjustments()
                    .iter()
                    .flat_map(|v| v.bytes()),
            )
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3116State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3116State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3116StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3116State> {
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
        let bytes = A3116StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3116StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
