#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// unsure what this is. values 0 to 2 are allowed. maybe switch to an enum when the meanings are determined.
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HearIdType(pub u8);
