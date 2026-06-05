use async_trait::async_trait;
use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{all_consuming, map},
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
            structures::{DualBattery, DualFirmwareVersion, SerialNumber, TwsStatus},
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
    pub sound_modes: a3968::structures::SoundModes,
}

impl FromPacketBody for A3968StateUpdatePacket {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "a3968 state update packet",
            all_consuming(map(
                (
                    TwsStatus::take,                     // tws status (host, both-connected)
                    DualBattery::take,                   // 4 bytes
                    DualFirmwareVersion::take,           // 10 bytes
                    SerialNumber::take,                  // 16 bytes
                    take(85usize), // equalizer + HearID + reserved (offsets 32..117)
                    a3968::structures::SoundModes::take, // sound-mode block (offsets 117..124)
                    take(19usize), // reserved (offsets 124..143)
                ),
                |(
                    tws_status,
                    dual_battery,
                    dual_firmware_version,
                    serial_number,
                    _eq_and_reserved,
                    mut sound_modes,
                    _reserved,
                )| {
                    // The device reports a live adaptive-sensitivity byte (255) and wind flags
                    // that are invalid as inputs and make the X20 reject NoiseCanceling on set.
                    // Normalize them so the set path always emits clean, accepted values.
                    sound_modes.noise_canceling_adaptive_sensitivity_level = 0;
                    sound_modes.wind_noise.is_suppression_enabled = false;
                    sound_modes.wind_noise.is_detected = false;
                    Self {
                        tws_status,
                        dual_battery,
                        dual_firmware_version,
                        serial_number,
                        sound_modes,
                    }
                },
            )),
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
            .chain(std::iter::repeat_n(0u8, 85))
            .chain(self.sound_modes.bytes())
            .chain(std::iter::repeat_n(0u8, 19))
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
