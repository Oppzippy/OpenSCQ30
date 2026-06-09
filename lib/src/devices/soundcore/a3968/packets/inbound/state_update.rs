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
        a3968::{self, state::A3968State},
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
                AmbientSoundModeCycleTws, AutoPowerOff, CaseBatteryLevel,
                CommonEqualizerConfiguration, CustomHearId, DualBattery, DualFirmwareVersion,
                SerialNumber, SurroundSound, TouchTone, TwsStatus,
                button_configuration::ButtonStatusCollection,
            },
        },
    },
};

/// State update packet for the Soundcore Sport X20 (A3968).
///
/// Body is 143 bytes. Confirmed layout:
///   0..2     tws status (host earbud, both-connected)
///   2..6     dual battery (left level, right level, left charging, right charging)
///   6..16    dual firmware version (left "01.65", right "01.65")
///   16..32   serial number (16 ASCII chars; "3968" + mac)
///   32..117  equalizer (preset id + 8 bands at offset 38, 120 = 0.0 dB) + right-channel /
///            HearID data + reserved. Left unparsed for now: the X20 ignores every standard
///            EQ "set" command, so EQ is deferred to a follow-up once that's reverse-engineered.
///   117..124 sound-mode block (A3968 format: ambient sound mode at offset 117)
///   124..143 reserved
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A3968StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_battery_level: CaseBatteryLevel,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub hear_id: CustomHearId<2, 10>,
    pub button_configuration: ButtonStatusCollection<6>,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycleTws,
    pub sound_modes: a3968::structures::SoundModes,
    pub dual_connections_enabled: bool,
    pub touch_tone: TouchTone,
    pub auto_power_off: AutoPowerOff,
    pub surround_sound: SurroundSound,
}

impl Default for A3968StateUpdatePacket {
    fn default() -> Self {
        Self {
            tws_status: Default::default(),
            dual_battery: Default::default(),
            dual_firmware_version: Default::default(),
            serial_number: Default::default(),
            case_battery_level: Default::default(),
            equalizer_configuration: Default::default(),
            hear_id: Default::default(),
            button_configuration: a3968::BUTTON_CONFIGURATION_SETTINGS.default_status_collection(),
            ambient_sound_mode_cycle: Default::default(),
            sound_modes: Default::default(),
            touch_tone: Default::default(),
            auto_power_off: Default::default(),
            dual_connections_enabled: Default::default(),
            surround_sound: Default::default(),
        }
    }
}

impl FromPacketBody for A3968StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3968 state update packet",
            map(
                (
                    TwsStatus::take,           // tws status (host, both-connected)
                    DualBattery::take,         // 4 bytes
                    DualFirmwareVersion::take, // 10 bytes
                    SerialNumber::take,        // 16 bytes
                    take(5usize),              // unknown
                    CaseBatteryLevel::take,
                    CommonEqualizerConfiguration::take,
                    take(1usize), // unknown
                    CustomHearId::take_with_music_genre_at_end,
                    take(1usize), // unknown,
                    ButtonStatusCollection::take(
                        a3968::BUTTON_CONFIGURATION_SETTINGS.parse_settings(),
                    ),
                    AmbientSoundModeCycleTws::take,
                    a3968::structures::SoundModes::take, // sound-mode block (offsets 117..124)
                    take(1usize),                        // unknown
                    SurroundSound::take,
                    take(1usize), // unknown
                    take_bool,    // dual connections enabled
                    TouchTone::take,
                    AutoPowerOff::take,
                ),
                |(
                    tws_status,
                    dual_battery,
                    dual_firmware_version,
                    serial_number,
                    _unknown1,
                    case_battery_level,
                    equalizer_configuration,
                    _unknown2,
                    hear_id,
                    _unknown3,
                    button_configuration,
                    ambient_sound_mode_cycle,
                    sound_modes,
                    _unknown5,
                    surround_sound,
                    _unknown6,
                    dual_connections_enabled,
                    touch_tone,
                    auto_power_off,
                )| {
                    Self {
                        tws_status,
                        dual_battery,
                        dual_firmware_version,
                        serial_number,
                        case_battery_level,
                        equalizer_configuration,
                        hear_id,
                        button_configuration,
                        ambient_sound_mode_cycle,
                        sound_modes,
                        dual_connections_enabled,
                        touch_tone,
                        auto_power_off,
                        surround_sound,
                    }
                },
            ),
        )
        .parse_complete(input)
    }
}

impl ToPacket for A3968StateUpdatePacket {
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
            .chain(std::iter::repeat_n(0, 5))
            .chain(self.case_battery_level.bytes())
            .chain(self.equalizer_configuration.bytes())
            .chain(std::iter::once(0))
            .chain(self.hear_id.bytes_with_music_genre_at_end())
            .chain(std::iter::once(0))
            .chain(
                self.button_configuration
                    .bytes(a3968::BUTTON_CONFIGURATION_SETTINGS.parse_settings()),
            )
            .chain(self.ambient_sound_mode_cycle.bytes())
            .chain(self.sound_modes.bytes())
            .chain(std::iter::once(0)) // unknown
            .chain(self.surround_sound.bytes())
            .chain(std::iter::once(0)) // unknown
            .chain(std::iter::once(self.dual_connections_enabled as u8))
            .chain(self.touch_tone.bytes())
            .chain(self.auto_power_off.bytes())
            .collect()
    }
}

struct StateUpdatePacketHandler;

#[async_trait]
impl PacketHandler<A3968State> for StateUpdatePacketHandler {
    async fn handle_packet(
        &self,
        state: &watch::Sender<A3968State>,
        packet: &packet::Inbound,
    ) -> device::Result<()> {
        let packet: A3968StateUpdatePacket = packet.try_to_packet()?;
        state.send_modify(|state| state.update(packet));
        Ok(())
    }
}

impl ModuleCollection<A3968State> {
    pub fn add_state_update(&mut self) {
        self.packet_handlers.set_handler(
            packet::inbound::STATE_COMMAND,
            Box::new(StateUpdatePacketHandler {}),
        );
    }
}
