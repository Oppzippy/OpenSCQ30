//! Sound modes (ambient sound mode + noise-canceling sub-mode + manual NC
//! level) for the Soundcore Liberty 5 Pro (D1203).
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
//! ## The three sound-mode settings and the combined payload
//!
//! The Sound Modes screen for the D1203 exposes THREE settings, all backed by
//! the same `06 81` write:
//!
//! - `ambientSoundMode`     — [`D1203AmbientSoundMode`] (Select, byte 0)
//! - `noiseCancelingMode`   — [`D1203NoiseCancelingMode`] (Select, byte 1)
//! - `manualNoiseCanceling` — [`D1203ManualNoiseCanceling`] (I32Range 1..=5, byte 3)
//!
//! The 7-byte payload layout was confirmed by reverse engineering the real
//! hardware:
//!
//! ```text
//! byte[0] = ambient sound mode      (0=NC, 1=Transparency, 2=Normal, 3=Adaptive)
//! byte[1] = noise canceling sub-mode (0=Manual, 2=Transportation)
//! byte[2] = transparency mode        (always 0 for now)
//! byte[3] = manual NC level          (1..=5)
//! byte[4] = 17                       (constant)
//! byte[5] = active flag              (1 when ambient is NC or Transparency, else 0)
//! byte[6] = 0
//! ```
//!
//! Real captures reproduced by [`build_sound_mode_payload`]:
//!
//! ```text
//! NC Manual lvl5:    [0, 0, 0, 5, 17, 1, 0]
//! NC Manual lvl1:    [0, 0, 0, 1, 17, 1, 0]
//! NC Transportation: [0, 2, 0, 5, 17, 1, 0]
//! Transparency:      [1, 0, 0, 5, 17, 1, 0]
//! Normal:            [2, 0, 0, 1, 17, 0, 0]
//! Adaptive:          [3, 0, 0, 5, 17, 1, 0]
//! ```
//!
//! Only the three mode bytes (0, 1, 3) are driven by user settings; the rest
//! are derived (byte 5 from the ambient mode) or constant. The packet uses
//! standard framing, so it goes through the generic `packet::Outbound`
//! serializer (`send_with_response`), NOT the raw path the custom EQ needs.

use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Range, Setting, SettingId, Value},
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

sound_mode_enum!(
    pub enum D1203NoiseCancelingMode {
        Manual = 0,
        Transportation = 2,
    }
);

// Transport type selected while the NC sub-mode is Transportation.
//
// Reverse engineering the real hardware confirmed that INSIDE the
// Transportation sub-mode (`nc_mode = 2`), byte[5] of the `06 81` payload no
// longer carries the "active flag" — it distinguishes the transport type:
//
//   Airplane: byte[5] = 0  ->  NC Transportation Airplane = [0, 2, 0, 5, 17, 0, 0]
//   Car:      byte[5] = 1  ->  NC Transportation Car      = [0, 2, 0, 5, 17, 1, 0]
//
// So byte[5] is overloaded: outside Transportation it is the active flag (1 for
// NoiseCanceling/Transparency, else 0), and inside Transportation it is this
// transport type. See `build_sound_mode_payload`. The byte value equals the
// enum repr (Airplane = 0, Car = 1), which `.byte()` returns directly.
sound_mode_enum!(
    pub enum D1203TransportationMode {
        Airplane = 0,
        Car = 1,
    }
);

/// Manual ANC level (byte 3 of the `06 81` payload), 1..=5. The device never
/// uses 0; the lowest real level observed in captures is 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct D1203ManualNoiseCanceling(pub u8);

impl D1203ManualNoiseCanceling {
    /// Lowest/highest manual ANC levels exposed to the user.
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 5;

