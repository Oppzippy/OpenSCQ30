use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    sequence::tuple,
};
use tokio::sync::watch;

use crate::devices::soundcore::{
    a3926::state::A3926State,
    standard::{
        modules::ModuleCollection,
        packet_manager::PacketHandler,
        packets::{
            Packet,
            inbound::{InboundPacket, TryIntoInboundPacket, state_update_packet},
            outbound::OutboundPacket,
        },
        structures::{
            AgeRange, BasicHearId, DualBattery, EqualizerConfiguration, Gender,
            MultiButtonConfiguration, TwsStatus,
        },
    },
};

// A3926 and A3926Z11
#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3926StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BasicHearId<2, 8>,
    pub button_configuration: MultiButtonConfiguration,
}

impl InboundPacket for A3926StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3926StateUpdatePacket, E> {
        context(
            "a3926 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    EqualizerConfiguration::take,
                    Gender::take,
                    AgeRange::take,
                    BasicHearId::take,
                    MultiButtonConfiguration::take,
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    hear_id,
                    button_configuration,
                )| {
                    A3926StateUpdatePacket {
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
        )(input)
    }
}

impl OutboundPacket for A3926StateUpdatePacket {
    fn command(&self) -> crate::devices::soundcore::standard::structures::Command {
        state_update_packet::COMMAND
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
            .chain(self.button_configuration.bytes())
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
    ) -> crate::Result<()> {
        let packet: A3926StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| state.update_from_state_update_packet(packet));
        Ok(())
    }
}

impl ModuleCollection<A3926State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            state_update_packet::COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
