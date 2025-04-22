#![allow(clippy::inherent_to_string)]

mod api;
pub mod connection;
mod device;
pub mod quick_presets;
pub mod serializable;

use log::LevelFilter;

uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn init_native_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("openscq30-lib"),
    )
}

#[derive(thiserror::Error, Debug, uniffi::Object)]
pub enum Error {
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    DeviceError(#[from] openscq30_lib::api::device::Error),
    #[error(transparent)]
    StorageError(#[from] openscq30_lib::storage::Error),
}

#[derive(uniffi::Error, thiserror::Error, Debug)]
pub enum AndroidError {
    #[error("{0}")]
    Other(String),
}
