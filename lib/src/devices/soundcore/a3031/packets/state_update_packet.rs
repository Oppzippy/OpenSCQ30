use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map},
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::devices::soundcore::{
    a3031::{self, state::A3031State},
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
            AutoPowerOff, CommonEqualizerConfiguration, DualBattery, SoundModes, TouchTone,
            TwsStatus, button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3031StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 8>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub sound_modes: SoundModes,
    pub auto_power_off: AutoPowerOff,
    pub side_tone: bool,
    pub touch_tone: TouchTone,
}

impl Default for A3031StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: a3031::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            sound_modes: Default::default(),
            auto_power_off: Default::default(),
            side_tone: Default::default(),
            touch_tone: Default::default(),
        }
    }
}

impl FromPacketBody for A3031StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3031 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    CommonEqualizerConfiguration::take,
                    ButtonStatusCollection::take(
                        a3031::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    SoundModes::take,
                    take_bool,
                    TouchTone::take,
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

impl ToPacket for A3031StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
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
            .chain(
                self.button_configuration
                    .bytes(a3031::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.sound_modes.bytes())
            .chain([self.side_tone as u8, self.touch_tone.0.into()])
            .chain(self.auto_power_off.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3031State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3031State>,
        packet: &packet::Inbound,
    ) -> crate::api::device::Result<()> {
        let packet: A3031StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update_from_state_update_packet(packet));
        Ok(())
    }
}

impl ModuleCollection<A3031State> {
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
        let bytes = A3031StateUpdatePacket::default().to_packet().bytes();
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3031StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
