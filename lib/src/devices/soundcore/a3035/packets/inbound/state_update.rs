use std::iter;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::be_u32,
};
use tokio::sync::watch;

use crate::{
    api::device,
    devices::soundcore::{
        a3035,
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
                AmbientSoundModeCycle, AutoPlayPause, AutoPowerOff, BatteryLevel,
                CommonEqualizerConfiguration, CommonVolumeAdjustments, CustomHearId,
                FirmwareVersion, HearIdMusicGenre, HearIdType, LimitHighVolume, SerialNumber,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3035StateUpdatePacket {
    pub battery_level: BatteryLevel,
    pub firmware_version: FirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<1, 10>,
    pub hear_id: CustomHearId<1, 10>,
    pub button_configuration: a3035::structures::ButtonConfiguration,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3035::structures::SoundModes,
    pub auto_play_pause: AutoPlayPause,
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub ambient_sound_mode_voice_prompt: a3035::structures::AmbientSoundModeVoicePrompt,
    pub battery_alert: a3035::structures::BatteryAlert,
    pub ldac: bool,
    pub dual_connections: bool,
}

impl FromPacketBody for A3035StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3035 state update packet",
            map(
                (
                    BatteryLevel::take,
                    take(1usize), // unknown
                    FirmwareVersion::take,
                    SerialNumber::take,
                    CommonEqualizerConfiguration::take,
                    take(2usize), // unknown
                    take_hear_id,
                    take(3usize), // unknown
                    a3035::structures::ButtonConfiguration::take,
                    AmbientSoundModeCycle::take,
                    a3035::structures::SoundModes::take,
                    take(1usize), //unknown
                    AutoPlayPause::take,
                    take(4usize), //unknown
                    take_bool,    // LDAC
                    take_bool,    // dual connections
                    take(2usize), // unknown
                    AutoPowerOff::take,
                    LimitHighVolume::take,
                    a3035::structures::AmbientSoundModeVoicePrompt::take,
                    a3035::structures::BatteryAlert::take,
                ),
                |(
                    battery_level,
                    _unknown1,
                    firmware_version,
                    serial_number,
                    equalizer_configuration,
                    _unknown2,
                    hear_id,
                    _unknown3,
                    button_configuration,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    _unknown4,
                    auto_play_pause,
                    _unkonwn5,
                    ldac,
                    dual_connections,
                    _unknown6,
                    auto_power_off,
                    limit_high_volume,
                    ambient_sound_mode_voice_prompt,
                    battery_alert,
                )| {
                    Self {
                        battery_level,
                        firmware_version,
                        serial_number,
                        equalizer_configuration,
                        hear_id,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        auto_play_pause,
                        ldac,
                        dual_connections,
                        auto_power_off,
                        limit_high_volume,
                        ambient_sound_mode_voice_prompt,
                        battery_alert,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

pub fn take_hear_id<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
    input: &'a [u8],
) -> IResult<&'a [u8], CustomHearId<1, 10>, E> {
    context(
        "custom hear id without music_type",
        map(
            (
                take_bool,
                CommonVolumeAdjustments::take,
                be_u32,
                HearIdType::take,
                CommonVolumeAdjustments::take,
                HearIdMusicGenre::take_one_byte,
            ),
            |(
                is_enabled,
                volume_adjustments,
                time,
                hear_id_type,
                custom_volume_adjustments,
                favorite_music_genre,
            )| {
                CustomHearId {
                    is_enabled,
                    volume_adjustments: [Some(volume_adjustments)],
                    time,
                    hear_id_type,
                    custom_volume_adjustments: [Some(custom_volume_adjustments)],
                    favorite_music_genre,
                }
            },
        ),
    )
    .parse_complete(input)
}

impl ToPacket for A3035StateUpdatePacket {
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
            .chain([0; 2])
            .chain(iter::once(self.hear_id.is_enabled.into()))
            .chain(self.hear_id.volume_adjustment_bytes())
            .chain(self.hear_id.time.to_be_bytes())
            .chain(iter::once(self.hear_id.hear_id_type as u8))
            .chain(self.hear_id.custom_volume_adjustment_bytes())
            .chain(iter::once(self.hear_id.favorite_music_genre.single_byte()))
            .chain([0; 3])
            .chain(self.button_configuration.bytes())
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0; 1])
            .chain(self.auto_play_pause.bytes())
            .chain([0; 4])
            .chain(iter::once(self.ldac.into()))
            .chain(iter::once(self.dual_connections.into()))
            .chain([0; 2])
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain(self.ambient_sound_mode_voice_prompt.bytes())
            .chain(self.battery_alert.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<a3035::state::A3035State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<a3035::state::A3035State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3035StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<a3035::state::A3035State> {
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
        let bytes = A3035StateUpdatePacket::default()
            .to_packet()
            .bytes_with_checksum();
        let (_, packet) = packet::Inbound::take_with_checksum::<VerboseError<_>>(&bytes).unwrap();
        let _: A3035StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
