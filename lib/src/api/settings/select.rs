use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    pub options: Vec<Cow<'static, str>>,
}

impl Select {
    pub(crate) fn from_enum<T>(variants: impl IntoIterator<Item = T>) -> Select
    where
        T: Into<&'static str>,
    {
        let options: Vec<Cow<'static, str>> = variants
            .into_iter()
            .map(|variant| variant.into())
            .map(Cow::from)
            .collect();
        Self { options }
    }
}
