#![allow(clippy::inherent_to_string)]

mod api;
pub mod connection;
mod device;
pub mod i18n;
pub mod quick_presets;
pub mod serializable;

use log::LevelFilter;
use openscq30_lib::devices::DeviceModel;
use strum::IntoEnumIterator;

uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn init_native_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("openscq30-lib"),
    )
}

#[derive(thiserror::Error, Debug, uniffi::Error)]
#[uniffi(flat_error)]
pub enum Error {
    #[error("JsonError: {0:?}")]
    JsonError(#[from] serde_json::Error),
    #[error("DeviceError: {0:?}")]
    DeviceError(#[from] openscq30_lib::api::device::Error),
    #[error("StorageError: {0:?}")]
    StorageError(#[from] openscq30_lib::storage::Error),
}

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum AndroidError {
    #[error("{0}")]
    Other(String),
}

#[uniffi::export]
pub fn device_models() -> Vec<serializable::DeviceModel> {
    DeviceModel::iter().map(serializable::DeviceModel).collect()
}
