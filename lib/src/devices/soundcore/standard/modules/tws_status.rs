mod packet_handler;
mod setting_handler;

use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::standard::structures::TwsStatus,
};

use super::ModuleCollection;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum TwsStatusSetting {
    TwsStatus,
    HostDevice,
}

impl From<TwsStatusSetting> for SettingId {
    fn from(setting: TwsStatusSetting) -> Self {
        match setting {
            TwsStatusSetting::HostDevice => SettingId::HostDevice,
            TwsStatusSetting::TwsStatus => SettingId::TwsStatus,
        }
    }
}

impl TryFrom<&SettingId> for TwsStatusSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::HostDevice => Ok(Self::HostDevice),
            SettingId::TwsStatus => Ok(Self::TwsStatus),
            _ => Err(()),
        }
    }
}

impl<T> ModuleCollection<T>
where
    T: AsMut<TwsStatus> + AsRef<TwsStatus> + Clone + Send + Sync,
{
    pub fn add_tws_status(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::DeviceInformation,
            setting_handler::TwsStatusSettingHandler::default(),
        );
        self.packet_handlers.set_handler(
            packet_handler::TwsStatusPacketHandler::COMMAND,
            Box::new(packet_handler::TwsStatusPacketHandler::default()),
        );
    }
}
