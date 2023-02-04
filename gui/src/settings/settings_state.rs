use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsState {
    pub window_width: i32,
    pub window_height: i32,
    pub is_maximized: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            window_width: -1,
            window_height: -1,
            is_maximized: false,
        }
    }
}
