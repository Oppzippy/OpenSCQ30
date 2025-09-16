use std::borrow::Cow;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, IntoDiscriminant, IntoEnumIterator};
use thiserror::Error;

use crate::i18n::fl;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
#[strum_discriminants(derive(strum::Display))]
pub enum Value {
    Bool(bool),
    U16(u16),
    U16Vec(Vec<u16>),
    OptionalU16(Option<u16>),
    I16Vec(Vec<i16>),
    I32(i32),
    String(Cow<'static, str>),
    StringVec(Vec<Cow<'static, str>>),
    OptionalString(Option<Cow<'static, str>>),
    ModifiableSelectCommand(ModifiableSelectCommand),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, EnumDiscriminants)]
#[serde(tag = "type", content = "name", rename_all = "camelCase")]
pub enum ModifiableSelectCommand {
    Add(Cow<'static, str>),
    Remove(Cow<'static, str>),
}

#[derive(Clone, Debug, Error)]
pub enum ValueError {
    #[error("expected value of type {expected}, got {}", .actual.discriminant())]
    WrongType {
        expected: ValueDiscriminants,
        actual: Value,
    },

    #[error("expected one of {}, got {actual}", .variants.join(", "))]
    InvalidEnumVariant {
        variants: Box<[&'static str]>,
        actual: Value,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(value) => write!(f, "{value}"),
            Self::U16(value) => write!(f, "{value}"),
            Self::U16Vec(value) => write!(f, "{value:?}"),
            Self::OptionalU16(value) => {
                if let Some(value) = value {
                    write!(f, "{value}")
                } else {
                    f.write_str(&fl!("none"))
                }
            }
            Self::I16Vec(values) => write!(f, "{values:?}"),
            Self::I32(value) => write!(f, "{value}"),
            Self::String(value) => f.write_str(value),
            Self::StringVec(values) => f.write_str(&values.iter().join(", ")),
            Self::OptionalString(value) => {
                if let Some(value) = value {
                    f.write_str(value)
                } else {
                    f.write_str(&fl!("none"))
                }
            }
            Self::ModifiableSelectCommand(_modifiable_select_command) => {
                f.write_str("ModifiableSelectCommand")
            }
        }
    }
}

impl Value {
    pub fn try_as_bool(&self) -> Result<bool, ValueError> {
        if let Self::Bool(value) = self {
            Ok(*value)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::Bool,
                actual: self.clone(),
            })
        }
    }

    pub fn try_as_optional_u16(&self) -> Result<Option<u16>, ValueError> {
        match &self {
            Self::U16(value) => Ok(Some(*value)),
            Self::OptionalU16(maybe_value) => Ok(*maybe_value),
            _ => Err(ValueError::WrongType {
                expected: ValueDiscriminants::OptionalU16,
                actual: self.clone(),
            }),
        }
    }

    pub fn try_as_optional_str(&self) -> Result<Option<&str>, ValueError> {
        match &self {
            Self::String(cow) => Ok(Some(cow)),
            Self::OptionalString(cow) => Ok(cow.as_deref()),
            _ => Err(ValueError::WrongType {
                expected: ValueDiscriminants::OptionalString,
                actual: self.clone(),
            }),
        }
    }

    pub fn try_as_str(&self) -> Result<&str, ValueError> {
        if let Self::String(cow) = self {
            Ok(cow)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::String,
                actual: self.clone(),
            })
        }
    }

    pub fn try_as_enum_variant<T>(&self) -> Result<T, ValueError>
    where
        T: IntoEnumIterator + for<'a> TryFrom<&'a str> + Into<&'static str>,
    {
        let str = self.try_as_str()?;
        T::try_from(str).map_err(|_| ValueError::InvalidEnumVariant {
            variants: T::iter().map(Into::into).collect(),
            actual: self.clone(),
        })
    }

    pub fn try_as_optional_enum_variant<T>(&self) -> Result<Option<T>, ValueError>
    where
        T: IntoEnumIterator + for<'a> TryFrom<&'a str> + Into<&'static str>,
    {
        self.try_as_optional_str()?
            .map(|str| {
                T::try_from(str).map_err(|_| ValueError::InvalidEnumVariant {
                    variants: T::iter().map(Into::into).collect(),
                    actual: self.clone(),
                })
            })
            .transpose()
    }

    pub fn try_as_i32(&self) -> Result<i32, ValueError> {
        if let Self::I32(i) = self {
            Ok(*i)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::I32,
                actual: self.clone(),
            })
        }
    }

    pub fn try_as_i16_slice(&self) -> Result<&[i16], ValueError> {
        if let Self::I16Vec(value) = self {
            Ok(value)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::I16Vec,
                actual: self.clone(),
            })
        }
    }

    pub fn try_into_i16_vec(self) -> Result<Vec<i16>, ValueError> {
        if let Self::I16Vec(value) = self {
            Ok(value)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::I16Vec,
                actual: self,
            })
        }
    }

    pub fn try_into_string_vec(self) -> Result<Vec<Cow<'static, str>>, ValueError> {
        if let Self::StringVec(value) = self {
            Ok(value)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::StringVec,
                actual: self,
            })
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl From<Vec<u16>> for Value {
    fn from(value: Vec<u16>) -> Self {
        Self::U16Vec(value)
    }
}

impl From<Option<u16>> for Value {
    fn from(value: Option<u16>) -> Self {
        Self::OptionalU16(value)
    }
}

impl From<Vec<i16>> for Value {
    fn from(value: Vec<i16>) -> Self {
        Self::I16Vec(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<Cow<'static, str>> for Value {
    fn from(value: Cow<'static, str>) -> Self {
        Self::String(value)
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Self::from(Cow::from(value))
    }
}

impl From<Option<Cow<'static, str>>> for Value {
    fn from(value: Option<Cow<'static, str>>) -> Self {
        Self::OptionalString(value)
    }
}

impl From<Option<&'static str>> for Value {
    fn from(value: Option<&'static str>) -> Self {
        Self::from(value.map(Cow::from))
    }
}

impl From<Vec<Cow<'static, str>>> for Value {
    fn from(value: Vec<Cow<'static, str>>) -> Self {
        Self::StringVec(value)
    }
}
