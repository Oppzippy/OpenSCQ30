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
        a3955,
        common::{
            self,
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
            },
            packet_manager::PacketHandler,
            structures::button_configuration::ButtonStatusCollection,
        },
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3955StateUpdatePacket {
    pub tws_status: common::structures::TwsStatus,
    pub dual_battery: common::structures::DualBattery,
    pub dual_firmware_version: common::structures::DualFirmwareVersion,
    pub serial_number: common::structures::SerialNumber,
    // 5 bytes here
    pub case_battery: common::structures::CaseBatteryLevel,
    pub equalizer_configuration: common::structures::CommonEqualizerConfiguration<2, 10>,
    pub age_range: common::structures::AgeRange,
    pub gender: common::structures::Gender,
    pub hear_id: common::structures::CustomHearId<2, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub ambient_sound_mode_cycle: common::structures::AmbientSoundModeCycle,
    pub sound_modes: a3955::structures::SoundModes,
    pub anc_personalized_to_ear_canal: a3955::structures::AncPersonalizedToEarCanal,
    pub touch_tone: common::structures::TouchTone,
    pub limit_high_volume: common::structures::LimitHighVolume,
    pub auto_power_off: common::structures::AutoPowerOff,
    pub low_battery_prompt: common::structures::LowBatteryPrompt,
    pub immersive_experience: a3955::structures::ImmersiveExperience,
    // pub gaming_mode: bool,
}

impl Default for A3955StateUpdatePacket {
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
            button_configuration: a3955::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            anc_personalized_to_ear_canal: Default::default(),
            touch_tone: Default::default(),
            auto_power_off: Default::default(),
            limit_high_volume: Default::default(),
            low_battery_prompt: Default::default(),
            gender: Default::default(),
            immersive_experience: Default::default(),
        }
    }
}

impl FromPacketBody for A3955StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3955 state update packet",
            map(
                (
                    (
                        common::structures::TwsStatus::take,
                        common::structures::DualBattery::take,
                        common::structures::DualFirmwareVersion::take,
                        common::structures::SerialNumber::take,
                        take(5usize),
                        common::structures::CaseBatteryLevel::take,
                        common::structures::EqualizerConfiguration::take,
                        common::structures::AgeRange::take,
                        common::structures::CustomHearId::take_with_music_genre_at_end,
                        take(1usize), // unknown
                        ButtonStatusCollection::take(
                            a3955::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                        ),
                        common::structures::AmbientSoundModeCycle::take,
                        a3955::structures::SoundModes::take,
                        take(6usize), // unknown
                        a3955::structures::AncPersonalizedToEarCanal::take,
                        take(1usize),
                        common::structures::TouchTone::take,
                        take(1usize),
                        common::structures::LimitHighVolume::take,
                        common::structures::AutoPowerOff::take,
                        take(1usize),
                    ),
                    common::structures::LowBatteryPrompt::take,
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
                        anc_personalized_to_ear_canal,
                        _unknown3,
                        touch_tone,
                        _unknown4,
                        limit_high_volume,
                        auto_power_off,
                        _unknown5,
                    ),
                    low_battery_prompt,
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
                        anc_personalized_to_ear_canal,
                        touch_tone,
                        limit_high_volume,
                        auto_power_off,
                        low_battery_prompt,
                        // TODO figure out where in the packet immersive experience is
                        // I tried setting all 0 bytes to 1, and that didn't enable immersive experience,
                        // so maybe there's a separate packet for it?
                        immersive_experience: Default::default(),
                        // unknown
                        gender: Default::default(),
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3955StateUpdatePacket {
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
            .chain([0; 5])
            .chain(iter::once(self.case_battery.0.0))
            .chain(self.equalizer_configuration.bytes())
            .chain(iter::once(self.age_range.0))
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(iter::once(0)) // unknown
            .chain(
                self.button_configuration
                    .bytes(a3955::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain([0; 6])
            .chain(self.anc_personalized_to_ear_canal.bytes())
            .chain(iter::once(0))
            .chain([self.touch_tone.0.into()])
            .chain(iter::once(0))
            .chain(self.limit_high_volume.bytes())
            .chain(self.auto_power_off.bytes())
            .chain(iter::once(0))
            .chain(self.low_battery_prompt.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler {}

#[async_trait]
impl PacketHandler<a3955::state::A3955State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<a3955::state::A3955State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3955StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| *state = packet.into());
        Ok(())
    }
}

impl ModuleCollection<a3955::state::A3955State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
