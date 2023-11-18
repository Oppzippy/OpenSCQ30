use openscq30_lib::devices::standard::structures;
use rifgen::rifgen_attr::{generate_interface, generate_interface_doc};

use super::{PresetEqualizerProfile, VolumeAdjustments};

#[generate_interface_doc]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EqualizerConfiguration {
    inner: structures::EqualizerConfiguration,
}

impl EqualizerConfiguration {
    #[generate_interface(constructor)]
    pub fn new_from_preset_profile(
        preset_profile: PresetEqualizerProfile,
    ) -> EqualizerConfiguration {
        Self {
            inner: structures::EqualizerConfiguration::new_from_preset_profile(
                preset_profile.into(),
            ),
        }
    }

    #[generate_interface(constructor)]
    pub fn new_custom_profile(volume_adjustments: &VolumeAdjustments) -> EqualizerConfiguration {
        Self {
            inner: structures::EqualizerConfiguration::new_custom_profile(
                volume_adjustments.to_owned().into(),
            ),
        }
    }

    #[generate_interface]
    pub fn profile_id(&self) -> i32 {
        self.inner.profile_id().into()
    }

    #[generate_interface]
    pub fn preset_profile(&self) -> Option<PresetEqualizerProfile> {
        self.inner.preset_profile().map(|profile| profile.into())
    }

    #[generate_interface]
    pub fn volume_adjustments(&self) -> VolumeAdjustments {
        self.inner.volume_adjustments().to_owned().into()
    }

    #[generate_interface]
    pub fn equals(&self, other: &EqualizerConfiguration) -> bool {
        self == other
    }
}

impl From<structures::EqualizerConfiguration> for EqualizerConfiguration {
    fn from(value: structures::EqualizerConfiguration) -> Self {
        Self { inner: value }
    }
}

impl From<EqualizerConfiguration> for structures::EqualizerConfiguration {
    fn from(value: EqualizerConfiguration) -> Self {
        value.inner
    }
}