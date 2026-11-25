//! Interactive custom 8-band equalizer setting for the Soundcore Liberty 5 Pro
//! (D1203).
//!
//! This wires the raw packet builder in
//! [`crate::devices::soundcore::d1203::packets::outbound::set_eq`] up to an
//! interactive [`Setting`], so the EQ can be driven from the CLI with:
//!
//! ```text
//! openscq30 device setting --set customEqualizerProfile=<g1>,<g2>,...,<g8>
//! ```
//!
//! where each `g` is a whole-dB gain between -6 and +6 (8 values, one per
//! band). The setting is exposed as a [`Setting::Equalizer`] so the CLI parses
//! the comma-separated gains natively (see `cli/src/parse.rs::parse_equalizer`).
//!
//! ## How the write reaches the device
//!
//! The write is sent in the shape the official app uses (see
//! [`custom_eq_payload`]) through the generic serializer, and waits for the
//! `03 8a` ACK. Firmware 06.54 ignores any other shape without replying, so a
//! missing ACK is the only sign of failure. If the app shape gets no ACK, the
//! older 124 byte frame captured on firmware 04.52 is sent once as a fallback.

use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Equalizer, Setting, SettingId, Value},
    },
    devices::soundcore::{
        common::{
            packet::{self, PacketIOController},
            settings_manager::{SettingHandler, SettingHandlerResult},
            state_modifier::StateModifier,
        },
        d1203::packets::outbound::{BAND_COUNT, BAND_FREQUENCIES, custom_eq_payload, set_eq_legacy},
    },
};

use super::super::state::D1203State;
use super::ModuleCollection;

/// Minimum/maximum per-band gain in whole dB exposed to the user.
const MIN_GAIN_DB: i16 = -6;
const MAX_GAIN_DB: i16 = 6;

/// The custom-EQ gains (whole dB, one per band) currently requested for the
/// D1203. Stored in device state so the setting is readable and the state
/// modifier can tell when it actually changed.
///
/// Defaults to all-zero (flat).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D1203CustomEqualizer(pub [i16; BAND_COUNT]);

impl Default for D1203CustomEqualizer {
    fn default() -> Self {
        Self([0; BAND_COUNT])
    }
}

/// Exposes the custom EQ as a readable/writable [`Setting::Equalizer`].
struct CustomEqualizerSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for CustomEqualizerSettingHandler
where
    T: Has<D1203CustomEqualizer> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::CustomEqualizerProfile]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let gains: &D1203CustomEqualizer = state.get();
        Some(Setting::Equalizer {
            setting: Equalizer {
                band_hz: BAND_FREQUENCIES
                    .iter()
                    .map(|hz| *hz as u16)
                    .collect::<Vec<_>>()
                    .into(),
                // Gains are expressed in whole dB, so no fractional digits.
                fraction_digits: 0,
                min: MIN_GAIN_DB,
                max: MAX_GAIN_DB,
            },
            read_only: false,
            value: gains.0.to_vec(),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        // `try_as_i16_array` enforces exactly BAND_COUNT values.
        let gains: &[i16; BAND_COUNT] = value.try_as_i16_array::<BAND_COUNT>()?;
        *state.get_mut() = D1203CustomEqualizer(*gains);
        Ok(())
    }
}

/// On change, sends the custom EQ and waits for the device's ACK.
struct CustomEqualizerStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for CustomEqualizerStateModifier
where
    T: Has<D1203CustomEqualizer> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: D1203CustomEqualizer = *target_state.get();
        {
            let state = state_sender.borrow();
            let current: &D1203CustomEqualizer = state.get();
            if *current == target {
                return Ok(());
            }
        }

        // Convert whole-dB i16 gains into the f32 gains `set_eq` expects.
        let mut gains_db = [0.0_f32; BAND_COUNT];
        for (out, gain) in gains_db.iter_mut().zip(target.0.iter()) {
            *out = *gain as f32;
        }

        let app_shape = packet::Outbound::new(
            packet::Command([0x03, 0x8a]),
            custom_eq_payload(gains_db),
        );
        match self.packet_io.send_with_response(&app_shape).await {
            Ok(_) => {}
            Err(device::Error::ActionTimedOut { .. }) => {
                // No ACK for the app shape. Try the frame captured on 04.52 once.
                // It has no ACK we can wait on here, so this path cannot confirm
                // that the earbuds applied it.
                tracing::warn!(
                    "no ACK for the custom EQ; falling back to the legacy 04.52 frame"
                );
                self.packet_io.send_raw(&set_eq_legacy(gains_db)).await?;
            }
            Err(err) => return Err(err),
        }

        state_sender.send_modify(|state| {
            *state.get_mut() = target;
        });
        Ok(())
    }
}

impl ModuleCollection<D1203State> {
    /// Registers the interactive custom 8-band equalizer for the D1203.
    pub fn add_d1203_custom_equalizer(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Equalizer, CustomEqualizerSettingHandler);
        self.state_modifiers
            .push(Box::new(CustomEqualizerStateModifier { packet_io }));
    }
}
