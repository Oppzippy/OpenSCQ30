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
                AutoPowerOff, CaseBatteryLevel, CommonEqualizerConfiguration, CustomHearId,
                DualBattery, DualFirmwareVersion, Ldac, LimitHighVolume, LowBatteryPrompt,
                SerialNumber, TouchTone, TwsStatus, button_configuration::ButtonStatusCollection,
            },
        },
        d1202::{self, state::D1202State},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct D1202StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_battery_level: CaseBatteryLevel,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub hear_id: CustomHearId<2, 10>,
    pub button_configuration: ButtonStatusCollection<8>,
    pub sound_modes: d1202::structures::SoundModes,
    pub touch_tone: TouchTone,
    pub low_battery_prompt: LowBatteryPrompt,
    pub ldac: Ldac,
    pub dual_connections_enabled: bool, // dual connections enabled
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub spatial_audio: d1202::structures::SpatialAudio,
}

impl Default for D1202StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            battery: Default::default(),
            firmware_version: Default::default(),
            serial_number: Default::default(),
            case_battery_level: Default::default(),
            equalizer_configuration: Default::default(),
            hear_id: Default::default(),
            button_configuration: d1202::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            sound_modes: Default::default(),
            touch_tone: Default::default(),
            low_battery_prompt: Default::default(),
            ldac: Default::default(),
            dual_connections_enabled: Default::default(),
            auto_power_off: Default::default(),
            limit_high_volume: Default::default(),
            spatial_audio: Default::default(),
        }
    }
}

impl FromPacketBody for D1202StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1202 state update packet",
            map(
                (
                    TwsStatus::take,
                    DualBattery::take,
                    DualFirmwareVersion::take,
                    SerialNumber::take,
                    take(5usize), // case fw? unknown
                    CaseBatteryLevel::take,
                    CommonEqualizerConfiguration::take,
                    take(1usize),
                    CustomHearId::take_with_music_genre_at_end,
                    take(1usize),
                    ButtonStatusCollection::take(
                        d1202::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    take(1usize),
                    d1202::structures::SoundModes::take,
                    take(1usize),
                    TouchTone::take,
                    LowBatteryPrompt::take,
                    Ldac::take,
                    take_bool, // dual connections enabled
                    AutoPowerOff::take,
                    LimitHighVolume::take,
                    d1202::structures::SpatialAudio::take,
                ),
                |(
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    _case_firmware_version,
                    case_battery_level,
                    equalizer_configuration,
                    _unknown1,
                    hear_id,
                    _unknown2,
                    button_configuration,
                    _unknown3,
                    sound_modes,
                    _unknown4,
                    touch_tone,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    limit_high_volume,
                    spatial_audio,
                )| Self {
                    tws_status,
                    battery,
                    firmware_version,
                    serial_number,
                    case_battery_level,
                    equalizer_configuration,
                    hear_id,
                    button_configuration,
                    sound_modes,
                    touch_tone,
                    low_battery_prompt,
                    ldac,
                    dual_connections_enabled,
                    auto_power_off,
                    limit_high_volume,
                    spatial_audio,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for D1202StateUpdatePacket {
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
            .chain(std::iter::repeat_n(0, 5))
            .chain(self.case_battery_level.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(std::iter::once(0))
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(std::iter::once(0))
            .chain(
                self.button_configuration
                    .bytes(d1202::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(std::iter::once(0))
            .chain(self.sound_modes.bytes())
            .chain(std::iter::once(0))
            .chain(self.touch_tone.bytes())
            .chain(self.low_battery_prompt.bytes())
            .chain(self.ldac.bytes())
            .chain(std::iter::once(u8::from(self.dual_connections_enabled)))
            .chain(self.auto_power_off.bytes())
            .chain(self.limit_high_volume.bytes())
            .chain(self.spatial_audio.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<D1202State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<D1202State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: D1202StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update(packet));
        Ok(())
    }
}

impl ModuleCollection<D1202State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler),
        );
    }
}
