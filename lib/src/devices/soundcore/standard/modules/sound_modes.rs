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
    devices::soundcore::standard::{
        packet::packet_io_controller::PacketIOController,
        structures::{AmbientSoundMode, NoiseCancelingMode, SoundModes, TransparencyMode},
    },
    macros::enum_subset,
};

use super::ModuleCollection;

enum_subset!(
    SettingId,
    #[derive(EnumString, EnumIter, IntoStaticStr)]
    enum SoundModeSetting {
        AmbientSoundMode,
        TransparencyMode,
        NoiseCancelingMode,
        CustomNoiseCanceling,
    }
);

pub struct AvailableSoundModes {
    pub ambient_sound_modes: Vec<AmbientSoundMode>,
    pub transparency_modes: Vec<TransparencyMode>,
    pub noise_canceling_modes: Vec<NoiseCancelingMode>,
}

impl<T> ModuleCollection<T>
where
    T: Has<SoundModes> + Clone + Send + Sync,
{
    pub fn add_sound_modes<C>(
        &mut self,
        packet_io: Arc<PacketIOController<C>>,
        available_sound_modes: AvailableSoundModes,
    ) where
        C: RfcommConnection + 'static + Send + Sync,
    {
        self.setting_manager.add_handler(
            CategoryId::SoundModes,
            SoundModesSettingHandler::new(available_sound_modes),
        );
        self.state_modifiers
            .push(Box::new(SoundModesStateModifier::new(packet_io)));
        self.packet_handlers.set_handler(
            SoundModesPacketHandler::COMMAND,
            Box::new(SoundModesPacketHandler::default()),
        );
    }
}