    /// Clamp an arbitrary value into the valid 1..=5 range.
    pub fn new(level: u8) -> Self {
        Self(level.clamp(Self::MIN, Self::MAX))
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

impl Default for D1203ManualNoiseCanceling {
    /// Defaults to the lowest level (1), matching the Normal-mode capture
    /// `[2, 0, 0, 1, 17, 0, 0]`.
    fn default() -> Self {
        Self(Self::MIN)
    }
}

/// Builds the full 7-byte `06 81` payload from the sound-mode fields.
///
/// Layout (see module docs): `[ambient, nc_mode, 0, manual_level, 17, byte5, 0]`.
///
/// `byte5` is overloaded, which is the whole point of this builder:
///
/// - When `nc_mode == Transportation`, byte[5] is the transport TYPE:
///   `transportation.byte()` (Airplane = 0, Car = 1). Confirmed by reverse
///   engineering — e.g. NC Transportation Car = `[0, 2, 0, 5, 17, 1, 0]`,
///   NC Transportation Airplane = `[0, 2, 0, 5, 17, 0, 0]`.
/// - Otherwise byte[5] is the "active flag": 1 when the ambient mode is
///   NoiseCanceling or Transparency, else 0 (unchanged legacy behaviour).
///
/// This is the single source of truth for the write path: every one of the
/// sound-mode settings recomputes the payload from the combined state through
/// it.
pub fn build_sound_mode_payload(
    ambient: D1203AmbientSoundMode,
    nc_mode: D1203NoiseCancelingMode,
    manual: D1203ManualNoiseCanceling,
    transportation: D1203TransportationMode,
) -> [u8; 7] {
    let byte5 = match nc_mode {
        // Inside Transportation, byte[5] selects the transport type.
        D1203NoiseCancelingMode::Transportation => transportation.byte(),
        // Outside Transportation, byte[5] is the legacy active flag.
        D1203NoiseCancelingMode::Manual => match ambient {
            D1203AmbientSoundMode::NoiseCanceling | D1203AmbientSoundMode::Transparency => 1,
            D1203AmbientSoundMode::Normal | D1203AmbientSoundMode::Adaptive => 0,
        },
    };
    [
        ambient.byte(),
        nc_mode.byte(),
        0,
        manual.value(),
        17,
        byte5,
        0,
    ]
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

/// Exposes the noise-canceling sub-mode (byte 1) as a [`Setting::Select`] of
/// `Manual` / `Transportation`.
struct NoiseCancelingModeSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for NoiseCancelingModeSettingHandler
where
    T: Has<D1203NoiseCancelingMode> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::NoiseCancelingMode]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let mode: &D1203NoiseCancelingMode = state.get();
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

/// Exposes the manual ANC level (byte 3) as an [`Setting::I32Range`] 1..=5.
struct ManualNoiseCancelingSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for ManualNoiseCancelingSettingHandler
where
    T: Has<D1203ManualNoiseCanceling> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::ManualNoiseCanceling]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let level: &D1203ManualNoiseCanceling = state.get();
        Some(Setting::I32Range {
            setting: Range {
                range: (D1203ManualNoiseCanceling::MIN as i32)
                    ..=(D1203ManualNoiseCanceling::MAX as i32),
                step: 1,
            },
            value: level.value() as i32,
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let level = value.try_as_i32()?;
        *state.get_mut() = D1203ManualNoiseCanceling::new(level.clamp(0, u8::MAX as i32) as u8);
        Ok(())
    }
}

/// Exposes the transport type (Airplane / Car) as a [`Setting::Select`]. This
/// only affects byte[5] of the `06 81` payload while the NC sub-mode is
/// Transportation; see [`build_sound_mode_payload`].
///
/// Read-back is a TODO: the D1203 state packet does not expose the current
/// transport type (no TLV tag identified), so it is initialized to `Default`
/// on connect. Writing works regardless.
struct TransportationModeSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for TransportationModeSettingHandler
where
    T: Has<D1203TransportationMode> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::TransportationMode]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let mode: &D1203TransportationMode = state.get();
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

