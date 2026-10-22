//! Button (gesture) configuration for the Soundcore Liberty 5 Pro (D1203).
//!
//! ## Protocol (reverse engineered, cmd `04 8c`)
//!
//! Each change to ONE gesture sends ONE packet with a 4-byte payload
//! `[side, b1, action, b3]` under command `04 8c`, standard framing (the length
//! field includes the checksum, checksum = sum % 256):
//!
//! ```text
//! 08 ee 00 00 00 04 8c 0e 00 [side] [b1] [action] [b3] [checksum]
//! ```
//!
//! Validated byte-for-byte against the real hardware — left single-press ->
//! Play/Pause is `08 ee 00 00 00 04 8c 0e 00 00 02 06 0f ab` (payload
//! `[0, 2, 6, 15]`).
//!
//! ### Payload layout
//!
//! - `byte[0] = side`: `0 = Left`, `1 = Right`.
//! - `(byte[1] = b1, byte[3] = b3)` is the pair that identifies the GESTURE.
//!   Four gestures are configurable (swipe up/down are NOT remappable and are
//!   not implemented):
//!
//!   | gesture      | b1 | b3 |
//!   |--------------|----|----|
//!   | SinglePress  | 2  | 15 |
//!   | DoublePress  | 0  | 6  |
//!   | TriplePress  | 5  | 15 |
//!   | LongPress    | 1  | 4  |
//!
//! - `byte[2] = action`:
//!
//!   | id | action                 |
//!   |----|------------------------|
//!   | 0  | VolumeUp               |
//!   | 1  | VolumeDown             |
//!   | 2  | PreviousTrack          |
//!   | 3  | NextTrack              |
//!   | 4  | AmbientSound           |
//!   | 5  | SystemVoiceAssistant   |
//!   | 6  | PlayPause              |
//!   | 10 | AiTranslation          |
//!   | 11 | AnkerVoiceAssistant    |
//!   | 15 | None                   |
//!
//!   Actions 4, 5, 10, 11 (AmbientSound, SystemVoiceAssistant, AiTranslation,
//!   AnkerVoiceAssistant) are only valid for gestures OTHER than SinglePress —
//!   the official app does not offer them for a single press. For SinglePress
//!   the valid actions are VolumeUp, VolumeDown, PreviousTrack, NextTrack,
//!   PlayPause and None.
//!
//! ## State / read-back
//!
//! There are 8 configurable combinations (2 sides x 4 gestures), exposed as 8
//! `Select` settings under [`CategoryId::ButtonConfiguration`]. The write path
//! is the only one that is known to work.
//!
//! TODO(read): the D1203 state packet does not expose button configuration in
//! any known form (state read-back for the D1203 has been investigated and is
//! infeasible for most fields), so the 8 actions are initialized to `Default`
//! on connect. Writing works regardless.

use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::IntoEnumIterator;
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Setting, Select, SettingId, Value},
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

// Reuse the sound_mode_enum macro: it derives FromRepr/IntoStaticStr/EnumString/
// EnumIter/VariantArray/Translate plus `byte()`, with `#[repr(u8)]` values equal
// to the on-wire action id. The first variant is `Default`.
//
// The ten remappable button actions (`byte[2]` of the `04 8c` payload). The
// `#[repr(u8)]` value is the on-wire action id.
sound_mode_enum!(
    pub enum D1203ButtonAction {
        VolumeUp = 0,
        VolumeDown = 1,
        PreviousTrack = 2,
        NextTrack = 3,
        AmbientSound = 4,
        SystemVoiceAssistant = 5,
        PlayPause = 6,
        AiTranslation = 10,
        AnkerVoiceAssistant = 11,
        None = 15,
    }
);

impl D1203ButtonAction {
    /// Actions valid for a single press: no AmbientSound / voice assistants /
    /// AI translation (the official app hides those on single press).
    pub const SINGLE_PRESS_ACTIONS: &'static [D1203ButtonAction] = &[
        D1203ButtonAction::VolumeUp,
        D1203ButtonAction::VolumeDown,
        D1203ButtonAction::PreviousTrack,
        D1203ButtonAction::NextTrack,
        D1203ButtonAction::PlayPause,
        D1203ButtonAction::None,
    ];
}

/// Which side of the earbuds a gesture belongs to (`byte[0]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D1203ButtonSide {
    Left = 0,
    Right = 1,
}

