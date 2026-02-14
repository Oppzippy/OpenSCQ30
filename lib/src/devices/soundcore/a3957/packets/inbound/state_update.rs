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
    api::device,
    devices::soundcore::{
        a3957,
        common::{
            self,
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            structures::button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3957StateUpdatePacket {
    pub tws_status: common::structures::TwsStatus,
    pub dual_battery: common::structures::DualBattery,
    pub dual_firmware_version: common::structures::DualFirmwareVersion,
    pub serial_number: common::structures::SerialNumber,
    // 5 bytes ("00.00")
    pub case_battery: common::structures::CaseBatteryLevel,
    pub equalizer_configuration: common::structures::CommonEqualizerConfiguration<2, 10>,
    pub age_range: common::structures::AgeRange,
    pub gender: common::structures::Gender,
    pub hear_id: common::structures::CustomHearId<2, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub ambient_sound_mode_cycle: common::structures::AmbientSoundModeCycle,
    pub sound_modes: a3957::structures::SoundModes,
    pub wearing_tone: a3957::structures::WearingTone,
    pub low_battery_prompt: common::structures::LowBatteryPrompt,
    pub ldac: bool,
    pub anc_personalized_to_ear_canal: a3957::structures::AncPersonalizedToEarCanal,
    pub auto_power_off: common::structures::AutoPowerOff,
    pub limit_high_volume: common::structures::LimitHighVolume,
    pub immersive_experience: a3957::structures::ImmersiveExperience,
    pub sound_leakage_compensation: a3957::structures::SoundLeakageCompensation,
    pub wearing_detection: a3957::structures::WearingDetection,
    pub touch_tone: common::structures::TouchTone,
    pub game_mode: a3957::structures::GameMode,
    pub pressure_sensitivity: a3957::structures::PressureSensitivity,
}

impl Default for A3957StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            case_battery: Default::default(),
            equalizer_configuration: Default::default(),
            age_range: Default::default(),
            hear_id: Default::default(),
            button_configuration: a3957::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            wearing_tone: Default::default(),
            low_battery_prompt: Default::default(),
            ldac: Default::default(),
            anc_personalized_to_ear_canal: Default::default(),
            auto_power_off: Default::default(),
            limit_high_volume: Default::default(),
            immersive_experience: Default::default(),
            sound_leakage_compensation: Default::default(),
            wearing_detection: Default::default(),
            touch_tone: Default::default(),
            game_mode: Default::default(),
            pressure_sensitivity: Default::default(),
            gender: Default::default(),
        }
    }
}

impl FromPacketBody for A3957StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3957 state update packet",
            map(
                (
                    (
                        common::structures::TwsStatus::take,
                        common::structures::DualBattery::take,
                        common::structures::DualFirmwareVersion::take,
                        common::structures::SerialNumber::take,
                        take(5usize), // "00.00"
                        common::structures::CaseBatteryLevel::take,
                        common::structures::EqualizerConfiguration::take,
                        common::structures::AgeRange::take,
                        common::structures::CustomHearId::take_with_music_genre_at_end,
                        take(1usize), // unknown (value 10)
                        ButtonStatusCollection::take(
                            a3957::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                        ),
                        common::structures::AmbientSoundModeCycle::take,
                        a3957::structures::SoundModes::take, // 7 bytes: 119-125
                        take(1usize),                        // unknown: 126 (always 0x33)
                        a3957::structures::WearingTone::take, // 127
                        common::structures::LowBatteryPrompt::take, // 128
                        take_bool,                           // 129: LDAC
                        a3957::structures::AncPersonalizedToEarCanal::take, // 130
                        common::structures::AutoPowerOff::take, // 131-132
                        common::structures::LimitHighVolume::take, // 133-135
                        a3957::structures::ImmersiveExperience::take, // 136
                    ),
                    (
                        take(6usize),                                      // unknown: 137-142
                        a3957::structures::SoundLeakageCompensation::take, // 143
                        a3957::structures::WearingDetection::take,         // 144
                        common::structures::TouchTone::take,               // 145
                        a3957::structures::GameMode::take,                 // 146
                        a3957::structures::PressureSensitivity::take,      // 147
                        take(6usize),                                      // unknown: 148-153
                    ),
                ),
                |(
                    (
                        tws_status,
                        dual_battery,
                        dual_firmware_version,
                        serial_number,
                        _unknown0,
                        case_battery,
                        equalizer_configuration,
                        age_range,
                        hear_id,
                        _unknown1,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        _unknown2,
                        wearing_tone,
                        low_battery_prompt,
                        ldac,
                        anc_personalized_to_ear_canal,
                        auto_power_off,
                        limit_high_volume,
                        immersive_experience,
                    ),
                    (
                        _unknown4,
                        sound_leakage_compensation,
                        wearing_detection,
                        touch_tone,
                        game_mode,
                        pressure_sensitivity,
                        _unknown5,
                    ),
                )| {
                    Self {
                        tws_status,
                        dual_battery,
                        dual_firmware_version,
                        serial_number,
                        equalizer_configuration,
                        age_range,
                        hear_id,
                        case_battery,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        wearing_tone,
                        low_battery_prompt,
                        ldac,
                        anc_personalized_to_ear_canal,
                        auto_power_off,
                        limit_high_volume,
                        immersive_experience,
                        sound_leakage_compensation,
                        wearing_detection,
                        touch_tone,
                        game_mode,
                        pressure_sensitivity,
                        // not parsed from packet
                        gender: Default::default(),
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3957StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.dual_battery.bytes())
            .chain(self.dual_firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain([0; 5]) // unknown: 32-36
            .chain(iter::once(self.case_battery.0.0))
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::once(self.age_range.0))
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(iter::once(0)) // unknown: 109
            .chain(
                self.button_configuration
                    .bytes(a3957::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain(iter::once(0x33)) // unknown: 126
            .chain(self.wearing_tone.bytes()) // 127
            .chain(self.low_battery_prompt.bytes()) // 128
            .chain(iter::once(self.ldac.into())) // 129: LDAC
            .chain(self.anc_personalized_to_ear_canal.bytes()) // 130
            .chain(self.auto_power_off.bytes()) // 131-132
            .chain(self.limit_high_volume.bytes()) // 133-135
            .chain(iter::once(self.immersive_experience as u8)) // 136
            .chain([0; 6]) // unknown: 137-142
            .chain(self.sound_leakage_compensation.bytes()) // 143
            .chain(self.wearing_detection.bytes()) // 144
            .chain([self.touch_tone.0.into()]) // 145
            .chain(self.game_mode.bytes()) // 146
            .chain(iter::once(self.pressure_sensitivity as u8)) // 147
            .chain([0xff; 6]) // unknown: 148-153
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<a3957::state::A3957State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<a3957::state::A3957State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3957StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<a3957::state::A3957State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
