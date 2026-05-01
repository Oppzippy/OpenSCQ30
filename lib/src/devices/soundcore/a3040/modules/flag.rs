use std::sync::Arc;

use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{
        a3040,
        common::{
            modules::{ModuleCollection, flag::FlagConfiguration},
            packet::{self, PacketIOController},
        },
    },
    settings::SettingId,
};

impl<T> ModuleCollection<T>
where
    T: Has<a3040::structures::LowBatteryPrompt> + Clone + Send + Sync,
{
    pub fn add_a3040_low_battery_prompt<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                setting_id: SettingId::LowBatteryPrompt,
                set_command: packet::Command([1, 175]),
                update_command: None,
            },
        );
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3040::structures::VoicePrompt> + Clone + Send + Sync,
{
    pub fn add_a3040_voice_prompt<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                setting_id: SettingId::VoicePrompt,
                set_command: packet::Command([1, 174]),
                update_command: None,
            },
        );
    }
}
