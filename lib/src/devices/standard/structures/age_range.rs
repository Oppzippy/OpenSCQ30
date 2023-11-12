use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AgeRange(pub u8);

impl AgeRange {
    pub fn supports_hear_id(&self) -> bool {
        self.0 != u8::MAX
    }
}
