use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u16,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3909::{self, state::A3909State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
            },
            packet_manager::PacketHandler,
            state::Update,
            structures::{
                AgeRange, DualBattery, Gender, TwsStatus, VolumeAdjustments,
                button_configuration::ButtonStatusCollection,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3909StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: a3909::structures::EqualizerConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: a3909::structures::HearId,
    pub buttons: ButtonStatusCollection<4>,
}

impl Default for A3909StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            equalizer_configuration: Default::default(),
            gender: Default::default(),
            age_range: Default::default(),
            hear_id: Default::default(),
            buttons: a3909::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
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
            map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    le_u16,
                    Gender::take,
                    AgeRange::take,
                    a3909::structures::HearId::take,
                    ButtonStatusCollection::take(
                        a3909::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                ),
                |(tws_status, battery, eq_preset_id, gender, age_range, hear_id, buttons)| Self {
                    tws_status,
                    battery,
                    equalizer_configuration: a3909::structures::EqualizerConfiguration::new(
                        eq_preset_id,
                        guess_volume_adjustments(eq_preset_id)
                            .map(|volume_adjustments| [volume_adjustments, volume_adjustments])
                            .unwrap_or_default(),
                    ),
                    gender,
                    age_range,
                    hear_id,
                    buttons,
                },
            ),
        )
        .parse_complete(input)
    }
}

fn guess_volume_adjustments(preset_id: u16) -> Option<VolumeAdjustments<8, -12, 12, 0>> {
    a3909::modules::equalizer::PRESETS
        .iter()
        .find(|preset| preset.id == preset_id)
        .map(|preset| preset.volume_adjustments)
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
            .chain(self.equalizer_configuration.preset_id().to_le_bytes())
            .chain([self.gender.0, self.age_range.0])
            .chain(self.hear_id.bytes())
            .chain(
                self.buttons
                    .bytes(a3909::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .collect()
    }
}

struct StateUpdatePacketHandler;

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
            Box::new(StateUpdatePacketHandler),
        );
    }
}
