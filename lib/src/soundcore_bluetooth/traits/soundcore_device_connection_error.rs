use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum SoundcoreDeviceConnectionError {
    #[error("device not found: {source}")]
    DeviceNotFound { source: Box<dyn Error> },

    #[error("not connected: {source}")]
    NotConnected { source: Box<dyn Error> },

    #[error("name of device with mac address `{mac_address}` not found")]
    NameNotFound { mac_address: String },

    #[error("characteristic `{0}` not found")]
    CharacteristicNotFound(String),

    #[error(transparent)]
    Other { source: Box<dyn Error> },
}
