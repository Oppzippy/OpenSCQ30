use std::borrow::Cow;

use openscq30_i18n::Translate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Select {
    pub options: Vec<Cow<'static, str>>,
    pub localized_options: Vec<String>,
}

impl Select {
    pub(crate) fn from_enum<T>(variants: impl IntoIterator<Item = T>) -> Select
    where
        T: Into<&'static str> + Translate,
    {
        let (localized_options, options) = variants
            .into_iter()
            .map(|variant| (variant.translate(), Cow::from(variant.into())))
            .collect();
        Self {
            options,
            localized_options,
        }
    }
}
