mod packet_handler;
mod setting_handler;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::common::structures::TwsStatus,
    macros::enum_subset,
};

use super::ModuleCollection;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum TwsStatusSetting {
        TwsStatus,
        HostDevice,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<TwsStatus> + Clone + Send + Sync,
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
