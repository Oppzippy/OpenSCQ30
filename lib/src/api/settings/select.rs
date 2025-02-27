use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Select {
    pub options: Vec<Cow<'static, str>>,
}

impl Select {
    pub fn new(options: Vec<Cow<'static, str>>) -> Self {
        Self {
            options,
            ..Default::default()
        }
    }

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
