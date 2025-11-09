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
        structures::{Flag, GamingMode, TouchTone},
    },
    settings::SettingId,
};

use paste::paste;

use super::ModuleCollection;

macro_rules! flag {
    ($flag_struct:ty, $flag_configuration:expr $(,)?) => {
        impl<T> ModuleCollection<T>
        where
            T: Has<$flag_struct> + Send + Sync,
        {
            paste! {
                pub fn [<add_ $flag_struct:snake>] <C>(&mut self, packet_io: Arc<PacketIOController<C>>)
                where
                    C: RfcommConnection + 'static + Send + Sync,
                {
                    self.add_flag(packet_io, $flag_configuration);
                }
            }
        }
    };
}

flag!(
    TouchTone,
    FlagConfiguration {
        setting_id: SettingId::TouchTone,
        set_command: packet::outbound::SET_TOUCH_TONE_COMMAND,
        update_command: None,
    },
);

flag!(
    GamingMode,
    FlagConfiguration {
        setting_id: SettingId::GamingMode,
        set_command: packet::outbound::SET_GAMING_MODE_COMMAND,
        update_command: Some(packet::inbound::GAMING_MODE_UPDATE_COMMAND),
    },
);

impl<T> ModuleCollection<T> {
    fn add_flag<C, FlagT: Flag>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        flag_configuration: FlagConfiguration,
    ) where
        C: RfcommConnection + 'static + Send + Sync,
        T: Has<FlagT> + Send + Sync,
        FlagT: Send + Sync + PartialEq + Copy + 'static,
    {
        if let Some(update_command) = flag_configuration.update_command {
            self.packet_handlers.set_handler(
                update_command,
                Box::new(packet_handler::FlagPacketHandler::default()),
            );
        }
        self.setting_manager.add_handler(
            CategoryId::Miscellaneous,
            setting_handler::FlagSettingHandler::new(flag_configuration.setting_id),
        );
        self.state_modifiers
            .push(Box::new(state_modifier::FlagStateModifier::new(
                packet_io,
                flag_configuration.set_command,
            )));
    }
}

struct FlagConfiguration {
    pub setting_id: SettingId,
    pub set_command: packet::Command,
    pub update_command: Option<packet::Command>,
}
