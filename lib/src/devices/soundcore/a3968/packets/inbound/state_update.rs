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
            },
            packet_manager::PacketHandler,
            structures::{
                CaseBatteryLevel, CommonEqualizerConfiguration, CustomHearId, DualBattery,
                DualFirmwareVersion, SerialNumber, TwsStatus,
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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct A3968StateUpdatePacket {
    pub tws_status: TwsStatus,
    pub dual_battery: DualBattery,
    pub dual_firmware_version: DualFirmwareVersion,
    pub serial_number: SerialNumber,
    pub case_battery_level: CaseBatteryLevel,
    pub equalizer_configuration: CommonEqualizerConfiguration<2, 10>,
    pub hear_id: CustomHearId<2, 10>,
    pub sound_modes: a3968::structures::SoundModes,
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
                    take(1usize),                        // unknown,
                    take(6usize),                        // TODO button configuration,
                    take(1usize),                        // unknown,
                    a3968::structures::SoundModes::take, // sound-mode block (offsets 117..124)
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
                    _todo_button_configuration,
                    _unknown4,
                    sound_modes,
                )| {
                    Self {
                        tws_status,
                        dual_battery,
                        dual_firmware_version,
                        serial_number,
                        case_battery_level,
                        equalizer_configuration,
                        hear_id,
                        sound_modes,
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
            .chain(std::iter::repeat_n(0, 6)) // TODO button configuration
            .chain(std::iter::once(0))
            .chain(self.sound_modes.bytes())
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
        state.send_modify(|state| *state = packet.into());
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
