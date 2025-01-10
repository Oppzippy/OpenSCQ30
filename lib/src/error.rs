use std::panic::Location;

use macaddr::MacAddr6;
use uuid::Uuid;

type InnerError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("device not found: {source:?}")]
    DeviceNotFound { source: Option<InnerError> },

    #[error("not connected: {source:?}")]
    NotConnected { source: Option<InnerError> },

    #[error("name of device with mac address `{mac_address}` not found")]
    NameNotFound {
        mac_address: MacAddr6,
        source: Option<InnerError>,
    },

    #[error("characteristic `{uuid}` not found: {source:?}")]
    CharacteristicNotFound {
        uuid: Uuid,
        source: Option<InnerError>,
    },

    #[error("service not found: {source:?}")]
    ServiceNotFound { source: Option<InnerError> },

    #[error("feature not supported: {feature_name}")]
    FeatureNotSupported { feature_name: &'static str },

    #[error("missing necessary data from headphones: {name}")]
    MissingData { name: &'static str },

    #[error("write failed: {source:?}")]
    WriteFailed { source: InnerError },

    #[error("incomplete state: {message:?}")]
    IncompleteStateError { message: &'static str },
    #[error("timed out: {action}")]
    TimedOut { action: &'static str },

    #[error("parse error: {message:?}")]
    ParseError { message: String },

    #[error("bluetooth adapter not available: {source:?}")]
    BluetoothAdapterNotAvailable { source: Option<InnerError> },

    #[error("{location}: {source:?}")]
    Other {
        source: InnerError,
        location: &'static Location<'static>,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
