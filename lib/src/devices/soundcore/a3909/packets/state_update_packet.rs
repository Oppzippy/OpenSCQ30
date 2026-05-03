use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    number::complete::be_u32,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3909::state::A3909State,
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
                AgeRange, CommonEqualizerConfiguration, DualBattery, Gender, SoundModes, TwsStatus,
            },
        },
    },
};

// A3909 (Soundcore Liberty 2 Pro)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3909StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id_is_enabled: bool,
    pub hear_id_time: u32,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
}

impl Default for A3909StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            equalizer_configuration: Default::default(),
            gender: Default::default(),
            age_range: Default::default(),
            hear_id_is_enabled: Default::default(),
            hear_id_time: Default::default(),
            sound_modes: Default::default(),
            side_tone: Default::default(),
        }
    }
}

impl FromPacketBody for A3909StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3909 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    CommonEqualizerConfiguration::take,
                    Gender::take,
                    AgeRange::take,
                    take_bool,
                    be_u32,
                    SoundModes::take,
                    take_bool,
                ),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id_is_enabled,
                    hear_id_time,
                    sound_modes,
                    side_tone,
                )| {
                    Self {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        hear_id_is_enabled,
                        hear_id_time,
                        sound_modes,
                        side_tone,
                    }
                },
            )),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3909StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain([self.gender.0, self.age_range.0])
            .chain([self.hear_id_is_enabled as u8])
            .chain(self.hear_id_time.to_be_bytes())
            .chain(self.sound_modes.bytes())
            .chain([self.side_tone as u8])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3909State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3909State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3909StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update(packet));
        Ok(())
    }
}

impl ModuleCollection<A3909State> {
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
        let bytes = A3909StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3909StateUpdatePacket = packet.try_to_packet().unwrap();
    }

    #[test]
    fn parse_live_packet() {
        // Captured from a real Liberty 2 Pro (A3909) device
        let body: &[u8] = &[
            0x01, 0x01, 0x05, 0x05, 0x00, 0x00, 0xfe, 0xfe, 0x01, 0x01, 0x01, 0x0c, 0x0c, 0x0c,
            0x0f, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x69,
            0x8e, 0xa8, 0x67, 0x02, 0x01, 0x03, 0x00, 0x00,
        ];
        let result = A3909StateUpdatePacket::take::<VerboseError<_>>(body);
        assert!(result.is_ok(), "parse failed: {:?}", result.err());
        let (remaining, packet) = result.unwrap();
        assert!(remaining.is_empty());
        assert!(packet.tws_status.is_connected);
        assert_eq!(packet.battery.left.level.0, 5);
        assert_eq!(packet.battery.right.level.0, 5);
        assert!(!packet.hear_id_is_enabled);
        assert_eq!(packet.hear_id_time, 0x698ea867);
    }
}
