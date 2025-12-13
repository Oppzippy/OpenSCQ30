use std::sync::Arc;

use openscq30_lib_has::Has;

use crate::{
    connection::RfcommConnection,
    devices::soundcore::{
        a3035,
        common::{
            modules::{ModuleCollection, flag::FlagConfiguration},
            packet::{self, PacketIOController},
        },
    },
    settings::SettingId,
};

impl<T> ModuleCollection<T>
where
    T: Has<a3035::structures::BatteryAlert> + Clone + Send + Sync,
{
    pub fn add_a3035_battery_alert<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
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
    T: Has<a3035::structures::AmbientSoundModeVoicePrompt> + Clone + Send + Sync,
{
    pub fn add_a3035_ambient_sound_mode_voice_prompt<C>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
    ) where
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
