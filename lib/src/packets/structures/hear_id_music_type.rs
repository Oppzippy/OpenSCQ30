use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct HearIdMusicType(pub u8);
