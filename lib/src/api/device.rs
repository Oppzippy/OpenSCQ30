use std::{panic::Location, sync::Arc};

use async_trait::async_trait;
use macaddr::MacAddr6;
use tokio::sync::watch;

use crate::{devices::DeviceModel, macros::impl_from_source_error_with_location, storage};

use super::{
    connection::{self, ConnectionDescriptor, ConnectionStatus},
    settings::{self, CategoryId, Setting, SettingId, Value},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("connection: {source}")]
    ConnectionError {
        source: connection::Error,
        location: &'static Location<'static>,
    },
    #[error("storage: {source}")]
    StorageError {
        source: storage::Error,
        location: &'static Location<'static>,
    },
    #[error("value: {source}")]
    ValueError {
        source: settings::ValueError,
        location: &'static Location<'static>,
    },
    #[error("{source}")]
    Other {
        source: Box<dyn std::error::Error + Send + Sync>,
        location: &'static Location<'static>,
    },

    #[error("{action} timed out")]
    ActionTimedOut { action: &'static str },
    #[error("device with mac address {mac_address} not found")]
    DeviceNotFound { mac_address: MacAddr6 },
}
pub type Result<T> = std::result::Result<T, Error>;

impl_from_source_error_with_location!(Error::ConnectionError(connection::Error));
impl_from_source_error_with_location!(Error::StorageError(storage::Error));
impl_from_source_error_with_location!(Error::ValueError(settings::ValueError));
impl_from_source_error_with_location!(Error::Other(Box<dyn std::error::Error + Send + Sync>));

impl Error {
    #[track_caller]
    pub fn other<E: std::error::Error + Send + Sync + 'static>(error: E) -> Self {
        (Box::new(error) as Box<dyn std::error::Error + Send + Sync>).into()
    }
}

#[async_trait]
pub trait OpenSCQ30DeviceRegistry {
    async fn devices(&self) -> Result<Vec<ConnectionDescriptor>>;
    async fn connect(
        &self,
        mac_address: MacAddr6,
    ) -> Result<Arc<dyn OpenSCQ30Device + Send + Sync>>;
}

#[async_trait]
pub trait OpenSCQ30Device {
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
    fn model(&self) -> DeviceModel;
    fn categories(&self) -> Vec<CategoryId>;
    fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId>;
    fn setting(&self, setting_id: &SettingId) -> Option<Setting>;
    fn watch_for_changes(&self) -> watch::Receiver<()>;
    async fn set_setting_values(&self, setting_values: Vec<(SettingId, Value)>) -> Result<()>;
}
