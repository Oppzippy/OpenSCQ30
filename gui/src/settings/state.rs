use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub window_width: i32,
    pub window_height: i32,
    pub is_maximized: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            window_width: -1,
            window_height: 400,
            is_maximized: false,
        }
    }
}
