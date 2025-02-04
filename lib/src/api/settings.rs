use equalizer::Equalizer;
pub use range::*;
pub use select::*;
use strum::IntoEnumIterator;
pub use value::*;

mod equalizer;
mod range;
mod select;
mod value;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy, Default)]
pub struct CategoryId<'a>(pub &'a str);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy, Default)]
pub struct SettingId<'a>(pub &'a str);

pub struct IdentifiedSetting {
    pub id: &'static str,
    pub setting: Setting,
}

#[derive(Clone, Debug)]
pub enum Setting {
    Toggle { value: bool },
    I32Range { setting: Range<i32>, value: i32 },
    Select { setting: Select, value: Option<u16> },
    OptionalSelect { setting: Select, value: Option<u16> },
    MultiSelect { setting: Select, value: Vec<u16> },
    Equalizer { setting: Equalizer, value: Vec<i16> },
}

impl From<Setting> for Value {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::Toggle { value } => value.into(),
            Setting::I32Range { value, .. } => value.into(),
            Setting::Select { value, .. } => value.into(),
            Setting::OptionalSelect { value, .. } => value.into(),
            Setting::MultiSelect { value, .. } => value.into(),
            Setting::Equalizer { value, .. } => value.into(),
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
            value: T::iter().position(|v| v == value).map(|i| i as u16),
        }
    }

    pub(crate) fn optional_select_from_enum_all_variants<T>(value: Option<T>) -> Self
    where
        T: IntoEnumIterator,
        T: PartialEq + Into<&'static str>,
    {
        Setting::OptionalSelect {
            setting: Select::from_enum(T::iter()),
            value: value.and_then(|selected_variant| {
                T::iter()
                    .position(|v| selected_variant == v)
                    .map(|i| i as u16)
            }),
        }
    }

    pub(crate) fn select_from_enum<T>(variants: &[T], value: T) -> Self
    where
        for<'a> &'a T: PartialEq + Into<&'static str>,
    {
        Self::Select {
            setting: Select::from_enum(variants),
            value: variants.iter().position(|v| v == &value).map(|i| i as u16),
        }
    }

    pub(crate) fn optional_select_from_enum<T>(variants: &[T], value: Option<T>) -> Self
    where
        for<'a> &'a T: PartialEq + Into<&'static str>,
    {
        Setting::OptionalSelect {
            setting: Select::from_enum(variants),
            value: value.and_then(|selected_variant| {
                variants
                    .iter()
                    .position(|v| &selected_variant == v)
                    .map(|i| i as u16)
            }),
        }
    }
}
