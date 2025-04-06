use nom::{
    IResult,
    combinator::map,
    error::{ContextError, ParseError, context},
    sequence::tuple,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::VolumeAdjustments;

#[derive(Clone, Debug, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct StereoVolumeAdjustments {
    pub left: VolumeAdjustments,
    pub right: VolumeAdjustments,
}

impl StereoVolumeAdjustments {
    pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
        let left_bytes = self.left.bytes();
        let right_bytes = self.right.bytes();
        left_bytes.chain(right_bytes)
    }

    pub(crate) fn take<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
        num_bands: usize,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], StereoVolumeAdjustments, E> {
        move |input| {
            context(
                "stereo volume adjustments",
                map(
                    tuple((
                        VolumeAdjustments::take(num_bands),
                        VolumeAdjustments::take(num_bands),
                    )),
                    |(left, right)| StereoVolumeAdjustments { left, right },
                ),
            )(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StereoVolumeAdjustments;
    use crate::devices::soundcore::standard::structures::VolumeAdjustments;

    #[test]
    fn it_orders_bytes_correctly() {
        let stereo_volume_adjustments = StereoVolumeAdjustments {
            left: VolumeAdjustments::new(vec![0, 1, 2, 3, 4, 5, 6, 7]).unwrap(),
            right: VolumeAdjustments::new(vec![8, 9, 10, 11, 12, 13, 14, 15]).unwrap(),
        };
        let bytes = stereo_volume_adjustments.bytes().collect::<Vec<_>>();
        assert_eq!(
            stereo_volume_adjustments.left.bytes().collect::<Vec<_>>(),
            bytes[0..8]
        );
        assert_eq!(
            stereo_volume_adjustments.right.bytes().collect::<Vec<_>>(),
            bytes[8..16]
        );
    }
}
