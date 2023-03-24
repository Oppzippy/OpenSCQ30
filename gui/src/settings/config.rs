use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::equalizer_custom_profile::EqualizerCustomProfile;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Config {
    equalizer_custom_profiles: HashMap<String, EqualizerCustomProfile>,
}

impl Config {
    pub fn set_custom_profile(&mut self, name: String, profile: EqualizerCustomProfile) {
        // If multiple profiles with the same volume offsets existed, it would be ambiguous which should be selected,
        // since the selection is determined only by volume offsets.
        self.equalizer_custom_profiles
            .retain(|_name, p| p.volume_offsets() != profile.volume_offsets());
        self.equalizer_custom_profiles.insert(name, profile);
    }

    pub fn remove_custom_profile(&mut self, name: &str) {
        self.equalizer_custom_profiles.remove(name);
    }

    pub fn custom_profiles(&self) -> &HashMap<String, EqualizerCustomProfile> {
        &self.equalizer_custom_profiles
    }
}
