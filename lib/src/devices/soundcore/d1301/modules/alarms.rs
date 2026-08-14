mod setting_handler;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{common::modules::ModuleCollection, d1301},
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum AlarmsSetting {
        Alarms,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<Vec<d1301::structures::Alarm>> + Clone + Send + Sync,
{
    pub fn add_d1301_alarms(&mut self) {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            setting_handler::AlarmsSettingHandler,
        );
    }
}
