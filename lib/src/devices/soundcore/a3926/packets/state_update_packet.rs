use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3926::{self, state::A3926State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command, Packet,
                inbound::{InboundPacket, TryIntoInboundPacket},
                outbound::OutboundPacket,
            },
            packet_manager::PacketHandler,
            structures::{
                AgeRange, BasicHearId, DualBattery, EqualizerConfiguration, Gender, TwsStatus,
                button_configuration_v2::ButtonStatusCollection,
            },
        },
    },
};

// A3926 and A3926Z11
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3926StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId<2, 8>,
    pub button_configuration: ButtonStatusCollection<6>,
}

impl Default for A3926StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            equalizer_configuration: Default::default(),
            gender: Default::default(),
            age_range: Default::default(),
            hear_id: Default::default(),
            button_configuration: a3926::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
        }
    }
}

impl InboundPacket for A3926StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3926 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    EqualizerConfiguration::take,
                    Gender::take,
                    AgeRange::take,
                    BasicHearId::take,
                    ButtonStatusCollection::take(
                        a3926::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                ),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    button_configuration,
                )| {
                    Self {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        hear_id,
                        button_configuration,
                    }
                },
            )),
        )
        .parse_complete(input)
    }
}

impl OutboundPacket for A3926StateUpdatePacket {
    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain([
                self.battery.left.is_charging as u8,
                self.battery.right.is_charging as u8,
                self.battery.left.level.0,
                self.battery.right.level.0,
            ])
            .chain(self.equalizer_configuration.bytes())
            .chain([self.gender.0, self.age_range.0])
            .chain(self.hear_id.bytes())
            .chain(
                self.button_configuration
                    .bytes(a3926::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3926State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3926State>,
        packet: &Packet,
    ) -> device::Result<()> {
        let packet: A3926StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| state.update_from_state_update_packet(packet));
        Ok(())
    }
}

impl ModuleCollection<A3926State> {
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

    use crate::devices::soundcore::common::packet::{
        inbound::TryIntoInboundPacket, outbound::OutboundPacketBytesExt,
    };

    use super::*;

    #[test]
    fn serialize_and_deserialize() {
        let bytes = A3926StateUpdatePacket::default().bytes();
        let (_, packet) = Packet::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3926StateUpdatePacket = packet.try_into_inbound_packet().unwrap();
    }
}
