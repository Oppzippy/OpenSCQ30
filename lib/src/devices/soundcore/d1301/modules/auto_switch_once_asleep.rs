mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        common::{modules::ModuleCollection, packet::PacketIOController},
        d1301,
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum AutoSwitchOnceAsleepSetting {
        AutoSwitchOnceAsleep,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<d1301::structures::AutoSwitchOnceAsleep>
        + Has<d1301::structures::AutoStopTimer>
        + Clone
        + Send
        + Sync,
{
    pub fn add_d1301_auto_switch_once_asleep(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            setting_handler::AutoSwitchOnceAsleepSettingHandler,
        );
        self.state_modifiers.push(Box::new(
            state_modifier::AutoSwitchOnceAsleepStateModifier::new(packet_io),
        ));
    }
}
