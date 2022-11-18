use std::error::Error;

use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum SoundcoreDeviceConnectionError {
    #[error("device not found: {source}")]
    DeviceNotFound { source: Box<dyn Error> },

    #[error("not connected: {source}")]
    NotConnected { source: Box<dyn Error> },

    #[error("name of device with mac address `{mac_address}` not found")]
    NameNotFound { mac_address: String },

    #[error("characteristic `{uuid}` not found: {source}")]
    CharacteristicNotFound { uuid: Uuid, source: Box<dyn Error> },

    #[error(transparent)]
    Other { source: Box<dyn Error> },

    #[error("device didn't respond to request")]
    NoResponse,
}
