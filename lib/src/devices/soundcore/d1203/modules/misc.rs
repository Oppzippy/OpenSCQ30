//! Miscellaneous settings for the Soundcore Liberty 5 Pro (D1203).
//!
//! Reverse engineering (see `D1203_REVERSE_ENGINEERING.md`, section
//! MISCELLANEOUS) showed a family of small feature toggles and one duration
//! selector that are each driven by their OWN dedicated command, independent of
//! the sound-mode (`06 81`) write. They all use the project's STANDARD framing
//! (the length field is the TOTAL packet size INCLUDING the trailing checksum
//! byte, and the checksum is `sum % 256`), so every one of them goes through
//! the generic [`packet::Outbound`] serializer via `send_with_response` — the
//! same path as [`super::wind_noise`], which is the model this module follows.
//!
//! ## Toggles — payload `[sub_id, state]`, `state` 0 = off / 1 = on
//!
//! Command `41 87` ([`TOGGLE_41_87_COMMAND`]):
//!
//! ```text
//! Touch Tone ON (sub 0):   08 ee 00 00 00 41 87 0c 00 00 01 cb
//! Low Battery ON (sub 1):  08 ee 00 00 00 41 87 0c 00 01 01 cc
//! Wearing Tone ON (sub 3): 08 ee 00 00 00 41 87 0c 00 03 01 ce
//! ```
//!
//! Command `41 88` ([`TOGGLE_41_88_COMMAND`] — the same command the wind-noise
//! toggle uses with sub-id `0x0b`):
//!
//! ```text
//! LDAC ON (sub 2):         08 ee 00 00 00 41 88 0c 00 02 01 ce
//! Easy Chat OFF (sub 14):  08 ee 00 00 00 41 88 0c 00 0e 00 d9
//! Soundstage OFF (sub 29): 08 ee 00 00 00 41 88 0c 00 1d 00 e8
//! ```
//!
//! ## Auto power off — command `01 86`, payload `[enabled, duration_idx]`
//!
//! `enabled` is `1` (on) or `0` (never); `duration_idx` selects the timeout
//! (`0` = 10 min, `1` = 20 min, `2` = 30 min, `3` = 60 min).
//!
//! ```text
//! 10 minutes: 08 ee 00 00 00 01 86 0c 00 01 00 8a   (payload [1, 0])
//! 60 minutes: 08 ee 00 00 00 01 86 0c 00 01 03 8d   (payload [1, 3])
//! Never:      08 ee 00 00 00 01 86 0c 00 00 00 89   (payload [0, 0])
//! ```
//!
//! TODO(read): the D1203 state packet TLV tags carrying the current value of
//! each of these settings are not yet identified, so they are all initialized
//! to `Default` on connect. Writing works regardless; only reading back the
//! device's current state is outstanding — exactly as for every other D1203
//! write-only setting.

use std::sync::Arc;

use async_trait::async_trait;
use openscq30_i18n::Translate;
use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};
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
    i18n::fl,
};

use super::super::state::D1203State;
use super::ModuleCollection;

/// Category `0x41`, command `0x87`: the "prompt tone" family of toggles
/// (Touch Tone, Low Battery Prompt, Wearing Tone).
const TOGGLE_41_87_COMMAND: packet::Command = packet::Command([0x41, 0x87]);
/// Category `0x41`, command `0x88`: the shared toggle command (also used by
/// wind noise with sub-id `0x0b`). Here: LDAC, Easy Chat, Soundstage.
const TOGGLE_41_88_COMMAND: packet::Command = packet::Command([0x41, 0x88]);
/// Category `0x01`, command `0x86`: the auto power off selector.
const AUTO_POWER_OFF_COMMAND: packet::Command = packet::Command([0x01, 0x86]);

// --- 41 87 sub-ids ---
const SUB_TOUCH_TONE: u8 = 0;
const SUB_LOW_BATTERY_PROMPT: u8 = 1;
const SUB_WEARING_TONE: u8 = 3;

// --- 41 88 sub-ids (0x0b = wind noise lives in its own module) ---
const SUB_LDAC: u8 = 2;
const SUB_EASY_CHAT: u8 = 14;
const SUB_SOUNDSTAGE: u8 = 29;

