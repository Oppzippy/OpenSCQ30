use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map, opt},
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3027::state::A3027State,
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AgeRange, BasicHearId, CommonEqualizerConfiguration, FirmwareVersion, Gender,
                SerialNumber, SingleBattery, SoundModes, WearingDetection,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3027StateUpdatePacket {
    pub battery: SingleBattery,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    // Two channels, but the second one is ignored
    pub hear_id: BasicHearId<2, 8>,
    pub sound_modes: SoundModes,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub wearing_detection: WearingDetection,
    // if length >= 72
    pub touch_func: Option<bool>,
}

impl FromPacketBody for A3027StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3027 state update packet",
            all_consuming(map(
                (
                    SingleBattery::take,
                    CommonEqualizerConfiguration::take,
                    Gender::take,
                    AgeRange::take,
                    BasicHearId::take,
                    SoundModes::take,
                    FirmwareVersion::take,
                    SerialNumber::take,
                    WearingDetection::take,
                    opt(take_bool),
                ),
                |(
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    sound_modes,
                    firmware_version,
                    serial_number,
                    wearing_detection,
                    touch_func,
                )| {
                    Self {
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        hear_id,
                        sound_modes,
                        firmware_version,
                        serial_number,
                        wearing_detection,
                        touch_func,
                    }
                },
            )),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3027StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.battery.level.0, self.battery.is_charging as u8]
            .into_iter()
            .chain(self.equalizer_configuration.bytes())
            .chain([self.gender.0])
            .chain([self.age_range.0])
            .chain(self.hear_id.bytes())
            .chain(self.sound_modes.bytes())
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.as_str().as_bytes().iter().copied())
            .chain(self.wearing_detection.bytes())
            .chain(self.touch_func.map(|v| v as u8))
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3027State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3027State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3027StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3027State> {
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
        let bytes = A3027StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3027StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
