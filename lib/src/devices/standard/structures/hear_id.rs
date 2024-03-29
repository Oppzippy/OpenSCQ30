#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{BasicHearId, CustomHearId};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", tag = "type"))]
pub enum HearId {
    Basic(BasicHearId),
    Custom(CustomHearId),
}

impl From<BasicHearId> for HearId {
    fn from(basic_hear_id: BasicHearId) -> Self {
        Self::Basic(basic_hear_id)
    }
}

impl From<CustomHearId> for HearId {
    fn from(custom_hear_id: CustomHearId) -> Self {
        Self::Custom(custom_hear_id)
    }
}
