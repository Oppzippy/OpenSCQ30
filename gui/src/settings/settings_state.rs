use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::equalizer_custom_profile::EqualizerCustomProfile;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsState {
    window_width: i32,
    window_height: i32,
    is_maximized: bool,
    equalizer_custom_profiles: HashMap<String, EqualizerCustomProfile>,
}

impl SettingsState {
    pub fn window_width(&self) -> i32 {
        self.window_width
    }

    pub fn set_window_width(&mut self, width: i32) {
        self.window_width = width
    }

    pub fn window_height(&self) -> i32 {
        self.window_height
    }

    pub fn set_window_height(&mut self, height: i32) {
        self.window_height = height
    }

    pub fn is_maximized(&self) -> bool {
        self.is_maximized
    }

    pub fn set_maximized(&mut self, is_maximized: bool) {
        self.is_maximized = is_maximized
    }

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

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            window_width: -1,
            window_height: -1,
            is_maximized: false,
            equalizer_custom_profiles: HashMap::new(),
        }
    }
}
