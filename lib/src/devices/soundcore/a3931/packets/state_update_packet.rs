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
        a3931::state::A3931State,
        standard::{
            modules::ModuleCollection,
            packet::{
                Command, Packet,
                inbound::{InboundPacket, TryIntoInboundPacket, state_update_packet},
                outbound::OutboundPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AutoPowerOff, DualBattery, EqualizerConfiguration, MultiButtonConfiguration,
                SoundModes, TwsStatus,
            },
        },
    },
};

// A3931 and A3935 and A3931XR and A3935W
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3931StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub button_configuration: MultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub touch_tone: bool,
    pub auto_power_off: AutoPowerOff,
}

impl InboundPacket for A3931StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3931 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    EqualizerConfiguration::take,
                    MultiButtonConfiguration::take,
                    SoundModes::take,
                    take_bool,
                    take_bool,
                    AutoPowerOff::take,
                ),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    button_configuration,
                    sound_modes,
                    side_tone,
                    touch_tone,
                    auto_power_off,
                )| {
                    Self {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        touch_tone,
                        auto_power_off,
                    }
                },
            )),
        )
        .parse_complete(input)
    }
}

impl OutboundPacket for A3931StateUpdatePacket {
    fn command(&self) -> Command {
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
            .chain(self.button_configuration.bytes())
            .chain(self.sound_modes.bytes())
            .chain([self.side_tone as u8, self.touch_tone as u8])
            .chain(self.auto_power_off.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3931State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3931State>,
        packet: &Packet,
    ) -> device::Result<()> {
        let packet: A3931StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| state.update_from_state_update_packet(packet));
        Ok(())
    }
}

impl ModuleCollection<A3931State> {
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
        let bytes = A3931StateUpdatePacket::default().bytes();
        let (_, packet) = Packet::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3931StateUpdatePacket = packet.try_into_inbound_packet().unwrap();
    }
}
