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
        uuid: Uuid,
        source: Option<InnerError>,
    },

    #[error("{source:?}")]
    Other { source: InnerError },

    #[error("device didn't respond to request")]
    NoResponse,
}

pub type Result<T> = std::result::Result<T, Error>;
