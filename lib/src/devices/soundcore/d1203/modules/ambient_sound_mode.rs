//! Ambient sound mode (ANC / Transparency / Normal / Adaptive) for the
//! Soundcore Liberty 5 Pro (D1203).
//!
//! ## Why the D1203 does NOT reuse the A3957 ambient sound mode
//!
//! The D1203 write command is byte-for-byte identical to the A3957
//! (`Command([0x06, 0x81])`, 7-byte payload, checksum = sum % 256), so earlier
//! phases reused `a3957::structures::SoundModes` wholesale. Reverse
//! engineering the real hardware then showed the D1203 exposes a FOURTH ambient
//! mode the A3957 enum cannot represent:
//!
//! | value | mode            |
//! |-------|-----------------|
//! | 0     | Noise Canceling |
//! | 1     | Transparency    |
//! | 2     | Normal          |
//! | 3     | Adaptive        |
//!
//! The common/A3957 `AmbientSoundMode` only has `0 = NoiseCanceling`,
//! `1 = Transparency`, `2 = Normal`. Its parser does `from_repr(3) -> None ->
//! default`, so the device's Adaptive mode decodes as NoiseCanceling and, more
//! importantly, there is no way to *send* the value `3`.
//!
//! This module therefore defines a D1203-specific [`D1203AmbientSoundMode`]
//! enum with all four variants and drives the `ambientSoundMode` setting from
//! it, so the write path can emit byte 0 = 3 for Adaptive.
//!
//! ## Payload
//!
//! The 7-byte payload was captured from the real device. Byte 0 is the ambient
//! sound mode; the remaining 6 bytes are two fixed "profiles" observed in the
//! captures:
//!
//! - NoiseCanceling / Transparency: `[mode, 0, 0, 5, 17, 1, 0]`
//! - Normal / Adaptive:             `[mode, 0, 0, 1, 17, 0, 0]`
//!
//! Only byte 0 is essential for selecting the ambient mode; the trailing bytes
//! are replayed verbatim per the captures to keep the device happy. The packet
//! uses standard framing, so it goes through the generic `packet::Outbound`
//! serializer (`send_with_response`), NOT the raw path the custom EQ needs.

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
        macros::sound_mode_enum,
        packet::{self, PacketIOController},
        settings_manager::{SettingHandler, SettingHandlerResult},
        state_modifier::StateModifier,
    },
};

use super::super::state::D1203State;
use super::ModuleCollection;

sound_mode_enum!(
    pub enum D1203AmbientSoundMode {
        NoiseCanceling = 0,
        Transparency = 1,
        Normal = 2,
        Adaptive = 3,
    }
);

impl D1203AmbientSoundMode {
    /// The full 7-byte `06 81` payload for this ambient sound mode.
    ///
    /// Byte 0 is the ambient sound mode value (0-3). The trailing 6 bytes are
    /// the fixed profile observed in the real captures: NoiseCanceling and
    /// Transparency share one profile, Normal and Adaptive share another.
    pub fn payload(self) -> [u8; 7] {
        match self {
            Self::NoiseCanceling => [0, 0, 0, 5, 17, 1, 0],
            Self::Transparency => [1, 0, 0, 5, 17, 1, 0],
            Self::Normal => [2, 0, 0, 1, 17, 0, 0],
            Self::Adaptive => [3, 0, 0, 1, 17, 0, 0],
        }
    }
}

/// Exposes the ambient sound mode as a readable/writable [`Setting::Select`]
/// listing all four variants.
struct AmbientSoundModeSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AmbientSoundModeSettingHandler
where
    T: Has<D1203AmbientSoundMode> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::AmbientSoundMode]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let mode: &D1203AmbientSoundMode = state.get();
        Some(Setting::select_from_enum_all_variants(*mode))
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        *state.get_mut() = value.try_as_enum_variant()?;
        Ok(())
    }
}

/// On change, sends the `06 81` ambient-sound-mode write with the captured
/// 7-byte payload for the target mode. Uses the generic serializer (standard
/// framing), so `send_with_response` is correct here.
struct AmbientSoundModeStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for AmbientSoundModeStateModifier
where
    T: Has<D1203AmbientSoundMode> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: D1203AmbientSoundMode = *target_state.get();
        if *state_sender.borrow().get() == target {
            return Ok(());
        }

        self.packet_io
            .send_with_response(&packet::Outbound::new(
                packet::Command([0x06, 0x81]),
                target.payload().to_vec(),
            ))
            .await?;

        state_sender.send_modify(|state| {
            *state.get_mut() = target;
        });
        Ok(())
    }
}

impl ModuleCollection<D1203State> {
    /// Registers the D1203 ambient sound mode (ANC / Transparency / Normal /
    /// Adaptive) setting handler and its `06 81` state modifier.
    pub fn add_d1203_ambient_sound_mode(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, AmbientSoundModeSettingHandler);
        self.state_modifiers
            .push(Box::new(AmbientSoundModeStateModifier { packet_io }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each mode maps to byte 0 = its numeric value, and the full payload
    /// matches the real hardware captures. Adaptive = 3 and Normal = 2 are the
    /// ones the A3957 enum could not represent.
    #[test]
    fn payload_byte_zero_matches_mode() {
        assert_eq!(D1203AmbientSoundMode::NoiseCanceling.payload(), [0, 0, 0, 5, 17, 1, 0]);
        assert_eq!(D1203AmbientSoundMode::Transparency.payload(), [1, 0, 0, 5, 17, 1, 0]);
        assert_eq!(D1203AmbientSoundMode::Normal.payload(), [2, 0, 0, 1, 17, 0, 0]);
        assert_eq!(D1203AmbientSoundMode::Adaptive.payload(), [3, 0, 0, 1, 17, 0, 0]);

        assert_eq!(D1203AmbientSoundMode::NoiseCanceling.payload()[0], 0);
        assert_eq!(D1203AmbientSoundMode::Transparency.payload()[0], 1);
        assert_eq!(D1203AmbientSoundMode::Normal.payload()[0], 2);
        assert_eq!(D1203AmbientSoundMode::Adaptive.payload()[0], 3);
    }

    /// The byte value equals the enum repr for every variant, in particular
    /// Adaptive = 3 which the A3957 `AmbientSoundMode` could not send.
    #[test]
    fn byte_repr_matches() {
        assert_eq!(D1203AmbientSoundMode::NoiseCanceling.byte(), 0);
        assert_eq!(D1203AmbientSoundMode::Transparency.byte(), 1);
        assert_eq!(D1203AmbientSoundMode::Normal.byte(), 2);
        assert_eq!(D1203AmbientSoundMode::Adaptive.byte(), 3);
    }

    /// The `06 81` packet for each mode serializes with standard framing and
    /// carries the expected ambient byte at offset 9 (header 5 + command 2 +
    /// length 2). Pins Adaptive -> 3 and Normal -> 2 on the wire.
    #[test]
    fn outbound_06_81_carries_ambient_byte() {
        for mode in [
            D1203AmbientSoundMode::NoiseCanceling,
            D1203AmbientSoundMode::Transparency,
            D1203AmbientSoundMode::Normal,
            D1203AmbientSoundMode::Adaptive,
        ] {
            let outbound =
                packet::Outbound::new(packet::Command([0x06, 0x81]), mode.payload().to_vec());
            let bytes = outbound.bytes_with_checksum();
            assert_eq!(&bytes[5..7], &[0x06, 0x81], "command must be 06 81");
            assert_eq!(bytes[7], 0x11, "length must be 0x11 (17)");
            assert_eq!(bytes[9], mode.byte(), "ambient byte must equal the mode value");
        }
    }
}
