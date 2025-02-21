use std::sync::Arc;

use tokio::sync::{mpsc, watch};

use crate::{
    api::settings::{SettingId, Value},
    futures::{Futures, MaybeSend, MaybeSync},
    soundcore_device::device::Packet,
};

use super::{
    packet_manager::PacketManager, settings_manager::SettingsManager, state_modifier::StateModifier,
};

pub mod equalizer;
pub mod sound_modes;

pub struct ModuleCollection<StateType> {
    pub setting_manager: SettingsManager<StateType>,
    pub packet_handlers: PacketManager<StateType>,
    #[cfg(target_arch = "wasm32")]
    pub state_modifiers: Vec<Box<dyn StateModifier<StateType>>>,
    #[cfg(not(target_arch = "wasm32"))]
    pub state_modifiers: Vec<Box<dyn StateModifier<StateType> + Send + Sync>>,
}

impl<T> Default for ModuleCollection<T> {
    fn default() -> Self {
        Self {
            setting_manager: Default::default(),
            packet_handlers: Default::default(),
            state_modifiers: Default::default(),
        }
    }
}

impl<StateType> ModuleCollection<StateType>
where
    StateType: Clone,
{
    pub async fn set_setting_values(
        &self,
        state_sender: &watch::Sender<StateType>,
        setting_values: impl IntoIterator<Item = (SettingId<'_>, Value)>,
    ) -> crate::Result<()> {
        let mut target_state = state_sender.borrow().clone();
        for (setting_id, value) in setting_values {
            self.setting_manager
                .set(&mut target_state, &setting_id, value)
                .await
                .unwrap()?;
        }
        for modifier in &self.state_modifiers {
            modifier.move_to_state(state_sender, &target_state).await?;
        }
        Ok(())
    }
}

pub trait ModuleCollectionSpawnPacketHandlerExt<T> {
    fn spawn_packet_handler<F>(
        &self,
        state_sender: watch::Sender<T>,
        packet_receiver: mpsc::Receiver<Packet>,
    ) -> F::JoinHandleType
    where
        F: Futures + MaybeSend + MaybeSync,
        T: 'static + MaybeSend + MaybeSync;
}

impl<T> ModuleCollectionSpawnPacketHandlerExt<T> for Arc<ModuleCollection<T>> {
    fn spawn_packet_handler<F>(
        &self,
        state_sender: watch::Sender<T>,
        mut packet_receiver: mpsc::Receiver<Packet>,
    ) -> F::JoinHandleType
    where
        F: Futures + MaybeSend + MaybeSync,
        T: 'static + MaybeSend + MaybeSync,
    {
        let module_collection = self.clone();
        let state_sender = state_sender.clone();
        F::spawn(async move {
            while let Some(packet) = packet_receiver.recv().await {
                match module_collection
                    .packet_handlers
                    .handle(&state_sender, &packet)
                    .await
                {
                    Ok(()) => (),
                    Err(err) => {
                        tracing::warn!("error handling packet: {packet:?}, error: {err:?}")
                    }
                }
            }
        })
    }
}