/// Builds the 2-byte `[sub_id, state]` payload shared by the `41 87` / `41 88`
/// toggle commands.
pub fn build_toggle_payload(sub_id: u8, enabled: bool) -> [u8; 2] {
    [sub_id, if enabled { 1 } else { 0 }]
}

/// Builds the 2-byte `[enabled, duration_idx]` payload of the `01 86` auto
/// power off command.
pub fn build_auto_power_off_payload(enabled: bool, duration_idx: u8) -> [u8; 2] {
    [if enabled { 1 } else { 0 }, duration_idx]
}

// ===========================================================================
// Generic single-toggle plumbing
// ===========================================================================

/// Describes one on/off setting backed by a `[sub_id, state]` payload under a
/// fixed command. One instance drives one [`SettingId`].
#[derive(Debug, Clone, Copy)]
struct ToggleSpec {
    setting_id: SettingId,
    command: packet::Command,
    sub_id: u8,
}

/// A readable/writable [`Setting::Toggle`] for one [`ToggleSpec`], reading from
/// and writing to the `bool`-wrapping state field `F`.
struct MiscToggleSettingHandler<F> {
    spec: ToggleSpec,
    _marker: std::marker::PhantomData<F>,
}

impl<F> MiscToggleSettingHandler<F> {
    fn new(spec: ToggleSpec) -> Self {
        Self {
            spec,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<F, T> SettingHandler<T> for MiscToggleSettingHandler<F>
where
    F: Copy + Into<bool> + From<bool> + Send + Sync + 'static,
    T: Has<F> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![self.spec.setting_id]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let current: &F = state.get();
        Some(Setting::Toggle {
            value: (*current).into(),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        *state.get_mut() = F::from(value.try_as_bool()?);
        Ok(())
    }
}

/// On any change to the backing toggle state, sends the dedicated
/// `[sub_id, 0|1]` write for that [`ToggleSpec`] (standard framing).
struct MiscToggleStateModifier<F> {
    spec: ToggleSpec,
    packet_io: Arc<PacketIOController>,
    _marker: std::marker::PhantomData<F>,
}

#[async_trait]
impl<F, T> StateModifier<T> for MiscToggleStateModifier<F>
where
    F: Copy + Into<bool> + PartialEq + Send + Sync + 'static,
    T: Has<F> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: F = *target_state.get();
        {
            let current = state_sender.borrow();
            let current_value: &F = current.get();
            if *current_value == target {
                return Ok(());
            }
        }

        let payload = build_toggle_payload(self.spec.sub_id, target.into());
        self.packet_io
            .send_with_response(&packet::Outbound::new(self.spec.command, payload.to_vec()))
            .await?;

        state_sender.send_modify(|state| {
            *state.get_mut() = target;
        });
        Ok(())
    }
}

// ===========================================================================
// Toggle state types — one per feature, each a boolean newtype
// ===========================================================================

macro_rules! bool_toggle_state {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name(pub bool);

        impl From<$name> for bool {
            fn from(value: $name) -> bool {
                value.0
            }
        }

        impl From<bool> for $name {
            fn from(value: bool) -> $name {
                $name(value)
            }
        }
    };
}

bool_toggle_state!(
    /// Touch Tone (`41 87` sub-id 0). Defaults to `false` (OFF).
    D1203TouchTone
);
bool_toggle_state!(
    /// Low Battery Prompt (`41 87` sub-id 1). Defaults to `false` (OFF).
    D1203LowBatteryPrompt
);
bool_toggle_state!(
    /// Wearing Tone (`41 87` sub-id 3). Defaults to `false` (OFF).
    D1203WearingTone
);
bool_toggle_state!(
    /// LDAC high-resolution codec (`41 88` sub-id 2). Defaults to `false` (OFF).
    D1203Ldac
);
bool_toggle_state!(
    /// Easy Chat (`41 88` sub-id 14). Defaults to `false` (OFF).
    D1203EasyChat
);
bool_toggle_state!(
    /// Soundstage / Soundfield rendering (`41 88` sub-id 29). Exposed via the
    /// [`SettingId::SpatialAudio`] setting. Defaults to `false` (OFF).
    D1203Soundstage
);

// ===========================================================================
// Auto power off selector
// ===========================================================================

