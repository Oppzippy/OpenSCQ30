use std::sync::Arc;

use openscq30_lib_has::Has;

use crate::{
    devices::soundcore::{
        a3062,
        common::{
            modules::{ModuleCollection, flag::FlagConfiguration},
            packet::{self, PacketIOController},
        },
    },
    settings::SettingId,
};

impl<T> ModuleCollection<T>
where
    T: Has<a3062::structures::DolbyAudio> + Clone + Send + Sync,
{
    pub fn add_a3062_dolby_audio(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                setting_id: SettingId::DolbyAudio,
                set_command: packet::Command([2, 134]),
                update_command: None,
            },
        );
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3062::structures::SideTone> + Clone + Send + Sync,
{
    pub fn add_a3062_side_tone(&mut self, packet_io: Arc<PacketIOController>) {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                setting_id: SettingId::SideTone,
                set_command: packet::Command([1, 132]),
                update_command: None,
            },
        );
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<a3062::structures::AmbientSoundModeVoicePrompt> + Clone + Send + Sync,
{
    pub fn add_a3062_ambient_sound_mode_voice_prompt(
        &mut self,
        packet_io: Arc<PacketIOController>,
    ) {
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
