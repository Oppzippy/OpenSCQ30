use std::array;

use async_trait::async_trait;
use itertools::Itertools;
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
        a3954::{self, state::A3954State},
        common::{
            modules::ModuleCollection,
            packet::{
                self, Command,
                inbound::{FromPacketBody, TryToPacket},
                outbound::ToPacket,
                parsing::take_bool,
            },
            packet_manager::PacketHandler,
            state::Update,
            structures::{
                AmbientSoundModeCycle, AutoPowerOff, CaseBatteryLevel,
                CommonEqualizerConfiguration, CustomHearId, DualBattery, DualFirmwareVersion, Ldac,
                LimitHighVolume, LowBatteryPrompt, SerialNumber, SoundLeakCompensation, TwsStatus,
                WearingDetection, button_configuration::ButtonStatusCollection,
            },
        },
    },
};

pub struct A3954StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_firmware_version: a3954::structures::CaseFirmwareVersion,
    pub case_battery_level: CaseBatteryLevel,
    pub case_serial_number: a3954::structures::CaseSerialNumber,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub hear_id: CustomHearId<2, 10>,
    pub button_configuration: ButtonStatusCollection<12>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub sound_modes: a3954::structures::SoundModes,
    pub case_features: a3954::structures::CaseFeatures,
    pub air_pressure: a3954::structures::AirPressure,
    pub low_battery_prompt: LowBatteryPrompt,
    pub ldac: Ldac,
    pub dual_connections_enabled: bool,
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub spatial_audio: a3954::structures::SpatialAudio,
    pub easy_chat: a3954::structures::EasyChat,
    pub sound_leak_compensation: SoundLeakCompensation,
    pub case_language: a3954::structures::CaseLanguage,
    pub wearing_detection: WearingDetection,
}

impl Default for A3954StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            case_firmware_version: Default::default(),
            case_serial_number: Default::default(),
            equalizer_configuration: Default::default(),
            hear_id: Default::default(),
            button_configuration: a3954::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            case_features: Default::default(),
            air_pressure: Default::default(),
            low_battery_prompt: Default::default(),
            ldac: Default::default(),
            dual_connections_enabled: Default::default(),
            auto_power_off: Default::default(),
            limit_high_volume: Default::default(),
            spatial_audio: Default::default(),
            easy_chat: Default::default(),
            sound_leak_compensation: Default::default(),
            case_language: Default::default(),
            wearing_detection: Default::default(),
            case_battery_level: Default::default(),
        }
    }
}

