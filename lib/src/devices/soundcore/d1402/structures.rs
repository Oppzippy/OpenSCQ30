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

/// The two bytes between the equalizer and hear id. The equalizer command writes these back in the
/// place where the Space One Pro has gender and age, so they need to be kept around to be able to
/// write them unchanged. Writing zeros was observed to overwrite the second byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EqualizerUnknownBytes(pub [u8; 2]);

impl EqualizerUnknownBytes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "d1402 equalizer unknown bytes",
            map((le_u8, le_u8), |(first, second)| Self([first, second])),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 2] {
        self.0
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct CustomNoiseCanceling(u8);

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
}

impl ButtonConfiguration {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        map(le_u8, |single_press_action| Self {
            double_press_action: ButtonAction::from_repr(single_press_action),
        })
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 1] {
        [self.double_press_action.map_or(0xF, |action| action as u8)]
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
