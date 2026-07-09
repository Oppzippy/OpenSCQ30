use std::sync::Arc;

use openscq30_lib_has::Has;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::settings::{CategoryId, SettingId},
    devices::soundcore::{
        a3876,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

mod setting_handler;
mod state_modifier;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum VolumeBalanceSetting {
        VolumeBalance,
    }
);

impl<T> ModuleCollection<T>
where
    T: Has<a3876::structures::VolumeBalance> + Clone + Send + Sync,
{
    pub fn add_a3876_volume_balance(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            setting_handler::VolumeBalanceSettingHandler,
        );
        self.state_modifiers
            .push(Box::new(state_modifier::VolumeBalanceStateModifier::new(
                packet_io,
            )));
    }
}
