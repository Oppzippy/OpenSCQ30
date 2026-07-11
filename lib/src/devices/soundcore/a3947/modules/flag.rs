use std::sync::Arc;

use openscq30_lib_has::Has;

use crate::{
    devices::soundcore::common::{
        modules::{ModuleCollection, flag::FlagConfiguration},
        packet::{self, PacketIOController},
        structures::GamingMode,
    },
    settings::{CategoryId, SettingId},
};

impl<T> ModuleCollection<T>
where
    T: Has<GamingMode> + Clone + Send + Sync,
{
    pub fn add_a3947_gaming_mode(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::GamingMode,
                set_command: packet::Command([0x10, 0x85]),
                update_command: None,
                is_inverted: false,
            },
        );
    }
}
