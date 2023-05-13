use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::custom_equalizer_profile::CustomEqualizerProfile;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
    equalizer_custom_profiles: HashMap<String, CustomEqualizerProfile>,
}

impl Config {
    pub fn set_custom_profile(&mut self, name: String, profile: CustomEqualizerProfile) {
        // If multiple profiles with the same volume adjustments existed, it would be ambiguous which should be selected,
        // since the selection is determined only by volume adjustments.
        self.equalizer_custom_profiles
            .retain(|_name, p| p.volume_adjustments() != profile.volume_adjustments());
        self.equalizer_custom_profiles.insert(name, profile);
    }

    pub fn remove_custom_profile(&mut self, name: &str) {
        self.equalizer_custom_profiles.remove(name);
    }

    pub fn custom_profiles(&self) -> &HashMap<String, CustomEqualizerProfile> {
        &self.equalizer_custom_profiles
    }
}