impl FromPacketBody for A3954StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        let button_parse_settings = a3954::BUTTON_CONFIGURATION_SETTINGS.parse_settings();
        context(
            "a3954 state update packet",
            map(
                (
                    (
                        TwsStatus::take,
                        DualBattery::take,
                        DualFirmwareVersion::take,
                        SerialNumber::take,
                        a3954::structures::CaseFirmwareVersion::take,
                        CaseBatteryLevel::take,
                        a3954::structures::CaseSerialNumber::take,
                        CommonEqualizerConfiguration::take,
                        take(1usize), // unknown
                        CustomHearId::take_with_music_genre_at_end,
                        take(1usize), // unknown
                        // The non-slide buttons and slide buttons are not together, so we have to split up parsing
                        ButtonStatusCollection::<8>::take(array::from_fn::<_, 8, _>(|i| {
                            button_parse_settings[i]
                        })),
                        AmbientSoundModeCycle::take,
                        a3954::structures::SoundModes::take,
                        take(3usize), // unknown
                        a3954::structures::CaseFeatures::take,
                        a3954::structures::AirPressure::take,
                        take(3usize), // unknown
                        LowBatteryPrompt::take,
                        Ldac::take,
                        take_bool, // dual connections enabled
                    ),
                    (
                        AutoPowerOff::take,
                        LimitHighVolume::take,
                        a3954::structures::SpatialAudio::take,
                        take_bool,    // Easy chat enabled
                        take(1usize), // unknown
                        SoundLeakCompensation::take,
                        take(3usize), // unknown
                        a3954::structures::CaseLanguage::take,
                        a3954::structures::EasyChatWaitTime::take,
                        WearingDetection::take,
                        take(1usize), // unknown
                        ButtonStatusCollection::<4>::take(array::from_fn::<_, 4, _>(|i| {
                            button_parse_settings[8 + i]
                        })),
                    ),
                ),
                |(
                    (
                        tws_status,
                        battery,
                        firmware_version,
                        serial_number,
                        case_firmware_version,
                        case_battery_level,
                        case_serial_number,
                        equalizer_configuration,
                        _unknown1,
                        hear_id,
                        _unknown2,
                        main_buttons,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        _unknown3,
                        case_features,
                        air_pressure,
                        _unknown4,
                        low_battery_prompt,
                        ldac,
                        dual_connections_enabled,
                    ),
                    (
                        auto_power_off,
                        limit_high_volume,
                        spatial_audio,
                        is_easy_chat_enabled,
                        _unknown5,
                        sound_leak_compensation,
                        _unknown6,
                        case_language,
                        easy_chat_wait_time,
                        wearing_detection,
                        _unknown7,
                        slide_buttons,
                    ),
                )| Self {
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    case_firmware_version,
                    case_battery_level,
                    case_serial_number,
                    equalizer_configuration,
                    hear_id,
                    button_configuration: ButtonStatusCollection(
                        main_buttons
                            .0
                            .into_iter()
                            .chain(slide_buttons.0.into_iter())
                            .collect_array::<12>()
                            .expect("we took size 8 and 4, so if that succeeded, we have 12"),
                    ),
                    ambient_sound_mode_cycle,
                    sound_modes,
                    case_features,
                    air_pressure,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    limit_high_volume,
                    spatial_audio,
                    easy_chat: a3954::structures::EasyChat {
                        is_enabled: is_easy_chat_enabled,
                        wait_time: easy_chat_wait_time,
                    },
                    sound_leak_compensation,
                    case_language,
                    wearing_detection,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3954StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn command(&self) -> Command {
        packet::inbound::STATE_COMMAND
    }

    fn body(&self) -> Vec<u8> {
        self.tws_status
            .bytes()
            .into_iter()
            .chain(self.battery.bytes())
            .chain(self.firmware_version.bytes())
            .chain(self.serial_number.bytes())
            .chain(self.case_firmware_version.bytes())
            .chain(self.case_battery_level.bytes())
            .chain(self.case_serial_number.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(std::iter::once(0)) // unknown
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(std::iter::once(0)) // unknown
            .chain(
                self.button_configuration
                    .bytes(a3954::BUTTON_CONFIGURATION_SETTINGS.parse_settings())
                    .take(8),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain(std::iter::repeat_n(0, 3)) // unknown
            .chain(self.case_features.bytes())
            .chain(self.air_pressure.bytes())
            .chain(std::iter::repeat_n(0, 3)) // unknown
            .chain(self.low_battery_prompt.bytes())
            .chain(self.ldac.bytes())
            .chain(std::iter::once(u8::from(self.dual_connections_enabled)))
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain(self.spatial_audio.bytes())
            .chain(std::iter::once(u8::from(self.easy_chat.is_enabled)))
            .chain(std::iter::once(0)) // unknown
            .chain(self.sound_leak_compensation.bytes())
            .chain(std::iter::repeat_n(0, 3)) // unknown
            .chain(self.case_language.bytes())
            .chain(self.easy_chat.wait_time.bytes())
            .chain(self.wearing_detection.bytes())
            .chain(std::iter::once(0)) // unknown
            .chain(
                self.button_configuration
                    .bytes(a3954::BUTTON_CONFIGURATION_SETTINGS.parse_settings())
                    .skip(8),
            )
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<A3954State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3954State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3954StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update(packet));
        Ok(())
    }
}

impl ModuleCollection<A3954State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler),
        );
    }
}
