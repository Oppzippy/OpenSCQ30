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
    enum AutoStopTimerSetting {
        AutoStopTimer,
        AutoStopTimerDuration,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<d1301::structures::AutoStopTimer> + Clone + Send + Sync,
{
    pub fn add_d1301_auto_stop_timer(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            setting_handler::AutoStopTimerSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::AutoStopTimerStateModifier::new(
                packet_io,
            )));
    }
}
