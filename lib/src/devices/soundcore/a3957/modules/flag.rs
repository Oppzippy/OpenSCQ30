use std::sync::Arc;

use crate::{
    devices::soundcore::{
        a3957::state::A3957State,
        common::{
            modules::{ModuleCollection, flag::FlagConfiguration},
            packet::{self, PacketIOController},
            structures::GamingMode,
        },
    },
    settings::{CategoryId, SettingId},
};

impl ModuleCollection<A3957State> {
    pub fn add_a3957_gaming_mode(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag::<GamingMode>(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::GamingMode,
                set_command: packet::Command([16, 133]),
                update_command: None,
                is_inverted: false,
            },
        );
    }
}
