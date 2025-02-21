use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Value {
    Bool(bool),
    U16(u16),
    U16Vec(Vec<u16>),
    OptionalU16(Option<u16>),
    I16Vec(Vec<i16>),
    I32(i32),
}

impl Value {
    /// Converts U16 and OptionalU16 into Option<u16>
    pub fn try_as_u16(&self) -> Option<u16> {
        match &self {
            Value::U16(value) => Some(*value),
            Value::OptionalU16(maybe_value) => maybe_value.clone(),
            _ => None,
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