/// The duration options offered by the auto power off selector, in the order
/// the device's `duration_idx` uses (`0` = 10 min .. `3` = 60 min), plus the
/// `Never` option which disables the timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString, IntoStaticStr)]
#[strum(serialize_all = "camelCase")]
pub enum D1203AutoPowerOff {
    #[strum(serialize = "10m")]
    TenMinutes,
    #[strum(serialize = "20m")]
    TwentyMinutes,
    #[strum(serialize = "30m")]
    ThirtyMinutes,
    #[strum(serialize = "60m")]
    SixtyMinutes,
    Never,
}

impl Default for D1203AutoPowerOff {
    fn default() -> Self {
        // Mirrors the "never" wire default (enabled = 0).
        Self::Never
    }
}

impl Translate for D1203AutoPowerOff {
    fn translate(&self) -> String {
        match self {
            Self::TenMinutes => fl!("x-minutes", minutes = 10),
            Self::TwentyMinutes => fl!("x-minutes", minutes = 20),
            Self::ThirtyMinutes => fl!("x-minutes", minutes = 30),
            Self::SixtyMinutes => fl!("x-minutes", minutes = 60),
            Self::Never => fl!("never"),
        }
    }
}

impl D1203AutoPowerOff {
    /// `(enabled, duration_idx)` for the `01 86` payload. `Never` disables the
    /// timer and resets the index to 0 (matching the real capture
    /// `08 ee 00 00 00 01 86 0c 00 00 00 89`).
    pub fn to_wire(self) -> (bool, u8) {
        match self {
            Self::TenMinutes => (true, 0),
            Self::TwentyMinutes => (true, 1),
            Self::ThirtyMinutes => (true, 2),
            Self::SixtyMinutes => (true, 3),
            Self::Never => (false, 0),
        }
    }
}

/// Exposes auto power off as a readable/writable [`Setting::Select`] over the
/// five [`D1203AutoPowerOff`] variants.
struct AutoPowerOffSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for AutoPowerOffSettingHandler
where
    T: Has<D1203AutoPowerOff> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::AutoPowerOff]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let current: &D1203AutoPowerOff = state.get();
        Some(Setting::select_from_enum_all_variants(*current))
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let selection: D1203AutoPowerOff = value.try_as_enum_variant()?;
        *state.get_mut() = selection;
        Ok(())
    }
}

/// On any change to the auto power off state, sends the `01 86` write with
/// payload `[enabled, duration_idx]` (standard framing).
struct AutoPowerOffStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for AutoPowerOffStateModifier
where
    T: Has<D1203AutoPowerOff> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: D1203AutoPowerOff = *target_state.get();
        {
            let current = state_sender.borrow();
            let current_value: &D1203AutoPowerOff = current.get();
            if *current_value == target {
                return Ok(());
            }
        }

        let (enabled, duration_idx) = target.to_wire();
        let payload = build_auto_power_off_payload(enabled, duration_idx);
        self.packet_io
            .send_with_response(&packet::Outbound::new(
                AUTO_POWER_OFF_COMMAND,
                payload.to_vec(),
            ))
            .await?;

        state_sender.send_modify(|state| {
            *state.get_mut() = target;
        });
        Ok(())
    }
}

// ===========================================================================
// Registration
// ===========================================================================

