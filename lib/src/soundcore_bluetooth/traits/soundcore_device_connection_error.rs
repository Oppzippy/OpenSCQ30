#[derive(thiserror::Error, Debug)]
pub enum SoundcoreDeviceError {
    #[error("name of device with uuid `{0}` not found")]
    NameNotFound(String),

    #[error("characteristic `{0}` not found")]
    CharacteristicNotFound(String),
}
