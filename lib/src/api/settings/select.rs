use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Select {
    pub options: Vec<Cow<'static, str>>,
    pub has_add_button: bool,
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
        Self {
            options,
            has_add_button: false,
        }
    }

    pub(crate) fn with_add_button(mut self, is_enabled: bool) -> Self {
        self.has_add_button = is_enabled;
        self
    }
}
