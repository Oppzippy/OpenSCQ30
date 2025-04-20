use std::sync::Arc;

use async_trait::async_trait;
use macaddr::MacAddr6;

use crate::{devices::DeviceModel, storage};

use super::{
    connection::{self, DeviceDescriptor},
    settings::{self, CategoryId, Setting, SettingId, Value},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ConnectionError(#[from] connection::Error),
    #[error(transparent)]
    StorageError(#[from] storage::Error),
    #[error(transparent)]
    ValueError(#[from] settings::ValueError),
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),

    #[error("{action} timed out")]
    ActionTimedOut { action: &'static str },
    #[error("device with mac address {mac_address} not found")]
    DeviceNotFound { mac_address: MacAddr6 },
}
pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
pub trait OpenSCQ30DeviceRegistry {
    async fn devices(&self) -> Result<Vec<DeviceDescriptor>>;
    async fn connect(
        &self,
        mac_address: MacAddr6,
    ) -> Result<Arc<dyn OpenSCQ30Device + Send + Sync>>;
}

#[async_trait]
pub trait OpenSCQ30Device {
    fn model(&self) -> DeviceModel;
    fn categories(&self) -> Vec<CategoryId>;
    fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId>;
    fn setting(&self, setting_id: &SettingId) -> Option<Setting>;
    // async fn watch_for_changes(&self) -> broadcast::Receiver<()>;
    async fn set_setting_values(&self, setting_values: Vec<(SettingId, Value)>) -> Result<()>;
}
