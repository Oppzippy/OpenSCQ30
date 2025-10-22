use async_trait::async_trait;
use nom::{
    IResult, Parser,
    combinator::{all_consuming, opt},
    error::{ContextError, ParseError, context},
    number::complete::{le_u8, le_u16},
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3951::{self, state::A3951State},
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
                AgeRange, CustomHearId, DualBattery, EqualizerConfiguration, Gender, SoundModes,
                TouchTone, TwsStatus, VolumeAdjustments,
                button_configuration::ButtonStatusCollection,
            },
        },
    },
};

// A3951
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3951StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub equalizer_configuration: EqualizerConfiguration<2, 8>,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub custom_hear_id: CustomHearId<2, 8>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub sound_modes: SoundModes,
    pub side_tone: bool,
    pub wear_detection: bool,
    pub touch_tone: TouchTone,
    pub hear_id_eq_preset: Option<u16>,
    pub supports_new_battery: bool, // yes if packet is >98, don't parse
    pub left_new_battery: u8,       // 0 to 9
    pub right_new_battery: u8,      // 0 to 9
}

impl Default for A3951StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            equalizer_configuration: Default::default(),
            gender: Default::default(),
            age_range: Default::default(),
            custom_hear_id: Default::default(),
            button_configuration: a3951::BUTTON_SETTINGS.default_status_collection(),
            sound_modes: Default::default(),
            side_tone: Default::default(),
            wear_detection: Default::default(),
            touch_tone: Default::default(),
            hear_id_eq_preset: Default::default(),
            supports_new_battery: Default::default(),
            left_new_battery: Default::default(),
            right_new_battery: Default::default(),
        }
    }
}

impl FromPacketBody for A3951StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3951 state update packet",
            all_consuming(|input| {
                // required fields
                let (
                    input,
                    (
                        tws_status,
                        battery,
                        equalizer_configuration,
                        gender,
                        age_range,
                        custom_hear_id,
                        button_configuration,
                        sound_modes,
                        side_tone,
                        wear_detection,
                        touch_tone,
                    ),
                ) = (
                    TwsStatus::take,
                    DualBattery::take,
                    EqualizerConfiguration::take,
                    Gender::take,
                    AgeRange::take,
                    CustomHearId::take_with_all_fields,
                    ButtonStatusCollection::take(a3951::BUTTON_SETTINGS.parse_settings()),
                    SoundModes::take,
                    take_bool,       // side tone
                    take_bool,       // wear detection
                    TouchTone::take, // touch tone
                )
                    .parse_complete(input)?;

                // >=96 length optional fields
                let (input, hear_id_eq_preset) = opt(le_u16).parse_complete(input)?;

                // >=98 length optional fields
                let (input, new_battery) = opt((le_u8, le_u8)).parse_complete(input)?;

                Ok((
                    input,
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
                        wear_detection,
                        touch_tone,
                        hear_id_eq_preset,
                        supports_new_battery: new_battery.is_some(),
                        left_new_battery: new_battery.as_ref().map(|b| b.0).unwrap_or_default(),
                        right_new_battery: new_battery.as_ref().map(|b| b.1).unwrap_or_default(),
                    },
                ))
            }),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3951StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

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
            .chain(
                self.button_configuration
                    .bytes(a3951::BUTTON_SETTINGS.parse_settings()),
            )
            .chain(self.sound_modes.bytes())
            .chain([
                self.side_tone as u8,
                self.wear_detection as u8,
                self.touch_tone as u8,
            ])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3951State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3951State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3951StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update_from_state_update_packet(packet));
        Ok(())
    }
}

impl ModuleCollection<A3951State> {
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
        let bytes = A3951StateUpdatePacket::default().to_packet().bytes();
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3951StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