impl ModuleCollection<D1203State> {
    /// Registers the D1203 miscellaneous settings under
    /// [`CategoryId::Miscellaneous`]: the Touch Tone / Low Battery Prompt /
    /// Wearing Tone toggles (`41 87`), the LDAC / Easy Chat / Soundstage
    /// toggles (`41 88`), and the Auto Power Off selector (`01 86`). Each gets
    /// its own setting handler plus a state modifier that sends the dedicated
    /// command through the generic serializer (standard framing).
    pub fn add_d1203_miscellaneous(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_d1203_toggle::<D1203TouchTone>(ToggleSpec {
            setting_id: SettingId::TouchTone,
            command: TOGGLE_41_87_COMMAND,
            sub_id: SUB_TOUCH_TONE,
        });
        self.add_d1203_toggle::<D1203LowBatteryPrompt>(ToggleSpec {
            setting_id: SettingId::LowBatteryPrompt,
            command: TOGGLE_41_87_COMMAND,
            sub_id: SUB_LOW_BATTERY_PROMPT,
        });
        self.add_d1203_toggle::<D1203WearingTone>(ToggleSpec {
            setting_id: SettingId::WearingTone,
            command: TOGGLE_41_87_COMMAND,
            sub_id: SUB_WEARING_TONE,
        });
        self.add_d1203_toggle::<D1203Ldac>(ToggleSpec {
            setting_id: SettingId::Ldac,
            command: TOGGLE_41_88_COMMAND,
            sub_id: SUB_LDAC,
        });
        self.add_d1203_toggle::<D1203EasyChat>(ToggleSpec {
            setting_id: SettingId::EasyChat,
            command: TOGGLE_41_88_COMMAND,
            sub_id: SUB_EASY_CHAT,
        });
        self.add_d1203_toggle::<D1203Soundstage>(ToggleSpec {
            setting_id: SettingId::SpatialAudio,
            command: TOGGLE_41_88_COMMAND,
            sub_id: SUB_SOUNDSTAGE,
        });

        // Auto power off selector (01 86).
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, AutoPowerOffSettingHandler);
        self.state_modifiers.push(Box::new(AutoPowerOffStateModifier {
            packet_io: packet_io.clone(),
        }));

