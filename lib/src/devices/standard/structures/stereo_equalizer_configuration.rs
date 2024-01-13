use super::{EqualizerConfiguration, VolumeAdjustments};

#[derive(Clone, Debug, PartialEq)]
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
}
