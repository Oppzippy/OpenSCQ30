use uuid::Uuid;

type InnerError = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("device not found: {source:?}")]
    DeviceNotFound { source: InnerError },

    #[error("not connected: {source:?}")]
    NotConnected { source: InnerError },

    #[error("name of device with mac address `{mac_address}` not found")]
    NameNotFound { mac_address: String },

    #[error("characteristic `{uuid}` not found: {source:?}")]
    CharacteristicNotFound {
        uuid: Uuid,
        source: Option<InnerError>,
    },

    #[error("service `{uuid}` not found: {source:?}")]
    ServiceNotFound {
        // TODO remove this field since we are looking for a range of uuids, not one in particular
        uuid: Uuid,
        source: Option<InnerError>,
    },

    #[error("{source:?}")]
    Other { source: InnerError },

    #[error("device didn't respond to request")]
    NoResponse,

    #[error("feature not supported: {feature_name}")]
    FeatureNotSupported { feature_name: &'static str },

    #[error("missing necessary data from headphones: {name}")]
    MissingData { name: &'static str },

    #[error("write failed: {source:?}")]
    WriteFailed { source: InnerError },

    #[error("incomplete state: {message:?}")]
    IncompleteStateError { message: &'static str },
}

pub type Result<T> = std::result::Result<T, Error>;
