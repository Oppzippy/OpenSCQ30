use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equalizer {
    pub band_hz: Cow<'static, [u16]>,
    pub fraction_digits: i16,
    pub min: i16,
    pub max: i16,
}
