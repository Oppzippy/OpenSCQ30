use std::sync::Arc;

use setting_handler::EqualizerSettingHandler;
use state_modifier::EqualizerStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::Connection,
        settings::{CategoryId, SettingId},
    },
    devices::standard::structures::EqualizerConfiguration,
    soundcore_device::{
        device::packet_io_controller::PacketIOController, device_model::DeviceModel,
    },
    storage::OpenSCQ30Database,
};

use super::ModuleCollection;

mod setting_handler;
mod state_modifier;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum EqualizerSetting {
    PresetProfile,
    CustomProfile,
    VolumeAdjustments,
}

impl TryFrom<&SettingId> for EqualizerSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::PresetProfile => Ok(Self::PresetProfile),
            SettingId::CustomProfile => Ok(Self::CustomProfile),
            SettingId::VolumeAdjustments => Ok(Self::VolumeAdjustments),
            _ => Err(()),
        }
    }
}

impl From<EqualizerSetting> for SettingId {
    fn from(setting: EqualizerSetting) -> Self {
        match setting {
            EqualizerSetting::PresetProfile => SettingId::PresetProfile,
            EqualizerSetting::CustomProfile => SettingId::CustomProfile,
            EqualizerSetting::VolumeAdjustments => SettingId::VolumeAdjustments,
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration> + Clone + Send + Sync,
{
    pub async fn add_equalizer<C>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        is_stereo: bool,
    ) where
        C: Connection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::Equalizer,
            EqualizerSettingHandler::new(database, device_model).await,
        );
        self.state_modifiers
            .push(Box::new(EqualizerStateModifier::new(packet_io, is_stereo)));
    }
}
