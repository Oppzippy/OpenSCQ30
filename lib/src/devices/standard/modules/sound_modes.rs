mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

use packet_handler::SoundModesPacketHandler;
use setting_handler::SoundModesSettingHandler;
use state_modifier::SoundModesStateModifier;
use strum::{EnumIter, EnumString, IntoStaticStr};

use crate::{
    api::{connection::Connection, settings::CategoryId},
    devices::standard::structures::{
        AmbientSoundMode, NoiseCancelingMode, SoundModes, TransparencyMode,
    },
    futures::{Futures, MaybeSend, MaybeSync},
    soundcore_device::device::packet_io_controller::PacketIOController,
};

use super::ModuleCollection;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum SoundModeSetting {
    AmbientSoundMode,
    TransparencyMode,
    NoiseCancelingMode,
    CustomNoiseCanceling,
}

pub struct AvailableSoundModes {
    pub ambient_sound_modes: Vec<AmbientSoundMode>,
    pub transparency_modes: Vec<TransparencyMode>,
    pub noise_canceling_modes: Vec<NoiseCancelingMode>,
}

pub trait AddSoundModesExt {
    fn add_sound_modes<C, F>(
        &mut self,
        packet_io: Arc<PacketIOController<C, F>>,
        available_sound_modes: AvailableSoundModes,
    ) where
        C: Connection + 'static + MaybeSend + MaybeSync,
        F: Futures + 'static + MaybeSend + MaybeSync;
}

impl<T> AddSoundModesExt for ModuleCollection<T>
where
    T: AsMut<SoundModes> + AsRef<SoundModes> + Clone + MaybeSend + MaybeSync,
{
    fn add_sound_modes<C, F>(
        &mut self,
        packet_io: Arc<PacketIOController<C, F>>,
        available_sound_modes: AvailableSoundModes,
    ) where
        C: Connection + 'static + MaybeSend + MaybeSync,
        F: Futures + 'static + MaybeSend + MaybeSync,
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