/// On any change to the ambient mode, NC sub-mode, manual NC level, or
/// transport type, sends the `06 81` write with the payload recomputed from
/// ALL FOUR fields of the combined state. Uses the generic serializer
/// (standard framing), so `send_with_response` is correct here.
///
/// A single modifier handles all four settings because the device accepts
/// only one combined 7-byte payload: changing any one field means re-sending
/// the whole thing.
struct SoundModeStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for SoundModeStateModifier
where
    T: Has<D1203AmbientSoundMode>
        + Has<D1203NoiseCancelingMode>
        + Has<D1203ManualNoiseCanceling>
        + Has<D1203TransportationMode>
        + Clone
        + Send
        + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target_ambient: D1203AmbientSoundMode = *target_state.get();
        let target_nc_mode: D1203NoiseCancelingMode = *target_state.get();
        let target_manual: D1203ManualNoiseCanceling = *target_state.get();
        let target_transportation: D1203TransportationMode = *target_state.get();

        {
            let current = state_sender.borrow();
            let current_ambient: &D1203AmbientSoundMode = current.get();
            let current_nc_mode: &D1203NoiseCancelingMode = current.get();
            let current_manual: &D1203ManualNoiseCanceling = current.get();
            let current_transportation: &D1203TransportationMode = current.get();
            if *current_ambient == target_ambient
                && *current_nc_mode == target_nc_mode
                && *current_manual == target_manual
                && *current_transportation == target_transportation
            {
                return Ok(());
            }
        }

        let payload = build_sound_mode_payload(
            target_ambient,
            target_nc_mode,
            target_manual,
            target_transportation,
        );
        self.packet_io
            .send_with_response(&packet::Outbound::new(
                packet::Command([0x06, 0x81]),
                payload.to_vec(),
            ))
            .await?;

        state_sender.send_modify(|state| {
            *Has::<D1203AmbientSoundMode>::get_mut(state) = target_ambient;
            *Has::<D1203NoiseCancelingMode>::get_mut(state) = target_nc_mode;
            *Has::<D1203ManualNoiseCanceling>::get_mut(state) = target_manual;
            *Has::<D1203TransportationMode>::get_mut(state) = target_transportation;
        });
        Ok(())
    }
}

