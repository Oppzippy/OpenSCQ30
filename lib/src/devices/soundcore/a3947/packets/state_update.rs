use std::iter;

use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map,
    error::{ContextError, ParseError, context},
};
use tokio::sync::watch;

use crate::{
    device,
    devices::soundcore::{
        a3947::{self, state::A3947State},
        common::{
            modules::ModuleCollection,
            packet::{
                self,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::{
                AmbientSoundModeCycle, AutoPowerOff, BatteryLevel, DualBattery,
                DualFirmwareVersion, EqualizerConfiguration, LimitHighVolume, SerialNumber,
                TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
            },
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3947StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub equalizer_configuration: EqualizerConfiguration<2, 10>,
    pub hear_id: a3947::structures::HearId<2, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3947::structures::SoundModes,
    pub charging_case_battery_level: BatteryLevel,
    pub sound_leak_compensation: bool,
    pub gaming_mode: bool,
    pub touch_tone: TouchTone,
    pub surround_sound: bool,
    pub limit_high_volume: LimitHighVolume,
    pub auto_play_pause: bool,
    pub wearing_tone: bool,
    pub auto_power_off: AutoPowerOff,
    pub touch_lock: bool,
    pub low_battery_prompt: bool,
}

impl Default for A3947StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            button_configuration: a3947::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            touch_tone: Default::default(),
            hear_id: Default::default(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            charging_case_battery_level: Default::default(),
            sound_leak_compensation: Default::default(),
            gaming_mode: Default::default(),
            limit_high_volume: Default::default(),
            auto_play_pause: Default::default(),
            wearing_tone: Default::default(),
            auto_power_off: Default::default(),
            touch_lock: Default::default(),
            low_battery_prompt: Default::default(),
            surround_sound: Default::default(),
        }
    }
}

impl FromPacketBody for A3947StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3947 state update packet",
            map(
                (
                    (
                        TwsStatus::take,
                        DualBattery::take,
                        DualFirmwareVersion::take,
                        SerialNumber::take,
                        take(5usize), // unknown, some kind of version number
                        EqualizerConfiguration::take,
                        take(1usize), // unknown
                        a3947::structures::HearId::take,
                        take(1usize), // unknown
                        ButtonStatusCollection::take(
                            a3947::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                        ),
                        AmbientSoundModeCycle::take,
                        a3947::structures::SoundModes::take,
                        take(6usize),       // unknown
                        BatteryLevel::take, // case battery
                        take(1usize),       // unknown
                        take_bool,          // sound leak compensation
                        take(1usize),       // unknown
                        take_bool,          // game mode
                        TouchTone::take,
                        take(3usize), // unknown
                        take_bool,    // 3d surround sound
                    ),
                    (
                        LimitHighVolume::take,
                        take_bool, // auto play/pause
                        take_bool, // wearing tone
                        AutoPowerOff::take,
                        take_bool, // touch lock
                        take_bool, // low battery prompt
                    ),
                ),
                |(
                    (
                        tws_status,
                        battery,
                        dual_firmware_version,
                        serial_number,
                        _unknown1,
                        equalizer_configuration,
                        _unknown2,
                        hear_id,
                        _unknown3,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        _unknown4,
                        charging_case_battery_level,
                        _unknown5,
                        sound_leak_compensation,
                        _unknown6,
                        gaming_mode,
                        touch_tone,
                        _unknown7,
                        surround_sound,
                    ),
                    (
                        limit_high_volume,
                        auto_play_pause,
                        wearing_tone,
                        auto_power_off,
                        touch_lock,
                        low_battery_prompt,
                    ),
                )| {
                    Self {
                        tws_status,
                        battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        button_configuration,
                        touch_tone,
                        charging_case_battery_level,
                        hear_id,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        sound_leak_compensation,
                        gaming_mode,
                        surround_sound,
                        limit_high_volume,
                        auto_play_pause,
                        wearing_tone,
                        auto_power_off,
                        touch_lock,
                        low_battery_prompt,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3947StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> packet::Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.to_string().into_bytes())
            .chain("00.00".as_bytes().iter().copied()) // unknown version of some sort
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::once(0)) // unknown
            .chain(self.hear_id.bytes())
            .chain(iter::once(0)) // unknown
            .chain(
                self.button_configuration
                    .bytes(a3947::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0; 6]) // unknown
            .chain([
                self.charging_case_battery_level.0,
                0, // unknown
                self.sound_leak_compensation.into(),
                0,
                self.gaming_mode.into(),
            ])
            .chain(self.touch_tone.bytes())
            .chain([0; 3])
            .chain(iter::once(self.surround_sound.into()))
            .chain(self.limit_high_volume.bytes())
            .chain([self.auto_play_pause.into(), self.wearing_tone.into()])
            .chain(self.auto_power_off.bytes())
            .chain([self.touch_lock.into(), self.low_battery_prompt.into()])
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<A3947State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3947State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3947StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<A3947State> {
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
        let bytes = A3947StateUpdatePacket::default().to_packet().bytes();
        let (_, packet) = packet::Inbound::take::<VerboseError<_>>(&bytes).unwrap();
        let _: A3947StateUpdatePacket = packet.try_to_packet().unwrap();
    }
}