impl D1203ButtonSide {
    pub fn byte(self) -> u8 {
        self as u8
    }
}

/// A configurable gesture, identified on the wire by the `(b1, b3)` pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D1203Gesture {
    SinglePress,
    DoublePress,
    TriplePress,
    LongPress,
}

impl D1203Gesture {
    /// `(b1, b3)` = `(byte[1], byte[3])` identifying this gesture.
    pub fn b1_b3(self) -> (u8, u8) {
        match self {
            Self::SinglePress => (2, 15),
            Self::DoublePress => (0, 6),
            Self::TriplePress => (5, 15),
            Self::LongPress => (1, 4),
        }
    }

    /// The actions selectable for this gesture. SinglePress is restricted to
    /// the six basic actions; every other gesture allows all ten.
    pub fn available_actions(self) -> Vec<D1203ButtonAction> {
        match self {
            Self::SinglePress => D1203ButtonAction::SINGLE_PRESS_ACTIONS.to_vec(),
            _ => D1203ButtonAction::iter().collect(),
        }
    }
}

/// One of the eight configurable (side x gesture) combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D1203Button {
    LeftSinglePress,
    RightSinglePress,
    LeftDoublePress,
    RightDoublePress,
    LeftTriplePress,
    RightTriplePress,
    LeftLongPress,
    RightLongPress,
}

impl D1203Button {
    /// All eight combinations, in a stable order used to index device state.
    pub const ALL: [D1203Button; 8] = [
        D1203Button::LeftSinglePress,
        D1203Button::RightSinglePress,
        D1203Button::LeftDoublePress,
        D1203Button::RightDoublePress,
        D1203Button::LeftTriplePress,
        D1203Button::RightTriplePress,
        D1203Button::LeftLongPress,
        D1203Button::RightLongPress,
    ];

    pub fn side(self) -> D1203ButtonSide {
        match self {
            Self::LeftSinglePress
            | Self::LeftDoublePress
            | Self::LeftTriplePress
            | Self::LeftLongPress => D1203ButtonSide::Left,
            Self::RightSinglePress
            | Self::RightDoublePress
            | Self::RightTriplePress
            | Self::RightLongPress => D1203ButtonSide::Right,
        }
    }

    pub fn gesture(self) -> D1203Gesture {
        match self {
            Self::LeftSinglePress | Self::RightSinglePress => D1203Gesture::SinglePress,
            Self::LeftDoublePress | Self::RightDoublePress => D1203Gesture::DoublePress,
            Self::LeftTriplePress | Self::RightTriplePress => D1203Gesture::TriplePress,
            Self::LeftLongPress | Self::RightLongPress => D1203Gesture::LongPress,
        }
    }

    /// The [`SettingId`] this combination is exposed under. The existing
    /// side+gesture setting ids are reused.
    pub fn setting_id(self) -> SettingId {
        match self {
            Self::LeftSinglePress => SettingId::LeftSinglePress,
            Self::RightSinglePress => SettingId::RightSinglePress,
            Self::LeftDoublePress => SettingId::LeftDoublePress,
            Self::RightDoublePress => SettingId::RightDoublePress,
            Self::LeftTriplePress => SettingId::LeftTriplePress,
            Self::RightTriplePress => SettingId::RightTriplePress,
            Self::LeftLongPress => SettingId::LeftLongPress,
            Self::RightLongPress => SettingId::RightLongPress,
        }
    }

    pub fn from_setting_id(setting_id: &SettingId) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|button| button.setting_id() == *setting_id)
    }

    /// Index of this button in [`D1203ButtonConfiguration`]'s action array,
    /// matching the order of [`D1203Button::ALL`].
    fn index(self) -> usize {
        Self::ALL
            .into_iter()
            .position(|button| button == self)
            .expect("every D1203Button is in ALL")
    }

    /// The 4-byte `04 8c` payload `[side, b1, action, b3]` that configures this
    /// combination to the given action.
    pub fn payload(self, action: D1203ButtonAction) -> [u8; 4] {
        let (b1, b3) = self.gesture().b1_b3();
        [self.side().byte(), b1, action.byte(), b3]
    }
}

/// The current action of each of the eight configurable buttons, in the order
/// of [`D1203Button::ALL`]. Stored in [`D1203State`] so the state modifier can
/// tell which buttons actually changed.
///
/// TODO(read): the D1203 state packet does not expose button configuration, so
/// every button defaults to `PlayPause` on connect (a valid action for every
/// gesture, including single press). Writing works regardless.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D1203ButtonConfiguration {
    actions: [D1203ButtonAction; 8],
}

