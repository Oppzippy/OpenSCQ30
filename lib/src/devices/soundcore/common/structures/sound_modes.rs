use nom::{
    IResult, Parser,
    combinator::map,
    error::{ContextError, ParseError, context},
    number::complete::le_u8,
};

use crate::devices::soundcore::common::macros::sound_mode_enum;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
}

impl SoundModes {
    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "group of sound modes",
            map(
                (
                    AmbientSoundMode::take,
                    NoiseCancelingMode::take,
                    TransparencyMode::take,
                    CustomNoiseCanceling::take,
                ),
                |(
                    ambient_sound_mode,
                    noise_canceling_mode,
                    transparency_mode,
                    custom_noise_canceling,
                )| {
                    Self {
                        ambient_sound_mode,
                        noise_canceling_mode,
                        transparency_mode,
                        custom_noise_canceling,
                    }
                },
            ),
        )
        .parse_complete(input)
    }

    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ambient_sound_mode.byte(),
            self.noise_canceling_mode.byte(),
            self.transparency_mode.byte(),
            self.custom_noise_canceling.value(),
        ]
    }
}

sound_mode_enum!(
    pub enum AmbientSoundMode {
        NoiseCanceling = 0,
        Transparency = 1,
        Normal = 2,
    }
);

sound_mode_enum!(
    pub enum NoiseCancelingMode {
        Transport = 0,
        Outdoor = 1,
        Indoor = 2,
        Custom = 3,
    }
);

sound_mode_enum!(
    pub enum TransparencyMode {
        FullyTransparent = 0,
        VocalMode = 1,
    }
);

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CustomNoiseCanceling {
    value: u8,
}

impl CustomNoiseCanceling {
    pub fn new(value: u8) -> Self {
        // Not sure what 255 means here, but it is allowed in addition to 0-10
        let clamped_value = if value == 255 {
            value
        } else {
            value.clamp(0, 10)
        };
        Self {
            value: clamped_value,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], Self, E> {
        context(
            "custom noise canceling",
            map(le_u8, |custom_noise_canceling_level| {
                Self::new(custom_noise_canceling_level)
            }),
        )
        .parse_complete(input)
    }
}
