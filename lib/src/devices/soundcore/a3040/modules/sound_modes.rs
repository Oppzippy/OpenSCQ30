mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use openscq30_lib_has::Has;
use packet_handler::SoundModesPacketHandler;
use setting_handler::SoundModesSettingHandler;
use state_modifier::SoundModesStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{
        connection::RfcommConnection,
        settings::{CategoryId, SettingId},
    },
    devices::soundcore::{
        a3040::structures::SoundModes,
        common::{modules::ModuleCollection, packet::PacketIOController},
    },
    macros::enum_subset,
};

enum_subset! {
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SoundModeSetting {
        AmbientSoundMode,
        TransparencyMode,
        NoiseCancelingMode,
        ManualNoiseCanceling,
        WindNoiseSuppression,
        ManualTransparency,
    }
}

impl<T> ModuleCollection<T>
where
    T: Has<SoundModes> + Clone + Send + Sync,
{
    pub fn add_a3040_sound_modes<C>(&mut self, packet_io: Arc<PacketIOController<C>>)
    where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager
            .add_handler(CategoryId::SoundModes, SoundModesSettingHandler::default());
        self.state_modifiers
            .push(Box::new(SoundModesStateModifier::new(packet_io)));
        self.packet_handlers.set_handler(
            SoundModesPacketHandler::COMMAND,
            Box::new(SoundModesPacketHandler::default()),
        );
    }
}
