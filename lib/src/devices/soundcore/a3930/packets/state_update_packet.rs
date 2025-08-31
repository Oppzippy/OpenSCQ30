use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, map, opt},
    error::{ContextError, ParseError, context},
    number::complete::le_u16,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3930::state::A3930State,
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command, Packet,
                inbound::{InboundPacket, TryIntoInboundPacket},
                outbound::OutboundPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AgeRange, CustomHearId, DualBattery, EqualizerConfiguration, Gender,
                MultiButtonConfiguration, SoundModes, TwsStatus, VolumeAdjustments,
            },
        },
    },
};

// A3930
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3930StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId<2, 8>,
    pub button_configuration: MultiButtonConfiguration,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    // length >= 94
    pub hear_id_eq_index: Option<u16>,
}

impl InboundPacket for A3930StateUpdatePacket {
    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3930 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    EqualizerConfiguration::take,
                    Gender::take,
                    AgeRange::take,
                    CustomHearId::take_with_all_fields,
                    MultiButtonConfiguration::take,
                    SoundModes::take,
                    take_bool,
                    opt(le_u16),
                ),
                |(
                    tws_status,
                    battery,
                    equalizer_configuration,
                    gender,
                    age_range,
                    custom_hear_id,
                    button_configuration,
                    sound_modes,
                    side_tone,
                    hear_id_eq_index,
                )| {
                    Self {
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        hear_id_eq_index,
                    }
                },
            )),
        )
        .parse_complete(input)
    }
}

impl OutboundPacket for A3930StateUpdatePacket {
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
            .chain([self.custom_hear_id.is_enabled as u8])
            .chain(
                self.custom_hear_id
                    .volume_adjustments
                    .iter()
                    .flat_map(|v| v.bytes()),
            )
            .chain(self.custom_hear_id.time.to_le_bytes())
            .chain([
                self.custom_hear_id.hear_id_type.0,
                self.custom_hear_id.hear_id_music_type.0,
            ])
            .chain(
                self.custom_hear_id
                    .custom_volume_adjustments
                    .as_ref()
                    .unwrap_or(&[
                        VolumeAdjustments::new([0; 8]),
                        VolumeAdjustments::new([0; 8]),
                    ])
                    .iter()
                    .flat_map(|v| v.bytes()),
            )
            .chain(self.button_configuration.bytes())
            .chain(self.sound_modes.bytes())
            .chain([self.side_tone as u8])
            .chain(
                self.hear_id_eq_index
                    .map(u16::to_le_bytes)
                    .into_iter()
                    .flatten(),
            )
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3930State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3930State>,
        packet: &Packet,
    ) -> device::Result<()> {
        let packet: A3930StateUpdatePacket = packet.try_into_inbound_packet()?;
        state.send_modify(|state| state.update_from_state_update_packet(packet));
        Ok(())
    }
}

impl ModuleCollection<A3930State> {
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
        let bytes = A3930StateUpdatePacket::default().bytes();
        let (_, packet) = Packet::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3930StateUpdatePacket = packet.try_into_inbound_packet().unwrap();
    }
}