impl Default for D1203ButtonConfiguration {
    fn default() -> Self {
        Self {
            actions: [D1203ButtonAction::PlayPause; 8],
        }
    }
}

impl D1203ButtonConfiguration {
    pub fn action(&self, button: D1203Button) -> D1203ButtonAction {
        self.actions[button.index()]
    }

    pub fn set_action(&mut self, button: D1203Button, action: D1203ButtonAction) {
        self.actions[button.index()] = action;
    }
}

/// A [`Select`] of the actions valid for a gesture, with each option's
/// serialized name and localized label.
fn select_for_gesture(gesture: D1203Gesture) -> Select {
    Select::from_enum(gesture.available_actions())
}

/// Handles all eight button settings: each is a [`Setting::Select`] whose
/// options are the actions valid for that button's gesture.
struct ButtonConfigurationSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for ButtonConfigurationSettingHandler
where
    T: Has<D1203ButtonConfiguration> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        D1203Button::ALL
            .into_iter()
            .map(|button| button.setting_id())
            .collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let button = D1203Button::from_setting_id(setting_id)?;
        let config: &D1203ButtonConfiguration = state.get();
        let current = config.action(button);
        Some(Setting::Select {
            setting: select_for_gesture(button.gesture()),
            value: std::borrow::Cow::Borrowed(current.into()),
        })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let button = D1203Button::from_setting_id(setting_id)
            .ok_or(crate::devices::soundcore::common::settings_manager::SettingHandlerError::DoesNotExist)?;
        let action: D1203ButtonAction = value.try_as_enum_variant()?;
        let config: &mut D1203ButtonConfiguration = state.get_mut();
        config.set_action(button, action);
        Ok(())
    }
}

/// On any change to a button's action, sends ONE `04 8c` packet per changed
/// button with payload `[side, b1, action, b3]`. Uses the generic serializer
/// (standard framing), so `send_with_response` is correct here.
struct ButtonConfigurationStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for ButtonConfigurationStateModifier
where
    T: Has<D1203ButtonConfiguration> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: D1203ButtonConfiguration = *target_state.get();

        // Collect the buttons whose action changed, without holding the borrow
        // across the awaits below.
        let changed: Vec<(D1203Button, D1203ButtonAction)> = {
            let current = state_sender.borrow();
            let current_config: &D1203ButtonConfiguration = current.get();
            D1203Button::ALL
                .into_iter()
                .filter_map(|button| {
                    let target_action = target.action(button);
                    (current_config.action(button) != target_action)
                        .then_some((button, target_action))
                })
                .collect()
        };

        if changed.is_empty() {
            return Ok(());
        }

        for (button, action) in &changed {
            let payload = button.payload(*action);
            self.packet_io
                .send_with_response(&packet::Outbound::new(
                    packet::Command([0x04, 0x8c]),
                    payload.to_vec(),
                ))
                .await?;
        }

        state_sender.send_modify(|state| {
            let config: &mut D1203ButtonConfiguration = state.get_mut();
            for (button, action) in &changed {
                config.set_action(*button, *action);
            }
        });
        Ok(())
    }
}

