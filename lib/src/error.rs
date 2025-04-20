use std::panic::Location;

use crate::api::settings::ValueError;

type InnerError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("device not found: {source:?}")]
    DeviceNotFound { source: Option<InnerError> },

    #[error("write failed: {source:?}")]
    WriteFailed { source: InnerError },

    #[error("timed out: {action}")]
    TimedOut { action: &'static str },

    #[error("bluetooth adapter not available: {source:?}")]
    BluetoothAdapterNotAvailable { source: Option<InnerError> },

    #[error(transparent)]
    Value(ValueError),

    #[error("{location}: {source:?}")]
    Other {
        source: InnerError,
        location: &'static Location<'static>,
    },
}

impl From<ValueError> for Error {
    fn from(value: ValueError) -> Self {
        Self::Value(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
