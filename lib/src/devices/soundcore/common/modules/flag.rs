mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;

use crate::{
    api::settings::CategoryId,
    connection::RfcommConnection,
    devices::soundcore::common::{
        packet::{self, PacketIOController},
        structures::{GamingMode, TouchTone},
    },
    settings::SettingId,
};

use super::ModuleCollection;

impl<T> ModuleCollection<T>
where
    T: Has<TouchTone> + Send + Sync,
{
    pub fn add_touch_tone<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                setting_id: SettingId::TouchTone,
                set_command: packet::outbound::SET_TOUCH_TONE_COMMAND,
                update_command: None,
                get_flag: |touch_tone| (*touch_tone).into(),
                set_flag: |touch_tone, is_enabled| *touch_tone = is_enabled.into(),
            },
        );
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<GamingMode> + Send + Sync,
{
    pub fn add_gaming_mode<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.add_flag(
            packet_io,
            FlagConfiguration {
                setting_id: SettingId::GamingMode,
                set_command: packet::outbound::SET_GAMING_MODE_COMMAND,
                update_command: Some(packet::inbound::GAMING_MODE_UPDATE_COMMAND),
                get_flag: |gaming_mode| gaming_mode.is_enabled,
                set_flag: |gaming_mode, is_enabled| gaming_mode.is_enabled = is_enabled,
            },
        );
    }
}

impl<T> ModuleCollection<T> {
    fn add_flag<C, Flag>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        flag_configuration: FlagConfiguration<Flag>,
    ) where
        C: RfcommConnection + 'static + Send + Sync,
        T: Has<Flag> + Send + Sync,
        Flag: Send + Sync + PartialEq + Copy + 'static,
    {
        if let Some(update_command) = flag_configuration.update_command {
            self.packet_handlers.set_handler(
                update_command,
                Box::new(packet_handler::FlagPacketHandler::new(
                    flag_configuration.get_flag,
                    flag_configuration.set_flag,
                )),
            );
        }
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            setting_handler::FlagSettingHandler::new(
                flag_configuration.setting_id,
                flag_configuration.get_flag,
                flag_configuration.set_flag,
            ),
        );
        self.state_modifiers
            .push(Box::new(state_modifier::FlagStateModifier::new(
                packet_io,
                flag_configuration.set_command,
                flag_configuration.get_flag,
            )));
    }
}

struct FlagConfiguration<Flag> {
    pub setting_id: SettingId,
    pub set_command: packet::Command,
    pub update_command: Option<packet::Command>,
    pub get_flag: for<'a> fn(&'a Flag) -> bool,
    pub set_flag: for<'a> fn(&'a mut Flag, bool),
}
