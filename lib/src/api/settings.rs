use std::borrow::Cow;

pub use equalizer::*;
pub use range::*;
pub use select::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
pub use value::*;

mod equalizer;
mod range;
mod select;
mod value;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Default, Serialize, Deserialize)]
pub struct CategoryId<'a>(pub Cow<'a, str>);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Default, Serialize, Deserialize)]
pub struct SettingId<'a>(pub Cow<'a, str>);

pub struct IdentifiedSetting {
    pub id: &'static str,
    pub setting: Setting,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "setting", rename_all = "camelCase")]
pub enum Setting {
    Toggle {
        value: bool,
    },
    I32Range {
        setting: Range<i32>,
        value: i32,
    },
    // Select/OptionalSelect is just a hint about whether None is an acceptable value or not.
    // The backing data is still Option<u16> for both and should be treated the same by the backend.
    Select {
        setting: Select,
        value: Cow<'static, str>,
    },
    OptionalSelect {
        setting: Select,
        value: Option<Cow<'static, str>>,
    },
    MultiSelect {
        setting: Select,
        value: Vec<Cow<'static, str>>,
    },
    Equalizer {
        setting: Equalizer,
        values: Vec<i16>,
    },
}

impl From<Setting> for Value {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::Toggle { value } => value.into(),
            Setting::I32Range { value, .. } => value.into(),
            Setting::Select { value, .. } => value.into(),
            Setting::OptionalSelect { value, .. } => value.into(),
            Setting::MultiSelect { value, .. } => value.into(),
            Setting::Equalizer { values: value, .. } => value.into(),
        }
    }
}

impl Setting {
    pub(crate) fn select_from_enum_all_variants<T>(value: T) -> Self
    where
        T: IntoEnumIterator,
        T: PartialEq + Into<&'static str>,
    {
        Self::Select {
            setting: Select::from_enum(T::iter()),
            value: Cow::Borrowed(value.into()),
        }
    }

    pub(crate) fn optional_select_from_enum_all_variants<T>(value: Option<T>) -> Self
    where
        T: IntoEnumIterator,
        T: PartialEq + Into<&'static str>,
    {
        Setting::OptionalSelect {
            setting: Select::from_enum(T::iter()),
            value: value.map(|v| Cow::Borrowed(v.into())),
        }
    }

    pub(crate) fn select_from_enum<T>(variants: &[T], value: T) -> Self
    where
        for<'a> &'a T: PartialEq + Into<&'static str>,
        T: Into<&'static str>,
    {
        Self::Select {
            setting: Select::from_enum(variants),
            value: Cow::Borrowed(value.into()),
        }
    }

    pub(crate) fn optional_select_from_enum<T>(variants: &[T], value: Option<T>) -> Self
    where
        for<'a> &'a T: PartialEq + Into<&'static str>,
        T: Into<&'static str>,
    {
        Setting::OptionalSelect {
            setting: Select::from_enum(variants),
            value: value.map(|v| Cow::Borrowed(v.into())),
        }
    }
}
