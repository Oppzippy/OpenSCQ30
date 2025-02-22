use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
    pub step: T,
}
