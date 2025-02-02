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

use super::settings::{CategoryId, Setting, SettingId, Value};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait OpenSCQ30DeviceRegistry {
    async fn devices(&self) -> crate::Result<Vec<GenericDeviceDescriptor>>;
    async fn connect(&self, mac_address: MacAddr6) -> crate::Result<Arc<dyn OpenSCQ30Device>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait OpenSCQ30Device {
    async fn categories(&self) -> Vec<CategoryId>;
    async fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId>;
    async fn setting(&self, setting_id: &SettingId) -> crate::Result<Setting>;
    // async fn watch_for_changes(&self) -> broadcast::Receiver<()>;
    async fn set_setting_values(
        &self,
        setting_values: Vec<(SettingId<'_>, Value)>,
    ) -> crate::Result<()>;
}
