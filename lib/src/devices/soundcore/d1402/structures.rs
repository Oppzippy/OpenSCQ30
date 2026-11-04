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
    packet::{self, inbound::FromPacketBody},
    structures::AmbientSoundMode,
};

common::structures::flag!(AmbientSoundModeVoicePrompt);
common::structures::flag!(HeyAnkaWakeWord);

/// Unknown byte between the equalizer and the age range. It has only been seen as 30. The
/// equalizer command writes it back where the Space One Pro has gender.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EqualizerUnknownByte(pub u8);

impl EqualizerUnknownByte {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context("d1402 equalizer unknown byte", map(le_u8, Self)).parse_complete(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, MigrationSteps)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    // The device ignores the noise canceling level unless it is already in noise canceling mode, so
    // changing it from any other mode requires switching to noise canceling first. The migration
    // steps take care of sending these as two separate packets.
    #[migration_requirement(field = ambient_sound_mode, value = AmbientSoundMode::NoiseCanceling)]
    pub custom_noise_canceling: CustomNoiseCanceling,
    custom_noise_canceling_low_nibble: u8,
    trailing_bytes: [u8; 4],
}

impl Default for SoundModes {
    fn default() -> Self {
        Self {
            ambient_sound_mode: AmbientSoundMode::default(),
            custom_noise_canceling: CustomNoiseCanceling::new(1),
            // 0xF was the first value captured from hardware, before any probing (0x1F). The issue
            // packets show 0x5F.
            custom_noise_canceling_low_nibble: 0xF,
            trailing_bytes: [255, 0, 0, 1],
        }
    }
}

impl SoundModes {
    pub fn bytes(&self) -> [u8; 6] {
        [
            self.ambient_sound_mode.byte(),
            (self.custom_noise_canceling.inner() << 4) | self.custom_noise_canceling_low_nibble,
            self.trailing_bytes[0],
            self.trailing_bytes[1],
            self.trailing_bytes[2],
            self.trailing_bytes[3],
        ]
    }
}

impl FromPacketBody for SoundModes {
    type DirectionMarker = packet::InboundMarker;

    fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1402 sound modes",
            map(
                (AmbientSoundMode::take, le_u8, le_u8, le_u8, le_u8, le_u8),
                |(
                    ambient_sound_mode,
                    custom_noise_canceling_byte,
                    trailing_byte1,
                    trailing_byte2,
                    trailing_byte3,
                    trailing_byte4,
                )| Self {
                    ambient_sound_mode,
                    custom_noise_canceling: CustomNoiseCanceling::new(
                        custom_noise_canceling_byte >> 4,
                    ),
                    custom_noise_canceling_low_nibble: custom_noise_canceling_byte & 0xF,
                    trailing_bytes: [
                        trailing_byte1,
                        trailing_byte2,
                        trailing_byte3,
                        trailing_byte4,
                    ],
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

// same as the Space One Pro
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub struct ButtonConfiguration {
    pub double_press_action: Option<ButtonAction>,
    /// A double press value this app doesn't know about. It's kept so writing the configuration
    /// back doesn't overwrite it.
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
    AnkaVoiceAssistant = 11,
    AiTranslation = 10,
    NapMode = 13,
}

#[cfg(test)]
mod tests {
    use nom_language::error::VerboseError;

    use super::*;

    #[test]
    fn button_configuration_round_trips_unknown_double_press_byte() {
        let (remaining, button_configuration) =
            ButtonConfiguration::take::<VerboseError<&[u8]>>(&[0x09]).unwrap();

        assert!(remaining.is_empty());
        assert_eq!(button_configuration.double_press_action, None);
        assert_eq!(button_configuration.bytes(), [0x09]);
    }

    #[test]
    fn button_configuration_writes_known_action_and_no_action_bytes() {
        let (_, bass_up) = ButtonConfiguration::take::<VerboseError<&[u8]>>(&[0x07]).unwrap();
        assert_eq!(bass_up.double_press_action, Some(ButtonAction::BassUp));
        assert_eq!(bass_up.bytes(), [0x07]);

        let (_, none) = ButtonConfiguration::take::<VerboseError<&[u8]>>(&[0x0F]).unwrap();
        assert_eq!(none.double_press_action, None);
        assert_eq!(none.bytes(), [0x0F]);
    }

    #[test]
    fn setting_double_press_action_replaces_unknown_byte() {
        let (_, mut button_configuration) =
            ButtonConfiguration::take::<VerboseError<&[u8]>>(&[0x09]).unwrap();

        button_configuration.set_double_press_action(None);

        assert_eq!(button_configuration.bytes(), [0x0F]);
    }
}
