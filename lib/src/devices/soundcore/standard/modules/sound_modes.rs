mod packet_handler;
mod setting_handler;
mod state_modifier;

use std::sync::Arc;

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
};

use super::ModuleCollection;

#[derive(EnumString, EnumIter, IntoStaticStr)]
enum SoundModeSetting {
    AmbientSoundMode,
    TransparencyMode,
    NoiseCancelingMode,
    CustomNoiseCanceling,
}

impl From<SoundModeSetting> for SettingId {
    fn from(setting: SoundModeSetting) -> Self {
        match setting {
            SoundModeSetting::AmbientSoundMode => Self::AmbientSoundMode,
            SoundModeSetting::TransparencyMode => Self::TransparencyMode,
            SoundModeSetting::NoiseCancelingMode => Self::NoiseCancelingMode,
            SoundModeSetting::CustomNoiseCanceling => Self::CustomNoiseCanceling,
        }
    }
}

impl TryFrom<&SettingId> for SoundModeSetting {
    type Error = ();

    fn try_from(setting_id: &SettingId) -> Result<Self, Self::Error> {
        match setting_id {
            SettingId::AmbientSoundMode => Ok(Self::AmbientSoundMode),
            SettingId::TransparencyMode => Ok(Self::TransparencyMode),
            SettingId::NoiseCancelingMode => Ok(Self::NoiseCancelingMode),
            SettingId::CustomNoiseCanceling => Ok(Self::CustomNoiseCanceling),
            _ => Err(()),
        }
    }
}

pub struct AvailableSoundModes {
    pub ambient_sound_modes: Vec<AmbientSoundMode>,
    pub transparency_modes: Vec<TransparencyMode>,
    pub noise_canceling_modes: Vec<NoiseCancelingMode>,
}

impl<T> ModuleCollection<T>
where
    T: AsMut<SoundModes> + AsRef<SoundModes> + Clone + Send + Sync,
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
