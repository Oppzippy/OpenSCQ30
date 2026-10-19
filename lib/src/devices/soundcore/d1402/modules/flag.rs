use std::sync::Arc;

use openscq30_lib_has::Has;

use crate::{
    devices::soundcore::{
        common::{
            modules::{ModuleCollection, flag::FlagConfiguration},
            packet::{self, PacketIOController},
        },
        d1402,
    },
    settings::{CategoryId, SettingId},
};

impl<T> ModuleCollection<T>
where
    T: Has<d1402::structures::AmbientSoundModeVoicePrompt> + Clone + Send + Sync,
{
    pub fn add_d1402_ambient_sound_mode_voice_prompt(
        &mut self,
        packet_io: Arc<PacketIOController>,
    ) {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                category_id: CategoryId::Miscellaneous,
                setting_id: SettingId::VoicePrompt,
                set_command: packet::Command([1, 174]),
                update_command: None,
                is_inverted: false,
            },
        );
    }
}