        // A state modifier per toggle (the handlers above all captured the same
        // packet_io clone; each modifier needs its own).
        self.register_toggle_modifiers(packet_io);
    }

    /// Registers one toggle's setting handler under Miscellaneous.
    fn add_d1203_toggle<F>(&mut self, spec: ToggleSpec)
    where
        F: Copy + Into<bool> + From<bool> + PartialEq + Send + Sync + 'static,
        D1203State: Has<F>,
    {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            MiscToggleSettingHandler::<F>::new(spec),
        );
    }

    /// Pushes a state modifier for each toggle feature. Kept separate from
    /// `add_d1203_toggle` so the generic handler registration and the modifier
    /// registration (which both need the concrete field type) stay readable.
    fn register_toggle_modifiers(&mut self, packet_io: Arc<PacketIOController>) {
        macro_rules! push_modifier {
            ($field:ty, $command:expr, $sub:expr) => {
                self.state_modifiers
                    .push(Box::new(MiscToggleStateModifier::<$field> {
                        spec: ToggleSpec {
                            // setting_id is unused by the modifier; filled for completeness.
                            setting_id: SettingId::TouchTone,
                            command: $command,
                            sub_id: $sub,
                        },
                        packet_io: packet_io.clone(),
                        _marker: std::marker::PhantomData,
                    }));
            };
        }
        push_modifier!(D1203TouchTone, TOGGLE_41_87_COMMAND, SUB_TOUCH_TONE);
        push_modifier!(
            D1203LowBatteryPrompt,
            TOGGLE_41_87_COMMAND,
            SUB_LOW_BATTERY_PROMPT
        );
        push_modifier!(D1203WearingTone, TOGGLE_41_87_COMMAND, SUB_WEARING_TONE);
        push_modifier!(D1203Ldac, TOGGLE_41_88_COMMAND, SUB_LDAC);
        push_modifier!(D1203EasyChat, TOGGLE_41_88_COMMAND, SUB_EASY_CHAT);
        push_modifier!(D1203Soundstage, TOGGLE_41_88_COMMAND, SUB_SOUNDSTAGE);
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn framed(command: packet::Command, payload: [u8; 2]) -> Vec<u8> {
        packet::Outbound::new(command, payload.to_vec()).bytes_with_checksum()
    }

    /// 41 87 toggles serialize byte-for-byte to the real captures (checksum =
    /// sum % 256).
    #[test]
    fn toggle_41_87_framing_matches_captures() {
        // Touch Tone ON  (sub 0): 08 ee 00 00 00 41 87 0c 00 00 01 cb
        assert_eq!(
            framed(TOGGLE_41_87_COMMAND, build_toggle_payload(SUB_TOUCH_TONE, true)),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x87, 0x0c, 0x00, 0x00, 0x01, 0xcb]
        );
        // Low Battery ON (sub 1): 08 ee 00 00 00 41 87 0c 00 01 01 cc
        assert_eq!(
            framed(
                TOGGLE_41_87_COMMAND,
                build_toggle_payload(SUB_LOW_BATTERY_PROMPT, true)
            ),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x87, 0x0c, 0x00, 0x01, 0x01, 0xcc]
        );
        // Wearing Tone ON (sub 3): 08 ee 00 00 00 41 87 0c 00 03 01 ce
        assert_eq!(
            framed(
                TOGGLE_41_87_COMMAND,
                build_toggle_payload(SUB_WEARING_TONE, true)
            ),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x87, 0x0c, 0x00, 0x03, 0x01, 0xce]
        );
    }

    /// 41 88 toggles serialize byte-for-byte to the real captures.
    #[test]
    fn toggle_41_88_framing_matches_captures() {
        // LDAC ON (sub 2):        08 ee 00 00 00 41 88 0c 00 02 01 ce
        assert_eq!(
            framed(TOGGLE_41_88_COMMAND, build_toggle_payload(SUB_LDAC, true)),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x88, 0x0c, 0x00, 0x02, 0x01, 0xce]
        );
        // Easy Chat OFF (sub 14): 08 ee 00 00 00 41 88 0c 00 0e 00 d9
        assert_eq!(
            framed(TOGGLE_41_88_COMMAND, build_toggle_payload(SUB_EASY_CHAT, false)),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x88, 0x0c, 0x00, 0x0e, 0x00, 0xd9]
        );
        // Soundstage OFF (sub 29): 08 ee 00 00 00 41 88 0c 00 1d 00 e8
        assert_eq!(
            framed(TOGGLE_41_88_COMMAND, build_toggle_payload(SUB_SOUNDSTAGE, false)),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x41, 0x88, 0x0c, 0x00, 0x1d, 0x00, 0xe8]
        );
    }

    /// Auto power off serializes byte-for-byte to the real captures.
    #[test]
    fn auto_power_off_framing_matches_captures() {
        // 10 minutes -> [1, 0]: 08 ee 00 00 00 01 86 0c 00 01 00 8a
        let (en, idx) = D1203AutoPowerOff::TenMinutes.to_wire();
        assert_eq!(
            framed(AUTO_POWER_OFF_COMMAND, build_auto_power_off_payload(en, idx)),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x86, 0x0c, 0x00, 0x01, 0x00, 0x8a]
        );
        // 60 minutes -> [1, 3]: 08 ee 00 00 00 01 86 0c 00 01 03 8d
        let (en, idx) = D1203AutoPowerOff::SixtyMinutes.to_wire();
        assert_eq!(
            framed(AUTO_POWER_OFF_COMMAND, build_auto_power_off_payload(en, idx)),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x86, 0x0c, 0x00, 0x01, 0x03, 0x8d]
        );
        // Never -> [0, 0]: 08 ee 00 00 00 01 86 0c 00 00 00 89
        let (en, idx) = D1203AutoPowerOff::Never.to_wire();
        assert_eq!(
            framed(AUTO_POWER_OFF_COMMAND, build_auto_power_off_payload(en, idx)),
            &[0x08, 0xee, 0x00, 0x00, 0x00, 0x01, 0x86, 0x0c, 0x00, 0x00, 0x00, 0x89]
        );
    }

    /// The duration index mapping follows the device's order (10/20/30/60 min)
    /// and Never disables the timer with index 0.
    #[test]
    fn auto_power_off_wire_mapping() {
        assert_eq!(D1203AutoPowerOff::TenMinutes.to_wire(), (true, 0));
        assert_eq!(D1203AutoPowerOff::TwentyMinutes.to_wire(), (true, 1));
        assert_eq!(D1203AutoPowerOff::ThirtyMinutes.to_wire(), (true, 2));
        assert_eq!(D1203AutoPowerOff::SixtyMinutes.to_wire(), (true, 3));
        assert_eq!(D1203AutoPowerOff::Never.to_wire(), (false, 0));
    }

    /// Payload layouts.
    #[test]
    fn payload_layouts() {
        assert_eq!(build_toggle_payload(0, true), [0, 1]);
        assert_eq!(build_toggle_payload(29, false), [29, 0]);
        assert_eq!(build_auto_power_off_payload(true, 3), [1, 3]);
        assert_eq!(build_auto_power_off_payload(false, 0), [0, 0]);
    }
}
