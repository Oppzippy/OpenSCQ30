use std::borrow::Cow;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::{EnumDiscriminants, IntoEnumIterator};
use thiserror::Error;

use crate::i18n::fl;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumDiscriminants)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumDiscriminants)]
#[serde(tag = "type", content = "name", rename_all = "camelCase")]
pub enum ModifiableSelectCommand {
    Add(Cow<'static, str>),
    Remove(Cow<'static, str>),
}

#[derive(Clone, Debug, Error)]
pub enum ValueError {
    #[error("expected value of type {expected:?}, got {actual:?}")]
    WrongType {
        expected: ValueDiscriminants,
        actual: Value,
    },

    #[error("expected one of {variants:?}, got {actual:?}")]
    InvalidEnumVariant {
        variants: Box<[&'static str]>,
        actual: Value,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{value}"),
            Value::U16(value) => write!(f, "{value}"),
            Value::U16Vec(value) => write!(f, "{value:?}"),
            Value::OptionalU16(value) => {
                if let Some(value) = value {
                    write!(f, "{value}")
                } else {
                    f.write_str(&fl!("none"))
                }
            }
            Value::I16Vec(values) => write!(f, "{values:?}"),
            Value::I32(value) => write!(f, "{value}"),
            Value::String(value) => f.write_str(value),
            Value::StringVec(values) => f.write_str(&values.iter().join(", ")),
            Value::OptionalString(value) => {
                if let Some(value) = value {
                    f.write_str(value)
                } else {
                    f.write_str(&fl!("none"))
                }
            }
            Value::ModifiableSelectCommand(_modifiable_select_command) => {
                f.write_str("ModifiableSelectCommand")
            }
        }
    }
}

impl Value {
    pub fn try_as_bool(&self) -> Result<bool, ValueError> {
        if let Value::Bool(value) = self {
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
            Value::U16(value) => Ok(Some(*value)),
            Value::OptionalU16(maybe_value) => Ok(*maybe_value),
            _ => Err(ValueError::WrongType {
                expected: ValueDiscriminants::OptionalU16,
                actual: self.clone(),
            }),
        }
    }

    pub fn try_as_optional_str(&self) -> Result<Option<&str>, ValueError> {
        match &self {
            Value::String(cow) => Ok(Some(cow)),
            Value::OptionalString(cow) => Ok(cow.as_deref()),
            _ => Err(ValueError::WrongType {
                expected: ValueDiscriminants::OptionalString,
                actual: self.clone(),
            }),
        }
    }

    pub fn try_as_str(&self) -> Result<&str, ValueError> {
        if let Value::String(cow) = self {
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
        if let Value::I32(i) = self {
            Ok(*i)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::I32,
                actual: self.clone(),
            })
        }
    }

    pub fn try_as_i16_slice(&self) -> Result<&[i16], ValueError> {
        if let Value::I16Vec(value) = self {
            Ok(value)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::I16Vec,
                actual: self.clone(),
            })
        }
    }

    pub fn try_into_i16_vec(self) -> Result<Vec<i16>, ValueError> {
        if let Value::I16Vec(value) = self {
            Ok(value)
        } else {
            Err(ValueError::WrongType {
                expected: ValueDiscriminants::I16Vec,
                actual: self,
            })
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<Vec<u16>> for Value {
    fn from(value: Vec<u16>) -> Self {
        Value::U16Vec(value)
    }
}

impl From<Option<u16>> for Value {
    fn from(value: Option<u16>) -> Self {
        Value::OptionalU16(value)
    }
}

impl From<Vec<i16>> for Value {
    fn from(value: Vec<i16>) -> Self {
        Value::I16Vec(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<Cow<'static, str>> for Value {
    fn from(value: Cow<'static, str>) -> Self {
        Self::String(value)
    }
}

impl From<Option<Cow<'static, str>>> for Value {
    fn from(value: Option<Cow<'static, str>>) -> Self {
        Self::OptionalString(value)
    }
}

impl From<Vec<Cow<'static, str>>> for Value {
    fn from(value: Vec<Cow<'static, str>>) -> Self {
        Self::StringVec(value)
    }
}
