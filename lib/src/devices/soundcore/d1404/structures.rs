use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};
use openscq30_i18n_macros::Translate;
use openscq30_lib_macros::MigrationSteps;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

use crate::devices::soundcore::common::{
    self,
    modules::sound_modes_v2,
    packet::{self, inbound::FromPacketBody, parsing::take_bool},
    structures::{AmbientSoundMode, EqualizerConfiguration},
};

common::structures::flag!(AmbientSoundModeVoicePrompt);

/// The Q31i encodes equalizer band values as f32 decibels in packets, while this app works
/// with signed 0.1dB steps throughout, so the values are converted at the packet boundary.
pub type D1404EqualizerConfiguration = EqualizerConfiguration<1, 8, -60, 60, 1>;

pub fn decibels_to_adjustment(decibels: f32) -> i16 {
    (decibels * 10.0).round() as i16
}

pub fn adjustment_to_decibels(adjustment: i16) -> f32 {
    adjustment as f32 / 10.0
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash)]
pub struct WindNoiseReduction(pub bool);

impl WindNoiseReduction {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("d1404 wind noise reduction", map(take_bool, Self)).parse_complete(input)
    }
}

// The sound mode command and state update bytes share this layout:
// [mode, level << 4, 0, 0, wind noise reduction, 0].
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    // The device ignores the noise canceling level unless it is already in noise canceling
    // mode, so changing it from any other mode requires switching to noise canceling first.
    // The migration steps take care of sending these as two separate packets.
    #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::NoiseCanceling)]
    pub custom_noise_canceling: CustomNoiseCanceling,
    /// Always 0 in captures from hardware.
    custom_noise_canceling_low_nibble: u8,
    unknown_bytes: [u8; 2],
    #[migration_requirement(
        field = ambient_sound_mode,
        value = AmbientSoundMode::NoiseCanceling,
        value2 = AmbientSoundMode::Transparency,
    )]
    pub wind_noise_reduction: WindNoiseReduction,
    trailing_byte: u8,
}

impl SoundModes {
    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode.byte(),
            (self.custom_noise_canceling.inner() << 4) | self.custom_noise_canceling_low_nibble,
            self.unknown_bytes[0],
            self.unknown_bytes[1],
            self.wind_noise_reduction.0.into(),
            self.trailing_byte,
        ]
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1404 sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    le_u8,
                    le_u8,
                    le_u8,
                    WindNoiseReduction::take,
                    le_u8,
                ),
                |(
                    ambient_sound_mode,
                    custom_noise_canceling_byte,
                    unknown_byte_1,
                    unknown_byte_2,
                    wind_noise_reduction,
                    trailing_byte,
                )| Self {
                    ambient_sound_mode,
                    custom_noise_canceling: CustomNoiseCanceling::new(
                        custom_noise_canceling_byte >> 4,
                    ),
                    custom_noise_canceling_low_nibble: custom_noise_canceling_byte & 0xF,
                    unknown_bytes: [unknown_byte_1, unknown_byte_2],
                    wind_noise_reduction,
                    trailing_byte,
                },
            ),
        )
        .parse_complete(input)
    }
}

impl sound_modes_v2::ToPacketBody for SoundModes {
    fn bytes(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomNoiseCanceling(u8);

impl Default for CustomNoiseCanceling {
    fn default() -> Self {
        Self(1)
    }
}

impl CustomNoiseCanceling {
    pub fn new(byte: u8) -> Self {
        Self(byte.clamp(1, 5))
    }

    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct ButtonConfiguration {
    pub double_press_action: Option<ButtonAction>,
    /// A double press value this app doesn't know about. It's kept so writing the
    /// configuration back doesn't overwrite it.
    unknown_double_press_action: Option<u8>,
}

impl ButtonConfiguration {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |double_press_action_byte| {
            let double_press_action = ButtonAction::from_repr(double_press_action_byte);
            let unknown_double_press_action =
                if double_press_action.is_none() && double_press_action_byte != 0xF {
                    Some(double_press_action_byte)
                } else {
                    None
                };
            Self {
                double_press_action,
                unknown_double_press_action,
            }
        })
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        let byte = match (self.double_press_action, self.unknown_double_press_action) {
            (Some(action), _) => action as u8,
            (None, Some(unknown_byte)) => unknown_byte,
            (None, None) => 0xF,
        };
        [byte]
    }

    /// Sets the double press action from a setting, which replaces any unknown value.
    pub fn set_double_press_action(&mut self, double_press_action: Option<ButtonAction>) {
        self.double_press_action = double_press_action;
        self.unknown_double_press_action = None;
    }
}

#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Eq,
    PartialEq,
    Debug,
    Default,
    FromRepr,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Translate,
)]
pub enum ButtonAction {
    #[default]
    BassUp = 7,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct MfbButtonConfiguration {
    pub action: Option<MfbButtonAction>,
}

impl MfbButtonConfiguration {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |action_byte| {
            let action = if action_byte == 15 {
                None
            } else {
                Some(MfbButtonAction::from_repr(action_byte).unwrap_or_default())
            };
            Self { action }
        })
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        [self.action.map_or(15, |action| action as u8)]
    }
}

#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Eq,
    PartialEq,
    Debug,
    Default,
    FromRepr,
    EnumIter,
    EnumString,
    IntoStaticStr,
    Translate,
)]
pub enum MfbButtonAction {
    #[default]
    SystemVoiceAssistant = 5,
    NapMode = 13,
}
