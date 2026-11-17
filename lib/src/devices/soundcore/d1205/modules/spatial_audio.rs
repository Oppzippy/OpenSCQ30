use std::sync::Arc;

use async_trait::async_trait;
use openscq30_lib_has::Has;
use tokio::sync::watch;

use crate::{
    api::{
        device,
        settings::{CategoryId, Setting, SettingId, Value},
    },
    devices::soundcore::{
        common::{
            modules::ModuleCollection,
            packet::PacketIOController,
            settings_manager::{SettingHandler, SettingHandlerResult},
            state_modifier::StateModifier,
        },
        d1205::{packets::outbound::set_spatial_audio, structures::SpatialAudio},
    },
};

impl<T> ModuleCollection<T>
where
    T: Has<SpatialAudio> + Clone + Send + Sync,
{
    pub fn add_d1205_spatial_audio(&mut self, packet_io: Arc<PacketIOController>) {
        self.setting_manager
            .add_handler(CategoryId::Miscellaneous, SpatialAudioSettingHandler);
        self.state_modifiers
            .push(Box::new(SpatialAudioStateModifier { packet_io }));
    }
}

struct SpatialAudioSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for SpatialAudioSettingHandler
where
    T: Has<SpatialAudio> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        vec![SettingId::SpatialAudio]
    }

    fn get(&self, state: &T, _setting_id: &SettingId) -> Option<Setting> {
        let spatial_audio: &SpatialAudio = state.get();
        Some(Setting::Toggle {
            value: spatial_audio.is_enabled,
        })
    }

    async fn set(
        &self,
        state: &mut T,
        _setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let spatial_audio: &mut SpatialAudio = state.get_mut();
        spatial_audio.is_enabled = value.try_as_bool()?;
        Ok(())
    }
}

struct SpatialAudioStateModifier {
    packet_io: Arc<PacketIOController>,
}

#[async_trait]
impl<T> StateModifier<T> for SpatialAudioStateModifier
where
    T: Has<SpatialAudio> + Clone + Send + Sync,
{
    async fn move_to_state(
        &self,
        state_sender: &watch::Sender<T>,
        target_state: &T,
    ) -> device::Result<()> {
        let target: SpatialAudio = *target_state.get();
        if *state_sender.borrow().get() == target {
            return Ok(());
        }
        // Unlike the D1202, the Soundcore app disables spatial audio on this device with the spatial audio packet
        self.packet_io
            .send_with_response(&set_spatial_audio(&target))
            .await?;
        state_sender.send_modify(|state| *state.get_mut() = target);
        Ok(())
    }
}
