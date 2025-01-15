use std::sync::Arc;

use tokio::sync::{mpsc, watch};

use crate::{futures::Futures, soundcore_device::device::Packet};

use super::{
    packet_manager::PacketManager, settings_manager::SettingsManager, state_modifier::StateModifier,
};

pub mod sound_modes;

pub struct ModuleCollection<StateType> {
    pub setting_manager: SettingsManager<StateType>,
    pub packet_handlers: PacketManager<StateType>,
    pub state_modifiers: Vec<Box<dyn StateModifier<StateType>>>,
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

pub trait ModuleCollectionSpawnPacketHandlerExt<T> {
    fn spawn_packet_handler<F>(
        &self,
        state_sender: watch::Sender<T>,
        packet_receiver: mpsc::Receiver<Packet>,
    ) -> F::JoinHandleType
    where
        F: Futures,
        T: 'static;
}

impl<T> ModuleCollectionSpawnPacketHandlerExt<T> for Arc<ModuleCollection<T>> {
    fn spawn_packet_handler<F>(
        &self,
        state_sender: watch::Sender<T>,
        mut packet_receiver: mpsc::Receiver<Packet>,
    ) -> F::JoinHandleType
    where
        F: Futures,
        T: 'static,
    {
        let module_collection = self.clone();
        let state_sender = state_sender.clone();
        F::spawn_local(async move {
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
