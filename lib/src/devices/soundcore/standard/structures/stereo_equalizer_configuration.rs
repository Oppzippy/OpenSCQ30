use super::{EqualizerConfiguration, VolumeAdjustments};
use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    sequence::tuple,
};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct StereoEqualizerConfiguration {
    pub left: EqualizerConfiguration,
    pub right: EqualizerConfiguration,
}

impl StereoEqualizerConfiguration {
    pub fn new(left: EqualizerConfiguration, right: VolumeAdjustments) -> Self {
        if left.preset_profile().is_some() {
            Self {
                right: left.to_owned(),
                left,
            }
        } else {
            Self {
                left,
                right: EqualizerConfiguration::new_custom_profile(right),
            }
        }
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        num_bands: usize,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], EqualizerConfiguration, E> {
        move |input| {
            context(
                "stereo equalizer configuration",
                map(
                    tuple((
                        EqualizerConfiguration::take(num_bands),
                        VolumeAdjustments::take(num_bands),
                    )),
                    // discard the right channel since we don't support it yet
                    |results| results.0,
                ),
            )(input)
        }
    }
}