impl ModuleCollection<D1203State> {
    /// Registers the four D1203 sound-mode settings (ambient sound mode, NC
    /// sub-mode, manual NC level, transport type) and their shared `06 81`
    /// state modifier.
    pub fn add_d1203_ambient_sound_mode(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, AmbientSoundModeSettingHandler);
        self.setting_manager
            .add_handler(CategoryId::SoundModes, NoiseCancelingModeSettingHandler);
        self.setting_manager
            .add_handler(CategoryId::SoundModes, ManualNoiseCancelingSettingHandler);
        self.setting_manager
            .add_handler(CategoryId::SoundModes, TransportationModeSettingHandler);
        self.state_modifiers
            .push(Box::new(SoundModeStateModifier { packet_io }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The combined payload builder reproduces every real capture byte-for-byte.
    #[test]
    fn combined_payload_matches_real_captures() {
        use D1203AmbientSoundMode::*;
        use D1203NoiseCancelingMode::*;

        // (a) NC + Manual + level 5 -> [0, 0, 0, 5, 17, 1, 0]
        assert_eq!(
            build_sound_mode_payload(
                NoiseCanceling,
                Manual,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane
            ),
            [0, 0, 0, 5, 17, 1, 0]
        );
        // NC + Manual + level 1 -> [0, 0, 0, 1, 17, 1, 0]
        assert_eq!(
            build_sound_mode_payload(
                NoiseCanceling,
                Manual,
                D1203ManualNoiseCanceling(1),
                D1203TransportationMode::Airplane
            ),
            [0, 0, 0, 1, 17, 1, 0]
        );
        // (b) NC + Transportation + Car -> byte[5] = 1 (full: [0, 2, 0, 5, 17, 1, 0])
        assert_eq!(
            build_sound_mode_payload(
                NoiseCanceling,
                Transportation,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Car
            ),
            [0, 2, 0, 5, 17, 1, 0]
        );
        // NC + Transportation + Airplane -> byte[5] = 0 (full: [0, 2, 0, 5, 17, 0, 0])
        assert_eq!(
            build_sound_mode_payload(
                NoiseCanceling,
                Transportation,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane
            ),
            [0, 2, 0, 5, 17, 0, 0]
        );
        // (d) Transparency -> [1, 0, 0, ?, 17, 1, 0]
        assert_eq!(
            build_sound_mode_payload(
                Transparency,
                Manual,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane
            ),
            [1, 0, 0, 5, 17, 1, 0]
        );
        // (e) Normal -> byte[0] = 2, byte[5] = 0
        assert_eq!(
            build_sound_mode_payload(
                Normal,
                Manual,
                D1203ManualNoiseCanceling(1),
                D1203TransportationMode::Airplane
            ),
            [2, 0, 0, 1, 17, 0, 0]
        );
        // Adaptive -> byte[0] = 3, byte[5] = 0
        assert_eq!(
            build_sound_mode_payload(
                Adaptive,
                Manual,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane
            ),
            [3, 0, 0, 5, 17, 0, 0]
        );
    }

    /// Inside the Transportation sub-mode, byte[5] is the transport TYPE, NOT
    /// the active flag: Car = 1, Airplane = 0. Confirmed by reverse engineering.
    #[test]
    fn transportation_mode_drives_byte_five() {
        use D1203AmbientSoundMode::*;
        use D1203NoiseCancelingMode::*;

        // NC + Transportation + Car -> [0, 2, 0, 5, 17, 1, 0]
        assert_eq!(
            build_sound_mode_payload(
                NoiseCanceling,
                Transportation,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Car
            ),
            [0, 2, 0, 5, 17, 1, 0]
        );
        // NC + Transportation + Airplane -> [0, 2, 0, 5, 17, 0, 0]
        assert_eq!(
            build_sound_mode_payload(
                NoiseCanceling,
                Transportation,
                D1203ManualNoiseCanceling(5),
                D1203TransportationMode::Airplane
            ),
            [0, 2, 0, 5, 17, 0, 0]
        );
        // The transport byte values themselves: Airplane = 0, Car = 1.
        assert_eq!(D1203TransportationMode::Airplane.byte(), 0);
        assert_eq!(D1203TransportationMode::Car.byte(), 1);
    }

    /// Outside Transportation (i.e. Manual sub-mode), byte[5] stays the legacy
    /// active flag and is unaffected by the transport-type field.
    #[test]
    fn transportation_mode_ignored_outside_transportation() {
        use D1203AmbientSoundMode::*;
        use D1203NoiseCancelingMode::*;

        // NC Manual lvl5 -> [0, 0, 0, 5, 17, 1, 0] regardless of transport type.
        for transportation in [D1203TransportationMode::Airplane, D1203TransportationMode::Car] {
            assert_eq!(
                build_sound_mode_payload(
                    NoiseCanceling,
                    Manual,
                    D1203ManualNoiseCanceling(5),
                    transportation
                ),
                [0, 0, 0, 5, 17, 1, 0],
                "byte[5] must follow the active flag outside Transportation"
            );
        }
        // Normal keeps flag 0 regardless of transport type.
        for transportation in [D1203TransportationMode::Airplane, D1203TransportationMode::Car] {
            assert_eq!(
                build_sound_mode_payload(
                    Normal,
                    Manual,
                    D1203ManualNoiseCanceling(1),
                    transportation
                )[5],
                0,
            );
        }
    }

    /// (b) The NC sub-mode drives byte[1] exactly (Manual=0, Transportation=2).
    #[test]
    fn nc_mode_drives_byte_one() {
        assert_eq!(D1203NoiseCancelingMode::Manual.byte(), 0);
        assert_eq!(D1203NoiseCancelingMode::Transportation.byte(), 2);

        let manual = build_sound_mode_payload(
            D1203AmbientSoundMode::NoiseCanceling,
            D1203NoiseCancelingMode::Manual,
            D1203ManualNoiseCanceling(5),
            D1203TransportationMode::Airplane,
        );
        assert_eq!(manual[1], 0);
        let transportation = build_sound_mode_payload(
            D1203AmbientSoundMode::NoiseCanceling,
            D1203NoiseCancelingMode::Transportation,
            D1203ManualNoiseCanceling(5),
            D1203TransportationMode::Airplane,
        );
        assert_eq!(transportation[1], 2);
    }

    /// (c) The manual NC level drives byte[3] exactly.
    #[test]
    fn manual_level_drives_byte_three() {
        for level in D1203ManualNoiseCanceling::MIN..=D1203ManualNoiseCanceling::MAX {
            let payload = build_sound_mode_payload(
                D1203AmbientSoundMode::NoiseCanceling,
                D1203NoiseCancelingMode::Manual,
                D1203ManualNoiseCanceling(level),
                D1203TransportationMode::Airplane,
            );
            assert_eq!(payload[3], level, "byte[3] must equal the manual level");
        }
    }

    /// Byte[5] (active flag) is 1 only for NoiseCanceling and Transparency.
    #[test]
    fn active_flag_depends_on_ambient_mode() {
        use D1203AmbientSoundMode::*;
        let flag = |ambient| {
            build_sound_mode_payload(
                ambient,
                D1203NoiseCancelingMode::Manual,
                D1203ManualNoiseCanceling(1),
                D1203TransportationMode::Airplane,
            )[5]
        };
        assert_eq!(flag(NoiseCanceling), 1);
        assert_eq!(flag(Transparency), 1);
        assert_eq!(flag(Normal), 0);
        assert_eq!(flag(Adaptive), 0);
    }

    /// `D1203ManualNoiseCanceling::new` clamps into 1..=5.
    #[test]
    fn manual_level_clamps() {
        assert_eq!(D1203ManualNoiseCanceling::new(0).value(), 1);
        assert_eq!(D1203ManualNoiseCanceling::new(1).value(), 1);
        assert_eq!(D1203ManualNoiseCanceling::new(5).value(), 5);
        assert_eq!(D1203ManualNoiseCanceling::new(99).value(), 5);
    }

    /// The byte value equals the enum repr for every ambient variant, in
    /// particular Adaptive = 3 which the A3957 `AmbientSoundMode` could not send.
    #[test]
    fn ambient_byte_repr_matches() {
        assert_eq!(D1203AmbientSoundMode::NoiseCanceling.byte(), 0);
        assert_eq!(D1203AmbientSoundMode::Transparency.byte(), 1);
        assert_eq!(D1203AmbientSoundMode::Normal.byte(), 2);
        assert_eq!(D1203AmbientSoundMode::Adaptive.byte(), 3);
    }

    /// The `06 81` packet for a combined payload serializes with standard
    /// framing and carries the expected ambient byte at offset 9 (header 5 +
    /// command 2 + length 2) and NC byte at offset 10.
    #[test]
    fn outbound_06_81_framing() {
        let payload = build_sound_mode_payload(
            D1203AmbientSoundMode::NoiseCanceling,
            D1203NoiseCancelingMode::Transportation,
            D1203ManualNoiseCanceling(5),
            D1203TransportationMode::Car,
        );
        let outbound = packet::Outbound::new(packet::Command([0x06, 0x81]), payload.to_vec());
        let bytes = outbound.bytes_with_checksum();
        assert_eq!(&bytes[5..7], &[0x06, 0x81], "command must be 06 81");
        assert_eq!(bytes[7], 0x11, "length must be 0x11 (17)");
        assert_eq!(bytes[9], 0, "ambient byte (NC) at offset 9");
        assert_eq!(bytes[10], 2, "NC sub-mode byte (Transportation) at offset 10");
    }
}
