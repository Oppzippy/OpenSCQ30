use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range<T> {
    pub range: RangeInclusive<T>,
    pub step: T,
}
