use async_trait::async_trait;
use nom::{
    IResult,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
    sequence::tuple,
};
use tokio::sync::watch;

use crate::devices::soundcore::{
    a3031::state::A3031State,
    standard::{
        modules::ModuleCollection,
        packet_manager::PacketHandler,
        packets::{
            Packet,
            inbound::{InboundPacket, TryIntoInboundPacket, state_update_packet},
            outbound::OutboundPacket,
            parsing::take_bool,
        },
        structures::{
            Command, DualBattery, EqualizerConfiguration, MultiButtonConfiguration, SoundModes,
            TwsStatus,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct A3031StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub button_configuration: MultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub touch_tone: bool,
    pub auto_power_off_on: bool,
    pub auto_power_off_on_index: u8,
}

impl InboundPacket for A3031StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], A3031StateUpdatePacket, E> {
        context(
            "a3031 state update packet",
            all_consuming(map(
                tuple((
                    TwsStatus::take,
                    DualBattery::take,
                    EqualizerConfiguration::take,
                    MultiButtonConfiguration::take,
                    SoundModes::take,
                    take_bool,
                    take_bool,
                    take_bool,
                    le_u8,
                )),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    button_configuration,
                    sound_modes,
                    side_tone,
                    touch_tone,
                    auto_power_off_on,
                    auto_power_off_on_index,
                )| {
                    A3031StateUpdatePacket {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        touch_tone,
                        auto_power_off_on,
                        auto_power_off_on_index,
                    }
                },
            )),
        )(input)
    }
}

impl OutboundPacket for A3031StateUpdatePacket {
    fn command(&self) -> Command {
        state_update_packet::COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain([
                self.battery.left.level.0,
                self.battery.right.level.0,
                self.battery.left.is_charging as u8,
                self.battery.right.is_charging as u8,
            ])
            .chain(self.equalizer_configuration.bytes())
            .chain(self.button_configuration.bytes())
            .chain(self.sound_modes.bytes())
            .chain([
                self.side_tone as u8,
                self.touch_tone as u8,
                self.auto_power_off_on as u8,
                self.auto_power_off_on_index,
            ])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3031State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3031State>,
        packet: &Packet,
    ) -> crate::Result<()> {
        let packet: A3031StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| state.update_from_state_update_packet(packet));
        Ok(())
    }
}

impl ModuleCollection<A3031State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            state_update_packet::COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
