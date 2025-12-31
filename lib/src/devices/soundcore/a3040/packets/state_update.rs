use std::iter;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    multi::count,
    number::complete::be_u32,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3040,
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
                AmbientSoundModeCycle, AutoPowerOff, BatteryLevel, CommonEqualizerConfiguration,
                CommonVolumeAdjustments, CustomHearId, FirmwareVersion, HearIdMusicGenre,
                HearIdType, LimitHighVolume, SerialNumber,
            },
        },
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct A3040StateUpdatePacket {
    pub battery_level: BatteryLevel,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    pub button_configuration: a3040::structures::ButtonConfiguration,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3040::structures::SoundModes,
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub ambient_sound_mode_prompt_tone: bool,
    pub battery_alert_prompt_tone: bool,
    pub hear_id: CustomHearId<2, 10>,
}

impl FromPacketBody for A3040StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3040 state update packet",
            map(
                (
                    BatteryLevel::take,
                    take(1usize), // unknown
                    FirmwareVersion::take,
                    SerialNumber::take,
                    CommonEqualizerConfiguration::take,
                    take(10usize), // equalizer dynamic range compression
                    take(2usize),  // unknown
                    a3040::structures::ButtonConfiguration::take,
                    AmbientSoundModeCycle::take,
                    a3040::structures::SoundModes::take,
                    take(10usize), // unknown
                    AutoPowerOff::take,
                    LimitHighVolume::take,
                    take_bool,    // ambient sound mode prompt
                    take_bool,    // battery alert
                    take(5usize), // unknown
                    take_hear_id,
                ),
                |(
                    battery_level,
                    _unknown1,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _dynamic_range_compression,
                    _unknown2,
                    double_press_action,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    _unknown3,
                    auto_power_off,
                    limit_high_volume,
                    ambient_sound_mode_prompt_tone,
                    battery_alert_prompt_tone,
                    _unknown4,
                    hear_id,
                )| {
                    Self {
                        battery_level,
                        firmware_version,
                        serial_number,
                        equalizer_configuration,
                        button_configuration: double_press_action,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        auto_power_off,
                        limit_high_volume,
                        ambient_sound_mode_prompt_tone,
                        battery_alert_prompt_tone,
                        hear_id,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

pub fn take_hear_id<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], CustomHearId<2, 10>, E> {
    context(
        "custom hear id without music_type",
        map(
            (
                take_bool,
                count(CommonVolumeAdjustments::take_optional, 2),
                be_u32,
                HearIdType::take,
                count(CommonVolumeAdjustments::take_optional, 2),
                take(10usize), // DRC
                HearIdMusicGenre::take_two_bytes,
            ),
            |(
                is_enabled,
                volume_adjustments,
                time,
                hear_id_type,
                custom_volume_adjustments,
                _drc,
                favorite_music_genre,
            )| {
                CustomHearId {
                    is_enabled,
                    volume_adjustments: volume_adjustments
                        .try_into()
                        .expect("count is guaranteed to return a vec with the desired length"),
                    time,
                    hear_id_type,
                    favorite_music_genre,
                    custom_volume_adjustments: custom_volume_adjustments
                        .try_into()
                        .expect("count is guaranteed to return a vec with the desired length"),
                }
            },
        ),
    )
    .parse_complete(input)
}

impl ToPacket for A3040StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        [self.battery_level.0, 0xFF]
            .into_iter()
            .chain(self.firmware_version.to_string().into_bytes())
            .chain(self.serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain([0; 10])
            .chain([0; 2])
            .chain(self.button_configuration.bytes())
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0; 10])
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain([
                self.ambient_sound_mode_prompt_tone.into(),
                self.battery_alert_prompt_tone.into(),
            ])
            .chain([0; 5])
            .chain(iter::once(self.hear_id.is_enabled.into()))
            .chain(self.hear_id.volume_adjustment_bytes())
            .chain(self.hear_id.time.to_be_bytes())
            .chain(iter::once(self.hear_id.hear_id_type as u8))
            .chain(self.hear_id.custom_volume_adjustment_bytes())
            .chain([0; 10]) // DRC but we ignore it so fill with 0s
            .chain(self.hear_id.favorite_music_genre.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<a3040::state::A3040State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<a3040::state::A3040State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3040StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<a3040::state::A3040State> {
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
        let bytes = A3040StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3040StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