impl ModuleCollection<D1203State> {
    /// Registers the eight D1203 button (gesture) settings under
    /// [`CategoryId::ButtonConfiguration`], plus the `04 8c` state modifier.
    pub fn add_d1203_button_configuration(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::ButtonConfiguration,
            ButtonConfigurationSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(ButtonConfigurationStateModifier { packet_io }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Left single-press -> Play/Pause serializes to exactly the real capture
    /// `08 ee 00 00 00 04 8c 0e 00 00 02 06 0f ab` (payload `[0, 2, 6, 15]`,
    /// checksum = sum % 256).
    #[test]
    fn left_single_play_pause_matches_real_capture() {
        let payload = D1203Button::LeftSinglePress.payload(D1203ButtonAction::PlayPause);
        assert_eq!(payload, [0, 2, 6, 15]);

        let outbound = packet::Outbound::new(packet::Command([0x04, 0x8c]), payload.to_vec());
        const EXPECTED: &[u8] = &[
            0x08, 0xee, 0x00, 0x00, 0x00, 0x04, 0x8c, 0x0e, 0x00, 0x00, 0x02, 0x06, 0x0f, 0xab,
        ];
        assert_eq!(outbound.bytes_with_checksum(), EXPECTED);
    }

    /// Left triple-press -> Play/Pause => payload `[0, 5, 6, 15]`.
    #[test]
    fn left_triple_play_pause_payload() {
        assert_eq!(
            D1203Button::LeftTriplePress.payload(D1203ButtonAction::PlayPause),
            [0, 5, 6, 15]
        );
    }

    /// Left long-press -> Volume Up => payload `[0, 1, 0, 4]`.
    #[test]
    fn left_long_volume_up_payload() {
        assert_eq!(
            D1203Button::LeftLongPress.payload(D1203ButtonAction::VolumeUp),
            [0, 1, 0, 4]
        );
    }

    /// Right single-press -> Next Track => payload `[1, 2, 3, 15]`.
    #[test]
    fn right_single_next_payload() {
        assert_eq!(
            D1203Button::RightSinglePress.payload(D1203ButtonAction::NextTrack),
            [1, 2, 3, 15]
        );
    }

    /// Every gesture's `(b1, b3)` pair matches the reverse-engineered map.
    #[test]
    fn gesture_pairs() {
        assert_eq!(D1203Gesture::SinglePress.b1_b3(), (2, 15));
        assert_eq!(D1203Gesture::DoublePress.b1_b3(), (0, 6));
        assert_eq!(D1203Gesture::TriplePress.b1_b3(), (5, 15));
        assert_eq!(D1203Gesture::LongPress.b1_b3(), (1, 4));
    }

    /// Action ids match the reverse-engineered map.
    #[test]
    fn action_bytes() {
        assert_eq!(D1203ButtonAction::VolumeUp.byte(), 0);
        assert_eq!(D1203ButtonAction::VolumeDown.byte(), 1);
        assert_eq!(D1203ButtonAction::PreviousTrack.byte(), 2);
        assert_eq!(D1203ButtonAction::NextTrack.byte(), 3);
        assert_eq!(D1203ButtonAction::AmbientSound.byte(), 4);
        assert_eq!(D1203ButtonAction::SystemVoiceAssistant.byte(), 5);
        assert_eq!(D1203ButtonAction::PlayPause.byte(), 6);
        assert_eq!(D1203ButtonAction::AiTranslation.byte(), 10);
        assert_eq!(D1203ButtonAction::AnkerVoiceAssistant.byte(), 11);
        assert_eq!(D1203ButtonAction::None.byte(), 15);
    }

    /// Side bytes: Left = 0, Right = 1.
    #[test]
    fn side_bytes() {
        assert_eq!(D1203ButtonSide::Left.byte(), 0);
        assert_eq!(D1203ButtonSide::Right.byte(), 1);
    }

    /// Single press offers only the six basic actions; the extended actions
    /// (AmbientSound, SystemVoiceAssistant, AiTranslation, AnkerVoiceAssistant)
    /// are absent from single press but present on the other gestures.
    #[test]
    fn single_press_action_restriction() {
        let single = D1203Gesture::SinglePress.available_actions();
        assert_eq!(single.len(), 6);
        for excluded in [
            D1203ButtonAction::AmbientSound,
            D1203ButtonAction::SystemVoiceAssistant,
            D1203ButtonAction::AiTranslation,
            D1203ButtonAction::AnkerVoiceAssistant,
        ] {
            assert!(!single.contains(&excluded), "single press must exclude {excluded:?}");
        }

        for gesture in [
            D1203Gesture::DoublePress,
            D1203Gesture::TriplePress,
            D1203Gesture::LongPress,
        ] {
            assert_eq!(gesture.available_actions().len(), 10);
        }
    }

    /// Each button maps to the expected (side, gesture) and the reused
    /// SettingId round-trips.
    #[test]
    fn button_side_gesture_and_setting_id() {
        assert_eq!(D1203Button::LeftSinglePress.side(), D1203ButtonSide::Left);
        assert_eq!(D1203Button::RightLongPress.side(), D1203ButtonSide::Right);
        assert_eq!(
            D1203Button::RightTriplePress.gesture(),
            D1203Gesture::TriplePress
        );
        for button in D1203Button::ALL {
            assert_eq!(
                D1203Button::from_setting_id(&button.setting_id()),
                Some(button)
            );
        }
    }
}
