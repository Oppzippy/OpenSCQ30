//! Wind Noise Reduction toggle for the Soundcore Liberty 5 Pro (D1203).
//!
//! ## Why this is NOT part of the sound-mode (`06 81`) command
//!
//! Reverse engineering the real hardware showed the wind noise reduction
//! feature is driven by its OWN command, independent of the ambient sound-mode
//! write (`06 81`). It uses category `0x41`, command `0x88`, with a 2-byte
//! payload `[0x0b, value]` where `0x0b` is a fixed sub-parameter and `value` is
//! `0x00` (OFF) or `0x01` (ON):
//!
//! ```text
//! OFF: 08 ee 00 00 00 41 88 0c 00 0b 00 d6
//! ON:  08 ee 00 00 00 41 88 0c 00 0b 01 d7
//! ```
//!
//! header `08 ee 00 00 00` | command `41 88` | length `0c 00` (=12 LE) |
//! payload `0b [00|01]` | checksum (sum % 256).
//!
//! The length field (`0x0c` = 12) is the TOTAL packet size INCLUDING the
//! trailing checksum byte (header 9 + payload 2 + checksum 1 = 12), which is
//! the project's STANDARD framing. So this packet goes through the generic
//! `packet::Outbound` serializer (`send_with_response`) with no raw-byte
//! handling — unlike the custom EQ `03 8a` command, whose length field excludes
//! the checksum.
//!
//! TODO(read): the D1203 state packet TLV tag carrying the current wind-noise
//! state is not yet identified, so the state is initialized to `Default`
//! (OFF/false) on connect. Writing works regardless; only reading back the
//! device's current state is outstanding.

use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Setting, SettingId, Value},
    },
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        settings_manager::{SettingHandler, SettingHandlerResult},
        state_modifier::StateModifier,
    },
};

use super::super::state::D1203State;
use super::ModuleCollection;

/// Fixed sub-parameter byte that precedes the on/off value in the `41 88`
/// payload.
const WIND_NOISE_SUBPARAMETER: u8 = 0x0b;

/// Whether wind noise reduction is enabled on the D1203. Stored in device state
/// so the toggle is readable and the state modifier can tell when it actually
/// changed.
///
/// Defaults to `false` (OFF).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct D1203WindNoise(pub bool);

/// Builds the 2-byte `41 88` payload for the given on/off state:
/// `[0x0b, 0x01]` when enabled, `[0x0b, 0x00]` when disabled.
pub fn build_wind_noise_payload(enabled: bool) -> [u8; 2] {
    [WIND_NOISE_SUBPARAMETER, if enabled { 1 } else { 0 }]
}

/// Exposes wind noise reduction as a readable/writable [`Setting::Toggle`].
struct WindNoiseSuppressionSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for WindNoiseSuppressionSettingHandler
where
    T: Has<D1203WindNoise> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::WindNoiseSuppression]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let wind_noise: &D1203WindNoise = state.get();
        Some(Setting::Toggle {
            value: wind_noise.0,
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        *state.get_mut() = D1203WindNoise(value.try_as_bool()?);
        Ok(())
    }
}

/// On any change to the wind-noise state, sends the dedicated `41 88` write
/// with payload `[0x0b, 0|1]`. Uses the generic serializer (standard framing),
/// so `send_with_response` is correct here.
struct WindNoiseStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for WindNoiseStateModifier
where
    T: Has<D1203WindNoise> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: D1203WindNoise = *target_state.get();
        {
            let current = state_sender.borrow();
            let current_wind_noise: &D1203WindNoise = current.get();
            if *current_wind_noise == target {
                return Ok(());
            }
        }

        let payload = build_wind_noise_payload(target.0);
        self.packet_io
            .send_with_response(&packet::Outbound::new(
                packet::Command([0x41, 0x88]),
                payload.to_vec(),
            ))
            .await?;

        state_sender.send_modify(|state| {
            *state.get_mut() = target;
        });
        Ok(())
    }
}

impl ModuleCollection<D1203State> {
    /// Registers the wind noise reduction toggle for the D1203 under
    /// [`CategoryId::SoundModes`], plus its `41 88` state modifier.
    pub fn add_d1203_wind_noise(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, WindNoiseSuppressionSettingHandler);
        self.state_modifiers
            .push(Box::new(WindNoiseStateModifier { packet_io }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Turning the toggle ON serializes to exactly the real ON capture
    /// `08 ee 00 00 00 41 88 0c 00 0b 01 d7`, and OFF to
    /// `08 ee 00 00 00 41 88 0c 00 0b 00 d6` (checksum = sum % 256).
    #[test]
    fn wind_noise_framing_matches_real_captures() {
        const EXPECTED_ON: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x88, 0x0c, 0x00, 0x0b, 0x01, 0xd7,
        ];
        const EXPECTED_OFF: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x88, 0x0c, 0x00, 0x0b, 0x00, 0xd6,
        ];

        let on = packet::Outbound::new(
            packet::Command([0x41, 0x88]),
            build_wind_noise_payload(true).to_vec(),
        );
        assert_eq!(on.bytes_with_checksum(), EXPECTED_ON);

        let off = packet::Outbound::new(
            packet::Command([0x41, 0x88]),
            build_wind_noise_payload(false).to_vec(),
        );
        assert_eq!(off.bytes_with_checksum(), EXPECTED_OFF);
    }

    /// The payload is the fixed sub-parameter `0x0b` followed by the on/off byte.
    #[test]
    fn payload_layout() {
        assert_eq!(build_wind_noise_payload(true), [0x0b, 0x01]);
        assert_eq!(build_wind_noise_payload(false), [0x0b, 0x00]);
    }
}
