use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    sequence::tuple,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{AmbientSoundMode, CustomNoiseCanceling, NoiseCancelingMode, TransparencyMode};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    pub transparency_mode: TransparencyMode,
    pub custom_noise_canceling: CustomNoiseCanceling,
}

impl SoundModes {
    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        input: &'a [u8],
    ) -> IResult<&'a [u8], SoundModes, E> {
        context(
            "group of sound modes",
            map(
                tuple((
                    AmbientSoundMode::take,
                    NoiseCancelingMode::take,
                    TransparencyMode::take,
                    CustomNoiseCanceling::take,
                )),
                |(
                    ambient_sound_mode,
                    noise_canceling_mode,
                    transparency_mode,
                    custom_noise_canceling,
                )| {
                    SoundModes {
                        ambient_sound_mode,
                        noise_canceling_mode,
                        transparency_mode,
                        custom_noise_canceling,
                    }
                },
            ),
        )(input)
    }

    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ambient_sound_mode.id(),
            self.noise_canceling_mode.id(),
            self.transparency_mode.id(),
            self.custom_noise_canceling.value(),
        ]
    }
}
