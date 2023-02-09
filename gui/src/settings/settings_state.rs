use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::equalizer_custom_profile::EqualizerCustomProfile;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsState {
    pub window_width: i32,
    pub window_height: i32,
    pub is_maximized: bool,
    pub equalizer_custom_profiles: HashMap<String, EqualizerCustomProfile>,
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
