mod device;
mod device_descriptor;
mod device_registry;
mod generic_device_descriptor;

use std::sync::Arc;

use async_trait::async_trait;
pub use device::*;
pub use device_descriptor::*;
pub use device_registry::*;
pub use generic_device_descriptor::*;
use macaddr::MacAddr6;

use crate::soundcore_device::device_model::DeviceModel;

use super::settings::{CategoryId, Setting, SettingId, Value};

#[async_trait]
pub trait OpenSCQ30DeviceRegistry {
    async fn devices(&self) -> crate::Result<Vec<GenericDeviceDescriptor>>;
    async fn connect(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Arc<dyn OpenSCQ30Device + Send + Sync>>;
}

#[async_trait]
pub trait OpenSCQ30Device {
    fn model(&self) -> DeviceModel;
    fn categories(&self) -> Vec<CategoryId>;
    fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId>;
    fn setting(&self, setting_id: &SettingId) -> Option<Setting>;
    // async fn watch_for_changes(&self) -> broadcast::Receiver<()>;
    async fn set_setting_values(
        &self,
        setting_values: Vec<(SettingId, Value)>,
    ) -> crate::Result<()>;
}
