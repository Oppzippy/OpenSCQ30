use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Setting, SettingId, Value},
    },
    devices::soundcore::{
        a6611::{packets::outbound::set_find_device, structures::FindDevice},
        common::{
            modules::ModuleCollection,
            packet::PacketIOController,
            settings_manager::{SettingHandler, SettingHandlerResult},
            state_modifier::StateModifier,
        },
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum FindDeviceSetting {
        FindDeviceLeft,
        FindDeviceRight,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<FindDevice> + Clone + Send + Sync,
{
    pub fn add_a6611_find_device(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, FindDeviceSettingHandler);
        self.state_modifiers
            .push(Box::new(FindDeviceStateModifier { packet_io }));
    }
}

struct FindDeviceSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for FindDeviceSettingHandler
where
    T: Has<FindDevice> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        FindDeviceSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let find_device: &FindDevice = state.get();
        let value = match FindDeviceSetting::try_from(*setting_id).ok()? {
            FindDeviceSetting::FindDeviceLeft => find_device.left,
            FindDeviceSetting::FindDeviceRight => find_device.right,
        };
        Some(Setting::Toggle { value })
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let setting: FindDeviceSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        let enabled = value.try_as_bool()?;
        let find_device: &mut FindDevice = state.get_mut();
        match setting {
            FindDeviceSetting::FindDeviceLeft => find_device.left = enabled,
            FindDeviceSetting::FindDeviceRight => find_device.right = enabled,
        }
        Ok(())
    }
}

struct FindDeviceStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for FindDeviceStateModifier
where
    T: Has<FindDevice> + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let current: FindDevice = *state_sender.borrow().get();
        let target: FindDevice = *target_state.get();
        if current != target {
            self.packet_io
                .send_with_response(&set_find_device(target))
                .await?;
            state_sender.send_modify(|state| *state.get_mut() = target);
        }
        Ok(())
    }
}
