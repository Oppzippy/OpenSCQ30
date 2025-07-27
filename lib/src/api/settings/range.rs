use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range<T> {
    #[serde(flatten)]
    pub range: RangeInclusive<T>,
    pub step: T,
}
