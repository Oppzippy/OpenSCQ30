use std::sync::Arc;

use tokio::{
    select,
    sync::{Semaphore, mpsc, watch},
    task::JoinHandle,
};
use tracing::{Instrument, debug_span, trace};

use crate::{
    api::{
        device,
        settings::{SettingId, Value},
    },
    devices::soundcore::common::packet,
};

use super::{
    packet_manager::PacketManager, settings_manager::SettingsManager, state_modifier::StateModifier,
};

pub mod ambient_sound_mode_cycle;
mod auto_power_off;
pub mod button_configuration;
mod case_battery_level;
pub mod dual_battery;
pub mod equalizer;
mod flag;
mod limit_high_volume;
pub mod reset_button_configuration;
pub mod serial_number_and_dual_firmware_version;
pub mod serial_number_and_firmware_version;
pub mod single_battery;
mod single_battery_level;
pub mod sound_modes;
pub mod tws_status;

pub struct ModuleCollection<StateType> {
    pub setting_manager: SettingsManager<StateType>,
    pub packet_handlers: PacketManager<StateType>,
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
        setting_values: impl IntoIterator<Item = (SettingId, Value)>,
    ) -> device::Result<()> {
        let mut target_state = state_sender.borrow().clone();
        for (setting_id, value) in setting_values {
            self.setting_manager
                .set(&mut target_state, &setting_id, value)
                .await
                .unwrap()
                .map_err(|err| err.into_settings_error(setting_id))?;
        }
        for modifier in &self.state_modifiers {
            modifier.move_to_state(state_sender, &target_state).await?;
        }
        Ok(())
    }
}

pub trait ModuleCollectionSpawnPacketHandlerExt<T> {
    async fn spawn_packet_handler(
        &self,
        state_sender: watch::Sender<T>,
        packet_receiver: mpsc::Receiver<packet::Inbound>,
        exit_signal: Arc<Semaphore>,
    ) -> JoinHandle<()>
    where
        T: 'static + Send + Sync;
}

impl<T> ModuleCollectionSpawnPacketHandlerExt<T> for Arc<ModuleCollection<T>> {
    async fn spawn_packet_handler(
        &self,
        state_sender: watch::Sender<T>,
        mut packet_receiver: mpsc::Receiver<packet::Inbound>,
        exit_signal: Arc<Semaphore>,
    ) -> JoinHandle<()>
    where
        T: 'static + Send + Sync,
    {
        let module_collection = self.clone();
        let state_sender = state_sender.clone();
        while let Ok(packet) = packet_receiver.try_recv() {
            match module_collection
                .packet_handlers
                .handle(&state_sender, &packet)
                .await
            {
                Ok(()) => (),
                Err(err) => {
                    tracing::warn!("error handling packet: {packet:?}, error: {err:?}");
                }
            }
        }
        tokio::spawn(
            async move {
                trace!("started receiving");
                loop {
                    select! {
                        maybe_packet = packet_receiver.recv() => {
                            if let Some(packet) = maybe_packet {
                                match module_collection
                                    .packet_handlers
                                    .handle(&state_sender, &packet)
                                    .await
                                {
                                    Ok(()) => (),
                                    Err(err) => {
                                        tracing::warn!("error handling packet: {packet:?}, error: {err:?}");
                                    }
                                }
                            }
                        }
                        _ = exit_signal.acquire() => break,
                    }
                }
                trace!("done receiving");
            }
            .instrument(debug_span!(
                "ModuleCollectionSpawnPacketHandlerExt::spawn_packet_handler"
            )),
        )
    }
}
